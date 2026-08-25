#!/usr/bin/env bash
set -euo pipefail

test "$(grep -Ec '^const LOCAL_WINDOW: i64 = 4;$' truelearner/crates/core/src/lib.rs)" = 1
test "$(grep -c 'participation_level' truelearner/crates/core/src/lib.rs)" -ge 5
test "$(grep -c 'eligible_until' truelearner/crates/core/src/lib.rs)" -ge 1
test "$(grep -c '^' results/tc_ds1_continuous_participation_v1/gate_a.csv)" = 321
test "$(grep -c '^' results/tc_ds1_continuous_participation_v1/decay.csv)" = 841
test "$(grep -c '^' results/tc_ds1_continuous_participation_v1/gate_b.csv)" = 321
grep -q 'Gate B desired discriminator: `false`' results/tc_ds1_continuous_participation_v1/report.md

if rg -n 'expires_at|remaining_ticks|participation.*eligible|participation.*pressure' \
  truelearner/crates/core/src/lib.rs \
  experiments/arms/tc-ds1-continuous-participation/src/main.rs; then
  printf 'forbidden TC-DS1 temporal scaffold detected\n' >&2
  exit 1
fi

(cd results/tc_ds1_continuous_participation_v1 && sha256sum -c SHA256SUMS)
printf 'TC_DS1_STATIC_AUDIT_OK gate_a_rows=320 decay_rows=840 gate_b_rows=320\n'
