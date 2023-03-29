use hivesim::{dyn_async, Client, Simulation, Suite, Test, TestSpec, TwoClientTestSpec};
use itertools::Itertools;

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
        name: "OFFER/ACCEPT interop".to_string(),
        description: "".to_string(),
        always_run: false,
        run: test_offer_accept,
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
   async fn test_offer_accept<'a> (test: &'a mut Test, _client: Option<Client>) {
        // Get all available portal clients
        let clients = test.sim.client_types().await;

        // Iterate over all combinations of clients and run the tests
        for combination in clients.iter().combinations(2) {
            let client_a = &combination[0];
            let client_b = &combination[1];

            // Test block header with proof
            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Header {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_header,
                    client_a: &(*client_a).clone(),
                    client_b: &(*client_b).clone(),
                }
            ).await;

            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Header {} --> {}", client_b.name, client_a.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_header,
                    client_a: &(*client_b).clone(),
                    client_b: &(*client_a).clone(),
                }
            ).await;

            // Test block body
            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Body {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_body,
                    client_a: &(*client_a).clone(),
                    client_b: &(*client_b).clone(),
                }
            ).await;

            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Block Body {} --> {}", client_b.name, client_a.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_body,
                    client_a: &(*client_b).clone(),
                    client_b: &(*client_a).clone(),
                }
            ).await;

            // Test receipt
            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Receipt {} --> {}", client_a.name, client_b.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_receipt,
                    client_a: &(*client_a).clone(),
                    client_b: &(*client_b).clone(),
                }
            ).await;

            test.run(
                TwoClientTestSpec {
                    name: format!("OFFER Receipt {} --> {}", client_b.name, client_a.name),
                    description: "".to_string(),
                    always_run: false,
                    run: test_offer_receipt,
                    client_a: &(*client_b).clone(),
                    client_b: &(*client_a).clone(),
                }
            ).await;
        }
   }
}

dyn_async! {
   async fn test_offer_header<'a> (_test: &'a mut Test, client_a: Client, client_b: Client) {
        println!("Running test_offer_header with {:?} and {:?}", client_a.kind, client_b.kind)
   }
}

dyn_async! {
   async fn test_offer_body<'a> (_test: &'a mut Test, client_a: Client, client_b: Client) {
        println!("Running test_offer_body with {:?} and {:?}", client_a.kind, client_b.kind)
   }
}

dyn_async! {
   async fn test_offer_receipt<'a> (_test: &'a mut Test, client_a: Client, client_b: Client) {
        println!("Running test_offer_receipt with {:?} and {:?}", client_a.kind, client_b.kind)
   }
}
