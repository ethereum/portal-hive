use crate::types::{SuiteID, TestID, TestResult};
use crate::Simulation;
use ::std::{boxed::Box, future::Future, pin::Pin};
use async_trait::async_trait;
use core::fmt::Debug;
use dyn_clone::DynClone;
use jsonrpc::simple_http::SimpleHttpTransport;
use std::net::IpAddr;
use std::sync::{Arc, RwLock};

use crate::utils::client_test_name;

pub type AsyncClientTestFunc = fn(
    Test,
    Client,
) -> Pin<
    Box<
        dyn Future<Output = ()> // future API / pollable
            + Send // required by non-single-threaded executors
            + 'static,
    >,
>;

pub type AsyncTestFunc = fn(
    &mut Test,
    Option<Client>,
) -> Pin<
    Box<
        dyn Future<Output = ()> // future API / pollable
            + Send // required by non-single-threaded executors
            + '_,
    >,
>;

#[async_trait]
pub trait Testable: DynClone + Send + Sync {
    async fn run_test(&self, simulation: Simulation, suite_id: SuiteID, suite: Suite);
}

impl Debug for dyn Testable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Testable")
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

/// Represents a running client.
#[derive(Debug, Clone)]
pub struct Client {
    pub kind: String,
    pub container: String,
    pub ip: IpAddr,
    pub rpc: Arc<RwLock<jsonrpc::Client>>,
    pub test: Test,
}

#[derive(Clone, Debug)]
pub struct TestRun {
    pub suite_id: SuiteID,
    pub suite: Suite,
    pub name: String,
    pub desc: String,
    pub always_run: bool,
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

async fn run_client_test(
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
    pub run: AsyncTestFunc,
    pub client: Option<Client>,
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

pub async fn run_test(
    host: Simulation,
    test: TestRun,
    client: Option<Client>,
    func: AsyncTestFunc,
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
    (func)(test, client).await;

    host.end_test(test.suite_id, test_id, test.result.clone())
        .await;
}
