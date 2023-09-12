use ethportal_api::types::distance::{Metric, XorMetric};
use ethportal_api::types::portal::ContentInfo;
use ethportal_api::{
    ContentValue, Discv5ApiClient, HistoryContentKey, HistoryContentValue, HistoryNetworkApiClient,
    PossibleHistoryContentValue,
};
use hivesim::{dyn_async, Client, Simulation, Suite, Test, TestSpec, TwoClientTestSpec};
use itertools::Itertools;
use serde_json::json;
use serde_yaml::Value;
use tokio::time::Duration;

// This is taken from Trin. It should be fairly standard
const MAX_PORTAL_CONTENT_PAYLOAD_SIZE: usize = 1165;

// Header with proof for block number 14764013
const HEADER_WITH_PROOF_KEY: &str =
    "0x00720704f3aa11c53cf344ea069db95cecb81ad7453c8f276b2a1062979611f09c";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut suite = Suite {
        name: "portal-interop".to_string(),
        description:
            "The portal interop test suite runs a set of scenarios to test interoperability between
        portal network clients"
                .to_string(),
        tests: vec![],
    };

    suite.add(TestSpec {
        name: "Portal Network interop".to_string(),
        description: "".to_string(),
        always_run: false,
        run: test_portal_interop,
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
   async fn test_portal_interop<'a> (test: &'a mut Test, _client: Option<Client>) {
        // Get all available portal clients
        let clients = test.sim.client_types().await;

        // Iterate over all possible pairings of clients and run the tests (including self-pairings)
        for (client_a, client_b) in clients.iter().cartesian_product(clients.iter()) {
            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Header: block number 1 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_header_block_1,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Header: block number 100 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_header_block_100,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Header: block number 7000000 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_header_block_7000000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Header: block number 15600000 (post-merge) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_header_block_15600000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Header: block number 17510000 (post-shanghai) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_header_block_17510000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Body: block number 1 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_block_body_block_1,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Body: block number 100 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_block_body_block_100,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Body: block number 7000000 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_block_body_block_7000000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Body: block number 15600000 (post-merge) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_block_body_block_15600000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Body: block number 17510000 (post-shanghai) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_block_body_block_17510000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Receipts: block number 7000000 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_receipts_block_7000000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Receipts: block number 15600000 (post-merge) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_receipts_block_15600000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Receipts: block number 17510000 (post-shanghai) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_receipts_block_17510000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test portal history ping
            test.run(TwoClientTestSpec {
                    name: format!("PING {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_ping,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test find content non-present
            test.run(TwoClientTestSpec {
                    name: format!("FIND_CONTENT non present {} --> {}", client_a.name, client_b.name),
                    description: "find content: calls find content that doesn't exist".to_string(),
                    always_run: false,
                    run: test_find_content_non_present,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test find nodes distance zero
            test.run(TwoClientTestSpec {
                    name: format!("FIND_NODES Distance 0 {} --> {}", client_a.name, client_b.name),
                    description: "find nodes: distance zero expect called nodes enr".to_string(),
                    always_run: false,
                    run: test_find_nodes_zero_distance,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test recursive find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("RecursiveFindContent Block Header: block number 1 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_recursive_find_content_header_block_1,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test recursive find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("RecursiveFindContent Block Header: block number 100 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_recursive_find_content_header_block_100,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test recursive find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("RecursiveFindContent Block Header: block number 7000000 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_recursive_find_content_header_block_7000000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test recursive find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("RecursiveFindContent Block Header: block number 15600000 (post-merge) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_recursive_find_content_header_block_15600000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test recursive find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("RecursiveFindContent Block Header: block number 17510000 (post-shanghai) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_recursive_find_content_header_block_17510000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test recursive find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("RecursiveFindContent Block Body: block number 1 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_recursive_find_content_block_body_block_1,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test recursive find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("RecursiveFindContent Block Body: block number 100 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_recursive_find_content_block_body_block_100,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test recursive find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("RecursiveFindContent Block Body: block number 7000000 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_recursive_find_content_block_body_block_7000000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test recursive find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("RecursiveFindContent Block Body: block number 15600000 (post-merge) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_recursive_find_content_block_body_block_15600000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test recursive find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("RecursiveFindContent Block Body: block number 17510000 (post-shanghai) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_recursive_find_content_block_body_block_17510000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test recursive find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("RecursiveFindContent Receipts: block number 7000000 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_recursive_find_content_receipts_block_7000000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test recursive find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("RecursiveFindContent Receipts: block number 15600000 (post-merge) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_recursive_find_content_receipts_block_15600000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test recursive find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("RecursiveFindContent Receipts: block number 17510000 (post-shanghai) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_recursive_find_content_receipts_block_17510000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test find nodes distance of client a
            test.run(TwoClientTestSpec {
                    name: format!("FIND_NODES distance of client A {} --> {}", client_a.name, client_b.name),
                    description: "find nodes: distance of client A expect seeded enr returned".to_string(),
                    always_run: false,
                    run: test_find_nodes_distance_of_client_a,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("FindContent Block Header: block number 1 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_find_content_header_block_1,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("FindContent Block Header: block number 100 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_find_content_header_block_100,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("FindContent Block Header: block number 7000000 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_find_content_header_block_7000000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("FindContent Block Header: block number 15600000 (post-merge) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_find_content_header_block_15600000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("FindContent Block Header: block number 17510000 (post-shanghai) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_find_content_header_block_17510000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("FindContent Block Body: block number 1 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_find_content_block_body_block_1,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("FindContent Block Body: block number 100 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_find_content_block_body_block_100,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("FindContent Block Body: block number 7000000 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_find_content_block_body_block_7000000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("FindContent Block Body: block number 15600000 (post-merge) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_find_content_block_body_block_15600000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("FindContent Block Body: block number 17510000 (post-shanghai) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_find_content_block_body_block_17510000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("FindContent Receipts: block number 7000000 {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_find_content_receipts_block_7000000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("FindContent Receipts: block number 15600000 (post-merge) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_find_content_receipts_block_15600000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test find content Header With Proof
            test.run(
                TwoClientTestSpec {
                    name: format!("FindContent Receipts: block number 17510000 (post-shanghai) {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_find_content_receipts_block_17510000,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;

            // Test gossiping a collection of blocks to node B (B will gossip back to A)
            test.run(
                TwoClientTestSpec {
                    name: format!("GOSSIP blocks from A:{} --> B:{}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_gossip_two_nodes,
                    client_a: client_a.clone(),
                    client_b: client_b.clone(),
                }
            ).await;
        }
   }
}

dyn_async! {
    // test that a node will not return content via FINDCONTENT.
    async fn test_find_content_non_present<'a> (client_a: Client, client_b: Client) {
        let header_with_proof_key: HistoryContentKey = serde_json::from_value(json!(HEADER_WITH_PROOF_KEY)).unwrap();

        let target_enr = match client_b.rpc.node_info().await {
            Ok(node_info) => node_info.enr,
            Err(err) => {
                panic!("Error getting node info: {err:?}");
            }
        };

        let result = client_a.rpc.find_content(target_enr, header_with_proof_key.clone()).await;

        match result {
            Ok(result) => {
                match result {
                    ContentInfo::Enrs{ enrs: val } => {
                        if !val.is_empty() {
                            panic!("Error: Unexpected FINDCONTENT response: expected ContentInfo::Enrs length 0 got {}", val.len());
                        }
                    },
                    ContentInfo::Content{ content: _, .. } => {
                        panic!("Error: Unexpected FINDCONTENT response: wasn't supposed to return back content");
                    },
                    other => {
                        panic!("Error: Unexpected FINDCONTENT response: {other:?}");
                    }
                }
            },
            Err(err) => {
                panic!("Error: Unable to get response from FINDCONTENT request: {err:?}");
            }
        }
    }
}

dyn_async! {
    async fn test_offer_header_block_1<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[0].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[0].get("content_value").unwrap().clone()).unwrap();
        test_offer_x(client_a, client_b, (header_key, header_value), None).await;
    }
}

dyn_async! {
    async fn test_offer_header_block_100<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[2].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[2].get("content_value").unwrap().clone()).unwrap();
        test_offer_x(client_a, client_b, (header_key, header_value), None).await;
    }
}

dyn_async! {
    async fn test_offer_header_block_7000000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[4].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[4].get("content_value").unwrap().clone()).unwrap();
        test_offer_x(client_a, client_b, (header_key, header_value), None).await;
    }
}

dyn_async! {
    async fn test_offer_header_block_15600000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[7].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[7].get("content_value").unwrap().clone()).unwrap();
        test_offer_x(client_a, client_b, (header_key, header_value), None).await;
    }
}

dyn_async! {
    async fn test_offer_header_block_17510000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[10].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[10].get("content_value").unwrap().clone()).unwrap();
        test_offer_x(client_a, client_b, (header_key, header_value), None).await;
    }
}

dyn_async! {
    async fn test_offer_block_body_block_1<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[0].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[0].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[1].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[1].get("content_value").unwrap().clone()).unwrap();
        test_offer_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    async fn test_offer_block_body_block_100<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[2].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[2].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[3].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[3].get("content_value").unwrap().clone()).unwrap();
        test_offer_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    async fn test_offer_block_body_block_7000000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[4].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[4].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[5].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[5].get("content_value").unwrap().clone()).unwrap();
        test_offer_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    async fn test_offer_block_body_block_15600000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[7].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[7].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[8].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[8].get("content_value").unwrap().clone()).unwrap();
        test_offer_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    async fn test_offer_block_body_block_17510000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[10].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[10].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[11].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[11].get("content_value").unwrap().clone()).unwrap();
        test_offer_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    async fn test_offer_receipts_block_7000000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[4].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[4].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[6].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[6].get("content_value").unwrap().clone()).unwrap();
        test_offer_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    async fn test_offer_receipts_block_15600000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[7].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[7].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[9].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[9].get("content_value").unwrap().clone()).unwrap();
        test_offer_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    async fn test_offer_receipts_block_17510000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[10].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[10].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[12].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[12].get("content_value").unwrap().clone()).unwrap();
        test_offer_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    async fn test_offer_x<'a>(client_a: Client, client_b: Client, target_content: (HistoryContentKey, HistoryContentValue), optional_content: Option<(HistoryContentKey, HistoryContentValue)>) {
        if let Some((optional_key, optional_value)) = optional_content {
            match client_b.rpc.store(optional_key, optional_value).await {
                Ok(result) => if !result {
                    panic!("Unable to store optional content for recursive find content");
                },
                Err(err) => {
                    panic!("Error storing optional content for recursive find content: {err:?}");
                }
            }
        }

        let (target_key, target_value) = target_content;
        match client_b.rpc.store(target_key.clone(), target_value.clone()).await {
            Ok(result) => if !result {
                panic!("Error storing target content for recursive find content");
            },
            Err(err) => {
                panic!("Error storing target content: {err:?}");
            }
        }

        let target_enr = match client_b.rpc.node_info().await {
            Ok(node_info) => node_info.enr,
            Err(err) => {
                panic!("Error getting node info: {err:?}");
            }
        };

        let _ = client_a.rpc.offer(target_enr, target_key.clone(), Some(target_value.clone())).await;

        tokio::time::sleep(Duration::from_secs(8)).await;

        match client_b.rpc.local_content(target_key).await {
            Ok(possible_content) => {
               match possible_content {
                    PossibleHistoryContentValue::ContentPresent(content) => {
                        if content != target_value {
                            panic!("Error receiving content: Expected content: {target_value:?}, Received content: {content:?}");
                        }
                    }
                    PossibleHistoryContentValue::ContentAbsent => {
                        panic!("Expected content not found!");
                    }
                }
            }
            Err(err) => {
                panic!("Unable to get received content: {err:?}");
            }
        }
    }
}

dyn_async! {
    async fn test_ping<'a>(client_a: Client, client_b: Client) {
        let target_enr = match client_b.rpc.node_info().await {
            Ok(node_info) => node_info.enr,
            Err(err) => {
                panic!("Error getting node info: {err:?}");
            }
        };

        let pong = client_a.rpc.ping(target_enr).await;

        if let Err(err) = pong {
                panic!("Unable to receive pong info: {err:?}");
        }

        // Verify that client_b stored client_a its ENR through the base layer
        // handshake mechanism.
        let stored_enr = match client_a.rpc.node_info().await {
            Ok(node_info) => node_info.enr,
            Err(err) => {
                panic!("Error getting node info: {err:?}");
            }
        };

        match HistoryNetworkApiClient::get_enr(&client_b.rpc, stored_enr.node_id()).await {
            Ok(response) => {
                if response != stored_enr {
                    panic!("Response from GetEnr didn't return expected ENR. Got: {response}; Expected: {stored_enr}")
                }
            },
            Err(err) => panic!("Failed while trying to get client A's ENR from client B: {err}"),
        }
    }
}

dyn_async! {
    async fn test_find_nodes_zero_distance<'a>(client_a: Client, client_b: Client) {
        let target_enr = match client_b.rpc.node_info().await {
            Ok(node_info) => node_info.enr,
            Err(err) => {
                panic!("Error getting node info: {err:?}");
            }
        };

        match client_a.rpc.find_nodes(target_enr.clone(), vec![0]).await {
            Ok(response) => {
                if response.len() != 1 {
                    panic!("Response from FindNodes didn't return expected length of 1");
                }

                match response.get(0) {
                    Some(response_enr) => {
                        if *response_enr != target_enr {
                            panic!("Response from FindNodes didn't return expected Enr");
                        }
                    },
                    None => panic!("Error find nodes zero distance wasn't supposed to return None"),
                }
            }
            Err(err) => panic!("{}", &err.to_string()),
        }
    }
}

dyn_async! {
    // test that a node will return a header via RECURSIVEFINDCONTENT that is stored locally
    async fn test_recursive_find_content_header_block_1<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[0].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[0].get("content_value").unwrap().clone()).unwrap();
        test_recursive_find_content_x(client_a, client_b, (header_key, header_value), None).await;
    }
}

dyn_async! {
    // test that a node will return a header via RECURSIVEFINDCONTENT that is stored locally
    async fn test_recursive_find_content_header_block_100<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[2].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[2].get("content_value").unwrap().clone()).unwrap();
        test_recursive_find_content_x(client_a, client_b, (header_key, header_value), None).await;
    }
}

