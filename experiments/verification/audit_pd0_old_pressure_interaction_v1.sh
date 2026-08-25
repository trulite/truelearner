#!/usr/bin/env bash
set -euo pipefail

test "$(sha256sum truelearner/crates/core/Cargo.toml | cut -d' ' -f1)" = \
  14d45bc379a5220d33b028b48f38319cb888f732d0b34655fda02b3941a829a8
test "$(sha256sum truelearner/crates/core/src/lib.rs | cut -d' ' -f1)" = \
  c5173e8d43d109465252813fba411288c59e3bfa274f790519747eb34314e894
test "$(sha256sum truelearner/crates/core/src/mechanics.rs | cut -d' ' -f1)" = \
  266b713130be6b221432022c7518cc413a0def30ca00371422af6aceeda900da

test "$(grep -c '^' results/pd0_old_pressure_interaction_v1/matrix.csv)" = 1501
grep -q -- '- physical cases: `750/750`' results/pd0_old_pressure_interaction_v1/report.md
grep -q -- '- mechanics rows: `1500/1500`' results/pd0_old_pressure_interaction_v1/report.md
grep -q -- '- PD0 characterization complete: `true`' \
  results/pd0_old_pressure_interaction_v1/report.md
grep -q -- '- core, constants, pressure, participation, PQLC, ARC, authority, oracle, or arch.md changes: `0`' \
  results/pd0_old_pressure_interaction_v1/report.md

if grep -En 'with_mut|participation_level|eligible_until|propagate_qualified_local|pressure_arrow' \
  experiments/arms/pd0-old-pressure-interaction/src/main.rs; then
  printf 'PD0 evaluator mutates or invokes hidden substrate state\n' >&2
  exit 1
fi

(cd results/pd0_old_pressure_interaction_v1 && sha256sum -c SHA256SUMS)
printf 'PD0_STATIC_AUDIT_OK physical_cases=750 mechanics_rows=1500 core_unchanged=true\n'
