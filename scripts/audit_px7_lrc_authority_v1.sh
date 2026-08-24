#!/bin/sh
set -eu

if root=$(git rev-parse --show-toplevel 2>/dev/null); then
    commit=$(git rev-parse HEAD)
else
    root=${PX7_SOURCE_ROOT:-$PWD}
    commit=${PX7_AUDITED_COMMIT:-}
    if [ -z "$commit" ]; then
        echo "archive audit requires PX7_AUDITED_COMMIT" >&2
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
require_hash 9e14b0f065ba37966c2ffc300f6d149b0847d092cb90e666456d3889d889d9c6 \
    results/px6_lrc_consequence_authority_v1.csv
require_hash 94a088fc732c24385a3b581af0e5cea2638645806c8bb8a73c81bfa39c9ec5a2 \
    results/px6_lrc_consequence_authority_v1.md
require_hash 653289cf42577dabb242475fd88abe24405b3e9a7e3cd4f2961489cc5fe6953a \
    experiments/pxc_active_surface_manifest_v4.csv
require_hash d248a8af479872d8148115a405ae7332f7d24ca229378d3fde898ffd3d19e63e \
    crates/px7-lrc-arrival/src/lib.rs
require_hash PROTOCOL_HASH_TBD \
    experiments/px7_lrc_cumulative_arrival_authority_protocol_v1.md
require_hash EVALUATOR_HASH_TBD \
    arms/px7-lrc-arrival-authority/src/main.rs
require_hash CARGO_HASH_TBD \
    arms/px7-lrc-arrival-authority/Cargo.toml

source=arms/px7-lrc-arrival-authority/src/main.rs

if grep -En \
    'unsafe[[:space:]]+(fn|impl|trait|\{|\()|std::cell|RefCell|UnsafeCell|thread_local|static mut|OnceLock|LazyLock|proc_macro|include!|include_bytes!|transmute|Box::leak|mem::forget' \
    "$source"; then
    echo "forbidden Rust technique in PX7 authority evaluator" >&2
    exit 1
fi

if grep -En \
    '\b(RewardSignal|CorrectnessObject|OutcomeObject|SemanticHistory|RouteOwner|StartCommand|RequestObject|QueryObject|SessionObject|LevelSelector|PX7State)\b' \
    "$source"; then
    echo "forbidden semantic mechanism in PX7 authority evaluator" >&2
    exit 1
fi

dependencies=$(
    awk '
        /^\[dependencies\]$/ { inside=1; next }
        /^\[/ { inside=0 }
        inside && /^[A-Za-z0-9_-]+[[:space:]]*=/ { print $1 }
    ' arms/px7-lrc-arrival-authority/Cargo.toml | LC_ALL=C sort
)
expected_dependencies=$(printf '%s\n' \
    lr1-modulatory-physical-return px4-lrc-lifetime px7-lrc-arrival | LC_ALL=C sort)
if [ "$dependencies" != "$expected_dependencies" ]; then
    echo "authority evaluator dependency surface changed" >&2
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
if grep -En '\b(run|replay|Body::new|field|PlasticSubstrate)[[:space:]]*\(' "$preflight"; then
    echo "authority preflight can construct or execute a physical world" >&2
    exit 1
fi

if [ "$(grep -c 'map(replay)' "$source")" -ne 1 ]; then
    echo "row runner has an unregistered authority call surface" >&2
    exit 1
fi
if [ "$(grep -c 'PX7_LRC_ARRIVAL_AUTHORITY_V1_EVIDENCE_SPENT' "$source")" -ne 1 ]; then
    echo "authority evidence marker count changed" >&2
    exit 1
fi

for authority_root in \
    761_001 761_002 761_003 761_004 761_005 761_006 761_007 761_008 \
    761_009 761_010 761_011 761_012 761_013 761_014 761_015 761_016
do
    if ! grep -q "$authority_root" "$source"; then
        echo "registered authority root missing: $authority_root" >&2
        exit 1
    fi
done
if grep -En '7_700_001|7_710_00|7_720_0|PROBE_CASES|MICRO_CASES|GATE_CASES|--probe|--micro|--gate' "$source"; then
    echo "isolated PX7 execution identity imported" >&2
    exit 1
fi

for artifact in \
    results/px7_lrc_arrival_authority_v1.csv \
    results/px7_lrc_arrival_authority_v1.md \
    results/px7_lrc_arrival_authority_v1.csv.staging \
    results/px7_lrc_arrival_authority_v1.md.staging
do
    if [ -e "$artifact" ]; then
        echo "PX7 authority artifact exists during preflight: $artifact" >&2
        exit 1
    fi
done

printf 'PX7_LRC_AUTHORITY_AUDIT_OK commit=%s active_sources=3 new_active_px7=1 evaluator_sources=1 unclassified=0\n' \
    "$commit"
