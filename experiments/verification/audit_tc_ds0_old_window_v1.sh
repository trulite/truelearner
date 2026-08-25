#!/usr/bin/env bash
set -euo pipefail

expected_core=d49a6b98081cee65c3e7a5f64e9cf6356fa06ef456a9cf78a41d0cd1187bcd58
actual_core=$(sha256sum truelearner/crates/core/src/lib.rs | cut -d' ' -f1)
test "$actual_core" = "$expected_core"

test "$(grep -Ec '^const LOCAL_WINDOW: i64 = 4;$' truelearner/crates/core/src/lib.rs)" = 1
test "$(grep -c '^' results/tc_ds0_old_window_v1/matrix.csv)" = 1921
test "$(grep -c ',reference,' results/tc_ds0_old_window_v1/matrix.csv)" = 960
test "$(grep -c ',production,' results/tc_ds0_old_window_v1/matrix.csv)" = 960
grep -q 'physical cases: `960/960`' results/tc_ds0_old_window_v1/report.md
grep -q 'mechanics rows: `1920/1920`' results/tc_ds0_old_window_v1/report.md
(cd results/tc_ds0_old_window_v1 && sha256sum -c SHA256SUMS)

if git diff --name-only 0033ab2 -- truelearner | grep -q .; then
  printf 'causal organism files changed from frozen ARC A2 candidate\n' >&2
  exit 1
fi

printf 'TC_DS0_STATIC_AUDIT_OK core=%s rows=1920\n' "$actual_core"
