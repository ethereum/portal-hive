#!/bin/bash

# Immediately abort the script on any error encountered
set -e

./trin-main --version | tail -1 | sed "s/ /\//g"
