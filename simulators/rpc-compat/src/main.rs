use ethportal_api::types::enr::generate_random_remote_enr;
use ethportal_api::Discv5ApiClient;
use ethportal_api::HistoryNetworkApiClient;
use ethportal_api::PossibleHistoryContentValue::{ContentAbsent, ContentPresent};
use hivesim::{dyn_async, Client, ClientTestSpec, Simulation, Suite, Test, TestSpec};
use serde_json::json;

// Header with proof for block number 14764013
const CONTENT_KEY: &str = "0x00720704f3aa11c53cf344ea069db95cecb81ad7453c8f276b2a1062979611f09c";
const CONTENT_VALUE: &str = "0x080000002d020000f90222a02c58e3212c085178dbb1277e2f3c24b3f451267a75a234945c1581af639f4a7aa058a694212e0416353a4d3865ccf475496b55af3a3d3b002057000741af9731919400192fb10df37c9fb26829eb2cc623cd1bf599e8a067a9fb631f4579f9015ef3c6f1f3830dfa2dc08afe156f750e90022134b9ebf6a018a2978fc62cd1a23e90de920af68c0c3af3330327927cda4c005faccefb5ce7a0168a3827607627e781941dc777737fc4b6beb69a8b139240b881992b35b854eab9010000200000400000001000400080080000000000010004010001000008000000002000110000000000000090020001110402008000080208040010000000a8000000000000000000210822000900205020000000000160020020000400800040000000000042080000000400004008084020001000001004004000001000000000000001000000110000040000010200844040048101000008002000404810082002800000108020000200408008000100000000000000002020000b00010080600902000200000050000400000000000000400000002002101000000a00002000003420000800400000020100002000000000000000c00040000001000000100187327bd7ad3116ce83e147ed8401c9c36483140db184627d9afa9a457468657265756d50504c4e532f326d696e6572735f55534133a0f1a32e24eb62f01ec3f2b3b5893f7be9062fbf5482bc0d490a54352240350e26882087fbb243327696851aae1651b6010cc53ffa2df1bae1550a0000000000000000000000000000000000000000000063d45d0a2242d35484f289108b3c80cccf943005db0db6c67ffea4c4a47fd529f64d74fa6068a3fd89a2c0d9938c3a751c4706d0b0e8f99dec6b517cf12809cb413795c8c678b3171303ddce2fa1a91af6a0961b9db72750d4d5ea7d5103d8d25f23f522d9af4c13fe8ac7a7d9d64bb08d980281eea5298b93cb1085fedc19d4c60afdd52d116cfad030cf4223e50afa8031154a2263c76eb08b96b5b8fdf5e5c30825d5c918eefb89daaf0e8573f20643614d9843a1817b6186074e4e53b22cf49046d977c901ec00aef1555fa89468adc2a51a081f186c995153d1cba0f2887d585212d68be4b958d309fbe611abe98a9bfc3f4b7a7b72bb881b888d89a04ecfe08b1c1a48554a48328646e4f864fe722f12d850f0be29e3829d1f94b34083032a9b6f43abd559785c996229f8e022d4cd6dcde4aafcce6445fe8743e1fcbe8672a99f9d9e3a5ca10c01f3751d69fbd22197f0680bc1529151130b22759bf185f4dbce357f46eb9cc8e21ea78f49b298eea2756d761fe23de8bea0d2e15aed136d689f6d252c54ebadc3e46b84a397b681edf7ec63522b9a298301084d019d0020000000000000000000000000000000000000000000000000000000000000";

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

    suite.add(TestSpec {
        name: "client launch".to_string(),
        description: "This test launches the client and collects its logs.".to_string(),
        always_run: false,
        run: run_all_client_tests,
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
    async fn run_all_client_tests<'a> (test: &'a mut Test, _client: Option<Client>) {
        test.run(ClientTestSpec {
            name: "discv5_nodeInfo".to_string(),
            description: "".to_string(),
            always_run: false,
            run: test_node_info,
        })
        .await;

        test.run(ClientTestSpec {
            name: "portal_historyLocalContent Expect ContentAbsent".to_string(),
            description: "".to_string(),
            always_run: false,
            run: test_history_local_content_expect_content_absent,
        })
        .await;

        test.run(ClientTestSpec {
            name: "portal_historyStore".to_string(),
            description: "".to_string(),
            always_run: false,
            run: test_history_store,
        })
        .await;

        test.run(ClientTestSpec {
            name: "portal_historyLocalContent Expect ContentPresent".to_string(),
            description: "".to_string(),
            always_run: false,
            run: test_history_local_content_expect_content_present,
        })
        .await;

        test.run(ClientTestSpec {
            name: "portal_historyAddEnr Expect true".to_string(),
            description: "".to_string(),
            always_run: false,
            run: test_history_add_enr_expect_true,
        })
        .await;

        test.run(ClientTestSpec {
            name: "portal_historyGetEnr None Found".to_string(),
            description: "".to_string(),
            always_run: false,
            run: test_history_get_enr_non_present,
        })
        .await;

        test.run(ClientTestSpec {
            name: "portal_historyGetEnr ENR Found".to_string(),
            description: "".to_string(),
            always_run: false,
            run: test_history_get_enr_enr_present,
        })
        .await;

        test.run(ClientTestSpec {
            name: "portal_historyGetEnr Local Enr".to_string(),
            description: "".to_string(),
            always_run: false,
            run: test_history_get_enr_local_enr,
        })
        .await;

        test.run(ClientTestSpec {
            name: "portal_historyDeleteEnr None Found".to_string(),
            description: "".to_string(),
            always_run: false,
            run: test_history_delete_enr_non_present,
        })
        .await;

        test.run(ClientTestSpec {
            name: "portal_historyDeleteEnr ENR Found".to_string(),
            description: "".to_string(),
            always_run: false,
            run: test_history_delete_enr_enr_present,
        })
        .await;

        test.run(ClientTestSpec {
            name: "portal_historyLookupEnr None Found".to_string(),
            description: "".to_string(),
            always_run: false,
            run: test_history_lookup_enr_non_present,
        })
        .await;

        test.run(ClientTestSpec {
            name: "portal_historyLookupEnr ENR Found".to_string(),
            description: "".to_string(),
            always_run: false,
            run: test_history_lookup_enr_enr_present,
        })
        .await;

        test.run(ClientTestSpec {
            name: "portal_historyLookupEnr Local Enr".to_string(),
            description: "".to_string(),
            always_run: false,
            run: test_history_lookup_enr_local_enr,
        })
        .await;
    }
}

