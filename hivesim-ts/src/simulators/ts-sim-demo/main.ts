import { Simulation } from '../../simulation.js'
import {
  ClientTestSpec,
  IClient,
  NClientTestSpec,
  Suite,
  Test,
  TestSpec,
  TwoClientTestSpec,
} from '../../testapi.js'
import { decodeENR } from '../../utils.js'

const client_enr_tag = async (test: Test, client: IClient) => {
  const clients: any = {
    trin: 't 0.1.1',
    fluffy: 'f 0.0.1',
    ultralight: 'u 0.0.1',
  }

  const res = await client.rpc.request('discv5_nodeInfo', [])
  const nodeInfo = res.result
  const enr = decodeENR(nodeInfo.enr)
  if (!nodeInfo.enr || !nodeInfo.nodeId) {
    test.fatal(`Expected response not received: ${res.error}`)
  }
  if (clients[client.kind] !== enr.c.slice(0, 7)) {
    test.fatal(`Expected client type ${clients[client.kind]}, got ${enr.c}`)
  }
}

const two_client_demo = async (test: Test, client_a: IClient, client_b: IClient) => {
  const res1 = await client_a.rpc.request('discv5_nodeInfo', [])
  const res2 = await client_b.rpc.request('discv5_nodeInfo', [])
  const nodeInfo1 = res1.result
  const nodeInfo2 = res2.result
  const ping1 = await client_a.rpc.request('portal_historyPing', [nodeInfo2.enr])
  const ping2 = await client_b.rpc.request('portal_historyPing', [nodeInfo1.enr])
  if (!ping1.result || !ping2.result) {
    test.fatal(`Expected response not received: ${ping1.error} ${ping2.error}`)
  }
}

const n_client_demo = async (test: Test, clients: IClient[]) => {
  const clientsInfo = await Promise.all(
    clients.map(async (client) => {
      const res = await client.rpc.request('discv5_nodeInfo', [])
      return [client.kind, res.result]
    }),
  )
  for (const [kind, info] of clientsInfo) {
    if (!info.enr || !info.nodeId) {
      test.fatal(`${kind}: Expected response not received`)
    }
  }
  for await (const [i, client] of clients.entries()) {
    const res = await client.rpc.request('portal_historyPing', [
      clientsInfo[(i + 1) % clients.length][1].enr,
    ])
    if (!res.result) {
      test.fatal(`${clientsInfo[(i + 1) % clients.length][0]}: Expected response not received`)
    }
  }
  const routingTables = await Promise.all(
    clients.map(async (client) => {
      const res = await client.rpc.request('portal_historyRoutingTableInfo', [])
      return [client.kind, res.result]
    }),
  )
  if (routingTables.some((table) => !table)) {
    test.fatal(`Expected response not received`)
  }
  const errors: string[] = []
  for (const [client, table] of routingTables) {
    const expected = clientsInfo.map(([_, c]) => [_, c.nodeId]).filter(([k, _]) => k !== client)
    const peers = Object.values(table.buckets).flat()
    if (peers.length < expected.length) {
      errors.push(
        `${client}: expected ${JSON.stringify(expected)} peers, got ${JSON.stringify(peers)}`,
      )
    }
  }
  if (errors.length > 0) {
    test.fatal(errors.join('\n'))
  }
}

const run_all_client_tests = async (test: Test) => {
  await test.run(
    new ClientTestSpec({
      name: 'discv5_nodeInfo',
      description: 'returns the node_id and ENR with correct client tag',
      always_run: true,
      run: client_enr_tag,
    }),
  )
  await test.run(
    new TwoClientTestSpec({
      name: 'portal_historyPing',
      description: 'returns the node_id and ENR with correct client tag',
      always_run: true,
      run: two_client_demo,
    }),
  )
  await test.run(
    new NClientTestSpec({
      name: 'portal_historyRoutingTableInfo',
      description: 'all clients should connect',
      always_run: true,
      run: n_client_demo,
    }),
  )
  // run other tests...
}

const main = async () => {
  const suite = new Suite(
    'ts-sim-demo',
    'The RPC-compatibility test suite runs a set of RPC related tests against a running node. It tests client implementations of the JSON-RPC API for conformance with the portal network API specification.',
  )
  suite.add(
    new TestSpec({
      name: 'client launch',
      description: 'This test launches the client and collects its logs.',
      always_run: true,
      run: run_all_client_tests,
    }),
  )

  const sim = new Simulation(`${process.env['HIVE_SIMULATOR']}`)
  await suite.run(sim)
}

main()
