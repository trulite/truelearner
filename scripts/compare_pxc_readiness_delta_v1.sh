#!/bin/sh
set -eu

if [ "$#" -ne 6 ]; then
    echo "usage: $0 LANE BEFORE_SUMMARY BEFORE_GUARD AFTER_SUMMARY AFTER_GUARD OUTPUT_DIR" >&2
    exit 2
fi

lane=$1
before_summary=$2
before_guard=$3
after_summary=$4
after_guard=$5
output_dir=$6

case "$lane" in
    PX4|PX5|PX6|PX7|PX8) ;;
    *)
        echo "lane must be one of PX4, PX5, PX6, PX7, or PX8" >&2
        exit 2
        ;;
esac

before_manifest=${PXC_BEFORE_MANIFEST_HASH:-}
after_manifest=${PXC_AFTER_MANIFEST_HASH:-}
if [ -z "$before_manifest" ] || [ -z "$after_manifest" ]; then
    echo "PXC_BEFORE_MANIFEST_HASH and PXC_AFTER_MANIFEST_HASH are required" >&2
    exit 2
fi

for path in "$before_summary" "$before_guard" "$after_summary" "$after_guard"; do
    if [ ! -f "$path" ]; then
        echo "PX-C readiness input is missing: $path" >&2
        exit 2
    fi
done

mkdir -p "$output_dir"
delta_csv="$output_dir/pxc_${lane}_readiness_delta_v1.csv"
kind_csv="$output_dir/pxc_${lane}_kind_delta_v1.csv"
layer_csv="$output_dir/pxc_${lane}_layer_delta_v1.csv"
new_surface_csv="$output_dir/pxc_${lane}_new_guarded_surfaces_v1.csv"
new_kind_csv="$output_dir/pxc_${lane}_new_seam_kinds_v1.csv"
report="$output_dir/pxc_${lane}_readiness_delta_v1.md"

temporary=$(mktemp -d)
trap 'rm -rf "$temporary"' EXIT HUP INT TERM

metric() {
    key=$1
    path=$2
    value=$(awk -F, -v key="$key" '$1 == key { print $2 }' "$path")
    if [ -z "$value" ]; then
        echo "PX-C metric is missing: $key in $path" >&2
        exit 2
    fi
    printf '%s' "$value"
}

check_summary() {
    path=$1
    total=$(metric TOTAL_OCCURRENCES "$path")
    kind_sum=$(awk -F, '$1 ~ /^KIND_/ { sum += $2 } END { print sum + 0 }' "$path")
    layer_sum=$(awk -F, '$1 ~ /^LAYER_/ { sum += $2 } END { print sum + 0 }' "$path")
    if [ "$kind_sum" -ne "$total" ]; then
        echo "PX-C kind sum is not exhaustive: path=$path kinds=$kind_sum total=$total" >&2
        exit 2
    fi
    if [ "$layer_sum" -ne "$total" ]; then
        echo "PX-C layer sum is not exhaustive: path=$path layers=$layer_sum total=$total" >&2
        exit 2
    fi
}

check_summary "$before_summary"
check_summary "$after_summary"

before_total=$(metric TOTAL_OCCURRENCES "$before_summary")
after_total=$(metric TOTAL_OCCURRENCES "$after_summary")
before_semantic=$(metric GUARD_semantic_condition "$before_summary")
after_semantic=$(metric GUARD_semantic_condition "$after_summary")
before_evaluator=$(metric GUARD_evaluator_derived_input "$before_summary")
after_evaluator=$(metric GUARD_evaluator_derived_input "$after_summary")

printf '%s\n' 'kind,before,after,delta' > "$kind_csv"
awk -F, '$1 ~ /^KIND_/ { print $1 }' "$before_summary" | LC_ALL=C sort -u \
    > "$temporary/before-kinds.txt"
awk -F, '$1 ~ /^KIND_/ { print $1 }' "$after_summary" | LC_ALL=C sort -u \
    > "$temporary/after-kinds.txt"
printf '%s\n' 'kind,reason' > "$new_kind_csv"
comm -13 "$temporary/before-kinds.txt" "$temporary/after-kinds.txt" \
    | while IFS= read -r kind; do
        printf '%s,new-taxonomy-kind\n' "${kind#KIND_}" >> "$new_kind_csv"
    done

for kind in \
    typed_representation \
    explicit_mechanism_invocation \
    episode_reset_boundary \
    seed_history_synthesis \
    semantic_condition \
    manual_temporary_cleanup \
    typed_handoff \
    evaluator_derived_input
do
    before=$(metric "KIND_$kind" "$before_summary")
    after=$(metric "KIND_$kind" "$after_summary")
    delta=$((after - before))
    printf '%s,%s,%s,%s\n' "$kind" "$before" "$after" "$delta" >> "$kind_csv"
    if [ "$before" -eq 0 ] && [ "$after" -gt 0 ]; then
        printf '%s,reintroduced-after-zero\n' "$kind" >> "$new_kind_csv"
    fi
