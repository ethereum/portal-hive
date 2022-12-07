[Overview] | [Hive Commands] | [Simulators] | [Clients]

## Portal Hive Clients

This page explains how client containers work in Hive.

Clients are docker images which can be instantiated by a simulation. A client definition
consist of a Dockerfile and associated resources. Client definitions live in
subdirectories of `clients/` in the hive repository.

When hive runs a simulation, it first builds all client docker images using their
Dockerfile, i.e. it basically runs `docker build .` in the client directory. Since most
client definitions wrap an existing Ethereum client, and building the client from source
may take a long time, it is usually best to base the hive client wrapper on a pre-built
docker image from Docker Hub.

Client Dockerfiles should support an optional argument named `branch`, which specifies the
requested client version. This argument can be set by users by appending it to the client
name like:

    ./hive --sim my-simulation --client trin:latest,fluffy:06.12.22

See the [go-ethereum client definition][trin-docker] for an example of a client
Dockerfile.

### hive.yaml

Hive reads additional metadata from the `hive.yaml` file in the client directory (next to
the Dockerfile). Currently, the only purpose of this file is specifying the client's role
list:

    roles:
      - "portal"
      - "other"

The role list is available to simulators and can be used to differentiate between clients
based on features. Declaring a client role also signals that the client supports certain
role-specific environment variables and files. If `hive.yml` is missing or doesn't declare
roles, the `portal` role is assumed.

### /version.txt

Client Dockerfiles are expected to generate a `/version.txt` file during build. Hive reads
this file after building the container and attaches version information to the output of
all test suites in which the client is launched.

### /hive-bin

Executables placed into the `/hive-bin` directory of the client container can be invoked
through the simulation API.

## Client Lifecycle

When the simulation requests a client instance, hive creates a docker container from the
client image. The simulator can customize the container by passing environment variables
with prefix `HIVE_`. It may also upload files into the container before it starts. Once
the container is created, hive simply runs the entry point defined in the `Dockerfile`.

For all client containers, hive waits for TCP port 8545 to open before considering the
client ready for use by the simulator. This port is configurable through the
`HIVE_CHECK_LIVE_PORT` variable, and the check can be disabled by setting it to `0`. If
the client container does not open this port within a certain timeout, hive assumes the
client has failed to start.

Environment variables and files interpreted by the entry point define a 'protocol' between
the simulator and client. While hive itself does not require support for any specific
variables or files, simulators usually expect client containers to be configurable in
certain ways. In order to run tests against multiple Ethereum clients, for example, the
simulator needs to be able to configure all clients for a specific blockchain and make
them join the peer-to-peer network used for testing.

## Portal Client Requirements

This section describes the requirements for the `portal` client role.

Portal clients must provide JSON-RPC over HTTP on TCP port 8545. They may also support
JSON-RPC over WebSocket on port 8546, but this is not strictly required.

### Environment

Clients must support the following environment variables. The client's entry point script
may map these to command line flags or use them generate a config file, for example.

| Variable                   | Value         |                                                |
|----------------------------|---------------|------------------------------------------------|
| `HIVE_LOGLEVEL`            | 0 - 5         | configures log level of client                 |
| `HIVE_BOOTNODE`            | ENR           | makes client connect to another node           |

[trin-docker]: ../clients/trin/Dockerfile
[Overview]: ./overview.md
[Hive Commands]: ./commandline.md
[Simulators]: ./simulators.md
[Clients]: ./clients.md
