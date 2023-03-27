#![allow(dead_code)]
use ::std::{boxed::Box, future::Future, pin::Pin};
use async_trait::async_trait;
use core::fmt::Debug;
use dyn_clone::DynClone;
use jsonrpc::simple_http::SimpleHttpTransport;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

#[macro_export]
macro_rules! dyn_async {(
    $( #[$attr:meta] )* // includes doc strings
    $pub:vis
    async
    fn $fname:ident<$lt:lifetime> ( $($args:tt)* ) $(-> $Ret:ty)?
    {
        $($body:tt)*
    }
) => (
    $( #[$attr] )*
    #[allow(unused_parens)]
    $pub
    fn $fname<$lt> ( $($args)* ) -> ::std::pin::Pin<::std::boxed::Box<
        dyn $lt + Send + ::std::future::Future<Output = ($($Ret)?)>
    >>
    {
        Box::pin(async move { $($body)* })
    }
)}

type AsyncClientTestFunc = fn(
    Test,
    Client,
) -> Pin<
    Box<
        dyn Future<Output = ()> // future API / pollable
            + Send // required by non-single-threaded executors
            + 'static,
    >,
>;

type SuiteID = u32;
type TestID = u32;

/// Represents a running client.
#[derive(Debug, Clone)]
pub struct Client {
    pub kind: String,
    pub container: String,
    pub ip: IpAddr,
    pub rpc: Arc<RwLock<jsonrpc::Client>>,
    pub test: Test,
}

/// StartNodeReponse is returned by the client startup endpoint.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StartNodeResponse {
    pub id: String, // Container ID.
    pub ip: String, // IP address in bridge network
}

// ClientMetadata is part of the ClientDefinition and lists metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientMetadata {
    roles: Vec<String>,
}

// ClientDefinition is served by the /clients API endpoint to list the available clients
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientDefinition {
    name: String,
    version: String,
    meta: ClientMetadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfoResponse {
    pub enr: String,     // Container ID.
    pub node_id: String, // IP address in bridge network
    pub ip: Option<String>,
}

/// A running test
#[derive(Clone, Debug)]
pub struct Test {
    sim: Simulation,
    test_id: TestID,
    suite: Suite,
    suite_id: SuiteID,
    result: TestResult,
}

impl Test {
    pub async fn start_client(&self, client_type: String) -> Client {
        let (container, ip) = self
            .sim
            .start_client(self.suite_id, self.test_id, client_type.clone())
            .await;

        let rpc_url = format!("http://{}:8545", ip);

        let transport = SimpleHttpTransport::builder()
            .url(&rpc_url)
            .unwrap()
            .build();

        let rpc_client = jsonrpc::Client::with_transport(transport);

        Client {
            kind: client_type,
            container,
            ip,
            rpc: Arc::new(RwLock::new(rpc_client)),
            test: Test {
                sim: self.sim.clone(),
                test_id: self.test_id,
                suite: self.suite.clone(),
                suite_id: self.suite_id,
                result: self.result.clone(),
            },
        }
    }

    /// Runs a subtest of this test.
    pub async fn run(&self, spec: TestSpec) {
        spec.run_test(self.sim.clone(), self.suite_id, self.suite.clone())
            .await
    }

    pub fn fatal(&mut self, msg: &str) {
        self.log_failure(msg);
        self.fail();
    }

    /// Prints to standard output, which goes to the simulation log file.
    fn log_failure(&mut self, msg: &str) {
        println!("{msg}");
        self.result.details = msg.to_owned()
    }

    // Fail signals that the test has failed.
    fn fail(&mut self) {
        self.result.pass = false
    }
}

dyn_clone::clone_trait_object!(Testable);
/// Description of a test suite
#[derive(Clone, Debug)]
pub struct Suite {
    pub name: String,
    pub description: String,
    pub tests: Vec<Box<dyn Testable>>,
}

impl Suite {
    pub fn add<T: Testable + 'static>(&mut self, test: T) {
        self.tests.push(Box::new(test))
    }
}

