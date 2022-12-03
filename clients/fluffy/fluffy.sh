#!/bin/bash

# Immediately abort the script on any error encountered
set -e

IP_ADDR=$(hostname -i | awk '{print $1}')

fluffy --rpc --rpc-address="0.0.0.0" --nat:extip:"$IP_ADDR"
