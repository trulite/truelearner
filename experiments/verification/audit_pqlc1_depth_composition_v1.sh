#!/usr/bin/env bash
set -euo pipefail

test "$(sha256sum truelearner/crates/core/Cargo.toml | cut -d' ' -f1)" = \
  14d45bc379a5220d33b028b48f38319cb888f732d0b34655fda02b3941a829a8
test "$(sha256sum truelearner/crates/core/src/lib.rs | cut -d' ' -f1)" = \
  c5173e8d43d109465252813fba411288c59e3bfa274f790519747eb34314e894
test "$(sha256sum truelearner/crates/core/src/mechanics.rs | cut -d' ' -f1)" = \
  266b713130be6b221432022c7518cc413a0def30ca00371422af6aceeda900da

test "$(grep -c '^' results/pqlc1_depth_composition_v1/matrix.csv)" = 1561
grep -q -- '- case variants: `39/39`' results/pqlc1_depth_composition_v1/report.md
grep -q -- '- physical cases: `780/780`' results/pqlc1_depth_composition_v1/report.md
grep -q -- '- mechanics rows: `1560/1560`' results/pqlc1_depth_composition_v1/report.md
grep -q -- '- core, pressure, eligibility, ARC, authority, oracle, or arch.md changes: `0`' \
  results/pqlc1_depth_composition_v1/report.md

test "$(grep -Ec 'add_arrow_with_trigger|TransmissionTrigger::QualifiedLocalParticipation' \
  experiments/arms/pqlc1-depth-composition/src/main.rs)" -ge 2
if grep -En 'with_mut|participation_level|eligible_until|propagate_qualified_local' \
  experiments/arms/pqlc1-depth-composition/src/main.rs; then
  printf 'PQLC1 evaluator mutates or invokes hidden substrate state\n' >&2
  exit 1
fi

(cd results/pqlc1_depth_composition_v1 && sha256sum -c SHA256SUMS)
printf 'PQLC1_STATIC_AUDIT_OK physical_cases=780 mechanics_rows=1560 core_unchanged=true\n'