#[async_trait]
pub trait Testable: DynClone + Send + Sync {
    async fn run_test(&self, simulation: Simulation, suite_id: SuiteID, suite: Suite);
}

impl Debug for dyn Testable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// ClientTestSpec is a test against a single client.
/// When used as a test in a suite, the test runs against all available client types.
#[derive(Clone, Debug)]
pub struct ClientTestSpec {
    // These fields are displayed in the UI. Be sure to add
    // a meaningful description here.
    pub name: String,
    // If AlwaysRun is true, the test will run even if Name does not match the test
    // pattern. This option is useful for tests that launch a client instance and
    // then perform further tests against it.
    pub description: String,
    // If AlwaysRun is true, the test will run even if Name does not match the test
    // pattern. This option is useful for tests that launch a client instance and
    // then perform further tests against it.
    pub always_run: bool,
    // The Run function is invoked when the test executes.
    pub run: AsyncClientTestFunc,
}

#[async_trait]
impl Testable for ClientTestSpec {
    async fn run_test(&self, simulation: Simulation, suite_id: SuiteID, suite: Suite) {
        let clients = simulation.client_types().await;

        for client in clients {
            let client_name = client.name;
            let test_run = TestRun {
                suite_id,
                suite: suite.clone(),
                name: client_test_name(self.name.clone(), client_name.clone()),
                desc: self.description.clone(),
                always_run: self.always_run,
            };

            run_client_test(simulation.clone(), test_run, client_name, self.run).await;
        }
    }
}

#[derive(Clone)]
pub struct TestSpec {
    // These fields are displayed in the UI. Be sure to add
    // a meaningful description here.
    pub name: String,
    pub description: String,
    // If AlwaysRun is true, the test will run even if Name does not match the test
    // pattern. This option is useful for tests that launch a client instance and
    // then perform further tests against it.
    pub always_run: bool,
    // The Run function is invoked when the test executes.
    pub run: fn(&mut Test, Option<Client>),
    pub client: Option<Client>,
}

#[derive(Clone, Debug)]
pub struct TestRun {
    pub suite_id: SuiteID,
    pub suite: Suite,
    pub name: String,
    pub desc: String,
    pub always_run: bool,
}

#[async_trait]
impl Testable for TestSpec {
    async fn run_test(&self, simulation: Simulation, suite_id: SuiteID, suite: Suite) {
        let test_run = TestRun {
            suite_id,
            suite,
            name: self.name.to_owned(),
            desc: self.description.to_owned(),
            always_run: self.always_run,
        };

        run_test(simulation, test_run, self.client.clone(), self.run).await;
    }
}

#[derive(Clone, Debug)]
pub struct TestMatcher {
    suite: String,
    test: String,
    pattern: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestRequest {
    name: String,
    description: String,
}

/// Wraps the simulation HTTP API provided by hive.
#[derive(Clone, Debug)]
pub struct Simulation {
    url: String,
    m: TestMatcher,
}

impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}

impl Simulation {
    /// New looks up the hive host URI using the HIVE_SIMULATOR environment variable
    /// and connects to it. It will panic if HIVE_SIMULATOR is not set.
    pub fn new() -> Self {
        let url = env::var("HIVE_SIMULATOR").expect("HIVE_SIMULATOR environment variable not set");

        if url.is_empty() {
            panic!("HIVE_SIMULATOR environment variable is empty")
        }

        // TODO: Handle test matcher pattern

        Self {
            url,
            m: TestMatcher {
                suite: "".to_string(),
                test: "".to_string(),
                pattern: "".to_string(),
            },
        }
    }

    pub async fn start_suite(
        &self,
        name: String,
        description: String,
        _sim_log: String,
    ) -> SuiteID {
        let url = format!("{}/testsuite", self.url);
        let client = reqwest::Client::new();
        let body = TestRequest { name, description };
        let res = client
            .post(url)
            .json(&body)
            .send()
            .await
            .unwrap()
            .json::<SuiteID>()
            .await
            .unwrap();

        res
    }