dyn_async! {
    // test that a node will return a header via RECURSIVEFINDCONTENT that is stored locally
    async fn test_recursive_find_content_header_block_7000000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[4].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[4].get("content_value").unwrap().clone()).unwrap();
        test_recursive_find_content_x(client_a, client_b, (header_key, header_value), None).await;
    }
}

dyn_async! {
    // test that a node will return a header via RECURSIVEFINDCONTENT that is stored locally
    async fn test_recursive_find_content_header_block_15600000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[7].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[7].get("content_value").unwrap().clone()).unwrap();
        test_recursive_find_content_x(client_a, client_b, (header_key, header_value), None).await;
    }
}

dyn_async! {
    // test that a node will return a header via RECURSIVEFINDCONTENT that is stored locally
    async fn test_recursive_find_content_header_block_17510000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[10].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[10].get("content_value").unwrap().clone()).unwrap();
        test_recursive_find_content_x(client_a, client_b, (header_key, header_value), None).await;
    }
}

dyn_async! {
    // test that a node will return a block body via RECURSIVEFINDCONTENT that is stored locally
    async fn test_recursive_find_content_block_body_block_1<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[0].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[0].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[1].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[1].get("content_value").unwrap().clone()).unwrap();
        test_recursive_find_content_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    // test that a node will return a block body via RECURSIVEFINDCONTENT that is stored locally
    async fn test_recursive_find_content_block_body_block_100<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[2].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[2].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[3].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[3].get("content_value").unwrap().clone()).unwrap();
        test_recursive_find_content_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    // test that a node will return a block body via RECURSIVEFINDCONTENT that is stored locally
    async fn test_recursive_find_content_block_body_block_7000000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[4].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[4].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[5].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[5].get("content_value").unwrap().clone()).unwrap();
        test_recursive_find_content_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    // test that a node will return a block body via RECURSIVEFINDCONTENT that is stored locally
    async fn test_recursive_find_content_block_body_block_15600000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[7].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[7].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[8].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[8].get("content_value").unwrap().clone()).unwrap();
        test_recursive_find_content_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    // test that a node will return a block body via RECURSIVEFINDCONTENT that is stored locally
    async fn test_recursive_find_content_block_body_block_17510000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[10].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[10].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[11].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[11].get("content_value").unwrap().clone()).unwrap();
        test_recursive_find_content_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    // test that a node will return a receipts via RECURSIVEFINDCONTENT that is stored locally
    async fn test_recursive_find_content_receipts_block_7000000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[4].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[4].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[6].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[6].get("content_value").unwrap().clone()).unwrap();
        test_recursive_find_content_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    // test that a node will return a receipts via RECURSIVEFINDCONTENT that is stored locally
    async fn test_recursive_find_content_receipts_block_15600000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[7].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[7].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[9].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[9].get("content_value").unwrap().clone()).unwrap();
        test_recursive_find_content_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    // test that a node will return a receipts via RECURSIVEFINDCONTENT that is stored locally
    async fn test_recursive_find_content_receipts_block_17510000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[10].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[10].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[12].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[12].get("content_value").unwrap().clone()).unwrap();
        test_recursive_find_content_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    // test that a node will return a content via RECURSIVEFINDCONTENT template that it has stored locally
    async fn test_recursive_find_content_x<'a>(client_a: Client, client_b: Client, target_content: (HistoryContentKey, HistoryContentValue), optional_content: Option<(HistoryContentKey, HistoryContentValue)>) {
        if let Some((optional_key, optional_value)) = optional_content {
            match client_b.rpc.store(optional_key, optional_value).await {
                Ok(result) => if !result {
                    panic!("Unable to store optional content for recursive find content");
                },
                Err(err) => {
                    panic!("Error storing optional content for recursive find content: {err:?}");
                }
            }
        }

        let (target_key, target_value) = target_content;
        match client_b.rpc.store(target_key.clone(), target_value.clone()).await {
            Ok(result) => if !result {
                panic!("Error storing target content for recursive find content");
            },
            Err(err) => {
                panic!("Error storing target content: {err:?}");
            }
        }

        let target_enr = match client_b.rpc.node_info().await {
            Ok(node_info) => node_info.enr,
            Err(err) => {
                panic!("Error getting node info: {err:?}");
            }
        };

        match HistoryNetworkApiClient::add_enr(&client_a.rpc, target_enr.clone()).await {
            Ok(response) => match response {
                true => (),
                false => panic!("AddEnr expected to get true and instead got false")
            },
            Err(err) => panic!("{}", &err.to_string()),
        }

        match client_a.rpc.recursive_find_content(target_key.clone()).await {
            Ok(result) => {
                match result {
                    ContentInfo::Content{ content: ethportal_api::PossibleHistoryContentValue::ContentPresent(val), utp_transfer } => {
                        if val != target_value {
                            panic!("Error: Unexpected RECURSIVEFINDCONTENT response: didn't return expected target content");
                        }

                        if target_value.encode().len() < MAX_PORTAL_CONTENT_PAYLOAD_SIZE {
                            if utp_transfer {
                                panic!("Error: Unexpected RECURSIVEFINDCONTENT response: utp_transfer was supposed to be false");
                            }
                        } else if !utp_transfer {
                            panic!("Error: Unexpected RECURSIVEFINDCONTENT response: utp_transfer was supposed to be true");
                        }
                    },
                    other => {
                        panic!("Error: Unexpected RECURSIVEFINDCONTENT response: {other:?}");
                    }
                }
            },
            Err(err) => {
                panic!("Error: Unable to get response from RECURSIVEFINDCONTENT request: {err:?}");
            }
        }
    }
}

