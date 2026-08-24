#!/bin/sh
set -eu

if root=$(git rev-parse --show-toplevel 2>/dev/null); then
    repository_checkout=true
else
    root=${PXC_SOURCE_ROOT:-$PWD}
    repository_checkout=false
fi
cd "$root"

authority=f9057fe78a86db9111b0b69310d03accef3bc970
manifest=${PXC_MANIFEST:-experiments/pxc_active_surface_manifest_v1.csv}
output_dir=${1:-results}
inventory="$output_dir/pxc_seam_taxonomy_inventory_v2.csv"
guard_inventory="$output_dir/pxc_seam_guard_inventory_v2.csv"
summary="$output_dir/pxc_seam_taxonomy_summary_v2.csv"
report="$output_dir/pxc_seam_taxonomy_baseline_v2.md"

if [ "$repository_checkout" = true ]; then
    if ! git merge-base --is-ancestor "$authority" HEAD; then
        echo "PX-C taxonomy audit must descend from $authority" >&2
        exit 1
    fi
    commit=$(git rev-parse HEAD)
else
    commit=${PXC_AUDITED_COMMIT:-}
    if [ -z "$commit" ]; then
        echo "archive audit requires PXC_AUDITED_COMMIT" >&2
        exit 1
    fi
fi

if [ ! -f "$manifest" ]; then
    echo "PX-C active-surface manifest is missing: $manifest" >&2
    exit 1
fi

if command -v sha256sum >/dev/null 2>&1; then
    sha256_file() { sha256sum "$1" | awk '{print $1}'; }
else
    sha256_file() { shasum -a 256 "$1" | awk '{print $1}'; }
fi

require_hash() {
    expected=$1
    path=$2
    actual=$(sha256_file "$path")
    if [ "$actual" != "$expected" ]; then
        echo "frozen PX-C v1 artifact moved: $path" >&2
        echo "expected=$expected actual=$actual" >&2
        exit 1
    fi
}

require_hash 499cd0b43790bbbee906e0738eae982369b2435af933070ef8a6bab8256e9093 \
    results/pxc_continuous_seam_baseline_v1.md
require_hash f40ca354be9c59e77f376064baf1578154250f7c70cd57f0144ea2b9a45cdbbf \
    results/pxc_continuous_seam_inventory_v1.csv
require_hash a76bcf979f46f004b2d8ff97c620aa56ca62739fba7a149e28df4cc9f77626ae \
    results/pxc_continuous_seam_summary_v1.csv

manifest_hash=$(sha256_file "$manifest")
if [ -n "${PXC_EXPECT_MANIFEST_HASH:-}" ] \
    && [ "$manifest_hash" != "$PXC_EXPECT_MANIFEST_HASH" ]; then
    echo "PX-C manifest hash changed: expected=$PXC_EXPECT_MANIFEST_HASH actual=$manifest_hash" >&2
    exit 1
fi

while IFS=, read -r layer path surface; do
    if [ "$layer" = layer ]; then
        continue
    fi
    if [ ! -f "$path" ]; then
        echo "PX-C manifested source is missing: $path" >&2
        exit 1
    fi
done < "$manifest"

if command -v rg >/dev/null 2>&1; then
    search_backend=rg
elif command -v grep >/dev/null 2>&1; then
    search_backend=grep
else
    echo "PX-C taxonomy audit requires rg or grep" >&2
    exit 1
fi

mkdir -p "$output_dir"
temporary=$(mktemp -d)
trap 'rm -rf "$temporary"' EXIT HUP INT TERM

csv_quote() {
    printf '%s' "$1" | sed 's/"/""/g'
}

unquote() {
    printf '%s' "$1" | sed 's/^"//; s/"$//; s/""/"/g'
}

search_file() {
    pattern=$1
    path=$2
    output=
    status=0
    if [ "$search_backend" = rg ]; then
        output=$(rg -n -o --no-heading --color never -e "$pattern" "$path" 2>/dev/null) \
            || status=$?
    else
        output=$(grep -Eno -e "$pattern" "$path" 2>/dev/null) || status=$?
    fi
    if [ "$status" -gt 1 ]; then
        echo "PX-C search failed: backend=$search_backend path=$path pattern=$pattern" >&2
        return "$status"
    fi
    if [ -n "$output" ]; then
        printf '%s\n' "$output"
    fi
}

raw="$temporary/headline.raw.csv"
printf '%s\n' 'category,layer,path,line,match' > "$raw"

scan_headline() {
    category=$1
    pattern=$2
    while IFS=, read -r layer path surface; do
        if [ "$layer" = layer ]; then
            continue
        fi
        if ! matches=$(search_file "$pattern" "$path"); then
            exit 1
        fi
        if [ -n "$matches" ]; then
            printf '%s\n' "$matches" | while IFS=: read -r line match; do
                printf '"%s","%s","%s",%s,"%s"\n' \
                    "$(csv_quote "$category")" \
                    "$(csv_quote "$layer")" \
                    "$(csv_quote "$path")" \
                    "$line" \
                    "$(csv_quote "$match")" >> "$raw"
            done
        fi
    done < "$manifest"
}

