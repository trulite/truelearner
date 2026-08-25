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
check_hash 509d5133899cd457cf9c46f7bd1a75e8922cbbcad4311bf70863c13943f2fecb \
  experiments/arms/sv0-symmetric-sign-variation/src/main.rs
check_hash 7a1331848974e21f45353da459b00c580bdd73c1d0a9433d6b1f153e7c52ddf0 \
  experiments/arms/sv0-symmetric-sign-variation/Cargo.toml
check_hash a5a5f5286e58ad8996026e9dbe0d38d1a1b61dc9ea2c2cd562efd1b6811d96fe \
  academy/docs/sv0_symmetric_sign_variation_protocol_v1.md

core=truelearner/crates/core/src/lib.rs
manifest=experiments/arms/sv0-symmetric-sign-variation/Cargo.toml
evaluator=experiments/arms/sv0-symmetric-sign-variation/src/main.rs
proposal_body="$(sed -n '/fn propose_local_arrows(/,/^    fn decay_cell(/p' "$core")"

grep -Fq 'sv0 = ["pd1"]' truelearner/crates/core/Cargo.toml
grep -Fq 'features = ["sv0"]' "$manifest"
if grep -Fq 'features = ["ce0"]' "$manifest"; then
  printf 'SV0 must not enable CE0 efficacy plasticity\n' >&2
  exit 1
fi

printf '%s\n' "$proposal_body" | grep -Fq 'let proposal_couplings: &[i32] = &[1, -1];'
printf '%s\n' "$proposal_body" | grep -Fq 'let proposal_couplings: &[i32] = &[1];'
printf '%s\n' "$proposal_body" | grep -Fq 'coupling: *coupling,'
printf '%s\n' "$proposal_body" | grep -Fq 'resistance: 1,'
printf '%s\n' "$proposal_body" | grep -Fq 'mode: TransmissionMode::Drive,'

if printf '%s\n' "$proposal_body" | grep -En \
  'target\.state|target\.threshold|oscillat|activity|reward|error|stability|cycle|random|choose|preferred|Modulatory'; then
  printf 'semantic or sign-selecting input found in SV0 proposal law\n' >&2
  exit 1
fi

test "$(grep -c 'body.add_arrow(ArrowSpec' "$evaluator")" -eq 1
grep -Fq 'mode: TransmissionMode::Modulatory,' "$evaluator"
if grep -Fq 'mode: TransmissionMode::Drive,' "$evaluator"; then
  printf 'SV0 evaluator manually constructs a Drive candidate\n' >&2
  exit 1
fi
grep -Fq 'const EXPECTED_CASES: usize = 72;' "$evaluator"
grep -Fq 'const EXPECTED_ROWS: usize = 144;' "$evaluator"
test "$(sed -n '/const ALL: \[Self; 6\]/,/];/p' "$evaluator" | grep -c 'Self::')" -eq 6
grep -Fq 'if selected_crossing_present {' "$evaluator"
grep -Fq 'observation.work.updates == 0' "$evaluator"
grep -Fq 'observation.work.updates == 1' "$evaluator"
grep -Fq 'observation.work.updates == 4' "$evaluator"
grep -Fq 'observation.peak_live_candidates == 2' "$evaluator"
grep -Fq 'assert!(all_pass, "SV0 symmetric sign variation gate failed")' "$evaluator"

if grep -En \
  'oscillation score|cycle detector|preferred sign|reward|correctness|target coupling|inhibit here|stabilize' \
  "$evaluator"; then
  printf 'forbidden semantic SV0 evaluator surface found\n' >&2
  exit 1
fi

printf 'SV0_SYMMETRIC_SIGN_VARIATION_V1_STATIC_AUDIT_PASS\n'