dyn_async! {
    // test that a node will return a header via FINDCONTENT that is stored locally
    async fn test_find_content_header_block_1<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[0].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[0].get("content_value").unwrap().clone()).unwrap();
        test_find_content_x(client_a, client_b, (header_key, header_value), None).await;
    }
}

dyn_async! {
    // test that a node will return a header via FINDCONTENT that is stored locally
    async fn test_find_content_header_block_100<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[2].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[2].get("content_value").unwrap().clone()).unwrap();
        test_find_content_x(client_a, client_b, (header_key, header_value), None).await;
    }
}

dyn_async! {
    // test that a node will return a header via FINDCONTENT that is stored locally
    async fn test_find_content_header_block_7000000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[4].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[4].get("content_value").unwrap().clone()).unwrap();
        test_find_content_x(client_a, client_b, (header_key, header_value), None).await;
    }
}

dyn_async! {
    // test that a node will return a header via FINDCONTENT that is stored locally
    async fn test_find_content_header_block_15600000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[7].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[7].get("content_value").unwrap().clone()).unwrap();
        test_find_content_x(client_a, client_b, (header_key, header_value), None).await;
    }
}

dyn_async! {
    // test that a node will return a header via FINDCONTENT that is stored locally
    async fn test_find_content_header_block_17510000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[10].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[10].get("content_value").unwrap().clone()).unwrap();
        test_find_content_x(client_a, client_b, (header_key, header_value), None).await;
    }
}