scan_headline typed_episode '\b(Episode|episode)\b'
scan_headline typed_history '\b([A-Za-z_][A-Za-z0-9_]*History[A-Za-z0-9_]*|history|histories)\b'
scan_headline typed_query '\b(Query|query)\b'
scan_headline begin_episode '\bbegin_episode\b'
scan_headline erase_temporary '\berase_temporary\b'
scan_headline seed_built_development '\b(fixture|event|broken_event|event_len|raw_consequence|raw|productive|contrast|route|distractors|session|chain_episode|duplicate_episode|branch_episode|cycle_episode)[[:space:]]*\([^;\n]*\bseed\b'
scan_headline explicit_mechanism_invocation '\bfrozen_[A-Za-z0-9_]+::[A-Za-z_][A-Za-z0-9_]*[[:space:]]*\('
scan_headline typed_layer_handoff '\b(FROZEN_[A-Z0-9_]*HANDOFF[A-Z0-9_]*|frozen_m[0-9]+|frozen_pre_m[0-9_]*|frozen_cumulative|frozen_event|frozen_request)\b'

{
    head -n 1 "$raw"
    tail -n +2 "$raw" | LC_ALL=C sort
} > "$temporary/headline.csv"

printf '%s\n' \
    'original_category,primary_kind,layer,path,line,match,semantic_risk,evaluator_input_risk,source_line' \
    > "$inventory"

