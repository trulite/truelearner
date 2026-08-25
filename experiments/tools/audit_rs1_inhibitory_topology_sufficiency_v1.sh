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

check_hash 45dd6af368776d68574ff2b00dd4db109d469bfeedc99b57eb76ad6b26ca111c \
  truelearner/crates/core/src/lib.rs
check_hash 5d794eae058f5cdd896064b0a37a6dfb124d9d7b6d03f8cfa9c53651e58460ef \
  truelearner/crates/core/Cargo.toml
check_hash c3d1b95ea1f568702230a4bc31832f6575bc4d3db0c5c31f8454c62f595ca786 \
  experiments/arms/rs1-inhibitory-topology-sufficiency/src/main.rs
check_hash 677943e3b525101a5cdef8a0219a4a780e7d0cbc14fcf5a467c12f4fedba4016 \
  experiments/arms/rs1-inhibitory-topology-sufficiency/Cargo.toml
check_hash bb175daa3a22fc03e99d8ad8f0a462054fa468a025469af4babadaf4ac6d8cee \
  academy/docs/rs1_inhibitory_topology_sufficiency_protocol_v1.md

core=truelearner/crates/core/src/lib.rs
manifest=experiments/arms/rs1-inhibitory-topology-sufficiency/Cargo.toml
evaluator=experiments/arms/rs1-inhibitory-topology-sufficiency/src/main.rs

grep -Fq 'features = ["rs0"]' "$manifest"
if grep -Fq 'features = ["ce0"]' "$manifest"; then
  printf 'RS1 must not enable CE0 efficacy plasticity\n' >&2
  exit 1
fi

grep -Fq 'coupling: i32' "$core"
grep -Fq 'target.state.saturating_add(spike.impulse)' "$core"
grep -Fq 'target.state.saturating_add(target.spec.decay)' "$core"
grep -Fq 'const EXPECTED_CASES: usize = 440;' "$evaluator"
grep -Fq 'const EXPECTED_ROWS: usize = 880;' "$evaluator"
grep -Fq 'const OBSERVATION_CEILING: u64 = 256;' "$evaluator"
grep -Fq 'const CONTINUATION_CEILING: u64 = 32;' "$evaluator"
grep -Fq 'const RESISTANCE: u32 = 1_000_000;' "$evaluator"
grep -Fq 'inhibitor,' "$evaluator"
grep -Fq -- '-strength,' "$evaluator"
grep -Fq 'assert!(all_pass, "RS1 inhibitory topology gate failed")' "$evaluator"

test "$(sed -n '/const ALL: \[Self; 22\]/,/];/p' "$evaluator" | grep -c 'Self::')" -eq 22

if grep -En \
  'TransmissionMode::Modulatory|TriggerMode::QualifiedLocalParticipation|apply_modulatory|plastic_support|local_participation|eligible_until|coupling\s*\+=|resistance\s*\+=|depletion|fatigue|adaptation|homeostasis|normalization|cycle detection' \
  "$evaluator"; then
  printf 'new learning or activity-limiting physics found in RS1 evaluator\n' >&2
  exit 1
fi

grep -Fq 'observation.work.modulation == 0' "$evaluator"
grep -Fq 'observation.work.updates == 0' "$evaluator"
grep -Fq 'observation.work.proposals == 0' "$evaluator"
grep -Fq 'observation.work.deallocations == 0' "$evaluator"
grep -Fq 'observation.work.qlp == 0' "$evaluator"
grep -Fq 'observation.activity_class == ActivityClass::Periodic' "$evaluator"
grep -Fq 'settled_cycle(observation, 8)' "$evaluator"
grep -Fq 'observation.excit_fire_counts[2] > 50' "$evaluator"

printf 'RS1_INHIBITORY_TOPOLOGY_SUFFICIENCY_V1_STATIC_AUDIT_PASS\n'
