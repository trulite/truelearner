#!/usr/bin/env bash
set -euo pipefail

core=truelearner/crates/core/src/lib.rs
manifest=truelearner/crates/core/Cargo.toml

search_quiet() {
  local pattern=$1
  local file=$2
  if command -v rg >/dev/null 2>&1; then
    rg -q -- "$pattern" "$file"
  else
    grep -Eq -- "$pattern" "$file"
  fi
}

require() {
  search_quiet "$1" "$2" || {
    echo "missing J0 surface: $1 in $2" >&2
    exit 1
  }
}

forbid() {
  local pattern=$1
  if command -v rg >/dev/null 2>&1; then
    matches=$(rg -n --ignore-case -- "$pattern" "$core" || true)
  else
    matches=$(grep -Ein -- "$pattern" "$core" || true)
  fi
  if test -n "$matches"; then
    printf '%s\n' "$matches"
    echo "forbidden J0 substrate surface: $pattern" >&2
    exit 1
  fi
}

require '^j0 = \["cl0"\]$' "$manifest"
require '#\[cfg\(feature = "j0"\)\]' "$core"
require 'arrow\.mode == TransmissionMode::Drive' "$core"
require '\(arrow\.from == cell \|\| arrow\.to == cell\)' "$core"
require 'required\.contains\(&cell\.id\)' "$core"
require 'work\.cell_deallocations = work\.cell_deallocations\.saturating_add\(1\)' "$core"

forbid 'ContactCell|JunctionCell|TemporaryCell'
forbid 'predecessor|path_id|route_id|hop_count|BackwardTraversal|TransmissionMode::Backward'
forbid 'selected_candidate|useful_branch|reward'

echo 'J0_STATIC_AUDIT_OK'
