import { Client, HttpClient } from "jayson";
import { Simulation } from "./simulation";
import { ClientDefinition, SuiteID, TestID, TestResult } from "./types";
import { client_test_name } from "./utils";

type AsyncTestFunc = (test: Test, client?: IClient) => Promise<void>;
type AsyncClientTestFunc = (test: Test, client: IClient) => Promise<void>;
type AsyncTwoClientsTestFunc = (test: Test, client_a: IClient, client_b: IClient) => Promise<void>;
type AsyncNClientsTestFunc = (test: Test, clients: IClient[]) => Promise<void>;

interface Testable {
  run_test: (
    simulation: Simulation,
    suite_id: SuiteID,
    suite: Suite
  ) => Promise<any>;
}

interface ISuite {
  name: string;
  description: string;
  tests: Testable[];
}
export class Suite implements ISuite{
  name: string;
  description: string;
  tests: Testable[];
  constructor(name: string, description: string) {
    this.name = name;
    this.description = description;
    this.tests = [];
  }
  add(test: Testable) {
    this.tests.push(test);
  }
  // async run(host: Simulation, ...suites: Suite[]) {
  //   for (const s of suites) {
  //     await this.runSuite(host, s);
  //   }
  //   return null;
  // }

  // // RunSuite runs all tests in a suite.
  // async runSuite(host: Simulation, suite: Suite): Promise<void> {
  //   if (!host.m.match(suite.name, "")) {
  //     console.log(
  //       `skipping suite ${suite.name} because it doesn't match test pattern ${host.m.pattern}`
  //     );
  //     return;
  //   }

  //   const suiteId = await host.start_suite(suite.name, suite.description);
  //   await Promise.all(
  //     suite.tests.map(async (test) => {
  //       return test(host, suiteId, suite);
  //     })
  //   );
  // }

  // async mustRunSuite(host: Simulation, suite: Suite): Promise<void> {
  //   try {
  //     await this.runSuite(host, suite);
  //   } catch (err: any) {
  //     console.log(`error running suite ${suite.name}: ${err}`);
  //     throw err;
  //   }
  // }
}

interface IClient {
  kind: string;
  container: string;
  ip: string;
  rpc: HttpClient;
  test: ITest;
}
interface ITestRun {
  suite_id: SuiteID;
  suite: Suite;
  name: string;
  desc: string;
  always_run: boolean;
}

interface ITest {
  sim: Simulation;
  suite_id: SuiteID;
  suite: Suite;
  test_id: TestID;
  result: TestResult;
}

export class Test {
  sim: Simulation;
  suite: Suite;
  suite_id: SuiteID;
  test_id: TestID;
  result: TestResult;

  constructor(
    sim: Simulation,
    suite_id: SuiteID,
    suite: Suite,
    test_id: TestID,
    result: TestResult = {
      details: '',
      pass: false,
    }
  ) {
    this.sim = sim;
    this.suite_id = suite_id;
    this.suite = suite;
    this.test_id = test_id;
    this.result = result;
  }

  async start_client(client_type: string): Promise<IClient> {
    const [container, ip] = await this.sim.start_client(
      this.suite_id,
      this.test_id,
      client_type
    );
    const rpc_client = Client.http({ port: 8545, host: ip });
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
    };
    return client;
  }

  async run(spec: Testable) {
    await spec.run_test(this.sim, this.suite_id, this.suite);
  }

  fatal(message: string) {
    this.log_failure(message);
    this.fail();
  }

  log_failure(message: string) {
    console.log(message);
    this.result.details = message;
  }

  fail() {
    this.result.pass = false;
  }
}


interface IClientTestSpec extends Testable {
  name: string;
  description: string;
  always_run: boolean;
  run: AsyncClientTestFunc;
}
export class ClientTestSpec implements IClientTestSpec {
  name: string
  description: string
  always_run: boolean
  run: AsyncClientTestFunc
  constructor(name: string, description: string, always_run: boolean, run: AsyncClientTestFunc) {
    this.name = name;
    this.description = description;
    this.always_run = always_run;
    this.run = run;
  }
  async run_test(sim: Simulation, suite_id: SuiteID, suite: Suite) {
    const clients = await sim.client_types()
    for (const client of clients) {
      const client_name = client.name
      const test_run: ITestRun = {
        suite_id: suite_id,
        suite: suite,
        name: client_test_name(this.name, client_name),
        desc: this.description,
        always_run: this.always_run,
      }
      await this.run_client_test(sim, test_run, client_name, this.run)
    }
  }
  async run_client_test(host: Simulation, test_run: ITestRun, client_name: string, run: AsyncClientTestFunc) {
    const test_id = await host.start_test(test_run.suite_id, test_run.name, this.description)
    const test: Test = new Test(
      host,
      test_id,
      test_run.suite,
      test_run.suite_id
    )
    test.result.pass = true

    await test.start_client(client_name)
    
    await host.end_test(test.suite_id, test_id)
  }
}


