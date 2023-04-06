[Overview] | [Hive Commands] | [Simulators] | [Clients]

## What is Portal Hive?

Portal Hive is a system for running integration tests against Portal Network clients.
It is a forked version from [Ethereum Hive].

In hive, integration tests are called 'simulations'. A simulation is controlled by a
program (the 'simulator') written in any language. The simulator launches clients and
contains test logic. It reports test results back to hive, where they are aggregated for
display in a web browser.

What makes hive different from other, generic CI infrastructure is the tight integration
of Portal Network clients and their features. Simulator programs usually don't need to care
about the differences between client implementations because hive provides a common
interface to launch and configure them all. At this time, clients can be configured for
any Portal Network definition. You can find more information about client configuration in the [client
documentation].

Ethereum Foundation operates a public instance of Portal Hive to check for peer-to-peer networking spec compliance, and user API support for most
Portal Network client implementations. You can find the latest test results at
<https://portal-hive.ethdevops.io/>.

## Overview of available simulators

This is an overview of some of the simulators which are currently implemented and running
continuously on the production hive instance:

- `rpc-compat`: The RPC simulator configures a client and runs
  various tests against the web3 JSON-RPC interface. These tests ensure that the client is
  compatible with the portal network [JSON-RPC specs].

## How it works

This section explains how a single simulation run works.

For a single run, the user provides the name of the simulator to run, and a set of client
names to run against. For example:

    ./hive --sim rpc-compat --client trin,fluffy,ultralight

Hive first builds simulator and client images using docker. It expects a Dockerfile in the
`./simulators/rpc-compat` directory as well as a Dockerfile for each client (in
`./clients/*/Dockerfile`).

While the simulator build must always work without error, it's OK for some client builds
to fail as long as one of them succeeds. This is because client code pulled from the
respective upstream repositories may occasionally fail to build.

![hive simulation docker containers](./img/sim-overview.svg)

Once all images are built, the simulator program is launched in a docker container. The
`HIVE_SIMULATOR` environment variable contains the HTTP server URL of the hive controller.
The [hive simulation API] can be accessed at this URL. The simulator launches clients and
reports test results through the API.

When the simulator requests a client instance, the hive controller launches a new docker
container using the built client image. The client container entry point receives
configuration through environment variables and files provided by the simulator.
The client is now expected to launch its network endpoints for RPC and p2p communication.

When the client has finished starting, the simulator program communicates with it on the
RPC and p2p endpoints. More than one client may be launched, and the clients can also
communicate with each other.

During the simulation run, information about 'test suites' and their test cases must be
provided by the simulator via the simulation API. The hive controller collects this
information in a JSON file. It also collects client logs as well as the output of the
simulator program. All files are written to the results directory (`./workspace/logs`).

When the simulator program exits, the simulator container and all client containers are
stopped and removed. The `hive` command then exits as well.

## Running a client built from source

To run a client built from source, we just hack portal-hive a little bit. Here is an example for how
to run against a local build of trin:

Build a local image with a known tag, like `trin-dev`, using:

    cd $TRIN_REPO
    docker build --tag trin-dev --file docker/Dockerfile .

Then modify the portal-hive `Dockerfile` to use the local image like so:

```patch
--- a/clients/trin/Dockerfile
+++ b/clients/trin/Dockerfile
@@ -1,6 +1,6 @@
 ARG branch=latest

-FROM portalnetwork/trin:$branch
+FROM trin-dev

 ADD trin.sh /trin.sh
 RUN chmod +x /trin.sh
 ```

 Now, running hive normally will run against your local build of trin.

## Simulation Output Files

The results of simulation runs are stored in the 'result directory'. For every test suite
executed by a simulator, a JSON file like the following is created:

    {
      "id": 0,
      "name": "rpc-compat",
      "description": "This test suite verifies that...",
      "clientVersions": {
        "fluffy": "",
        "trin": ""
      },
      "simLog": "1612356621-simulator-a9a2e71a6aabe509bbde35c79e7f0ed9c259a642c19ba0da6167fa9efd0ea5a1.log"
      "testCases": {
        "1": {
          "name": "client launch (fluffy)",
          "description": "This test launches the client and collects its logs.",
          "start": "2021-02-03T12:50:21.77396767Z",
          "end": "2021-02-03T12:51:56.080650164Z",
          "summaryResult": {
            "pass": true,
            "details": ""
          },
          "clientInfo": {
            "893a6ea2": {
              "ip": "172.17.0.4",
              "name": "fluffy",
              "instantiatedAt": "2021-02-03T12:51:04.371913809Z",
              "logFile": "fluffy/client-893a6ea2.log"
            }
          }
        }
      }
    }

The result directory also contains log files of simulator and client output.

[hive simulation API]: ./simulators.md#simulation-api-reference
[client documentation]: ./clients.md
[Overview]: ./overview.md
[Hive Commands]: ./commandline.md
[Simulators]: ./simulators.md
[Clients]: ./clients.md
[Ethereum Hive]: https://github.com/ethereum/hive
[JSON-RPC specs]: https://playground.open-rpc.org/?schemaUrl=https://raw.githubusercontent.com/ethereum/portal-network-specs/assembled-spec/jsonrpc/openrpc.json&uiSchema%5BappBar%5D%5Bui:splitView%5D=false&uiSchema%5BappBar%5D%5Bui:input%5D=false&uiSchema%5BappBar%5D%5Bui:examplesDropdown%5D=false