dyn_async! {
    // test that a node will return a block body via FINDCONTENT that is stored locally
    async fn test_find_content_block_body_block_1<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[0].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[0].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[1].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[1].get("content_value").unwrap().clone()).unwrap();
        test_find_content_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    // test that a node will return a block body via FINDCONTENT that is stored locally
    async fn test_find_content_block_body_block_100<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[2].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[2].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[3].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[3].get("content_value").unwrap().clone()).unwrap();
        test_find_content_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    // test that a node will return a block body via FINDCONTENT that is stored locally
    async fn test_find_content_block_body_block_7000000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[4].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[4].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[5].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[5].get("content_value").unwrap().clone()).unwrap();
        test_find_content_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    // test that a node will return a block body via FINDCONTENT that is stored locally
    async fn test_find_content_block_body_block_15600000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[7].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[7].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[8].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[8].get("content_value").unwrap().clone()).unwrap();
        test_find_content_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    // test that a node will return a block body via FINDCONTENT that is stored locally
    async fn test_find_content_block_body_block_17510000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[10].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[10].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[11].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[11].get("content_value").unwrap().clone()).unwrap();
        test_find_content_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    // test that a node will return a receipts via FINDCONTENT that is stored locally
    async fn test_find_content_receipts_block_7000000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[4].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[4].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[6].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[6].get("content_value").unwrap().clone()).unwrap();
        test_find_content_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    // test that a node will return a receipts via FINDCONTENT that is stored locally
    async fn test_find_content_receipts_block_15600000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[7].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[7].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[9].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[9].get("content_value").unwrap().clone()).unwrap();
        test_find_content_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    // test that a node will return a receipts via FINDCONTENT that is stored locally
    async fn test_find_content_receipts_block_17510000<'a> (client_a: Client, client_b: Client) {
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();
        let header_key: HistoryContentKey =
            serde_yaml::from_value(values[10].get("content_key").unwrap().clone()).unwrap();
        let header_value: HistoryContentValue =
            serde_yaml::from_value(values[10].get("content_value").unwrap().clone()).unwrap();
        let content_key: HistoryContentKey =
            serde_yaml::from_value(values[12].get("content_key").unwrap().clone()).unwrap();
        let content_value: HistoryContentValue =
            serde_yaml::from_value(values[12].get("content_value").unwrap().clone()).unwrap();
        test_find_content_x(client_a, client_b, (content_key, content_value), Some((header_key, header_value))).await;
    }
}