    pub async fn end_suite(&self, test_suite: SuiteID) {
        let url = format!("{}/testsuite/{}", self.url, test_suite);
        let client = reqwest::Client::new();
        client.delete(url).send().await.unwrap();
    }

    /// Starts a new test case, returning the testcase id as a context identifier
    pub async fn start_test(
        &self,
        test_suite: SuiteID,
        name: String,
        description: String,
    ) -> TestID {
        let url = format!("{}/testsuite/{}/test", self.url, test_suite);
        let client = reqwest::Client::new();
        let body = TestRequest { name, description };

        let res = client
            .post(url)
            .json(&body)
            .send()
            .await
            .unwrap()
            .json::<TestID>()
            .await
            .unwrap();

        res
    }

    /// Finishes the test case, cleaning up everything, logging results, and returning
    /// an error if the process could not be completed.
    pub async fn end_test(&self, test_suite: SuiteID, test: TestID, test_result: TestResult) {
        let url = format!("{}/testsuite/{}/test/{}", self.url, test_suite, test);
        let client = reqwest::Client::new();

        client.post(url).json(&test_result).send().await.unwrap();
    }

    /// Starts a new node (or other container).
    /// Returns container id and ip.
    pub async fn start_client(
        &self,
        test_suite: SuiteID,
        test: TestID,
        client_type: String,
    ) -> (String, IpAddr) {
        let url = format!("{}/testsuite/{}/test/{}/node", self.url, test_suite, test);
        let client = reqwest::Client::new();

        let mut config = HashMap::new();
        config.insert("client", client_type);

        let config = serde_json::to_string(&config).unwrap();
        let form = reqwest::multipart::Form::new().text("config", config);

        let resp = client
            .post(url)
            .multipart(form)
            .send()
            .await
            .unwrap()
            .json::<StartNodeResponse>()
            .await
            .unwrap();

        let ip = IpAddr::from_str(&resp.ip).unwrap();

        (resp.id, ip)
    }

    pub async fn client_types(&self) -> Vec<ClientDefinition> {
        let url = format!("{}/clients", self.url);
        let client = reqwest::Client::new();
        client
            .get(&url)
            .send()
            .await
            .unwrap()
            .json::<Vec<ClientDefinition>>()
            .await
            .unwrap()
    }
}

/// Describes the outcome of a test.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TestResult {
    pass: bool,
    details: String,
}

/// Ensures that 'name' contains the client type.
pub fn client_test_name(name: String, client_type: String) -> String {
    if name.is_empty() {
        return client_type;
    }
    if name.contains("CLIENT") {
        return name.replace("CLIENT", &client_type);
    }
    format!("{} ({})", name, client_type)
}

pub async fn run_test(
    host: Simulation,
    test: TestRun,
    client: Option<Client>,
    f: fn(&mut Test, Option<Client>),
) {
    // Register test on simulation server and initialize the T.
    let test_id = host.start_test(test.suite_id, test.name, test.desc).await;

    let mut test = &mut Test {
        sim: host.clone(),
        test_id,
        suite: test.suite,
        suite_id: test.suite_id,
        result: Default::default(),
    };

    test.result.pass = true;

    // run test function
    f(test, client);

    host.end_test(test.suite_id, test_id, test.result.clone())
        .await;
}

pub async fn run_client_test(
    host: Simulation,
    test: TestRun,
    client_name: String,
    func: AsyncClientTestFunc,
) {
    // Register test on simulation server and initialize the Test.
    let test_id = host.start_test(test.suite_id, test.name, test.desc).await;

    let mut test = Test {
        sim: host.clone(),
        test_id,
        suite: test.suite,
        suite_id: test.suite_id,
        result: Default::default(),
    };

    test.result.pass = true;

    // run test function
    let client = test.start_client(client_name).await;

    (func)(test.clone(), client).await;

    host.end_test(test.suite_id, test_id, test.result).await;
}
