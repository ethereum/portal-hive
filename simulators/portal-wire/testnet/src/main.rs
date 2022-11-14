use testnet::hivesim::{Simulation, Suite, Test, TestSpec};

#[tokio::main]
async fn main() {
    let mut suite = Suite {
        name: "portal-testnet".to_string(),
        description: "collection of different testnet compositions and assertions".to_string(),
        tests: vec![],
    };

    suite.add(TestSpec {
        name: "portal_testnets".to_string(),
        description: "collection of different testnet compositions and assertions".to_string(),
        always_run: false,
        run: test_baba,
    });

    let sim = Simulation::new();
    run_suite(sim, suite).await;
}

async fn run_suite(host: Simulation, suite: Suite) {
    let name = suite.clone().name;
    let description = suite.clone().description;

    let suite_id = host.start_suite(name, description, "".to_string()).await;

    for test in &suite.tests {
        test.run_test(host.clone(), suite_id, suite.clone());
    }

    host.end_suite(suite_id).await;
}

fn test_baba(_t: Test) {
    println!("Hello baba");
}
