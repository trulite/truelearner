#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../.."

core=truelearner/crates/core/src/lib.rs
modulation_body="$(sed -n '/    fn apply_modulatory_return(/,/    fn propagate_qualified_local(/p' "$core")"

test "$(printf '%s\n' "$modulation_body" | grep -Ec 'cell\.resistance|target\.resistance|cells\.with_mut' || true)" -eq 0
test "$(grep -Ec 'cell\.resistance[[:space:]]*=[^;]*saturating_add|target\.resistance[[:space:]]*=[^;]*saturating_add' "$core" || true)" -eq 0
grep -Fq 'arrow.resistance = arrow.resistance.saturating_add(gain);' "$modulation_body"

printf 'gate_9=negative\n'
printf 'accepted_arrow_consolidation=true\n'
printf 'accepted_cell_consolidation=false\n'
printf 'cell_resistance_increase_paths=0\n'
printf 'new_cell_consolidation_added=false\n'
printf 'CL0_ORDINARY_CELL_LIFETIME_GATE_9_STATIC_NEGATIVE_V1\n'