dyn_async! {
    async fn test_node_info<'a> (client: Client) {
       let response = client
            .rpc
            .node_info().await;

        if let Err(err) = response {
            panic!("Expected response not received: {err}");
        }
    }
}

dyn_async! {
    async fn test_history_local_content_expect_content_absent<'a>(client: Client) {
        let content_key =
        serde_json::from_value(json!(CONTENT_KEY));

        match content_key {
            Ok(content_key) => {
                let response = client
                    .rpc
                    .local_content(content_key).await;

                match response {
                    Ok(response) => {
                        match response {
                            ContentAbsent => (),
                            _ => panic!("Expected ContentAbsent, got ContentPresent")
                        }
                    },
                    Err(err) => {
                        panic!("{}", &err.to_string());
                    },
                }
            }
            Err(err) => {
                panic!("{}", &err.to_string());
            }
        }
    }
}

dyn_async! {
    async fn test_history_store<'a>(client: Client) {
        let content_key =
        serde_json::from_value(json!(CONTENT_KEY));

        let content_value =
        serde_json::from_value(json!(CONTENT_VALUE));

        match content_key {
            Ok(content_key) => {
                match content_value {
                    Ok(content_value) => {
                        let response = client
                            .rpc
                            .store(content_key, content_value).await;

                        if let Err(err) = response {
                            panic!("{}", &err.to_string());
                        }
                    }
                    Err(err) => {
                        panic!("{}", &err.to_string());
                    }
                }
            }
            Err(err) => {
                panic!("{}", &err.to_string());
            }
        }
    }
}

dyn_async! {
    async fn test_history_local_content_expect_content_present<'a>(client: Client) {
        let content_key: Result<ethportal_api::HistoryContentKey, serde_json::Error> =
        serde_json::from_value(json!(CONTENT_KEY));

        let content_value =
        serde_json::from_value(json!(CONTENT_VALUE));


        match content_key {
            Ok(content_key) => {
                // seed content_key/content_value onto the local node to test local_content expect content present
                match content_value {
                    Ok(content_value) => {
                        let response = client
                            .rpc
                            .store(content_key.clone(), content_value).await;

                        if let Err(err) = response {
                            panic!("{}", &err.to_string());
                        }
                    }
                    Err(err) => {
                        panic!("{}", &err.to_string());
                    }
                }

                // Here we are calling local_content RPC to test if the content is present
                let response = client
                    .rpc
                    .local_content(content_key).await;

                match response {
                    Ok(response) => {
                        match response {
                            ContentPresent(_) => (),
                            _ => panic!("Expected ContentPresent, got ContentAbsent")
                        }
                    },
                    Err(err) => {
                        panic!("{}", &err.to_string());
                    },
                }
            }
            Err(err) => {
                panic!("{}", &err.to_string());
            }
        }
    }
}

dyn_async! {
    async fn test_history_add_enr_expect_true<'a>(client: Client) {
        let (_, enr) = generate_random_remote_enr();
        match HistoryNetworkApiClient::add_enr(&client.rpc, enr).await {
            Ok(response) => match response {
                true => (),
                false => panic!("AddEnr expected to get true and instead got false")
            },
            Err(err) => panic!("{}", &err.to_string()),
        }
    }
}

dyn_async! {
    async fn test_history_get_enr_non_present<'a>(client: Client) {
        let (_, enr) = generate_random_remote_enr();

        if (HistoryNetworkApiClient::get_enr(&client.rpc, enr.node_id()).await).is_ok() {
            panic!("GetEnr in this case is not supposed to return a value")
        }
    }
}

