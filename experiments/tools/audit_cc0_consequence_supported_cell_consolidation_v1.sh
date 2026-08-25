#!/usr/bin/env bash
set -euo pipefail

core=truelearner/crates/core/src/lib.rs
mechanics=truelearner/crates/core/src/mechanics.rs
manifest=truelearner/crates/core/Cargo.toml

require() {
  local pattern=$1
  local file=$2
  if command -v rg >/dev/null 2>&1; then
    rg -q -- "$pattern" "$file"
  else
    grep -Eq -- "$pattern" "$file"
  fi || {
    echo "missing required CC0 surface: $pattern in $file" >&2
    exit 1
  }
}

forbid() {
  local pattern=$1
  if command -v rg >/dev/null 2>&1; then
    matches=$(rg -n --ignore-case -- "$pattern" "$core" "$mechanics" || true)
  else
    matches=$(grep -Ein -- "$pattern" "$core" "$mechanics" || true)
  fi
  if test -n "$matches"; then
    printf '%s\n' "$matches"
    echo "forbidden CC0 substrate surface: $pattern" >&2
    exit 1
  fi
}

require '^cc0 = \["cl0"\]$' "$manifest"
require 'participation_level: u64' "$core"
require 'target\.participation_level = target' "$core"
require 'PhysicalEvent::CellResistance' "$core"
require 'cell_state\.decay_load = 0' "$core"
require 'relax_participation\(cell\.participation_level, elapsed\)' "$core"
require 'local_consequence_gain\(cell_state\.participation_level\)' "$core"
require 'local_consequence_gain\(participation\)' "$core"

if command -v rg >/dev/null 2>&1; then
  gain_calls=$(rg -c 'local_consequence_gain\(' "$core")
else
  gain_calls=$(grep -Ec 'local_consequence_gain\(' "$core")
fi
test "$gain_calls" -eq 3 || {
  echo "expected one shared law plus exactly two structural call sites; got $gain_calls" >&2
  exit 1
}

forbid 'ContactCell|TemporaryCell|EphemeralCell'
forbid 'eligible_until|expires_at|remaining_ticks'
forbid 'reward|useful_cell|cell_role|target_resistance'

echo 'CC0_STATIC_AUDIT_OK'
