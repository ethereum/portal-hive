#!/bin/bash

# Immediately abort the script on any error encountered
set -e

node /app/packages/cli/dist/index.js --bindAddress="127.0.0.1:9000" --dataDir="./data"
