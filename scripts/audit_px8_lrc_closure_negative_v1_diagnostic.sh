#!/bin/sh
set -eu

if root=$(git rev-parse --show-toplevel 2>/dev/null); then
    commit=$(git rev-parse HEAD)
else
    root=${PX8_DIAGNOSTIC_SOURCE_ROOT:-$PWD}
    commit=${PX8_DIAGNOSTIC_COMMIT:-}
    if [ -z "$commit" ]; then
        echo "archive audit requires PX8_DIAGNOSTIC_COMMIT" >&2
        exit 1
    fi
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
    source_file=$2
    actual=$(sha "$source_file")
    if [ "$actual" != "$expected" ]; then
        echo "frozen input changed: $source_file expected=$expected actual=$actual" >&2
        exit 1
    fi
}

require_hash 7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10 \
    crates/lr1-modulatory-physical-return/src/lib.rs
require_hash a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71 \
    arms/px4-lrc-lifetime/src/lib.rs
require_hash d248a8af479872d8148115a405ae7332f7d24ca229378d3fde898ffd3d19e63e \
    crates/px7-lrc-arrival/src/lib.rs
require_hash 8623cae7dc1b14e666140a192c49e59ae72df681bc14f0359f0c4465e0d11e8f \
    arms/px8-lrc-physical-closure/src/lib.rs
require_hash ccbf3547ae0534ccbbb0c00e8d058f47f9471afb4a30733cc124e981a0f606d0 \
    arms/px8-lrc-closure-authority/src/main.rs
require_hash 3c8df23536157cc91d315c96862d408027aabdb28c65bab133696405132b3116 \
    experiments/px8_lrc_cumulative_closure_authority_negative_diagnostic_v1.md
require_hash 0d769dfb4b6c9a0420cfdf8f6c299aa89c8b614720cc15386365ca6c6a2577a5 \
    experiments/px8_lrc_closure_negative_v1_diagnostic_protocol_v1.md
require_hash dd1a9e61b866ec64f3e96be0d948dc668efd0b4bbf05a6c3d5bf5fa30be94a64 \
    arms/px8-lrc-closure-diagnostic/src/main.rs
require_hash f337b45ea430f596d56b5630228ab2c4e1bd9d5e54ae8788a7552193816fc797 \
    arms/px8-lrc-closure-diagnostic/Cargo.toml

source=arms/px8-lrc-closure-diagnostic/src/main.rs

dependencies=$(
    awk '
        /^\[dependencies\]$/ { inside=1; next }
        /^\[/ { inside=0 }
        inside && /^[A-Za-z0-9_-]+[[:space:]]*=/ { print $1 }
    ' arms/px8-lrc-closure-diagnostic/Cargo.toml | LC_ALL=C sort
)
expected_dependencies=$(printf '%s\n' px7-lrc-arrival px8-lrc-physical-closure | LC_ALL=C sort)
if [ "$dependencies" != "$expected_dependencies" ]; then
    echo "diagnostic dependency surface changed" >&2
    exit 1
fi

if grep -Fq 'PX8_LRC_CLOSURE_AUTHORITY_V1_EVIDENCE_SPENT' "$source"; then
    echo "authority-v1 evidence marker leaked into diagnostic" >&2
    exit 1
fi
if grep -En -- '--authority-v1|px8_lrc_closure_authority_v1\.(csv|md)' "$source"; then
    echo "authority-v1 execution or result surface leaked into diagnostic" >&2
    exit 1
fi
if [ "$(grep -c 'PX8_LRC_CLOSURE_NEGATIVE_V1_DIAGNOSTIC_SPENT' "$source")" -ne 1 ]; then
    echo "diagnostic marker count changed" >&2
    exit 1
fi
if [ "$(grep -c 'map(reconstruct)' "$source")" -ne 1 ]; then
    echo "diagnostic row execution surface changed" >&2
    exit 1
fi

for root_value in \
    862_001 862_002 862_003 862_004 862_005 862_006 862_007 862_008 \
    862_009 862_010 862_011 862_012 862_013 862_014 862_015 862_016
do
    if ! grep -q "$root_value" "$source"; then
        echo "registered diagnostic root missing: $root_value" >&2
        exit 1
    fi
done
if grep -En '861_00|861_01|1_208_|--probe|--micro|--gate' "$source"; then
    echo "authority or isolated execution identity leaked into diagnostic" >&2
    exit 1
fi

mode=${PX8_DIAGNOSTIC_AUDIT_MODE:-preflight}
case "$mode" in
    preflight)
        for artifact in \
            results/px8_lrc_closure_negative_v1_diagnostic.csv \
            results/px8_lrc_closure_negative_v1_diagnostic.md \
            results/px8_lrc_closure_negative_v1_diagnostic.csv.staging \
            results/px8_lrc_closure_negative_v1_diagnostic.md.staging
        do
            if [ -e "$artifact" ]; then
                echo "diagnostic artifact exists during preflight: $artifact" >&2
                exit 1
            fi
        done
        ;;
    result)
        test -f results/px8_lrc_closure_negative_v1_diagnostic.csv
        test -f results/px8_lrc_closure_negative_v1_diagnostic.md
        test ! -e results/px8_lrc_closure_negative_v1_diagnostic.csv.staging
        test ! -e results/px8_lrc_closure_negative_v1_diagnostic.md.staging
        awk -F, '
            NR == 1 { if (NF != 21) exit 1; next }
            {
                records++
                roots[$1]++
                clauses[$1 SUBSEP $7]++
                if ($19 != "true") replay_failures++
            }
            END {
                if (records != 224) exit 1
                root_count = 0
                for (root in roots) {
                    root_count++
                    if (roots[root] != 14) exit 1
                }
                if (root_count != 16) exit 1
                for (clause in clauses) if (clauses[clause] != 1) exit 1
                if (replay_failures != 0) exit 1
            }
        ' results/px8_lrc_closure_negative_v1_diagnostic.csv
        grep -Fq 'Outcome: **DIAGNOSTIC COMPLETE; NOT AUTHORITY**.' \
            results/px8_lrc_closure_negative_v1_diagnostic.md
        grep -Fq -- '- roots serialized: `16/16`;' \
            results/px8_lrc_closure_negative_v1_diagnostic.md
        grep -Fq -- '- clause records serialized: `224/224`;' \
            results/px8_lrc_closure_negative_v1_diagnostic.md
        ;;
    *)
        echo "unknown PX8_DIAGNOSTIC_AUDIT_MODE: $mode" >&2
        exit 1
        ;;
esac

printf 'PX8_LRC_CLOSURE_DIAGNOSTIC_AUDIT_OK commit=%s mode=%s active_changes=0 evaluator_sources=1 unclassified=0\n' \
    "$commit" "$mode"
