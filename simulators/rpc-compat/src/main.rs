use rpc_compat::hivesim::{Client, ClientTestSpec, NodeInfoResponse, Testable};
use rpc_compat::hivesim::{Simulation, Suite, Test, TestSpec};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut suite = Suite {
        name: "rpc-compat".to_string(),
        description: "The RPC-compatibility test suite runs a set of RPC related tests against a
        running node. It tests client implementations of the JSON-RPC API for
        conformance with the portal network API specification."
            .to_string(),
        tests: vec![],
    };

    suite.add(ClientTestSpec {
        name: "client launch".to_string(),
        description: "This test launches the client and collects its logs.".to_string(),
        always_run: true,
        run: run_test_spec,
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

fn run_test_spec(test: Test, client: Client) {
    futures::executor::block_on(run_all_tests_impl(test, client));
}

async fn run_all_tests_impl(test: Test, client: Client) {
    test.run(TestSpec {
        name: format!("discv5_nodeInfo ({})", client.kind),
        description: "".to_string(),
        always_run: false,
        run: test_node_info,
        client: Some(client),
    })
    .await;
}

fn test_node_info(mut test: Test, client: Option<Client>) -> Test {
    let client = client.expect("Client should be available for discv5_nodeInfo test");
    let request = client
        .rpc
        .read()
        .unwrap()
        .build_request("discv5_nodeInfo", &[]);

    let response = client.rpc.read().unwrap().send_request(request).unwrap();
    let result = response.result;

    match result {
        None => test.fatal("Expected response not received"),
        Some(result) => {
            let result: Result<NodeInfoResponse, _> = serde_json::from_str(result.get());
            if let Err(msg) = result {
                test.fatal(&msg.to_string());
            }
        }
    }

    test
}