dyn_async! {
    // test that a node will return a x content via FINDCONTENT that it has stored locally
    async fn test_find_content_x<'a> (client_a: Client, client_b: Client, target_content: (HistoryContentKey, HistoryContentValue), optional_content: Option<(HistoryContentKey, HistoryContentValue)>) {
        if let Some((optional_key, optional_value)) = optional_content {
            match client_b.rpc.store(optional_key, optional_value).await {
                Ok(result) => if !result {
                    panic!("Unable to store optional content for find content");
                },
                Err(err) => {
                    panic!("Error storing optional content for find content: {err:?}");
                }
            }
        }

        let (target_key, target_value) = target_content;
        match client_b.rpc.store(target_key.clone(), target_value.clone()).await {
            Ok(result) => if !result {
                panic!("Error storing target content for find content");
            },
            Err(err) => {
                panic!("Error storing target content: {err:?}");
            }
        }

        let target_enr = match client_b.rpc.node_info().await {
            Ok(node_info) => node_info.enr,
            Err(err) => {
                panic!("Error getting node info: {err:?}");
            }
        };

        match client_a.rpc.find_content(target_enr, target_key.clone()).await {
            Ok(result) => {
                match result {
                    ContentInfo::Content{ content: ethportal_api::PossibleHistoryContentValue::ContentPresent(val), utp_transfer } => {
                        if val != target_value {
                            panic!("Error: Unexpected FINDCONTENT response: didn't return expected block body");
                        }

                        if target_value.encode().len() < MAX_PORTAL_CONTENT_PAYLOAD_SIZE {
                            if utp_transfer {
                                panic!("Error: Unexpected FINDCONTENT response: utp_transfer was supposed to be false");
                            }
                        } else if !utp_transfer {
                            panic!("Error: Unexpected FINDCONTENT response: utp_transfer was supposed to be true");
                        }
                    },
                    other => {
                        panic!("Error: Unexpected FINDCONTENT response: {other:?}");
                    }
                }
            },
            Err(err) => {
                panic!("Error: Unable to get response from FINDCONTENT request: {err:?}");
            }
        }
    }
}

