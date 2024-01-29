#!/bin/bash

# Immediately abort the script on any error encountered
set -e

IP_ADDR=$(hostname -i | awk '{print $1}')
# Providing atrusted block root is required currently to enable the beacon network.
# It can be a made up value for now as tests are not doing any sync.
FLAGS="--trusted-block-root:0x0000000000000000000000000000000000000000000000000000000000000000"

if [ "$HIVE_CLIENT_PRIVATE_KEY" != "" ]; then
    FLAGS="$FLAGS --netkey-unsafe=0x$HIVE_CLIENT_PRIVATE_KEY"
fi

# Fluffy runs all networks by default, so we can not configure to run networks individually
fluffy --rpc --rpc-address="0.0.0.0" --nat:extip:"$IP_ADDR" --network=none --log-level="debug" $FLAGS
