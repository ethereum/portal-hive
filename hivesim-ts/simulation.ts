import { Test } from "./testapi"
import { TestMatcher } from "./testmatch"
import { ClientDefinition, StartNodeResponse, SuiteID, TestID, TestRequest } from "./types"
export class Simulation {
    url: string
    m: TestMatcher
    constructor(url: string) {
        this.url = url
        this.m = new TestMatcher("", "", "")
    }

    async start_suite(name: string, description: string): Promise<SuiteID> {
        const body: TestRequest = {name, description}
        const url = this.url + '/testsuite'
        const response = await fetch(url, {body: JSON.stringify(body), method: 'POST'})
        const suite_id: SuiteID = await response.json()
        return suite_id
    }

    async end_suite(test_suite: SuiteID) {
        const url = this.url + '/testsuite/' + test_suite
        await fetch(url, {
            method: 'POST',
            body: JSON.stringify({
                pass: true,
                details: 'all tests passed',
            })
        })
        const response = await fetch(url, {method: 'DELETE'})
        return response
    }

    async start_test(test_suite: SuiteID, name: string, description: string): Promise<SuiteID> {
        let url = this.url + '/testsuite/' + test_suite + '/test'
        const body: TestRequest = {name, description}
        const request: RequestInit = {
            body: JSON.stringify(body),
            method: 'POST',
        }
        const response = await fetch(url, request)
        const testId: TestID = await response.json()
        return testId
    }

    async end_test(test: Test) {
        let url = this.url + '/testsuite/' + test.suite_id + '/test/' + test.test_id
        const request = {
            method: 'POST',
            body: JSON.stringify(test.result)
        }
        const response = await fetch(url, request)
        return response
    }

    async start_client(test_suite: SuiteID, test: SuiteID, client_type: string): Promise<[string, string]> {
        let url = this.url + '/testsuite/' + test_suite + '/test/' + test + '/node'

        let config = {
            client: client_type,
        }
        const form = new FormData()
        form.append('config', JSON.stringify(config))

        const request: RequestInit = {
            body: form,
            method: 'POST',
            
        }
        const resp = await fetch(url, request)
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