dyn_async! {
    async fn test_history_get_enr_local_enr<'a>(client: Client) {
        // get our local enr from NodeInfo
        let target_enr = match client.rpc.node_info().await {
            Ok(node_info) => node_info.enr,
            Err(err) => {
                panic!("Error getting node info: {err:?}");
            }
        };

        // check if we can fetch data from routing table
        match HistoryNetworkApiClient::get_enr(&client.rpc, target_enr.node_id()).await {
            Ok(response) => {
                if response != target_enr {
                    panic!("Response from GetEnr didn't return expected Enr")
                }
            },
            Err(err) => panic!("{}", &err.to_string()),
        }
    }
}

dyn_async! {
    async fn test_history_get_enr_enr_present<'a>(client: Client) {
        let (_, enr) = generate_random_remote_enr();

        // seed enr into routing table
        match HistoryNetworkApiClient::add_enr(&client.rpc, enr.clone()).await {
            Ok(response) => match response {
                true => (),
                false => panic!("AddEnr expected to get true and instead got false")
            },
            Err(err) => panic!("{}", &err.to_string()),
        }

        // check if we can fetch data from routing table
        match HistoryNetworkApiClient::get_enr(&client.rpc, enr.node_id()).await {
            Ok(response) => {
                if response != enr {
                    panic!("Response from GetEnr didn't return expected Enr")
                }
            },
            Err(err) => panic!("{}", &err.to_string()),
        }
    }
}

dyn_async! {
    async fn test_history_delete_enr_non_present<'a>(client: Client) {
        let (_, enr) = generate_random_remote_enr();
        match HistoryNetworkApiClient::delete_enr(&client.rpc, enr.node_id()).await {
            Ok(response) => match response {
                true => panic!("DeleteEnr expected to get false and instead got true"),
                false => ()
            },
            Err(err) => panic!("{}", &err.to_string()),
        };
    }
}

dyn_async! {
    async fn test_history_delete_enr_enr_present<'a>(client: Client) {
        let (_, enr) = generate_random_remote_enr();

        // seed enr into routing table
        match HistoryNetworkApiClient::add_enr(&client.rpc, enr.clone()).await {
            Ok(response) => match response {
                true => (),
                false => panic!("AddEnr expected to get true and instead got false")
            },
            Err(err) => panic!("{}", &err.to_string()),
        }

        // check if data was seeded into the table
        match HistoryNetworkApiClient::get_enr(&client.rpc, enr.node_id()).await {
            Ok(response) => {
                if response != enr {
                    panic!("Response from GetEnr didn't return expected Enr")
                }
            },
            Err(err) => panic!("{}", &err.to_string()),
        }

        // delete the data from routing table
        match HistoryNetworkApiClient::delete_enr(&client.rpc, enr.node_id()).await {
            Ok(response) => match response {
                true => (),
                false => panic!("DeleteEnr expected to get true and instead got false")
            },
            Err(err) => panic!("{}", &err.to_string()),
        };

        // check if the enr was actually deleted out of the table or not
        if (HistoryNetworkApiClient::get_enr(&client.rpc, enr.node_id()).await).is_ok() {
            panic!("GetEnr in this case is not supposed to return a value")
        }
    }
}

dyn_async! {
    async fn test_history_lookup_enr_non_present<'a>(client: Client) {
        let (_, enr) = generate_random_remote_enr();

        if (HistoryNetworkApiClient::lookup_enr(&client.rpc, enr.node_id()).await).is_ok() {
            panic!("LookupEnr in this case is not supposed to return a value")
        }
    }
}

dyn_async! {
    async fn test_history_lookup_enr_enr_present<'a>(client: Client) {
        let (_, enr) = generate_random_remote_enr();

        // seed enr into routing table
        match HistoryNetworkApiClient::add_enr(&client.rpc, enr.clone()).await {
            Ok(response) => match response {
                true => (),
                false => panic!("AddEnr expected to get true and instead got false")
            },
            Err(err) => panic!("{}", &err.to_string()),
        }

        // check if we can fetch data from routing table
        match HistoryNetworkApiClient::lookup_enr(&client.rpc, enr.node_id()).await {
            Ok(response) => {
                if response != enr {
                    panic!("Response from LookupEnr didn't return expected Enr")
                }
            },
            Err(err) => panic!("{}", &err.to_string()),
        }
    }
}

dyn_async! {
    async fn test_history_lookup_enr_local_enr<'a>(client: Client) {
        // get our local enr from NodeInfo
        let target_enr = match client.rpc.node_info().await {
            Ok(node_info) => node_info.enr,
            Err(err) => {
                panic!("Error getting node info: {err:?}");
            }
        };

        // check if we can fetch data from routing table
        match HistoryNetworkApiClient::lookup_enr(&client.rpc, target_enr.node_id()).await {
            Ok(response) => {
                if response != target_enr {
                    panic!("Response from LookupEnr didn't return expected Enr")
                }
            },
            Err(err) => panic!("{}", &err.to_string()),
        }
    }
}
