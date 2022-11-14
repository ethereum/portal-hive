use serde::{Deserialize, Serialize};
use std::env;

type SuiteID = u32;
type TestID = u32;

#[derive(Clone, Debug)]
pub struct Test {
    sim: Simulation,
    test_id: TestID,
    suite: Suite,
    suite_id: SuiteID,
    result: TestResult,
}

/// Description of a test suite
#[derive(Clone, Debug)]
pub struct Suite {
    pub name: String,
    pub description: String,
    pub tests: Vec<TestSpec>,
}

impl Suite {
    pub fn add(&mut self, test: TestSpec) {
        self.tests.push(test)
    }
}

#[derive(Clone, Debug)]
pub struct TestSpec {
    // These fields are displayed in the UI. Be sure to add
    // a meaningful description here.
    pub name: String,
    pub description: String,
    // If AlwaysRun is true, the test will run even if Name does not match the test
    // pattern. This option is useful for tests that launch a client instance and
    // then perform further tests against it.
    pub always_run: bool,
    pub run: fn(Test),
}

#[derive(Clone, Debug)]
pub struct TestRun {
    pub suite_id: SuiteID,
    pub suite: Suite,
    pub name: String,
    pub desc: String,
    pub always_run: bool,
}

impl TestSpec {
    pub fn run_test(&self, simulation: Simulation, suite_id: SuiteID, suite: Suite) {
        let test_run = TestRun {
            suite_id,
            suite,
            name: self.name.to_owned(),
            desc: self.description.to_owned(),
            always_run: self.always_run,
        };

        run_test(simulation, test_run, self.run);
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
}

#[derive(Clone, Debug, Default)]
pub struct TestResult {
    pass: bool,
    details: String,
}

pub fn run_test(host: Simulation, test: TestRun, f: fn(Test)) {
    let test = Test {
        sim: host,
        test_id: Default::default(),
        suite: test.suite,
        suite_id: test.suite_id,
        result: Default::default(),
    };

    f(test);
}
