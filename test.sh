#!/bin/bash

#
# This is a little test-script, that can be used for some trial runs of clients.
#

HIVEHOME="./"

# Store results in temp
RESULTS="/tmp/TestResults"

FLAGS="--loglevel 4"
FLAGS="$FLAGS --results-root $RESULTS "
FLAGS="$FLAGS --sim.parallelism 1 --client.checktimelimit=20s"

echo "Running the quick'n'dirty version of the Hive tests, for local development"
echo "To the hive viewer up, you can do"
echo ""
echo "  cd $HIVEHOME/cmd/hiveview && ln -s /tmp/TestResults/ Results && python3 -m http.server"
echo ""
echo "And then visit http://localhost:8000/ with your browser. "
echo "Log-files and stuff are available in $RESULTS."
echo ""
echo ""


function run {
  echo "$HIVEHOME> $1"
  (cd $HIVEHOME && $1)
}

function testrpc {
  client=$1
  echo "$(date) Starting hive rpc-compat simulation [$client]"
  run "./hive --sim rpc-compat --client $client --sim.loglevel 5 $FLAGS"
}

mkdir $RESULTS

testrpc trin_latest
