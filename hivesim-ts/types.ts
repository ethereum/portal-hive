export type SuiteID = number;
export type TestID = number;

// StartNodeReponse is returned by the client startup endpoint.
export interface StartNodeResponse {
    id: string; // Container ID.
    ip: string; // IP address in bridge network
}

// ClientMetadata is part of the ClientDefinition and lists metadata
export interface ClientMetadata {
    roles: string[];
}

// ClientDefinition is served by the /clients API endpoint to list the available clients
export interface ClientDefinition {
    name: string;
    version: string;
    meta: ClientMetadata;
}

export interface TestRequest {
    name: string;
    description: string;
}

// Describes the outcome of a test.
export interface TestResult {
    pass: boolean;
    details: string;
}

export interface TestMatcher {
    suite: string;
    test: string;
    pattern: string;
  }

export interface ExecInfo {
    Stdout: string;
    Stderr: string;
    ExitCode: number;
}

export interface ClientMetadata {
    roles: string[];
}

export interface IClientDefinition {
    name: string;
    version: string;
    meta: ClientMetadata;
}

export class ClientDefinition implements IClientDefinition {
    hasRole = (role: string) => {
        for (const [_, m] of this.meta.roles ) {
            if (m === role) {
                return true;
            }
        }
        return false;
    }
}
