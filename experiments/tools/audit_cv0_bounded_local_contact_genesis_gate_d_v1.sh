#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../.."

check_hash() {
  local expected="$1"
  local path="$2"
  local actual
  actual="$(sha256sum "$path" | awk '{print $1}')"
  test "$actual" = "$expected" || {
    printf 'hash mismatch: %s\nexpected %s\nactual   %s\n' \
      "$path" "$expected" "$actual" >&2
    exit 1
  }
}

check_hash b6b7f2a47818d84ac2fd69aab466f5f917e6d3ba7cfc8f8c5db4ce91b97fbae5 \
  truelearner/crates/core/src/lib.rs
check_hash 4cb6d665d738cdea61f928975fa34ddf89d62aa9150420748d94d574ed731aeb \
  truelearner/crates/core/Cargo.toml

core=truelearner/crates/core/src/lib.rs
add_cell_body="$(sed -n '/    pub fn add_cell(/,/    pub fn add_arrow(/p' "$core")"
cell_decay_body="$(sed -n '/    fn decay_cell(/,/    fn require_cell(/p' "$core")"
arrow_decay_body="$(sed -n '/fn decay_arrow(/,/fn relax_participation(/p' "$core")"

printf '%s\n' "$add_cell_body" | grep -Fq 'let id = CellId(self.cell_slots.len() as u64);'
printf '%s\n' "$add_cell_body" | grep -Fq 'resistance: spec.resistance,'
printf '%s\n' "$add_cell_body" | grep -Fq 'live: spec.resistance > 0,'

test "$(printf '%s\n' "$add_cell_body" | grep -c 'position(|cell| !cell.live)' || true)" -eq 0
test "$(printf '%s\n' "$cell_decay_body" | grep -Ec 'resistance|\.live|generation')" -eq 0
test "$(grep -Ec 'cell\.live[[:space:]]*=[[:space:]]*false|target\.live[[:space:]]*=[[:space:]]*false' "$core")" -eq 0
test "$(grep -Ec 'cells\.remove|cell_slots\[[^]]+\][[:space:]]*=[[:space:]]*None' "$core")" -eq 0

printf '%s\n' "$arrow_decay_body" | grep -Fq 'arrow.live = false;'
printf '%s\n' "$arrow_decay_body" | grep -Fq 'arrow.generation = Generation('
grep -Fq 'work.physical_deallocations = work.physical_deallocations.saturating_add(1);' "$core"

printf 'gate_d=negative\n'
printf 'cell_construction_sets_live_from_resistance=true\n'
printf 'cell_decay_relaxes_activation_only=true\n'
printf 'cell_lifetime_evolution=false\n'
printf 'cell_deallocation=false\n'
printf 'cell_slot_reuse=false\n'
printf 'arrow_deallocation=true\n'
printf 'contact_genesis_implemented=false\n'
printf 'runtime_selection_gates_constructed=false\n'
printf 'CV0_BOUNDED_LOCAL_CONTACT_GENESIS_GATE_D_STATIC_NEGATIVE_V1\n'
