use crate::types::{ClientDefinition, StartNodeResponse, SuiteID, TestID, TestRequest, TestResult};
use crate::TestMatcher;
use std::collections::HashMap;
use std::env;
use std::net::IpAddr;
use std::str::FromStr;

/// Wraps the simulation HTTP API provided by hive.
#[derive(Clone, Debug)]
pub struct Simulation {
    pub url: String,
    pub m: TestMatcher,
}

impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}

// A struct in the structure of the JSON config shown in simulators.md
// it is used to pass information to the Hive Simulators
#[derive(serde::Serialize, serde::Deserialize)]
struct SimulatorConfig {
    client: String,
    environment: HashMap<String, String>,
}

impl SimulatorConfig {
    pub fn new() -> Self {
        Self {
            client: "".to_string(),
            environment: Default::default(),
        }
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

        match client.post(url).json(&body).send().await {
            Ok(response) => match response.json::<SuiteID>().await {
                Ok(json) => json,
                Err(err) => panic!("Failed to convert response to json: {}", err),
            },
            Err(err) => panic!("Failed to send start suite request: {}", err),
        }
    }

    pub async fn end_suite(&self, test_suite: SuiteID) {
        let url = format!("{}/testsuite/{}", self.url, test_suite);
        let client = reqwest::Client::new();
        match client.delete(url).send().await {
            Ok(_) => (),
            Err(err) => panic!("Failed to send a end suite request: {}", err),
        }
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

        match client.post(url).json(&body).send().await {
            Ok(response) => match response.json::<TestID>().await {
                Ok(test_id) => test_id,
                Err(err) => panic!("Failed to convert response to json: {}", err),
            },
            Err(err) => panic!("Failed to send start test request: {}", err),
        }
    }

    /// Finishes the test case, cleaning up everything, logging results, and returning
    /// an error if the process could not be completed.
    pub async fn end_test(&self, test_suite: SuiteID, test: TestID, test_result: TestResult) {
        let url = format!("{}/testsuite/{}/test/{}", self.url, test_suite, test);
        let client = reqwest::Client::new();

        match client.post(url).json(&test_result).send().await {
            Ok(_) => (),
            Err(err) => panic!("Failed to send end test request: {}", err),
        }
    }

    /// Starts a new node (or other container).
    /// Returns container id and ip.
    pub async fn start_client(
        &self,
        test_suite: SuiteID,
        test: TestID,
        client_type: String,
        private_key: Option<&String>,
    ) -> (String, IpAddr) {
        let url = format!("{}/testsuite/{}/test/{}/node", self.url, test_suite, test);
        let client = reqwest::Client::new();

        let mut config = SimulatorConfig::new();
        config.client = client_type;
        if let Some(private_key) = private_key {
            config.environment.insert(
                "HIVE_CLIENT_PRIVATE_KEY".to_string(),
                private_key.to_string(),
            );
        }

        let config = match serde_json::to_string(&config) {
            Ok(response) => response,
            Err(err) => panic!("Failed to parse config to serde_json: {}", err),
        };
        let form = reqwest::multipart::Form::new().text("config", config);

        let resp = match client.post(url).multipart(form).send().await {
            Ok(response) => match response.json::<StartNodeResponse>().await {
                Ok(json) => json,
                Err(err) => panic!("Failed to convert response to json: {}", err),
            },
            Err(err) => panic!("Failed to send start client request: {}", err),
        };

        let ip = match IpAddr::from_str(&resp.ip) {
            Ok(ip) => ip,
            Err(err) => panic!("Failed to send start suite request: {}", err),
        };

        (resp.id, ip)
    }

    /// Returns all client types available to this simulator run. This depends on
    /// both the available client set and the command line filters.
    pub async fn client_types(&self) -> Vec<ClientDefinition> {
        let url = format!("{}/clients", self.url);
        let client = reqwest::Client::new();
        match client.get(&url).send().await {
            Ok(response) => match response.json::<Vec<ClientDefinition>>().await {
                Ok(client_types) => client_types,
                Err(err) => panic!("Failed to convert response to json: {}", err),
            },
            Err(err) => panic!("Failed to send get client types request: {}", err),
        }
    }
}
