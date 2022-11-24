#!/bin/bash

# Immediately abort the script on any error encountered
set -e

RUST_LOG=debug TRIN_INFURA_PROJECT_ID="your-key-here" ./trin-main --web3-transport http --web3-http-address 0.0.0.0:8545
