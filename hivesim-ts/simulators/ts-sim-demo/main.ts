import { Simulation } from "../../simulation.js";
import { ClientTestSpec, IClient, Suite, Test, TestSpec, TwoClientTestSpec } from "../../testapi.js";
import { decodeENR } from "../../utils.js";

const client_enr_tag = async (test: Test, client: IClient) => {
    const clients: any = {
        'trin': 't 0.1.1',
        'fluffy': 'f 0.0.1',
        'ultralight': 'u 0.0.1'
    }
    
    const res = await client.rpc.request("discv5_nodeInfo", [])
    const nodeInfo = res.result
    const enr = decodeENR(nodeInfo.enr)
    if (!nodeInfo.enr || !nodeInfo.nodeId) {
        test.fatal(`Expected response not received: ${res.error}`) 
    }
    const nodeId = nodeInfo.nodeId
    if (clients[client.kind] !== (enr.c).slice(0,7)) {
        test.fatal(`Expected client type ${clients[client.kind]}, got ${enr.c}`)
    }
}

const two_client_demo = async (test: Test, client_a: IClient, client_b: IClient) => {
    const res1 = await client_a.rpc.request("discv5_nodeInfo", [])
    const res2 = await client_b.rpc.request("discv5_nodeInfo", [])
    const nodeInfo1 = res1.result
    const nodeInfo2 = res2.result
    const ping1 = await client_a.rpc.request("portal_historyPing", [nodeInfo2.enr])
    const ping2 = await client_b.rpc.request("portal_historyPing", [nodeInfo1.enr])
    console.log({
        nodeInfo1,
        nodeInfo2,
        ping1: ping1.result,
        ping2: ping2.result

    })
    if (!ping1.result || !ping2.result) {
        test.fatal(`Expected response not received: ${ping1.error} ${ping2.error}`)
    }
}

const run_all_client_tests = async (test: Test) => {
    await test.run(new ClientTestSpec({
        name: "discv5_nodeInfo",
        description: "returns the node_id and ENR with correct client tag",
        always_run: true,
        run: client_enr_tag,
    }))
    await test.run(new TwoClientTestSpec({
        name: "portal_historyPing",
        description: "returns the node_id and ENR with correct client tag",
        always_run: true,
        run: two_client_demo,
    }))
// run other tests...
}

const main = async () => {
    const suite = new Suite(
        "ts-sim-demo",
        "The RPC-compatibility test suite runs a set of RPC related tests against a running node. It tests client implementations of the JSON-RPC API for conformance with the portal network API specification.",
        )
    suite.add(
        new TestSpec({
            name: "client launch",
            description: "This test launches the client and collects its logs.",
            always_run: true,
            run: run_all_client_tests,
        })
    )

        const sim = new Simulation(`${process.env['HIVE_SIMULATOR']}`)
        await suite.run(sim)

}

main()