done
new_kinds=$(tail -n +2 "$new_kind_csv" | LC_ALL=C sort -u | wc -l | tr -d ' ')

printf '%s\n' 'layer,before,after,delta' > "$layer_csv"
for layer in 'PX0-PX3+LR-C' PX4 PX5 PX6 PX7 PX8; do
    before=$(metric "LAYER_$layer" "$before_summary")
    after=$(metric "LAYER_$layer" "$after_summary")
    delta=$((after - before))
    printf '%s,%s,%s,%s\n' "$layer" "$before" "$after" "$delta" >> "$layer_csv"
done

normalize_guard_surfaces() {
    path=$1
    awk -F, 'NR > 1 { print $1 "," $2 "," $3 "," $5 }' "$path" | LC_ALL=C sort -u
}

normalize_guard_surfaces "$before_guard" > "$temporary/before-surfaces.csv"
normalize_guard_surfaces "$after_guard" > "$temporary/after-surfaces.csv"
printf '%s\n' 'guard,layer,path,match' > "$new_surface_csv"
comm -13 "$temporary/before-surfaces.csv" "$temporary/after-surfaces.csv" \
    >> "$new_surface_csv"
new_surfaces=$(tail -n +2 "$new_surface_csv" | wc -l | tr -d ' ')

total_pass=false
semantic_pass=false
evaluator_pass=false
kind_pass=false
surface_pass=false
primary_decrease_pass=true

if [ "$after_total" -le "$before_total" ]; then total_pass=true; fi
if [ "$after_semantic" -le "$before_semantic" ]; then semantic_pass=true; fi
if [ "$after_evaluator" -le "$before_evaluator" ]; then evaluator_pass=true; fi
if [ "$new_kinds" -eq 0 ]; then kind_pass=true; fi
if [ "$new_surfaces" -eq 0 ]; then surface_pass=true; fi
if [ "${PXC_REQUIRE_PRIMARY_DECREASE:-0}" = 1 ] \
    && [ "$after_total" -ge "$before_total" ]; then
    primary_decrease_pass=false
fi

printf '%s\n' 'metric,before,after,delta,passed' > "$delta_csv"
printf 'primary_seams,%s,%s,%s,%s\n' \
    "$before_total" "$after_total" "$((after_total - before_total))" "$total_pass" >> "$delta_csv"
printf 'semantic_guard,%s,%s,%s,%s\n' \
    "$before_semantic" "$after_semantic" "$((after_semantic - before_semantic))" "$semantic_pass" >> "$delta_csv"
printf 'evaluator_guard,%s,%s,%s,%s\n' \
    "$before_evaluator" "$after_evaluator" "$((after_evaluator - before_evaluator))" "$evaluator_pass" >> "$delta_csv"
printf 'new_seam_kinds,0,%s,%s,%s\n' \
    "$new_kinds" "$new_kinds" "$kind_pass" >> "$delta_csv"
printf 'new_semantic_surfaces,0,%s,%s,%s\n' \
    "$new_surfaces" "$new_surfaces" "$surface_pass" >> "$delta_csv"

verdict=FAIL
if [ "$total_pass" = true ] \
    && [ "$semantic_pass" = true ] \
    && [ "$evaluator_pass" = true ] \
    && [ "$kind_pass" = true ] \
    && [ "$surface_pass" = true ] \
    && [ "$primary_decrease_pass" = true ]; then
    verdict=PASS
fi

{
    printf '# PX-C %s readiness delta v1\n\n' "$lane"
    printf 'Verdict: **%s**.\n\n' "$verdict"
    printf 'Before manifest: `%s`.\n\n' "$before_manifest"
    printf 'After manifest: `%s`.\n\n' "$after_manifest"
    printf '| metric | before | after | delta | accepted |\n'
    printf '|---|---:|---:|---:|:---:|\n'
    tail -n +2 "$delta_csv" | while IFS=, read -r name before after delta passed; do
        printf '| `%s` | %s | %s | %+d | %s |\n' \
            "$name" "$before" "$after" "$delta" "$passed"
    done
    printf '\nA readiness claim requires functional success, complete active-surface manifest coverage, no rising counter, no reintroduced kind, and no new guarded semantic surface.\n'
} > "$report"

printf 'PX-C %s readiness delta: verdict=%s primary=%s->%s semantic=%s->%s evaluator=%s->%s new_kinds=%s new_surfaces=%s\n' \
    "$lane" "$verdict" "$before_total" "$after_total" "$before_semantic" "$after_semantic" \
    "$before_evaluator" "$after_evaluator" "$new_kinds" "$new_surfaces"

if [ "$verdict" != PASS ]; then
    exit 1
fi
