#!/usr/bin/env bash
set -euo pipefail

printf '%s  %s\n' \
  '027ec827afbf998df07749e428468196f82eb33824401b78aa15a6b48680a6cb' \
  'truelearner/crates/core/src/lib.rs' \
  '5093e259a324b72a2fd661e1d402030fed356ac19d3b948549d7eea37f8b7295' \
  'truelearner/crates/core/src/mechanics.rs' \
  | sha256sum -c -

test "$(grep -c '^' results/cpc2_adjacent_local_causal_closure_v1/matrix.csv)" = 721
grep -q -- '- physical cases: `360/360`' \
  results/cpc2_adjacent_local_causal_closure_v1/report.md
grep -q -- '- mechanics rows: `720/720`' \
  results/cpc2_adjacent_local_causal_closure_v1/report.md
grep -q -- '- runtime or substrate-law changes: `0`' \
  results/cpc2_adjacent_local_causal_closure_v1/report.md

if grep -En \
  'causal_path|parent_arrow|parent_pointer|predecessor|path_stack|max_back_steps|backward_traversal|CreditPacket|RewardPacket|ReturnPacket' \
  experiments/arms/cpc2-adjacent-local-causal-closure/src/main.rs; then
  printf 'forbidden CPC2 addressed propagation surface detected\n' >&2
  exit 1
fi

(cd results/cpc2_adjacent_local_causal_closure_v1 && sha256sum -c SHA256SUMS)
printf 'CPC2_STATIC_AUDIT_OK physical_cases=360 mechanics_rows=720\n'
