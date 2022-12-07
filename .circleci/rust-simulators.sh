#!/bin/bash

failed=""
sims=$(find simulators -name Cargo.toml)
for d in $sims; do
    d="$(dirname "$d")"
    echo "Lint, build, test $d"
    ( cd "$d" || exit;
    cargo fmt --all -- --check;
    cargo clippy --all --all-targets --all-features --no-deps -- --deny warnings;
    cargo test --workspace -- --nocapture;
    )
    status=$?
    if [ $status -ne 0 ]; then
        failed="y"
        echo "failed with exit status $status"
    fi
done

# Exit with non-zero status code if any build failed.
if [ -n "$failed" ]; then
    exit 1
fi
