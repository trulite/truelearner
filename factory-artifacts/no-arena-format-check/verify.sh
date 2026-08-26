#!/bin/sh
set -eu

if output=$(CARGO_TARGET_DIR=/tmp/truelearner-no-arena-format-target \
    cargo check --locked \
    --manifest-path factory-artifacts/no-arena-format-check/Cargo.toml 2>&1)
then
    echo "external crate unexpectedly imported a private body type" >&2
    exit 1
fi

printf '%s\n' "$output" | rg -q 'no `Arena` in the root'
printf '%s\n' "$output" | rg -q 'no `ArenaBody` in the root'
printf '%s\n' "$output" | rg -q 'no `Body` in the root'
