#!/bin/sh
set -eu

if root=$(git rev-parse --show-toplevel 2>/dev/null); then
    commit=$(git rev-parse HEAD)
else
    root=${PX8_SOURCE_ROOT:-$PWD}
    commit=${PX8_AUDITED_COMMIT:-}
    if [ -z "$commit" ]; then
        echo "archive audit requires PX8_AUDITED_COMMIT" >&2
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
require_hash 96baddb76ef1c58dc0173f14a8fabded8c6237e73b729030e4698cf2fe300137 \
    results/px7_lrc_arrival_authority_v1.csv
require_hash 9b23ad3bd34050e1f13b8638b05baa5b9e29adc6cef5208db60dde36b7b058dc \
    results/px7_lrc_arrival_authority_v1.md
require_hash db4758baa5aeba36a87251f7d2ccb85cd2215f9489a1189eae4fd9d6408001c2 \
    experiments/pxc_active_surface_manifest_v5.csv
require_hash 510915f264be35318f0f84a62b2277335984458912b431f51da90c7aa1086f7c \
    experiments/px8_lrc_cumulative_closure_authority_protocol_v1.md
require_hash 8623cae7dc1b14e666140a192c49e59ae72df681bc14f0359f0c4465e0d11e8f \
    arms/px8-lrc-physical-closure/src/lib.rs
require_hash 646ea5f86baf276fefaee3ed3e06be56834281439959d436580ae300bb6fa9c6 \
    arms/px8-lrc-physical-closure/Cargo.toml
require_hash ccbf3547ae0534ccbbb0c00e8d058f47f9471afb4a30733cc124e981a0f606d0 \
    arms/px8-lrc-closure-authority/src/main.rs
require_hash 9e957a848c951241b9753557f1985baa26ce83b13a8e02c9a0a0dcce2b269278 \
    arms/px8-lrc-closure-authority/Cargo.toml
require_hash a601b9f0431d100109f014ebb72354a877de46abc1cd4fceff4dbfbb07226bf5 \
    experiments/px8_lrc_cumulative_closure_authority_coverage_audit_v1.md

source=arms/px8-lrc-closure-authority/src/main.rs
active=arms/px8-lrc-physical-closure/src/lib.rs

if grep -En \
    'unsafe[[:space:]]+(fn|impl|trait|\{|\()|std::cell|RefCell|UnsafeCell|thread_local|static mut|OnceLock|LazyLock|proc_macro|include!|include_bytes!|transmute|Box::leak|mem::forget' \
    "$source"; then
    echo "forbidden Rust technique in PX8 authority evaluator" >&2
    exit 1
fi

if grep -En \
    '\b(Episode|Query|RewardSignal|CorrectnessObject|OutcomeObject|SemanticHistory|RouteOwner|StartCommand|RequestObject|SessionObject|LevelSelector|PX8State|FinishAction)\b' \
    "$active"; then
    echo "forbidden semantic or lifecycle mechanism in active PX8" >&2
    exit 1
fi

