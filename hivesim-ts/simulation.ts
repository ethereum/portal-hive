import { TestMatcher } from "./testmatch"
import { ClientDefinition, StartNodeResponse, SuiteID, TestID, TestRequest } from "./types"
export class Simulation {
    default(): Simulation {
        return new Simulation()
    }
    url: string
    m: TestMatcher
    constructor() {
        if (!process.env.HIVE_SIMULATOR) {
            throw new Error("HIVE_SIMULATOR environment variable not set")
        }
        this.url = process.env.HIVE_SIMULATOR
        this.m = new TestMatcher("", "", "")
    }

    async start_suite(name: string, description: string): Promise<SuiteID> {
        const body: TestRequest = {name, description}
        const url = this.url + '/testsuite'
        const response = await fetch(url, {body: JSON.stringify(body), method: 'POST'})
        const json: SuiteID = await response.json()
        return json
    }

    async end_suite(test_suite: SuiteID) {
        const url = this.url + '/testsuite/' + test_suite
        const response = await fetch(url, {method: 'DELETE'})
        return response
    }

    async start_test(test_suite: SuiteID, name: string, description: string): Promise<SuiteID> {
        let url = this.url + '/testsuite/' + test_suite + '/test'
        const response = await fetch(url, {body: JSON.stringify({name, description}), method: 'POST'})
        const testId: TestID = await response.json()
        return testId
    }

    async end_test(test_suite: SuiteID, test: SuiteID) {
        let url = this.url + '/testsuite/' + test_suite + '/test/' + test
        const response = await fetch(url, {method: 'DELETE'})
        return response
    }

    async start_client(test_suite: SuiteID, test: SuiteID, client_type: string): Promise<[string, string]> {
        let url = this.url + '/testsuite/' + test_suite + '/test/' + test + '/client'

        let config = {
            client: client_type,
        }
        let form = {
            name: "config",
            value: config
        }
        const resp = await fetch(url, {body: JSON.stringify(form), method: 'POST'})
        const startNode: StartNodeResponse = await resp.json()
        return [startNode.id, startNode.ip]    
    }

    async client_types(): Promise<ClientDefinition[]> {
        let url = this.url + '/clients'
        const response = await fetch(url)
        const client_definition: ClientDefinition[] = await response.json()
        return client_definition
    }
}