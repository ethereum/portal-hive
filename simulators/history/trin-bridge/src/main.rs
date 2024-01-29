mod constants;

use crate::constants::{
    BOOTNODES_ENVIRONMENT_VARIABLE, HIVE_CHECK_LIVE_PORT, TEST_DATA_FILE_PATH,
    TRIN_BRIDGE_CLIENT_TYPE,
};
use ethportal_api::HistoryContentKey;
use ethportal_api::HistoryContentValue;
use ethportal_api::PossibleHistoryContentValue;
use ethportal_api::{Discv5ApiClient, HistoryNetworkApiClient};
use hivesim::types::ClientDefinition;
use hivesim::{dyn_async, Client, NClientTestSpec, Simulation, Suite, Test, TestSpec};
use itertools::Itertools;
use portal_spec_test_utils_rs::get_flair;
use serde_yaml::Value;
use std::collections::HashMap;
use tokio::time::Duration;

fn process_content(content: Vec<(HistoryContentKey, HistoryContentValue)>) -> Vec<String> {
    let mut last_header = content.first().unwrap().clone();

    let mut result: Vec<String> = vec![];
    for history_content in content.into_iter() {
        if let HistoryContentKey::BlockHeaderWithProof(_) = &history_content.0 {
            last_header = history_content.clone();
        }
        let comment =
            if let HistoryContentValue::BlockHeaderWithProof(header_with_proof) = &last_header.1 {
                let content_type = match &history_content.0 {
                    HistoryContentKey::BlockHeaderWithProof(_) => "header".to_string(),
                    HistoryContentKey::BlockBody(_) => "body".to_string(),
                    HistoryContentKey::BlockReceipts(_) => "receipt".to_string(),
                    HistoryContentKey::EpochAccumulator(_) => "epoch accumulator".to_string(),
                };
                format!(
                    "{}{} {}",
                    header_with_proof.header.number,
                    get_flair(header_with_proof.header.number),
                    content_type
                )
            } else {
                unreachable!("History test dated is formatted incorrectly")
            };
        result.push(comment)
    }
    result
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut suite = Suite {
        name: "trin-bridge-tests".to_string(),
        description: "The portal bridge test suite".to_string(),
        tests: vec![],
    };

    suite.add(TestSpec {
        name: "Trin bridge tests".to_string(),
        description: "".to_string(),
        always_run: false,
        run: test_portal_bridge,
        client: None,
    });

    let sim = Simulation::new();
    run_suite(sim, suite).await;
}

async fn run_suite(host: Simulation, suite: Suite) {
    let name = suite.clone().name;
    let description = suite.clone().description;

    let suite_id = host.start_suite(name, description, "".to_string()).await;

    for test in &suite.tests {
        test.run_test(host.clone(), suite_id, suite.clone()).await;
    }

    host.end_suite(suite_id).await;
}

dyn_async! {
   async fn test_portal_bridge<'a> (test: &'a mut Test, _client: Option<Client>) {
        // Get all available portal clients
        let clients = test.sim.client_types().await;
        if !clients.iter().any(|client_definition| client_definition.name == *TRIN_BRIDGE_CLIENT_TYPE) {
            panic!("This simulator is required to be ran with client `trin-bridge`")
        }
        let clients: Vec<ClientDefinition> = clients.into_iter().filter(|client| client.name != *TRIN_BRIDGE_CLIENT_TYPE).collect();

        // Iterate over all possible pairings of clients and run the tests (including self-pairings)
        for (client_a, client_b) in clients.iter().cartesian_product(clients.iter()) {
            test.run(
                NClientTestSpec {
                    name: format!("Bridge test. A:{} --> B:{}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_bridge,
                    environments: None,
                    test_data: None,
                    clients: vec![client_a.clone(), client_b.clone()],
                }
            ).await;
        }
   }
}

dyn_async! {
    async fn test_bridge<'a>(clients: Vec<Client>, _: Option<Vec<(String, String)>>) {
        let (client_a, client_b) = match clients.iter().collect_tuple() {
            Some((client_a, client_b)) => (client_a, client_b),
            None => {
                panic!("Unable to get expected amount of clients from NClientTestSpec");
            }
        };

        let client_b_enr = match client_b.rpc.node_info().await {
            Ok(node_info) => node_info.enr,
            Err(err) => {
                panic!("Error getting node info: {err:?}");
            }
        };
        match HistoryNetworkApiClient::add_enr(&client_a.rpc, client_b_enr.clone()).await {
            Ok(response) => if !response {
                panic!("AddEnr expected to get true and instead got false")
            },
            Err(err) => panic!("{}", &err.to_string()),
        }

        let client_a_enr = match client_a.rpc.node_info().await {
            Ok(node_info) => node_info.enr,
            Err(err) => {
                panic!("Error getting node info: {err:?}");
            }
        };
        client_a.test.start_client(TRIN_BRIDGE_CLIENT_TYPE.to_string(), Some(HashMap::from([(BOOTNODES_ENVIRONMENT_VARIABLE.to_string(), client_a_enr.to_base64()), (HIVE_CHECK_LIVE_PORT.to_string(), 0.to_string())]))).await;



        // With default node settings nodes should be storing all content
        let values = std::fs::read_to_string(TEST_DATA_FILE_PATH)
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let content_vec: Vec<(HistoryContentKey, HistoryContentValue)> = values.as_sequence().unwrap().iter().map(|value| {
            let content_key: HistoryContentKey =
                serde_yaml::from_value(value.get("content_key").unwrap().clone()).unwrap();
            let content_value: HistoryContentValue =
                serde_yaml::from_value(value.get("content_value").unwrap().clone()).unwrap();
            (content_key, content_value)
        }).collect();
        let comments = process_content(content_vec.clone());

        // wait content_vec.len() seconds for data to propagate, giving more time if more items are propagating
        tokio::time::sleep(Duration::from_secs(content_vec.len() as u64)).await;

        let mut result = vec![];
        for (index, (content_key, content_value)) in content_vec.into_iter().enumerate() {
            match client_b.rpc.local_content(content_key.clone()).await {
                Ok(possible_content) => {
                   match possible_content {
                        PossibleHistoryContentValue::ContentPresent(content) => {
                            if content != content_value {
                                result.push(format!("Error content received for block {} was different then expected", comments[index]));
                            }
                        }
                        PossibleHistoryContentValue::ContentAbsent => {
                            result.push(format!("Error content for block {} was absent", comments[index]));
                        }
                    }
                }
                Err(err) => {
                    panic!("Unable to get received content: {err:?}");
                }
            }
        }

        if !result.is_empty() {
            panic!("Client B: {:?}", result);
        }
    }
}
