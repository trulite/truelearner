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

check_hash dce7ee46a040695bcb318bcbf1ff284cfe7ec71738760c63f0df19f85b873c89 \
  truelearner/crates/core/src/lib.rs
check_hash c07bea851a56526c375363fae980154ac70125f903476b64e353beb00992d15d \
  truelearner/crates/core/Cargo.toml
check_hash 10089a6ee5b482f3ccae86c0505f89f23236addcec2cd3fc8791a014b83a8456 \
  experiments/arms/ce0-consequence-supported-efficacy/src/main.rs
check_hash 821a7053eec2644edf5e23f957eb2a6d36ef50c355b26ceb90cdd18fa39d3c8a \
  experiments/arms/ce0-consequence-supported-efficacy/Cargo.toml
check_hash d3f51b90253a8d28a44ad9ed67505a001bdf03ea04e20eae2bc98481c0eb7c74 \
  academy/docs/ce0_consequence_supported_efficacy_protocol_v1.md
check_hash a8b3874eee7108f9b471024a39dea809f04dd5a52864c0d9f8e1bfe7b9a1da83 \
  academy/docs/cr0_coupling_necessity_handoff_v2.md

core=truelearner/crates/core/src/lib.rs
evaluator=experiments/arms/ce0-consequence-supported-efficacy/src/main.rs

grep -Fq 'ce0 = ["pd1"]' truelearner/crates/core/Cargo.toml
grep -Fq 'let completed_before = support_before / PARTICIPATION_IMPULSE;' "$core"
grep -Fq 'let completed_after = arrow.plastic_support / PARTICIPATION_IMPULSE;' "$core"
grep -Fq 'let efficacy_gain = completed_after.saturating_sub(completed_before);' "$core"
grep -Fq 'arrow.coupling = arrow.coupling.saturating_add(efficacy_gain);' "$core"
grep -Fq 'PhysicalEvent::Coupling' "$core"

apply_body="$(sed -n '/fn apply_modulatory_return(/,/fn propagate_qualified_local(/p' "$core")"
if printf '%s\n' "$apply_body" | grep -En \
  'threshold|target\.state|target\.refractory|desired|correctness|reward|path_id|route_id|predecessor|parent|hop_count|depth|coupling\s*\+=\s*1'; then
  printf 'forbidden target-aware or unconditional efficacy logic found\n' >&2
  exit 1
fi

test "$(grep -c '^        Self::' "$evaluator")" -ge 10
grep -Fq 'Self::RecurrentStability' "$evaluator"
grep -Fq 'observation.measures[1] == 1' "$evaluator"
grep -Fq 'observation.measures[2] == 1' "$evaluator"
grep -Fq 'assert!(all_pass, "CE0 matrix failed")' "$evaluator"

if grep -En \
  'desired coupling|target coupling|coupling ceiling|cycle detection|hop_count|path_id|route_id|predecessor' \
  "$evaluator"; then
  printf 'forbidden CE0 evaluator surface found\n' >&2
  exit 1
fi

printf 'CE0_CONSEQUENCE_SUPPORTED_EFFICACY_V1_STATIC_AUDIT_PASS\n'
