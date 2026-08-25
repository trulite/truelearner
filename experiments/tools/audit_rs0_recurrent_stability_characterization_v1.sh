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

check_hash 007764acac5c45d920512af759c5ac22344074f53b9d091c56ffe5abbd0a341d \
  truelearner/crates/core/src/lib.rs
check_hash 5d794eae058f5cdd896064b0a37a6dfb124d9d7b6d03f8cfa9c53651e58460ef \
  truelearner/crates/core/Cargo.toml
check_hash 37a59bbb7a109d7a916f8c3591ebe32c3161f44f00f4bc317dbd7136dcd640ac \
  experiments/arms/rs0-recurrent-stability-characterization/src/main.rs
check_hash d0f7bc6e48b0aad1167ca5535e0653db06e9c8cfe10b65ad5a9968b7e89ded14 \
  experiments/arms/rs0-recurrent-stability-characterization/Cargo.toml
check_hash ce64bbcdaca7665f7938d8a945b566a1f77c29432a6c9f6aef43985c6f14dee0 \
  academy/docs/rs0_recurrent_stability_characterization_protocol_v1.md
check_hash 926683f29535310ee8ebbaa9d46ecc6f2b6cb50411e903237121f697d56b7274 \
  academy/docs/ce0_consequence_supported_efficacy_handoff_v1.md

core=truelearner/crates/core/src/lib.rs
evaluator=experiments/arms/rs0-recurrent-stability-characterization/src/main.rs
manifest=experiments/arms/rs0-recurrent-stability-characterization/Cargo.toml

grep -Fq 'rs0 = ["pd1"]' truelearner/crates/core/Cargo.toml
grep -Fq 'features = ["rs0"]' "$manifest"
if grep -Fq 'features = ["ce0"]' "$manifest"; then
  printf 'RS0 must not enable CE0 plasticity\n' >&2
  exit 1
fi

grep -Fq 'propagate_with_observation_ceiling' "$core"
grep -Fq 'self.propagate_with_optional_ceiling(None).0' "$core"
grep -Fq 'scheduled_deliveries == ceiling' "$core"
grep -Fq 'const OBSERVATION_CEILING: u64 = 256;' "$evaluator"
grep -Fq 'const CONTINUATION_CEILING: u64 = 32;' "$evaluator"
grep -Fq 'const RESISTANCE: u32 = 1_000_000;' "$evaluator"
grep -Fq 'const EXPECTED_CASES: usize = 400;' "$evaluator"
grep -Fq 'const EXPECTED_ROWS: usize = 800;' "$evaluator"

test "$(sed -n '/const ALL: \[Self; 20\]/,/];/p' "$evaluator" | grep -c 'Self::')" -eq 20

if grep -En \
  'TransmissionMode::Modulatory|apply_modulatory|coupling\s*\+=|coupling\s*=\s*coupling|plastic_support|local_participation|reward|correctness|depletion|homeostasis|normalization|cycle detection' \
  "$evaluator"; then
  printf 'plasticity or candidate stability mechanism found in RS0 evaluator\n' >&2
  exit 1
fi

grep -Fq 'observation.work.modulation == 0' "$evaluator"
grep -Fq 'observation.work.updates == 0' "$evaluator"
grep -Fq 'observation.work.deallocations == 0' "$evaluator"
grep -Fq 'observation.first_firings == 8' "$evaluator"
grep -Fq 'observation.traversals == 7' "$evaluator"
grep -Fq 'assert!(all_pass, "RS0 characterization gate failed")' "$evaluator"

printf 'RS0_RECURRENT_STABILITY_CHARACTERIZATION_V1_STATIC_AUDIT_PASS\n'

