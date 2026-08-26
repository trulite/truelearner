#!/bin/sh
set -eu

if output=$(CARGO_TARGET_DIR=/tmp/truelearner-harness-privacy-target \
    cargo check --locked \
    --manifest-path factory-artifacts/harness-privacy-check/Cargo.toml 2>&1)
then
    echo "external crate unexpectedly imported Body" >&2
    exit 1
fi

printf '%s\n' "$output" | rg -q 'no `Body` in the root'
