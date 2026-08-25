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

check_hash 6d77cedc36b9bd82fe05481e48872287496163187363567fb9c83f6585799655 \
  truelearner/crates/core/src/lib.rs
check_hash 40f3ae01cae1afc3cf8c4481a41db5bcb8d508258db3ee701238e0199bf6b3a9 \
  truelearner/crates/core/src/mechanics.rs
check_hash 8bc8529c190ec653b378efe38359c0865dabca343d0a33d1c3ba53e67d5d9278 \
  truelearner/crates/core/Cargo.toml
check_hash e46d83ff13c2d21d9a25170935e7d9c69579531e1cac2f93d03b31649fced5a0 \
  experiments/arms/cl0-ordinary-cell-lifetime/src/main.rs

core=truelearner/crates/core/src/lib.rs
mechanics=truelearner/crates/core/src/mechanics.rs
add_cell_body="$(sed -n '/    pub fn add_cell(/,/    pub fn add_arrow(/p' "$core")"
cell_decay_body="$(sed -n '/    fn elapse_cells_to(/,/    fn propose_local_arrows(/p' "$core")"
cell_death_body="$(sed -n '/fn decay_cell_structure(/,/fn relax_participation(/p' "$core")"

grep -Fq 'cl0 = ["pd1"]' truelearner/crates/core/Cargo.toml
printf '%s\n' "$add_cell_body" | grep -Fq 'position(|cell| !cell.live)'
printf '%s\n' "$add_cell_body" | grep -Fq 'let id = CellId(self.cell_slots.len() as u64);'
printf '%s\n' "$add_cell_body" | grep -Fq 'prior.generation'
printf '%s\n' "$add_cell_body" | grep -Fq 'CellSlot(index)'
printf '%s\n' "$cell_decay_body" | grep -Fq 'LOCAL_DECAY_PERIOD'
printf '%s\n' "$cell_decay_body" | grep -Fq 'cell.decay_load'
printf '%s\n' "$cell_decay_body" | grep -Fq 'work.cell_deallocations'
printf '%s\n' "$cell_death_body" | grep -Fq 'cell.resistance = cell.resistance.saturating_sub(amount);'
printf '%s\n' "$cell_death_body" | grep -Fq 'cell.generation = Generation(cell.generation.0.wrapping_add(1));'
printf '%s\n' "$cell_death_body" | grep -Fq 'cell.live = false;'
test "$(printf '%s\n' "$cell_decay_body$cell_death_body" | grep -Ec 'for .*arrow|incident|degree|orphan|retain\(' || true)" -eq 0

grep -Fq 'target_generation: Generation' "$core"
grep -Fq 'target_physical: u64' "$core"
grep -Fq 'target.id != spike.target' "$core"
grep -Fq 'to.generation != arrow.target_generation' "$core"
grep -Fq 'target.generation != arrow.target_generation' "$core"
grep -Fq 'target_generations: Vec<super::Generation>' "$mechanics"
grep -Fq 'decay_loads: Vec<u64>' "$mechanics"

if grep -Eq 'TemporaryCell|ContactCell|EphemeralCell|expires_at|remaining_ticks|gc_flag|orphan_flag' \
  "$core" "$mechanics"; then
  printf 'forbidden supplied CELL class/lifetime token found\n' >&2
  exit 1
fi

printf 'implementation_static=pass\n'
printf 'ordinary_cell_class_only=true\n'
printf 'local_phase_free_decay=true\n'
printf 'fresh_identity_reused_slot=true\n'
printf 'generation_advanced_on_death=true\n'
printf 'stale_endpoint_generation_guard=true\n'
printf 'incident_cascade_deletion=false\n'
printf 'CL0_ORDINARY_CELL_LIFETIME_IMPLEMENTATION_STATIC_PASS_V1\n'
