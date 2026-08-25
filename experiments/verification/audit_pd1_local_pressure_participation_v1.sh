#!/usr/bin/env bash
set -euo pipefail

test "$(sha256sum truelearner/crates/core/Cargo.toml | cut -d' ' -f1)" = \
  c919b87fb2628f23e019a59ec59eab3fefb7faffa3a48fa03e6e9ea4d1ebbb4c
test "$(sha256sum truelearner/crates/core/src/lib.rs | cut -d' ' -f1)" = \
  f5d61de5b0ad57ccba2a44d0cb1020aec5e2008e3051857030f69c6593f76be5
test "$(sha256sum truelearner/crates/core/src/mechanics.rs | cut -d' ' -f1)" = \
  7521549b1e348be07e3b2ee943f6d2cf763201cd54de6d8a576ac6592d6e6bb8
test "$(sha256sum experiments/arms/pd1-local-pressure-participation/src/main.rs | cut -d' ' -f1)" = \
  b3b3439e088bc96b6831e9c49a5e38b0a25955562e0ec128f12f3dbf39fee7eb

grep -q '^pd1 = \["pqlc0"\]$' truelearner/crates/core/Cargo.toml
grep -q 'pressure_load: u64' truelearner/crates/core/src/lib.rs
grep -q 'fn elapse_pd1_pressure' truelearner/crates/core/src/lib.rs
grep -q 'PARTICIPATION_IMPULSE.saturating_sub(absorbed)' \
  truelearner/crates/core/src/lib.rs
grep -q 'arrow.pressure_load %= PARTICIPATION_IMPULSE' \
  truelearner/crates/core/src/lib.rs

pressure_body="$({
  sed -n '/^    fn elapse_pd1_pressure/,/^    fn propose_local_arrows/p' \
    truelearner/crates/core/src/lib.rs
})"
if grep -Eq 'eligible_until|LOCAL_WINDOW|UNSUPPORTED_USE_PRESSURE' <<<"$pressure_body"; then
  printf 'PD1 pressure consults rectangular eligibility machinery\n' >&2
  exit 1
fi
if grep -Eq 'participation_level[[:space:]]*[!<>=]=?[[:space:]]*0' <<<"$pressure_body"; then
  printf 'PD1 pressure contains Boolean participation protection\n' >&2
  exit 1
fi

candidate_diff="$(git diff 03f2eed -- truelearner/crates/core/Cargo.toml \
  truelearner/crates/core/src/lib.rs truelearner/crates/core/src/mechanics.rs)"
if grep '^+' <<<"$candidate_diff" | grep -Eiq \
  'ARC|reward|credit|cause|path_id|route_id|parent|depth|hop_count|expected_answer'; then
  printf 'PD1 substrate diff contains forbidden semantic surface\n' >&2
  exit 1
fi

if test -f results/pd1_local_pressure_participation_v1/matrix.csv; then
  test "$(grep -c '^' results/pd1_local_pressure_participation_v1/matrix.csv)" = 401
  grep -q -- '- physical cases: `200/200`' \
    results/pd1_local_pressure_participation_v1/report.md
  grep -q -- '- mechanics rows: `400/400`' \
    results/pd1_local_pressure_participation_v1/report.md
  grep -q -- '- local load exchange candidate: `PASS`' \
    results/pd1_local_pressure_participation_v1/report.md
  grep -q -- '- rectangular eligibility consulted by pressure: `false`' \
    results/pd1_local_pressure_participation_v1/report.md
  (cd results/pd1_local_pressure_participation_v1 && sha256sum -c SHA256SUMS)
fi

printf 'PD1_STATIC_AUDIT_OK candidate=local_load_exchange eligible_pressure=false\n'
