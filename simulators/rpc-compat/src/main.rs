use rpc_compat::hivesim::{Client, ClientTestSpec, Testable};
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
        name: format!("test_name_1 ({})", client.kind),
        description: "".to_string(),
        always_run: false,
        run: |_test| run_rpc_tests(),
    })
    .await;
}

fn run_rpc_tests() {
    println!("GO GO GO");
}
