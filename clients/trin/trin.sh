#!/bin/bash

# Immediately abort the script on any error encountered
set -e

IP_ADDR=$(hostname -i | awk '{print $1}')

RUST_LOG_STYLE=never RUST_LOG=debug TRIN_INFURA_PROJECT_ID="your-key-here" trin --web3-transport http --web3-http-address http://0.0.0.0:8545 --external-address "$IP_ADDR":9000
