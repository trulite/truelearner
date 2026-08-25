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

if grep -En 'expires_at|remaining_ticks|if .*participation|participation.*eligible|eligible.*participation|participation.*pressure' \
  truelearner/crates/core/src/lib.rs \
  experiments/arms/cpc1-local-temporal-participation/src/main.rs; then
  printf 'forbidden CPC1 temporal scaffold detected\n' >&2
  exit 1
fi

if grep -En 'PhysicalEvent::Participation|return_id|credit_id|reward_id' \
  truelearner/crates/core/src/lib.rs \
  experiments/arms/cpc1-local-temporal-participation/src/main.rs; then
  printf 'forbidden CPC1 attribution or trace surface detected\n' >&2
  exit 1
fi

(cd results/cpc1_local_temporal_participation_v1 && sha256sum -c SHA256SUMS)
printf 'CPC1_STATIC_AUDIT_OK physical_cases=620 mechanics_rows=1240\n'
