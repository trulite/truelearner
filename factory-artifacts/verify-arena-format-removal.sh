#!/bin/sh
set -eu

test ! -e truelearner/crates/arena-format/Cargo.toml

if rg -n --hidden --glob '!**/target/**' -F \
    -e truelearner_arena_format \
    -e truelearner-arena-format \
    -e ArenaBody \
    -e ArenaId \
    -e '.read(0).body' \
    -e '.read().body' \
    truelearner academy
then
    echo "arena format or body-shaped observation remains in active code" >&2
    exit 1
fi
