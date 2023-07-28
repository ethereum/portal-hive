#!/bin/bash

# Immediately abort the script on any error encountered
set -e

IP_ADDR=$(hostname -i | awk '{print $1}')

# if HIVE_PRIVATE_KEY isn't set or doesn't exist do y, else do z
if [ -z ${HIVE_PRIVATE_KEY+x} ]; then
  node /ultralight/packages/cli/dist/index.js --bindAddress="$IP_ADDR:9000" --dataDir="./data" --rpcPort=8545
else
  node /ultralight/packages/cli/dist/index.js --bindAddress="$IP_ADDR:9000" --dataDir="./data" --rpcPort=8545 --pk=${HIVE_PRIVATE_KEY}
fi
