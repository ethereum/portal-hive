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
  const res = await client.rpc.request('discv5_nodeInfo', [])
  const nodeInfo = res.result
  const enr = decodeENR(nodeInfo.enr)
  if (!nodeInfo.enr || !nodeInfo.nodeId) {
    test.fatal(`Expected response not received: ${res.error}`)
  }
  if (client.kind[0] !== enr.c[0]) {
    test.fatal(`Expected client type to begin with: ${client.kind[0]}, got ${enr.c}`)
  }
}

const two_client_demo = async (test: Test, client_a: IClient, client_b: IClient) => {
  const res1 = await client_a.rpc.request('discv5_nodeInfo', [])
  const res2 = await client_b.rpc.request('discv5_nodeInfo', [])
  const nodeInfo1 = res1.result
  const nodeInfo2 = res2.result
  if (!nodeInfo1.enr || !nodeInfo1.nodeId || !nodeInfo2.enr || !nodeInfo2.nodeId) {
    test.fatal(`Expected response not received: \n ${res1} \n ${res2}`)
  }
  const ping1 = await client_a.rpc.request('portal_historyPing', [nodeInfo2.enr])
  const ping2 = await client_b.rpc.request('portal_historyPing', [nodeInfo1.enr])
  console.log({
    ping1,
    ping2,
  })
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
    for await (const [_i, _client] of clients.entries()) {
      if (i === _i) {
        continue
      }
      const res = await client.rpc.request('portal_historyPing', [clientsInfo[_i][1].enr])
      if (!res.result) {
        test.fatal(`ping {${client} --> ${clientsInfo[_i]}}: Expected response not received`)
      }
    }
  }
  await new Promise((resolve) => setTimeout(resolve, 1000))
  const routingTables = await Promise.all(
    clients.map(async (client) => {
      const res = await client.rpc.request('portal_historyRoutingTableInfo', [])
      return [client.kind, res.result]
    }),
  )
  if (routingTables.some((table) => !table || !table[1].buckets || !table[1].localNodeId)) {
    test.fatal(`Expected response not received`)
  }
  const errors: string[] = []
  const passing: string[] = []
  for (const [client, table] of routingTables) {
    const expected = clientsInfo.filter(([k, _]) => k !== client).map(([_, c]) => c.nodeId)
    const peers = Object.values(table.buckets).flat() as string[]
    if (peers.length < expected.length) {
      errors.push(
        `${client}: ${peers.length} peers (fail)\n    expected: [${expected.join(
          `,\n               `,
        )}] \n         got: [${peers.join(
          `,\n               `,
        )}] \n------\n(${client}): portal_historyRoutingTableInfo => ${JSON.stringify(table, null, 2)}`,
      )
    } else {
      passing.push(
        `${client}: ${peers.length} peers (pass)\n${' '.repeat(4)}  [${peers
          .map((p) => `${p.startsWith('0x') ? `${p.slice(0, 10)}` : `0x${p.slice(0, 8)}`}...`)
          .join(', ')}]`,
      )
    }
  }
  if (errors.length > 0) {
    test.fatal([...passing, ...errors].join('\n'))
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
    'ts-sim-test',
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
