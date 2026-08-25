#!/usr/bin/env bash
set -euo pipefail

root=${1:-.}
runtime="$root/truelearner/crates/core/src/lib.rs"
mechanics="$root/truelearner/crates/core/src/mechanics.rs"

test -f "$runtime"
test ! -e "$mechanics"

python3 - "$runtime" <<'PY'
from pathlib import Path
import re
import sys

path = Path(sys.argv[1])
source = path.read_text()

def require(fragment: str) -> None:
    if fragment not in source:
        raise SystemExit(f"missing AH0 invariant: {fragment}")

def reject(fragment: str) -> None:
    if fragment in source:
        raise SystemExit(f"forbidden AH0 causal construct: {fragment}")

def function_body(signature: str) -> str:
    start = source.index(signature)
    brace = source.index("{", start)
    depth = 0
    for index in range(brace, len(source)):
        if source[index] == "{":
            depth += 1
        elif source[index] == "}":
            depth -= 1
            if depth == 0:
                return source[start:index + 1]
    raise SystemExit(f"unterminated function: {signature}")

require("mod mechanics {")
reject("mod mechanics;")
require("active_cells: HashSet<CellId>")
if source.count("collect::<HashSet<_>>()") < 2:
    raise SystemExit("active/required membership is not hash-based")
reject("outgoing.sort_unstable()")
reject("collect::<BTreeSet<CellId>>")
reject("collect::<BTreeSet<ArrowId>>")

causal_key = function_body("fn causal_order_key")
for token in ("CellId", "ArrowId", "origin_physical", "target_physical", "target.0", "arrow.0"):
    if token in causal_key:
        raise SystemExit(f"handle-derived transition ordering remains in causal_order_key: {token}")

minimum = function_body("fn minimum_index")
if "causal_order_key" not in minimum:
    raise SystemExit("scheduler minimum does not use the AH0 causal key")

physical = function_body("fn physical_arrow_order")
for token in (".id", "physical_id", "CellId", "ArrowId"):
    if token in physical:
        raise SystemExit(f"opaque handle leaked into physical ARROW ordering: {token}")

required_physical_fields = (
    "phase:", "delay:", "from_position:", "to_position:", "mode:",
    "trigger:", "coupling:", "resistance:", "participation:",
)
for field in required_physical_fields:
    if field not in physical:
        raise SystemExit(f"physical ARROW ordering omits {field}")

handle_sorts = [
    line.strip()
    for line in source.splitlines()
    if "sort_by_key" in line and re.search(r"(?:cell|arrow)\.id", line)
]
if len(handle_sorts) != 8:
    raise SystemExit(f"unexpected handle-sort inventory: {len(handle_sorts)}")

allowed_regions = "\n".join((
    function_body("impl LiveCheckpoint"),
    function_body("pub fn from_arena_body_with_packing"),
    function_body("pub fn compact_resident"),
))
for line in handle_sorts:
    if line not in allowed_regions:
        raise SystemExit(f"handle sort escaped storage/layout boundary: {line}")

if source.count("AH0_STORAGE_ONLY") < 6:
    raise SystemExit("storage/layout handle ordering is not explicitly classified")

if source.count("fn propagate_si0") != 1 or source.count("drain_minimum_wave") < 2:
    raise SystemExit("SI0 causal-wave runtime is not present exactly once")

print("AH0_HANDLE_ORDERING_REMOVAL_STATIC_AUDIT_OK")
PY

runtime_sources=$(find "$root/truelearner/crates/core/src" -maxdepth 1 -type f -name '*.rs' ! -name main.rs | wc -l | tr -d ' ')
test "$runtime_sources" = 1

printf 'AH0_ONE_FILE_RUNTIME_CLOSURE_OK\n'
