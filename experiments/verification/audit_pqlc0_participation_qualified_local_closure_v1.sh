#!/usr/bin/env bash
set -euo pipefail

test "$(grep -Ec '^    Drive,$|^    Modulatory,$' truelearner/crates/core/src/lib.rs)" = 2
test "$(grep -Ec '^    SourceFires,$|^    QualifiedLocalParticipation,$' truelearner/crates/core/src/lib.rs)" = 2
test "$(grep -c '^' results/pqlc0_participation_qualified_local_closure_v1/matrix.csv)" = 401
grep -q -- '- physical cases: `200/200`' \
  results/pqlc0_participation_qualified_local_closure_v1/report.md
grep -q -- '- mechanics rows: `400/400`' \
  results/pqlc0_participation_qualified_local_closure_v1/report.md
grep -q -- '- pressure, durable-learning, ARC, or authority changes: `0`' \
  results/pqlc0_participation_qualified_local_closure_v1/report.md

candidate_source=$(mktemp)
trap 'rm -f "$candidate_source"' EXIT
sed -n \
  -e '/pub enum TransmissionTrigger/,/pub enum TraversalKind/p' \
  -e '/pub fn add_arrow_with_trigger/,/pub fn enter/p' \
  -e '/let qualified_local =/,/fn elapse_to/p' \
  truelearner/crates/core/src/lib.rs > "$candidate_source"
grep -n 'feature = "pqlc0"\|TransmissionTrigger' \
  truelearner/crates/core/src/mechanics.rs >> "$candidate_source"

if grep -Eniw \
  'backward|previous|predecessor|credit|reward|cause|path_id|route_id|parent|depth|hop_count' \
  "$candidate_source"; then
  printf 'forbidden PQLC0 semantic routing surface detected\n' >&2
  exit 1
fi

if grep -En 'trace_consum|attenuat|ttl|cycle_detect|source_fired|fired.*QualifiedLocalParticipation' \
  "$candidate_source"; then
  printf 'forbidden PQLC0 damping or firing trigger detected\n' >&2
  exit 1
fi

(cd results/pqlc0_participation_qualified_local_closure_v1 && sha256sum -c SHA256SUMS)
printf 'PQLC0_STATIC_AUDIT_OK physical_cases=200 mechanics_rows=400\n'
