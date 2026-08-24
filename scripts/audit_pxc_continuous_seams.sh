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
manifest=experiments/pxc_active_surface_manifest_v1.csv
output_dir=${1:-results}
inventory="$output_dir/pxc_continuous_seam_inventory_v1.csv"
summary="$output_dir/pxc_continuous_seam_summary_v1.csv"
report="$output_dir/pxc_continuous_seam_baseline_v1.md"

if [ "$repository_checkout" = true ]; then
    if ! git merge-base --is-ancestor "$authority" HEAD; then
        echo "PX-C audit must descend from PX3+LR-C authority $authority" >&2
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

while IFS=, read -r layer path surface; do
    if [ "$layer" = "layer" ]; then
        continue
    fi
    if [ ! -f "$path" ]; then
        echo "PX-C manifested source is missing: $path" >&2
        exit 1
    fi
done < "$manifest"

mkdir -p "$output_dir"
temporary=$(mktemp -d)
trap 'rm -rf "$temporary"' EXIT HUP INT TERM

raw="$temporary/inventory.raw.csv"
printf '%s\n' 'category,layer,path,line,match' > "$raw"

csv_quote() {
    printf '%s' "$1" | sed 's/"/""/g'
}

scan_category() {
    category=$1
    pattern=$2

    while IFS=, read -r layer path surface; do
        if [ "$layer" = "layer" ]; then
            continue
        fi
        rg -n -o --no-heading --color never -e "$pattern" "$path" 2>/dev/null \
            | while IFS=: read -r line match; do
                q_category=$(csv_quote "$category")
                q_layer=$(csv_quote "$layer")
                q_path=$(csv_quote "$path")
                q_match=$(csv_quote "$match")
                printf '"%s","%s","%s",%s,"%s"\n' \
                    "$q_category" "$q_layer" "$q_path" "$line" "$q_match" >> "$raw"
            done || true
    done < "$manifest"
}

scan_category typed_episode '\b(Episode|episode)\b'
scan_category typed_history '\b([A-Za-z_][A-Za-z0-9_]*History[A-Za-z0-9_]*|history|histories)\b'
scan_category typed_query '\b(Query|query)\b'
scan_category begin_episode '\bbegin_episode\b'
scan_category erase_temporary '\berase_temporary\b'
scan_category seed_built_development '\b(fixture|event|broken_event|event_len|raw_consequence|raw|productive|contrast|route|distractors|session|chain_episode|duplicate_episode|branch_episode|cycle_episode)[[:space:]]*\([^;\n]*\bseed\b'
scan_category explicit_mechanism_invocation '\bfrozen_[A-Za-z0-9_]+::[A-Za-z_][A-Za-z0-9_]*[[:space:]]*\('
scan_category typed_layer_handoff '\b(FROZEN_[A-Z0-9_]*HANDOFF[A-Z0-9_]*|frozen_m[0-9]+|frozen_pre_m[0-9_]*|frozen_cumulative|frozen_event|frozen_request)\b'

{
    head -n 1 "$raw"
    tail -n +2 "$raw" | LC_ALL=C sort
} > "$inventory"

count_category() {
    category=$1
    count=$(rg -c "^\"$category\"," "$inventory" 2>/dev/null || true)
    if [ -z "$count" ]; then
        count=0
    fi
    printf '%s' "$count"
}

total=$(tail -n +2 "$inventory" | wc -l | tr -d ' ')
unique_lines=$(
    tail -n +2 "$inventory" \
        | cut -d, -f2-4 \
        | LC_ALL=C sort -u \
        | wc -l \
        | tr -d ' '
)

printf '%s\n' 'category,count' > "$summary"
for category in \
    typed_episode \
    typed_history \
    typed_query \
    begin_episode \
    erase_temporary \
    seed_built_development \
    explicit_mechanism_invocation \
    typed_layer_handoff
do
    printf '%s,%s\n' "$category" "$(count_category "$category")" >> "$summary"
done
printf 'TOTAL_OCCURRENCES,%s\n' "$total" >> "$summary"
printf 'UNIQUE_SOURCE_LINES,%s\n' "$unique_lines" >> "$summary"

if command -v sha256sum >/dev/null 2>&1; then
    manifest_hash=$(sha256sum "$manifest" | awk '{print $1}')
else
    manifest_hash=$(shasum -a 256 "$manifest" | awk '{print $1}')
fi

{
    printf '# PX-C continuous seam baseline v1\n\n'
    printf 'Authority ancestor: `%s`.\n\n' "$authority"
    printf 'Audited commit: `%s`.\n\n' "$commit"
    printf 'Manifest SHA-256: `%s`.\n\n' "$manifest_hash"
    printf 'The count is a conservative lexical inventory over only the manifested active surface. Frozen reports, evaluator arms, binaries, build scripts, and superseded PX0--PX3 sources are excluded.\n\n'
    printf '| category | occurrences |\n'
    printf '|---|---:|\n'
    tail -n +2 "$summary" | while IFS=, read -r category count; do
        case "$category" in
            TOTAL_OCCURRENCES|UNIQUE_SOURCE_LINES) continue ;;
        esac
        printf '| `%s` | %s |\n' "$category" "$count"
    done
    printf '\nTotal occurrences: **%s** across **%s** unique manifested source lines.\n\n' "$total" "$unique_lines"
    printf 'This nonzero result is the frozen starting supply for the PX4--PX8 serial authority line, not a failure classification. Future authority-line audits must be equal or lower unless a new protocol version explicitly justifies an expanded surface.\n'
} > "$report"

if [ -n "${PXC_MAX_SEAMS:-}" ] && [ "$total" -gt "$PXC_MAX_SEAMS" ]; then
    echo "PX-C seam total $total exceeds ceiling $PXC_MAX_SEAMS" >&2
    exit 1
fi

printf 'PX-C seam audit: total=%s unique_lines=%s manifest=%s\n' \
    "$total" "$unique_lines" "$manifest_hash"
cat "$summary"
