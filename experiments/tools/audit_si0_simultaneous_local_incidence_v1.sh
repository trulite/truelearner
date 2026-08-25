#!/usr/bin/env bash
set -euo pipefail

root=${1:-.}
core="$root/truelearner/crates/core/src/lib.rs"
mechanics="$root/truelearner/crates/core/src/mechanics.rs"
evaluator="$root/experiments/arms/si0-simultaneous-local-incidence/src/main.rs"

test -f "$core"
test -f "$mechanics"
test -f "$evaluator"

candidate=$(sed -n '/fn propagate_si0/,/fn pop_scheduled/p' "$core")

printf '%s\n' "$candidate" | grep -q 'drain_minimum_wave'
printf '%s\n' "$candidate" | grep -q 'causal_wave.saturating_add(1)'
printf '%s\n' "$candidate" | grep -q 'state.saturating_add(impulse)'
printf '%s\n' "$candidate" | grep -q 'Vec<(CellId, Vec<Spike>)>'
printf '%s\n' "$candidate" | grep -q 'SI0 defines Drive incidence only'

if printf '%s\n' "$candidate" | grep -Eq 'BTreeMap<CellId|sort_by_key\(.*(target_id|arrow_id|physical_id)|cycle detector|hop_count|max_wave|path_id|route_id|predecessor|previous'; then
    printf 'forbidden SI0 causal ordering/routing construct found\n' >&2
    exit 1
fi

if printf '%s\n' "$candidate" | grep -Eq 'serial[[:space:]]*(<|>|==)|causal_wave:[[:space:]]*.*(serial|physical|target)'; then
    printf 'serial or handle data defines SI0 causation\n' >&2
    exit 1
fi

grep -q 'type OrderKey = (i64, i32, u64, u64, u64, u64)' "$mechanics"
grep -q 'let prefix = (first.0.arrival_tick, first.0.phase, first.0.causal_wave)' "$mechanics"
grep -q 'features = \["si0"\]' "$root/experiments/arms/si0-simultaneous-local-incidence/Cargo.toml"
if grep -Eq 'TransmissionMode::Modulatory|reward|credit|cycle detector|hop_count|max_wave|path_id|route_id' "$evaluator"; then
    printf 'out-of-scope semantic or Modulatory behavior found in SI0 evaluator\n' >&2
    exit 1
fi

printf 'SI0_SIMULTANEOUS_LOCAL_INCIDENCE_STATIC_AUDIT_OK\n'