// Certain implementations only return nodes which are seen from find_nodes hence instead of
// generating random enrs we will use client A which client B has "seen"
dyn_async! {
    async fn test_find_nodes_distance_of_client_a<'a>(client_a: Client, client_b: Client) {
        let target_enr = match client_b.rpc.node_info().await {
            Ok(node_info) => node_info.enr,
            Err(err) => {
                panic!("Error getting node info: {err:?}");
            }
        };

        // We are adding client A to our list so we then can assume only one client per bucket
        let client_a_enr = match client_a.rpc.node_info().await {
            Ok(node_info) => node_info.enr,
            Err(err) => {
                panic!("Error getting node info: {err:?}");
            }
        };

        // seed enr into routing table
        match HistoryNetworkApiClient::add_enr(&client_b.rpc, client_a_enr.clone()).await {
            Ok(response) => match response {
                true => (),
                false => panic!("AddEnr expected to get true and instead got false")
            },
            Err(err) => panic!("{}", &err.to_string()),
        }

        if let Some(distance) = XorMetric::distance(&target_enr.node_id().raw(), &client_a_enr.node_id().raw()).log2() {
            match client_a.rpc.find_nodes(target_enr.clone(), vec![distance as u16]).await {
                Ok(response) => {
                    if response.is_empty() {
                        panic!("FindNodes expected to have received a non-empty response");
                    }

                    if response.len() != 1 {
                        panic!("FindNodes expected to have received only 1 enr instead got: {}", response.len());
                    }

                    if !response.contains(&client_a_enr) {
                        panic!("FindNodes {distance} distance expected to contained seeded Enr");
                    }
                }
                Err(err) => panic!("{}", &err.to_string()),
            }
        } else {
            panic!("Distance calculation failed");
        }
    }
}