tail -n +2 "$temporary/headline.csv" \
    | while IFS=, read -r q_category q_layer q_path line q_match; do
        category=$(unquote "$q_category")
        layer=$(unquote "$q_layer")
        path=$(unquote "$q_path")
        match=$(unquote "$q_match")
        source_line=$(sed -n "${line}p" "$path")

        case "$category" in
            typed_episode|typed_history|typed_query)
                primary=typed_representation
                ;;
            begin_episode)
                primary=episode_reset_boundary
                ;;
            erase_temporary)
                primary=manual_temporary_cleanup
                ;;
            explicit_mechanism_invocation)
                primary=explicit_mechanism_invocation
                ;;
            typed_layer_handoff)
                primary=typed_handoff
                ;;
            seed_built_development)
                case "$match" in
                    fixture\(*|session\(*|chain_episode\(*|duplicate_episode\(*|branch_episode\(*|cycle_episode\(*)
                        primary=evaluator_derived_input
                        ;;
                    productive\(*|contrast\(*|raw\(*|raw_consequence\(*)
                        primary=semantic_condition
                        ;;
                    *)
                        primary=seed_history_synthesis
                        ;;
                esac
                ;;
            *)
                echo "unclassified PX-C category: $category" >&2
                exit 1
                ;;
        esac

        semantic_risk=0
        evaluator_risk=0
        if printf '%s\n' "$source_line" | grep -Eq '\b(BindingOutcome|successful|correct|productive|contrast|functional_relation|ordinary_consequence|target_request|answer|terminal|passed|reconstructed|expected|outcome)\b'; then
            semantic_risk=1
        fi
        if printf '%s\n' "$source_line" | grep -Eq '\b(PROTOCOL|PROBE_SEED|MICRO_SEEDS|GATE_SEEDS|SEEDS|seed|fixture|heldout|held_out|HarnessMode|report|snapshot|controls|acquisition)\b'; then
            evaluator_risk=1
        fi

        printf '"%s","%s","%s","%s",%s,"%s",%s,%s,"%s"\n' \
            "$(csv_quote "$category")" \
            "$(csv_quote "$primary")" \
            "$(csv_quote "$layer")" \
            "$(csv_quote "$path")" \
            "$line" \
            "$(csv_quote "$match")" \
            "$semantic_risk" \
            "$evaluator_risk" \
            "$(csv_quote "$source_line")" >> "$inventory"
    done

guard_raw="$temporary/guard.raw.csv"
printf '%s\n' 'guard,layer,path,line,match' > "$guard_raw"

scan_guard() {
    guard=$1
    pattern=$2
    while IFS=, read -r layer path surface; do
        if [ "$layer" = layer ]; then
            continue
        fi
        if ! matches=$(search_file "$pattern" "$path"); then
            exit 1
        fi
        if [ -n "$matches" ]; then
            printf '%s\n' "$matches" | while IFS=: read -r line match; do
                printf '"%s","%s","%s",%s,"%s"\n' \
                    "$(csv_quote "$guard")" \
                    "$(csv_quote "$layer")" \
                    "$(csv_quote "$path")" \
                    "$line" \
                    "$(csv_quote "$match")" >> "$guard_raw"
            done
        fi
    done < "$manifest"
}

scan_guard semantic_condition '\b(BindingOutcome|successful|correct|productive|contrast|functional_relation|ordinary_consequence|target_request|answer|terminal|passed|reconstructed|expected|outcome)\b'
scan_guard evaluator_derived_input '\b(PROTOCOL|PROBE_SEED|MICRO_SEEDS|GATE_SEEDS|SEEDS|seed|fixture|heldout|held_out|HarnessMode|report|snapshot|controls|acquisition)\b'

{
    head -n 1 "$guard_raw"
    tail -n +2 "$guard_raw" | LC_ALL=C sort
} > "$guard_inventory"

headline_total=$(tail -n +2 "$inventory" | wc -l | tr -d ' ')
unique_lines=$(
    tail -n +2 "$inventory" | cut -d, -f3-5 | LC_ALL=C sort -u | wc -l | tr -d ' '
)
semantic_guard=$(awk -F, '$1 == "\"semantic_condition\"" { n += 1 } END { print n + 0 }' "$guard_inventory")
evaluator_guard=$(awk -F, '$1 == "\"evaluator_derived_input\"" { n += 1 } END { print n + 0 }' "$guard_inventory")

printf '%s\n' 'metric,count' > "$summary"
printf 'TOTAL_OCCURRENCES,%s\n' "$headline_total" >> "$summary"
printf 'UNIQUE_SOURCE_LINES,%s\n' "$unique_lines" >> "$summary"
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
    count=$(awk -F, -v key="\"$kind\"" '$2 == key { n += 1 } END { print n + 0 }' "$inventory")
    printf 'KIND_%s,%s\n' "$kind" "$count" >> "$summary"
done
printf 'GUARD_semantic_condition,%s\n' "$semantic_guard" >> "$summary"
printf 'GUARD_evaluator_derived_input,%s\n' "$evaluator_guard" >> "$summary"
for layer in 'PX0-PX3+LR-C' PX4 PX5 PX6 PX7 PX8; do
    count=$(awk -F, -v key="\"$layer\"" '$3 == key { n += 1 } END { print n + 0 }' "$inventory")
    printf 'LAYER_%s,%s\n' "$layer" "$count" >> "$summary"
done

kind_sum=$(awk -F, '$1 ~ /^KIND_/ { sum += $2 } END { print sum + 0 }' "$summary")
if [ "$kind_sum" -ne "$headline_total" ]; then
    echo "PX-C primary taxonomy is not exhaustive: kinds=$kind_sum total=$headline_total" >&2
    exit 1
fi

if [ -n "${PXC_MAX_TOTAL:-}" ] && [ "$headline_total" -gt "$PXC_MAX_TOTAL" ]; then
    echo "PX-C headline total $headline_total exceeds ceiling $PXC_MAX_TOTAL" >&2
    exit 1
fi
if [ -n "${PXC_MAX_SEMANTIC_GUARD:-}" ] && [ "$semantic_guard" -gt "$PXC_MAX_SEMANTIC_GUARD" ]; then
    echo "PX-C semantic guard $semantic_guard exceeds ceiling $PXC_MAX_SEMANTIC_GUARD" >&2
    exit 1
fi
if [ -n "${PXC_MAX_EVALUATOR_GUARD:-}" ] && [ "$evaluator_guard" -gt "$PXC_MAX_EVALUATOR_GUARD" ]; then
    echo "PX-C evaluator guard $evaluator_guard exceeds ceiling $PXC_MAX_EVALUATOR_GUARD" >&2
    exit 1
fi

{
    printf '# PX-C seam taxonomy baseline v2\n\n'
    printf 'Frozen v1 reference: **368 occurrences / 295 source lines**.\n\n'
    printf 'Audited commit: `%s`.  \n' "$commit"
    printf 'Manifest: `%s`.  \n' "$manifest"
    printf 'Manifest SHA-256: `%s`.  \n' "$manifest_hash"
    printf 'Search backend: `%s`.\n\n' "$search_backend"
    printf '| primary kind | count |\n|---|---:|\n'
    awk -F, '$1 ~ /^KIND_/ { sub(/^KIND_/, "", $1); printf "| `%s` | %s |\n", $1, $2 }' "$summary"
    printf '\nHeadline total: **%s** across **%s** unique lines.\n\n' "$headline_total" "$unique_lines"
    printf '| layer | occurrences |\n|---|---:|\n'
    awk -F, '$1 ~ /^LAYER_/ { sub(/^LAYER_/, "", $1); printf "| `%s` | %s |\n", $1, $2 }' "$summary"
    printf '\n'
    printf '| relocation guard | count |\n|---|---:|\n'
    printf '| semantic condition | %s |\n' "$semantic_guard"
    printf '| evaluator-derived input | %s |\n\n' "$evaluator_guard"
    printf 'The guard counts are orthogonal and may overlap. They do not rewrite the immutable v1 headline total. All three metrics must be non-increasing at lane readiness.\n'
} > "$report"

printf 'PX-C taxonomy: total=%s unique_lines=%s semantic_guard=%s evaluator_guard=%s\n' \
    "$headline_total" "$unique_lines" "$semantic_guard" "$evaluator_guard"
printf 'manifest=%s backend=%s\n' "$manifest_hash" "$search_backend"
cat "$summary"
