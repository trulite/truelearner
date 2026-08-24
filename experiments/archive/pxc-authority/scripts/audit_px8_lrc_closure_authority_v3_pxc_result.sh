#!/bin/sh
set -eu

if root=$(git rev-parse --show-toplevel 2>/dev/null); then
    commit=$(git rev-parse HEAD)
else
    root=${PX8_V3_PXC_SOURCE_ROOT:-$PWD}
    commit=${PX8_V3_PXC_COMMIT:-}
    test -n "$commit" || {
        echo "archive audit requires PX8_V3_PXC_COMMIT" >&2
        exit 1
    }
fi
cd "$root"

sha() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | awk '{print $1}'
    else
        shasum -a 256 "$1" | awk '{print $1}'
    fi
}

require_hash() {
    expected=$1
    file=$2
    actual=$(sha "$file")
    test "$actual" = "$expected" || {
        echo "frozen PX-C result changed: $file expected=$expected actual=$actual" >&2
        exit 1
    }
}

require_hash db4758baa5aeba36a87251f7d2ccb85cd2215f9489a1189eae4fd9d6408001c2 experiments/pxc_active_surface_manifest_v5.csv
require_hash 5205d1b115e476f1ec0efea603a04425b5c9bff92a4398ea46ef89607b134f49 experiments/pxc_active_surface_manifest_v6.csv
require_hash b67add85e46265999a606cb81e866f3d87d56a3e55052e0f5f59036647970cb3 results/px8_authority_pxc_after_v6/pxc_seam_guard_inventory_v2.csv
require_hash ee83975770282ed22851c850788a373644c73311b428e59bb6c03b910a6dc0fb results/px8_authority_pxc_after_v6/pxc_seam_taxonomy_baseline_v2.md
require_hash 69a462ef864cfea79596d3b4547175a0e6cd14e768f4836da344025bc28f870f results/px8_authority_pxc_after_v6/pxc_seam_taxonomy_inventory_v2.csv
require_hash 55a318766e289645a0da947f3cdfeeac82d3c3aa39744a2a68ff910746c911db results/px8_authority_pxc_after_v6/pxc_seam_taxonomy_summary_v2.csv
require_hash 2f8db98b1bc0ee349eeda0652109ec08439a4471a1b236bfba804dfe03792462 results/px8_authority_pxc_delta_v6/pxc_PX8_kind_delta_v1.csv
require_hash 85ac78e4897814a987eae0bc2a3aa89ed11ed5ebc09bfe0272a32ed8b9b35be1 results/px8_authority_pxc_delta_v6/pxc_PX8_layer_delta_v1.csv
require_hash d5033ae75b748d89a215895d25406b7ab5155f622e42dcd59ec72db19a3f7ca9 results/px8_authority_pxc_delta_v6/pxc_PX8_new_guarded_surfaces_v1.csv
require_hash 7e5ecf41e673f27bfc5957420ba466da02c700c15e982bfeed4727058ce3c0de results/px8_authority_pxc_delta_v6/pxc_PX8_new_seam_kinds_v1.csv
require_hash 9a09bcfbbe5d4e50ac08039a21c4ce935eeccf9ae42662e05175579f11af4ef9 results/px8_authority_pxc_delta_v6/pxc_PX8_readiness_delta_v1.csv
require_hash 534469238c4c089be9af241774063fcf8150d462b3a64d1bd81f6b0023ac6d6c results/px8_authority_pxc_delta_v6/pxc_PX8_readiness_delta_v1.md

summary=results/px8_authority_pxc_after_v6/pxc_seam_taxonomy_summary_v2.csv
metric() {
    awk -F, -v key="$1" '$1 == key { print $2 }' "$summary"
}

test "$(metric TOTAL_OCCURRENCES)" -eq 0
test "$(metric GUARD_semantic_condition)" -le 36
test "$(metric GUARD_evaluator_derived_input)" -le 136
for layer in PX0-PX3+LR-C PX4 PX5 PX6 PX7 PX8; do
    test "$(metric LAYER_$layer)" -eq 0
done

awk -F, '
    NR == 1 { next }
    $1 == "primary_seams" { ok = ok && $2 == 110 && $3 == 0 && $4 == -110 && $5 == "true" }
    NR == 2 { ok = $1 == "primary_seams" && $2 == 110 && $3 == 0 && $4 == -110 && $5 == "true" }
    NR > 2 { ok = ok && $5 == "true" }
    END { exit !(NR == 6 && ok) }
' results/px8_authority_pxc_delta_v6/pxc_PX8_readiness_delta_v1.csv

test "$(wc -l < results/px8_authority_pxc_delta_v6/pxc_PX8_new_seam_kinds_v1.csv)" -eq 1
test "$(wc -l < results/px8_authority_pxc_delta_v6/pxc_PX8_new_guarded_surfaces_v1.csv)" -eq 1
grep -Fq 'Verdict: **PASS**.' results/px8_authority_pxc_delta_v6/pxc_PX8_readiness_delta_v1.md

printf 'PX8_LRC_CLOSURE_AUTHORITY_V3_PXC_RESULT_OK commit=%s primary=110->0 semantic=36->0 evaluator=136->0 foundation=0 new_kinds=0 new_surfaces=0\n' "$commit"
