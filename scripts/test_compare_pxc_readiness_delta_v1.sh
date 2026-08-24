#!/bin/sh
set -eu

root=${PXC_SOURCE_ROOT:-$PWD}
cd "$root"

comparator=scripts/compare_pxc_readiness_delta_v1.sh
summary=results/pxc_seam_taxonomy_summary_v2.csv
inventory=results/pxc_seam_taxonomy_inventory_v2.csv
guard=results/pxc_seam_guard_inventory_v2.csv
manifest=472440f5e989387044fa3d36c5364b2d65f30d01659742a829d007cb67f7ef9a

temporary=$(mktemp -d)
trap 'rm -rf "$temporary"' EXIT HUP INT TERM

run_compare() {
    output=$1
    after_summary=$2
    after_inventory=$3
    after_guard=$4
    require_decrease=$5
    PXC_BEFORE_MANIFEST_HASH=$manifest \
    PXC_AFTER_MANIFEST_HASH=$manifest \
    PXC_REQUIRE_PRIMARY_DECREASE=$require_decrease \
        "$comparator" \
        PX4 \
        "$summary" \
        "$inventory" \
        "$guard" \
        "$after_summary" \
        "$after_inventory" \
        "$after_guard" \
        "$output"
}

# Exact replay is a valid comparator preflight when strict readiness reduction
# is disabled.
mkdir "$temporary/positive"
run_compare "$temporary/positive" "$summary" "$inventory" "$guard" 0
grep -q '^new_semantic_surfaces,0,0,0,true$' \
    "$temporary/positive/pxc_PX4_readiness_delta_v1.csv"

# A claimed readiness handoff must strictly reduce the primary seam total.
mkdir "$temporary/strict"
if run_compare "$temporary/strict" "$summary" "$inventory" "$guard" 1; then
    echo "strict no-change control unexpectedly passed" >&2
    exit 1
fi
grep -q 'Verdict: \*\*FAIL\*\*' "$temporary/strict/pxc_PX4_readiness_delta_v1.md"

# A hand-edited summary that disagrees with its inventory must fail closed.
mkdir "$temporary/tamper"
cp "$summary" "$temporary/tamper/after-summary.csv"
sed -i 's/^TOTAL_OCCURRENCES,368$/TOTAL_OCCURRENCES,369/' \
    "$temporary/tamper/after-summary.csv"
if run_compare \
    "$temporary/tamper" \
    "$temporary/tamper/after-summary.csv" \
    "$inventory" \
    "$guard" \
    0; then
    echo "tampered summary unexpectedly passed" >&2
    exit 1
fi

# A structurally consistent but rising primary count must be rejected.
mkdir "$temporary/rise"
cp "$summary" "$temporary/rise/after-summary.csv"
cp "$inventory" "$temporary/rise/after-inventory.csv"
sed -i \
    -e 's/^TOTAL_OCCURRENCES,368$/TOTAL_OCCURRENCES,369/' \
    -e 's/^UNIQUE_SOURCE_LINES,295$/UNIQUE_SOURCE_LINES,296/' \
    -e 's/^KIND_typed_representation,87$/KIND_typed_representation,88/' \
    -e 's/^LAYER_PX4,71$/LAYER_PX4,72/' \
    "$temporary/rise/after-summary.csv"
printf '%s\n' \
    '"typed_episode","typed_representation","PX4","synthetic_adapter.rs",1,"Episode",0,0,"struct Episode;"' \
    >> "$temporary/rise/after-inventory.csv"
if run_compare \
    "$temporary/rise" \
    "$temporary/rise/after-summary.csv" \
    "$temporary/rise/after-inventory.csv" \
    "$guard" \
    0; then
    echo "rising primary total unexpectedly passed" >&2
    exit 1
fi
grep -q '^primary_seams,368,369,1,false$' \
    "$temporary/rise/pxc_PX4_readiness_delta_v1.csv"

# Trading an existing occurrence for a new taxonomy kind must be rejected even
# when the headline total is unchanged.
mkdir "$temporary/new-kind"
cp "$summary" "$temporary/new-kind/after-summary.csv"
cp "$inventory" "$temporary/new-kind/after-inventory.csv"
sed -i 's/^KIND_typed_representation,87$/KIND_typed_representation,86/' \
    "$temporary/new-kind/after-summary.csv"
printf '%s\n' 'KIND_semantic_adapter,1' \
    >> "$temporary/new-kind/after-summary.csv"
sed -i '0,/"typed_representation"/s//"semantic_adapter"/' \
    "$temporary/new-kind/after-inventory.csv"
if run_compare \
    "$temporary/new-kind" \
    "$temporary/new-kind/after-summary.csv" \
    "$temporary/new-kind/after-inventory.csv" \
    "$guard" \
    0; then
    echo "new taxonomy kind unexpectedly passed" >&2
    exit 1
fi
grep -q '^semantic_adapter,new-taxonomy-kind$' \
    "$temporary/new-kind/pxc_PX4_new_seam_kinds_v1.csv"

# A new guarded semantic surface must be rejected and serialized.
mkdir "$temporary/new-surface"
cp "$summary" "$temporary/new-surface/after-summary.csv"
cp "$guard" "$temporary/new-surface/after-guard.csv"
sed -i 's/^GUARD_semantic_condition,218$/GUARD_semantic_condition,219/' \
    "$temporary/new-surface/after-summary.csv"
printf '%s\n' \
    '"semantic_condition","PX4","arms/new_semantic_adapter.rs",1,"correct"' \
    >> "$temporary/new-surface/after-guard.csv"
if run_compare \
    "$temporary/new-surface" \
    "$temporary/new-surface/after-summary.csv" \
    "$inventory" \
    "$temporary/new-surface/after-guard.csv" \
    0; then
    echo "new guarded semantic surface unexpectedly passed" >&2
    exit 1
fi
grep -q '"arms/new_semantic_adapter.rs"' \
    "$temporary/new-surface/pxc_PX4_new_guarded_surfaces_v1.csv"

printf '%s\n' \
    'PX-C readiness comparator controls: positive=PASS strict=REJECT tamper=REJECT rising=REJECT new_kind=REJECT new_surface=REJECT'
