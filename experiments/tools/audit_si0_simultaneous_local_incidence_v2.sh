#!/usr/bin/env bash
set -euo pipefail

root=${1:-.}

cd "$root"
printf '%s  %s\n' \
    d7d34bb477bc74657d8d1486d2c04fef759bb5f91ce5b08b805891f0bd75819c \
    truelearner/crates/core/Cargo.toml \
    f19a89ac92c12cc4910047021c8bdedfa42b4c4dc2f5c3fcfa83e2a0b2a4c978 \
    truelearner/crates/core/src/lib.rs \
    5f1172a0eaa0628d1775029c44e7a1b5bb2c4525c713b468f756a0705ef822a4 \
    truelearner/crates/core/src/mechanics.rs \
    | sha256sum --check --status

evaluator=experiments/arms/si0-simultaneous-local-incidence/src/main.rs
grep -q 'type WaveKey = (i64, i32, u64)' "$evaluator"
grep -q 'struct WaveObservation' "$evaluator"
grep -q 'incidences.sort()' "$evaluator"
grep -q 'fires.sort()' "$evaluator"
grep -q 'effects.sort()' "$evaluator"
grep -q 'SI0_SIMULTANEOUS_LOCAL_INCIDENCE_POSITIVE_V2' "$evaluator"

if sed -n '/fn normalize_trace/,/fn read_u32/p' "$evaluator" \
    | grep -Eq 'last_mut|chunks|CellId|ArrowId|origin_physical|serial'; then
    printf 'sequential ownership or handle ordering remains in v2 normalizer\n' >&2
    exit 1
fi

printf 'SI0_SIMULTANEOUS_LOCAL_INCIDENCE_V2_STATIC_AUDIT_OK\n'