dyn_async! {
    async fn test_gossip_two_nodes<'a> (client_a: Client, client_b: Client) {
        // connect clients
        let client_b_enr = match client_b.rpc.node_info().await {
            Ok(node_info) => node_info.enr,
            Err(err) => {
                panic!("Error getting node info: {err:?}");
            }
        };
        match HistoryNetworkApiClient::add_enr(&client_a.rpc, client_b_enr.clone()).await {
            Ok(response) => match response {
                true => (),
                false => panic!("AddEnr expected to get true and instead got false")
            },
            Err(err) => panic!("{}", &err.to_string()),
        }

        // With default node settings nodes should be storing all content
        let values = std::fs::read_to_string("./test-data/test_data_collection_of_forks_blocks.yaml")
            .expect("cannot find test asset");
        let values: Value = serde_yaml::from_str(&values).unwrap();

        for value in values.as_sequence().unwrap() {
            let content_key: HistoryContentKey =
                serde_yaml::from_value(value.get("content_key").unwrap().clone()).unwrap();
            let content_value: HistoryContentValue =
                serde_yaml::from_value(value.get("content_value").unwrap().clone()).unwrap();

            match client_a.rpc.gossip(content_key.clone(), content_value.clone()).await {
                Ok(nodes_gossiped_to) => {
                   if nodes_gossiped_to != 1 {
                        panic!("We expected to gossip to 1 node instead we gossiped to: {nodes_gossiped_to}");
                    }
                }
                Err(err) => {
                    panic!("Unable to get received content: {err:?}");
                }
            }

            if let HistoryContentKey::BlockHeaderWithProof(_) = content_key {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }

        // wait 8 seconds for data to propagate
        // This value is determined by how long the sleeps are in the gossip code
        // So we may lower this or remove it depending on what we find.
        tokio::time::sleep(Duration::from_secs(8)).await;

        let comments = vec!["1 header", "1 block body", "100 header",
            "100 block body", "7000000 header", "7000000 block body",
            "7000000 receipt", "15600000 (post-merge) header", "15600000 (post-merge) block body", "15600000 (post-merge) receipt",
            "17510000 (post-shanghai) header", "17510000 (post-shanghai) block body", "17510000 (post-shanghai) receipt"];

        let mut result = vec![];
        for (index, value) in values.as_sequence().unwrap().iter().enumerate() {
            let content_key: HistoryContentKey =
                serde_yaml::from_value(value.get("content_key").unwrap().clone()).unwrap();
            let content_value: HistoryContentValue =
                serde_yaml::from_value(value.get("content_value").unwrap().clone()).unwrap();

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
