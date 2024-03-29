import jayson from 'jayson/promise/index.js'
import { Simulation } from './simulation.js'
import { SuiteID, TestID, TestResult } from './types.js'
import { client_test_name } from './utils.js'

const { Client } = jayson
type HttpClient = jayson.HttpClient
type AsyncTestFunc = (test: Test, client?: IClient) => Promise<void>
type AsyncClientTestFunc = (test: Test, client: IClient) => Promise<void>
type AsyncTwoClientsTestFunc = (test: Test, client_a: IClient, client_b: IClient) => Promise<void>
type AsyncNClientsTestFunc = (test: Test, clients: IClient[]) => Promise<void>

type TestFunc =
  | AsyncTestFunc
  | AsyncClientTestFunc
  | AsyncTwoClientsTestFunc
  | AsyncNClientsTestFunc
export interface Testable {
  name: string
  description: string
  always_run: boolean
  run: TestFunc
  run_test: (simulation: Simulation, suite_id: SuiteID, suite: Suite) => Promise<any>
}

export class Suite {
  name: string
  description: string
  tests: Testable[]
  constructor(name: string, description: string) {
    this.name = name
    this.description = description
    this.tests = []
  }
  add(test: Testable) {
    this.tests.push(test)
  }
  // // RunSuite runs all tests in a suite.
  async run(host: Simulation) {
    const suiteId = await host.start_suite(this.name, this.description)
    for await (const test of this.tests) {
      await test.run_test(host, suiteId, this)
    }
    await host.end_suite(suiteId)
  }

  async mustRunSuite(host: Simulation): Promise<void> {
    await this.run(host)
  }
}

export interface IHttpClient {
  transport: HttpClient
  request_timeout: number
}
export interface IClient {
  kind: string
  container: string
  ip: string
  rpc: HttpClient
  test: ITest
}
export interface ITestRun {
  suite_id: SuiteID
  suite: Suite
  name: string
  desc: string
  always_run: boolean
}

export interface ITest {
  sim: Simulation
  suite_id: SuiteID
  suite: Suite
  test_id: TestID
  result: TestResult
}

export class Test {
  sim: Simulation
  suite: Suite
  suite_id: SuiteID
  test_id: TestID
  result: TestResult

  constructor(
    sim: Simulation,
    suite_id: SuiteID,
    suite: Suite,
    test_id: TestID,
    result: TestResult = {
      details: '',
      pass: false,
    },
  ) {
    this.sim = sim
    this.suite_id = suite_id
    this.suite = suite
    this.test_id = test_id
    this.result = result
  }

  async start_client(client_type: string): Promise<IClient> {
    const [container, ip] = await this.sim.start_client(this.suite_id, this.test_id, client_type)
    const rpc_client = Client.http({ port: 8545, host: ip })
    const client: IClient = {
      kind: client_type,
      container: container,
      ip: ip,
      rpc: rpc_client,
      test: {
        sim: this.sim,
        suite_id: this.suite_id,
        suite: this.suite,
        test_id: this.test_id,
        result: this.result!,
      },
    }
    return client
  }

  async run(spec: Testable) {
    await spec.run_test(this.sim, this.suite_id, this.suite)
  }

  fatal(message: string) {
    this.log_failure(message)
    this.fail()
  }

  log_failure(message: string) {
    this.result.details = message
  }

  fail() {
    this.result.pass = false
  }
}
interface ITestSpec extends Testable {
  name: string
  description: string
  always_run: boolean
  run: AsyncTestFunc
  client?: IClient
}

interface IClientTestSpec extends Testable {
  name: string
  description: string
  always_run: boolean
  run: AsyncClientTestFunc
}
interface I2ClientTestSpec extends Testable {
  name: string
  description: string
  always_run: boolean
  run: AsyncTwoClientsTestFunc
}
export class ClientTestSpec implements IClientTestSpec {
  name: string
  description: string
  always_run: boolean
  run: AsyncClientTestFunc
  constructor(opts: {
    name: string
    description: string
    always_run: boolean
    run: AsyncClientTestFunc
  }) {
    this.name = opts.name
    this.description = opts.description
    this.always_run = opts.always_run
    this.run = opts.run
  }
  async run_test(sim: Simulation, suite_id: SuiteID, suite: Suite) {
    const clients = await sim.client_types()
    for (const client of clients) {
      const client_name = client.name
      const name = client_test_name(this.name, client_name)
      const test_run: ITestRun = {
        suite_id,
        suite,
        name,
        desc: this.description,
        always_run: this.always_run,
      }
      await this.run_client_test(sim, test_run, client_name, this.run)
    }
  }
  async run_client_test(
    host: Simulation,
    test_run: ITestRun,
    client_name: string,
    run: AsyncClientTestFunc,
  ) {
    const test_id = await host.start_test(test_run.suite_id, test_run.name, test_run.desc)
    const test: Test = new Test(host, test_run.suite_id, test_run.suite, test_id)
    test.result.pass = true

    const client = await test.start_client(client_name)
    await run(test, client)
    await host.end_test(test)
  }
}

export class TestSpec implements ITestSpec {
  name: string
  description: string
  always_run: boolean
  run: AsyncTestFunc
  client?: IClient
  constructor(opts: {
    name: string
    description: string
    always_run: boolean
    run: AsyncTestFunc
    client?: IClient
  }) {
    this.name = opts.name
    this.description = opts.description
    this.always_run = opts.always_run
    this.run = opts.run
    this.client = opts.client
  }

