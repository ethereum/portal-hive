#!/bin/bash

# Immediately abort the script on any error encountered
set -e

IP_ADDR=$(hostname -i | awk '{print $1}')

DEBUG=* node /ultralight/packages/cli/dist/index.js --bindAddress="$IP_ADDR:9000" --dataDir="./data" --rpcPort=8545