interface ITestSpec extends Testable {
  name: string;
  description: string;
  always_run: boolean;
  run: AsyncTestFunc;
  client?: IClient
}


export class TestSpec implements ITestSpec {
  name: string
  description: string
  always_run: boolean
  run: AsyncTestFunc
  client?: IClient
  constructor(name: string, description: string, always_run: boolean, run: AsyncTestFunc, client?: IClient) {
    this.name = name;
    this.description = description;
    this.always_run = always_run;
    this.run = run;
    this.client = client;
  }


  async run_test(simulation: Simulation, suite_id: SuiteID, suite: Suite) {
    const test_run: ITestRun = {
      suite_id: suite_id,
      suite: suite,
      name: this.name,
      desc: this.description,
      always_run: this.always_run,
    }
    await run_test(
      simulation,
      test_run,
      this.client,
      this.run
    )
  }

}





const run_test = async (
  host: Simulation,
  test: ITestRun,
  client: IClient | undefined,
  func: AsyncTestFunc
) => {
  const test_id = await host.start_test(test.suite_id, test.name, test.desc)
  const t: Test = new Test(
    host,
    test_id,
    test.suite,
    test.suite_id
  )
  t.result.pass = true
  
  await func(t, client)

  await host.end_test(test.suite_id, test_id)
}

export class TwoClientTestSpec implements Testable {
  name: string
  description: string
  always_run: boolean
  run: AsyncTwoClientsTestFunc
  client_a: ClientDefinition
  client_b: ClientDefinition

  constructor(name: string, description: string, always_run: boolean, run: AsyncTwoClientsTestFunc, client_a: ClientDefinition, client_b: ClientDefinition) {
    this.name = name;
    this.description = description;
    this.always_run = always_run;
    this.run = run;
    this.client_a = client_a;
    this.client_b = client_b;
  }

  async run_test(simulation: Simulation, suite_id: SuiteID, suite: Suite) {
    const test_run: ITestRun = {
      suite_id: suite_id,
      suite: suite,
      name: this.name,
      desc: this.description,
      always_run: this.always_run,
    }
    await run_two_client_test(simulation, test_run, this.client_a, this.client_b, this.run) 
  }

  async run_two_client_test (
    host: Simulation,
    test_run: ITestRun,
    func: AsyncTwoClientsTestFunc
  ) {
    const test_id = await host.start_test(test_run.suite_id, test_run.name, test_run.desc)
    const test: Test = new Test(
      host,
      test_id,
      test_run.suite,
      test_run.suite_id,
    )
    test.result.pass = true
    const _client_a  =await test.start_client(this.client_a.name)
    const _client_b  =await test.start_client(this.client_b.name)
    await func(test, _client_a, _client_b)
    await host.end_test(test_run.suite_id, test_id)
  }
}

const run_two_client_test = async (
  host: Simulation,
  test_run: ITestRun,
  client_a: ClientDefinition,
  client_b: ClientDefinition,
  func: AsyncTwoClientsTestFunc
) => {
  const test_id = await host.start_test(test_run.suite_id, test_run.name, test_run.desc)
  const test: Test = new Test(
    host,
    test_id,
    test_run.suite,
    test_run.suite_id,
  )
  test.result.pass = true
  const _client_a  =  await test.start_client(client_a.name)
  const _client_b  =  await test.start_client(client_b.name)
  await func(test, _client_a, _client_b)
  await host.end_test(test_run.suite_id, test_id)
}


export class NClientTestSpec implements Testable {
  name: string
  description: string
  always_run: boolean
  run: AsyncNClientsTestFunc
  clients: ClientDefinition[]
  constructor(name: string, description: string, always_run: boolean, run: AsyncNClientsTestFunc, clients: ClientDefinition[]) {
    this.name = name;
    this.description = description;
    this.always_run = always_run;
    this.run = run;
    this.clients = clients;
  }
  async run_test(simulation: Simulation, suite_id: SuiteID, suite: Suite) {
    const test_run: ITestRun = {
      suite_id: suite_id,
      suite: suite,
      name: this.name,
      desc: this.description,
      always_run: this.always_run,
    }
    await run_n_client_test(simulation, test_run, this.clients, this.run)
  }
}

const run_n_client_test = async (
  host: Simulation,
  test_run: ITestRun,
  clients: ClientDefinition[],
  func: AsyncNClientsTestFunc
) => {
  const test_id = await host.start_test(test_run.suite_id, test_run.name, test_run.desc)
  const test: Test = new Test(
    host,
    test_id,
    test_run.suite,
    test_run.suite_id,
  )
  test.result.pass = true
  const client_vec: IClient[] = await Promise.all(clients.map((client) => {
    return test.start_client(client.name)
  }))
  for (const client of clients) {
    await test.start_client(client.name)
  }
  await func(test, client_vec)
  await host.end_test(test_run.suite_id, test_id)
}



