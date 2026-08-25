#!/usr/bin/env bash
set -euo pipefail

test "$(grep -Ec '^const LOCAL_WINDOW: i64 = 4;$' truelearner/crates/core/src/lib.rs)" = 1
test "$(grep -c '^' results/cpc1_local_temporal_participation_v1/curve.csv)" = 881
test "$(grep -c '^' results/cpc1_local_temporal_participation_v1/controls.csv)" = 361
grep -q -- '- total physical cases: `620/620`' \
  results/cpc1_local_temporal_participation_v1/report.md
grep -q -- '- total mechanics rows: `1240/1240`' \
  results/cpc1_local_temporal_participation_v1/report.md
grep -q -- '- pressure or durable-resistance candidate interaction: `none`' \
  results/cpc1_local_temporal_participation_v1/report.md

candidate_sources=(
  truelearner/crates/core/src/lib.rs
  truelearner/crates/core/src/mechanics.rs
)

if grep -En \
  'expires_at|remaining_ticks|ticks_remaining|^[[:space:]]*if[[:space:]].*participation|participation.*eligible|eligible.*participation|participation.*pressure|pressure.*participation' \
  "${candidate_sources[@]}"; then
  printf 'forbidden CPC1 temporal scaffold detected\n' >&2
  exit 1
fi

if grep -En 'PhysicalEvent::Participation|return_id|credit_id|reward_id' \
  "${candidate_sources[@]}"; then
  printf 'forbidden CPC1 attribution or trace surface detected\n' >&2
  exit 1
fi

(cd results/cpc1_local_temporal_participation_v1 && sha256sum -c SHA256SUMS)
printf 'CPC1_STATIC_AUDIT_V2_OK physical_cases=620 mechanics_rows=1240\n'
