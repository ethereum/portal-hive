use hivesim::dyn_async;
use hivesim::{Client, ClientTestSpec, NodeInfoResponse, Testable};
use hivesim::{Simulation, Suite, Test, TestSpec};
use jsonrpc::arg;

const CONTENT_KEY: &str = "0x00720704f3aa11c53cf344ea069db95cecb81ad7453c8f276b2a1062979611f09c";
const CONTENT_VALUE: &str = "0xf90222a02c58e3212c085178dbb1277e2f3c24b3f451267a75a234945c1581af639f4a7aa058a694212e0416353a4d3865ccf475496b55af3a3d3b002057000741af9731919400192fb10df37c9fb26829eb2cc623cd1bf599e8a067a9fb631f4579f9015ef3c6f1f3830dfa2dc08afe156f750e90022134b9ebf6a018a2978fc62cd1a23e90de920af68c0c3af3330327927cda4c005faccefb5ce7a0168a3827607627e781941dc777737fc4b6beb69a8b139240b881992b35b854eab9010000200000400000001000400080080000000000010004010001000008000000002000110000000000000090020001110402008000080208040010000000a8000000000000000000210822000900205020000000000160020020000400800040000000000042080000000400004008084020001000001004004000001000000000000001000000110000040000010200844040048101000008002000404810082002800000108020000200408008000100000000000000002020000b00010080600902000200000050000400000000000000400000002002101000000a00002000003420000800400000020100002000000000000000c00040000001000000100187327bd7ad3116ce83e147ed8401c9c36483140db184627d9afa9a457468657265756d50504c4e532f326d696e6572735f55534133a0f1a32e24eb62f01ec3f2b3b5893f7be9062fbf5482bc0d490a54352240350e26882087fbb243327696851aae1651b6f90222a02c58e3212c085178dbb1277e2f3c24b3f451267a75a234945c1581af639f4a7aa058a694212e0416353a4d3865ccf475496b55af3a3d3b002057000741af9731919400192fb10df37c9fb26829eb2cc623cd1bf599e8a067a9fb631f4579f9015ef3c6f1f3830dfa2dc08afe156f750e90022134b9ebf6a018a2978fc62cd1a23e90de920af68c0c3af3330327927cda4c005faccefb5ce7a0168a3827607627e781941dc777737fc4b6beb69a8b139240b881992b35b854eab9010000200000400000001000400080080000000000010004010001000008000000002000110000000000000090020001110402008000080208040010000000a8000000000000000000210822000900205020000000000160020020000400800040000000000042080000000400004008084020001000001004004000001000000000000001000000110000040000010200844040048101000008002000404810082002800000108020000200408008000100000000000000002020000b00010080600902000200000050000400000000000000400000002002101000000a00002000003420000800400000020100002000000000000000c00040000001000000100187327bd7ad3116ce83e147ed8401c9c36483140db184627d9afa9a457468657265756d50504c4e532f326d696e6572735f55534133a0f1a32e24eb62f01ec3f2b3b5893f7be9062fbf5482bc0d490a54352240350e26882087fbb243327696851aae1651b6f90222a02c58e3212c085178dbb1277e2f3c24b3f451267a75a234945c1581af639f4a7aa058a694212e0416353a4d3865ccf475496b55af3a3d3b002057000741af9731919400192fb10df37c9fb26829eb2cc623cd1bf599e8a067a9fb631f4579f9015ef3c6f1f3830dfa2dc08afe156f750e90022134b9ebf6a018a2978fc62cd1a23e90de920af68c0c3af3330327927cda4c005faccefb5ce7a0168a3827607627e781941dc777737fc4b6beb69a8b139240b881992b35b854eab9010000200000400000001000400080080000000000010004010001000008000000002000110000000000000090020001110402008000080208040010000000a8000000000000000000210822000900205020000000000160020020000400800040000000000042080000000400004008084020001000001004004000001000000000000001000000110000040000010200844040048101000008002000404810082002800000108020000200408008000100000000000000002020000b00010080600902000200000050000400000000000000400000002002101000000a00002000003420000800400000020100002000000000000000c00040000001000000100187327bd7ad3116ce83e147ed8401c9c36483140db184627d9afa9a457468657265756d50504c4e532f326d696e6572735f55534133a0f1a32e24eb62f01ec3f2b3b5893f7be9062fbf5482bc0d490a54352240350e26882087fbb243327696851aae1651b6";

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
        run: run_all_client_tests,
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
    async fn run_all_client_tests<'a> (test: Test, client: Client) {
        test.run(TestSpec {
            name: format!("discv5_nodeInfo ({})", client.kind),
            description: "".to_string(),
            always_run: false,
            run: test_node_info,
            client: Some(client.clone()),
        })
        .await;

        test.run(TestSpec {
            name: format!("portal_historyLocalContent ({})", client.kind),
            description: "".to_string(),
            always_run: false,
            run: test_history_local_content,
            client: Some(client.clone()),
        })
        .await;

        test.run(TestSpec {
            name: format!("portal_historyStore ({})", client.kind),
            description: "".to_string(),
            always_run: false,
            run: test_history_store,
            client: Some(client),
        })
        .await;
    }
}

fn test_node_info(test: &mut Test, client: Option<Client>) {
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
}

fn test_history_local_content(test: &mut Test, client: Option<Client>) {
    let client = client.expect("Client should be available for portal_historySendOffer test");

    let params = [arg(CONTENT_KEY)];

    let request = client
        .rpc
        .read()
        .unwrap()
        .build_request("portal_historyLocalContent", &params);

    let response = client.rpc.read().unwrap().send_request(request);

    match response {
        Ok(resp) => {
            let result = resp.result;
            match result {
                None => test.fatal("Expected response not received"),
                Some(_) => {}
            }
        }
        Err(msg) => {
            test.fatal(&msg.to_string());
        }
    }
}

fn test_history_store(test: &mut Test, client: Option<Client>) {
    let client = client.expect("Client should be available for portal_historySendOffer test");

    let params = [arg(CONTENT_KEY), arg(CONTENT_VALUE)];

    let request = client
        .rpc
        .read()
        .unwrap()
        .build_request("portal_historyStore", &params);

    let response = client.rpc.read().unwrap().send_request(request);

    match response {
        Ok(resp) => {
            let result = resp.result;
            match result {
                None => test.fatal("Expected response not received"),
                Some(_) => {}
            }
        }
        Err(msg) => {
            test.fatal(&msg.to_string());
        }
    }
}