dependencies=$(
    awk '
        /^\[dependencies\]$/ { inside=1; next }
        /^\[/ { inside=0 }
        inside && /^[A-Za-z0-9_-]+[[:space:]]*=/ { print $1 }
    ' arms/px8-lrc-closure-authority/Cargo.toml | LC_ALL=C sort
)
expected_dependencies=$(printf '%s\n' px7-lrc-arrival px8-lrc-physical-closure | LC_ALL=C sort)
if [ "$dependencies" != "$expected_dependencies" ]; then
    echo "authority evaluator dependency surface changed" >&2
    exit 1
fi

px8_dependencies=$(
    awk '
        /^\[dependencies\]$/ { inside=1; next }
        /^\[/ { inside=0 }
        inside && /^[A-Za-z0-9_-]+[[:space:]]*=/ { print $1 }
    ' arms/px8-lrc-physical-closure/Cargo.toml | LC_ALL=C sort
)
if [ -n "$px8_dependencies" ]; then
    echo "active PX8 Cargo dependency surface changed" >&2
    exit 1
fi
if ! grep -Fq '#[path = "../../../crates/lr1-modulatory-physical-return/src/lib.rs"]' "$active"; then
    echo "active PX8 no longer resolves the retained LR-C source directly" >&2
    exit 1
fi

px7_dependencies=$(
    awk '
        /^\[dependencies\]$/ { inside=1; next }
        /^\[/ { inside=0 }
        inside && /^[A-Za-z0-9_-]+[[:space:]]*=/ { print $1 }
    ' crates/px7-lrc-arrival/Cargo.toml | LC_ALL=C sort
)
if [ "$px7_dependencies" != lr1-modulatory-physical-return ]; then
    echo "active PX7 dependency surface changed" >&2
    exit 1
fi

preflight=$(mktemp)
trap 'rm -f "$preflight"' EXIT HUP INT TERM
awk '
    /^fn preflight\(\)/ { inside=1 }
    /^fn authority\(/ { inside=0 }
    inside { print }
' "$source" > "$preflight"
if grep -En '\b(run|replay|RecursiveBody::new|CompactBody::new|Body::new)[[:space:]]*\(' "$preflight"; then
    echo "authority preflight can construct or execute a physical world" >&2
    exit 1
fi

if [ "$(grep -c 'map(replay)' "$source")" -ne 1 ]; then
    echo "row runner has an unregistered authority call surface" >&2
    exit 1
fi
if [ "$(grep -c 'PX8_LRC_CLOSURE_AUTHORITY_V1_EVIDENCE_SPENT' "$source")" -ne 1 ]; then
    echo "authority evidence marker count changed" >&2
    exit 1
fi

for authority_root in \
    861_001 861_002 861_003 861_004 861_005 861_006 861_007 861_008 \
    861_009 861_010 861_011 861_012 861_013 861_014 861_015 861_016
do
    if ! grep -q "$authority_root" "$source"; then
        echo "registered authority root missing: $authority_root" >&2
        exit 1
    fi
done
if grep -En '1_208_|PROBE_CASES|MICRO_CASES|GATE_CASES|--probe|--micro|--gate' "$source" "$active"; then
    echo "isolated PX8 execution identity or runner imported" >&2
    exit 1
fi

mode=${PX8_AUDIT_MODE:-preflight}
case "$mode" in
    preflight)
        for artifact in \
            results/px8_lrc_closure_authority_v1.csv \
            results/px8_lrc_closure_authority_v1.md \
            results/px8_lrc_closure_authority_v1.csv.staging \
            results/px8_lrc_closure_authority_v1.md.staging
        do
            if [ -e "$artifact" ]; then
                echo "PX8 authority artifact exists during preflight: $artifact" >&2
                exit 1
            fi
        done
        ;;
    result)
        test -f results/px8_lrc_closure_authority_v1.csv
        test -f results/px8_lrc_closure_authority_v1.md
        test ! -e results/px8_lrc_closure_authority_v1.csv.staging
        test ! -e results/px8_lrc_closure_authority_v1.md.staging
        awk -F, '
            NR == 1 { if (NF != 30) exit 1; next }
            {
                rows++
                if ($28 != "true" || $30 != "true") exit 1
                count = split($29, claims, "|")
                if (count != 14) exit 1
                for (index = 1; index <= count; index++) if (claims[index] != "true") exit 1
            }
            END { if (rows != 16) exit 1 }
        ' results/px8_lrc_closure_authority_v1.csv
        grep -Fq 'Outcome: **DEFINITIVE POSITIVE**.' results/px8_lrc_closure_authority_v1.md
        grep -Fq -- '- total clauses: `230/230`;' results/px8_lrc_closure_authority_v1.md
        ;;
    *)
        echo "unknown PX8_AUDIT_MODE: $mode" >&2
        exit 1
        ;;
esac

printf 'PX8_LRC_CLOSURE_AUTHORITY_AUDIT_OK commit=%s mode=%s active_sources=4 new_active_px8=1 evaluator_sources=1 unclassified=0\n' \
    "$commit" "$mode"