  async run_test(simulation: Simulation, suite_id: SuiteID, suite: Suite) {
    const test_run: ITestRun = {
      suite_id: suite_id,
      suite: suite,
      name: this.name,
      desc: this.description,
      always_run: this.always_run,
    }
    await run_test(simulation, test_run, this.client, this.run)
  }
}

export const run_test = async (
  host: Simulation,
  test: ITestRun,
  client: IClient | undefined,
  func: AsyncTestFunc,
) => {
  const test_id = await host.start_test(test.suite_id, test.name, test.desc)
  const t: Test = new Test(host, test.suite_id, test.suite, test_id)
  t.result.pass = true

  await func(t, client)

  await host.end_test(t)
}

export class TwoClientTestSpec implements I2ClientTestSpec {
  name: string
  description: string
  always_run: boolean
  run: AsyncTwoClientsTestFunc
  constructor(opts: {
    name: string
    description: string
    always_run: boolean
    run: AsyncTwoClientsTestFunc
  }) {
    this.name = opts.name
    this.description = opts.description
    this.always_run = opts.always_run
    this.run = opts.run
  }

  async run_test(sim: Simulation, suite_id: SuiteID, suite: Suite) {
    const clients = await sim.client_types()
    for (const client of clients) {
      const client_name = client.name
      for (const client2 of clients) {
        const client_2_name = client2.name
        const name = client_test_name(this.name, client_name)
        const test_run: ITestRun = {
          suite_id,
          suite,
          name: name + ' --> ' + client_2_name,
          desc: this.description,
          always_run: this.always_run,
        }
        await this.run_2_client_test(sim, test_run, client_name, client_2_name, this.run)
      }
    }
  }

  async run_2_client_test(
    host: Simulation,
    test_run: ITestRun,
    client_a_name: string,
    client_b_name: string,
    run: AsyncTwoClientsTestFunc,
  ) {
    const test_id = await host.start_test(test_run.suite_id, test_run.name, test_run.desc)
    const test: Test = new Test(host, test_run.suite_id, test_run.suite, test_id)
    test.result.pass = true

    const client_a = await test.start_client(client_a_name)
    const client_b = await test.start_client(client_b_name)
    await run(test, client_a, client_b)
    await host.end_test(test)
  }
}

export class NClientTestSpec implements Testable {
  name: string
  description: string
  always_run: boolean
  run: AsyncNClientsTestFunc
  constructor(opts: {
    name: string
    description: string
    always_run: boolean
    run: AsyncNClientsTestFunc
  }) {
    this.name = opts.name
    this.description = opts.description
    this.always_run = opts.always_run
    this.run = opts.run
  }
  async run_test(simulation: Simulation, suite_id: SuiteID, suite: Suite) {
    let clients = await simulation.client_types()
    if (clients.length === 1) {
      clients = [clients[0], clients[0]]
    }
    const clientNames = clients.map((client) => client.name)
    const test_run: ITestRun = {
      suite_id,
      suite,
      name: this.name + '-->' + clients.map((c) => c.name).join(' + '),
      desc: this.description,
      always_run: this.always_run,
    }
    await this.run_n_client_test(simulation, test_run, clientNames, this.run)
  }
  async run_n_client_test(
    host: Simulation,
    test_run: ITestRun,
    clientNames: string[],
    run: AsyncNClientsTestFunc,
  ) {
    const test_id = await host.start_test(test_run.suite_id, test_run.name, test_run.desc)
    const test: Test = new Test(host, test_run.suite_id, test_run.suite, test_id)
    test.result.pass = true
    const client_vec: IClient[] = await Promise.all(
      clientNames.map((name) => {
        return test.start_client(name)
      }),
    )
    await run(test, client_vec)
    await host.end_test(test)
  }
}
export class NetworkNClientTestSpec implements Testable {
  name: string
  description: string
  always_run: boolean
  run: AsyncNClientsTestFunc
  size: number
  constructor(opts: {
    name: string
    description: string
    always_run: boolean
    run: AsyncNClientsTestFunc
    size?: number
  }) {
    this.name = opts.name
    this.description = opts.description
    this.always_run = opts.always_run
    this.run = opts.run
    this.size = opts.size ?? 1
  }
  async run_test(simulation: Simulation, suite_id: SuiteID, suite: Suite) {
    const clients = await simulation.client_types()
    const clientNames = clients.map((client) => client.name)
    for await (const client of clients) {
      const _clientNames = [client.name]
      for (let i = 0; i < this.size; i++) {
        _clientNames.push(...clientNames)
      }
      while (_clientNames.length < 4) {
        _clientNames.push(..._clientNames)
      }
      const test_run: ITestRun = {
        suite_id,
        suite,
        name: `${this.name} +--> [${client.name}] (network size: ${_clientNames.length})`,
        desc: this.description,
        always_run: this.always_run,
      }
      await this.run_n_client_test(simulation, test_run, _clientNames, this.run)
    }
  }
  async run_n_client_test(
    host: Simulation,
    test_run: ITestRun,
    clientNames: string[],
    run: AsyncNClientsTestFunc,
  ) {
    const test_id = await host.start_test(test_run.suite_id, test_run.name, test_run.desc)
    const test: Test = new Test(host, test_run.suite_id, test_run.suite, test_id)
    test.result.pass = true
    const client_vec: IClient[] = await Promise.all(
      clientNames.map((name) => {
        return test.start_client(name)
      }),
    )
    await run(test, client_vec)
    await host.end_test(test)
  }
}
