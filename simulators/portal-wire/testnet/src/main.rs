use testnet::hivesim::{Simulation, Suite, Test, TestSpec};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut suite = Suite {
        name: "portal-testnet".to_string(),
        description: "Collection of different testnet compositions and assertions".to_string(),
        tests: vec![],
    };

    suite.add(TestSpec {
        name: "offer-accept".to_string(),
        description: "Test portal wire OFFER/ACCEPT messages".to_string(),
        always_run: false,
        run: test_offer_accept,
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

fn test_offer_accept(test: Test) {
    futures::executor::block_on(offer_accept_impl(test));
}

async fn offer_accept_impl(test: Test) {
    // 1. Run trin client
    // let trin = test.start_client("trin".to_string()).await;
    // 2. Run fluffy client
    let _fluffy = test.start_client("fluffy".to_string()).await;
    // 3. Send offer from Trin to Fluffy
    // 4. CHeck that fluffy stored the accepted content
    println!("Hello offer/accept");
}
