#!/bin/bash

# Immediately abort the script on any error encountered
set -e

IP_ADDR=$(hostname -i | awk '{print $1}')

# if CLIENT_PRIVATE_KEY isn't set or doesn't exist do y, else do z
if [ -z ${CLIENT_PRIVATE_KEY+x} ]; then
  RUST_LOG=debug TRIN_INFURA_PROJECT_ID="your-key-here" trin --web3-transport http --web3-http-address http://0.0.0.0:8545 --external-address "$IP_ADDR":9000 --bootnodes none
else
  RUST_LOG=debug TRIN_INFURA_PROJECT_ID="your-key-here" trin --web3-transport http --web3-http-address http://0.0.0.0:8545 --external-address "$IP_ADDR":9000 --bootnodes none --unsafe-private-key ${CLIENT_PRIVATE_KEY}
fi
