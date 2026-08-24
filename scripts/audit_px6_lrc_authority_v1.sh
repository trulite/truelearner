#!/bin/sh
set -eu

if root=$(git rev-parse --show-toplevel 2>/dev/null); then
    commit=$(git rev-parse HEAD)
else
    root=${PX6_SOURCE_ROOT:-$PWD}
    commit=${PX6_AUDITED_COMMIT:-}
    if [ -z "$commit" ]; then
        echo "archive audit requires PX6_AUDITED_COMMIT" >&2
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
require_hash 1e6c947660b55d6f21060734c57a54f56541303b65069f23010344ca7d362a97 \
    experiments/px5_lrc_cumulative_allocation_authority_handoff_v1.md
require_hash 5ccfa15b6da93ac276b9474c4d501ef9c7769748c52dbf7a8882620758b1259a \
    results/px5_lrc_allocation_authority_v1.csv
require_hash e96622614e4c9569f1f90d60fa0ef822072afae5e09c316b2c37344e31f194ed \
    results/px5_lrc_allocation_authority_v1.md
require_hash 32fda2d86f6c836438fdd01fc5433c8731f4ec92e3e052e5d1a64fe751d15388 \
    experiments/pxc_active_surface_manifest_v3.csv
require_hash bec04fbcefa97567ab8e3034c38915517460693acd2d57376c41eae4dd898990 \
    experiments/px6_lrc_cumulative_consequence_authority_protocol_v1.md
require_hash 3b9477d63d13e80ee0e50328d42a10f458e43b80fbd607d0cacc893e6312e1a2 \
    arms/px6-lrc-consequence-authority/src/main.rs
require_hash ce46ecec4237431600859ba090346fcbf821e8c8df8c7e906b02c33cb6a5908b \
    arms/px6-lrc-consequence-authority/Cargo.toml

source=arms/px6-lrc-consequence-authority/src/main.rs

if grep -En \
    'unsafe[[:space:]]+(fn|impl|trait|\{|\()|std::cell|RefCell|UnsafeCell|thread_local|static mut|OnceLock|LazyLock|proc_macro|include!|include_bytes!|transmute|Box::leak|mem::forget' \
    "$source"; then
    echo "forbidden Rust technique in PX6 authority evaluator" >&2
    exit 1
fi

if grep -En \
    '\b(RewardSignal|CorrectnessObject|OutcomeObject|SemanticHistory|RouteOwner|CreditCommand|EvaluatorValue|PX6State)\b' \
    "$source"; then
    echo "forbidden semantic mechanism in PX6 authority evaluator" >&2
    exit 1
fi

dependencies=$(
    awk '
        /^\[dependencies\]$/ { inside=1; next }
        /^\[/ { inside=0 }
        inside && /^[A-Za-z0-9_-]+[[:space:]]*=/ { print $1 }
    ' arms/px6-lrc-consequence-authority/Cargo.toml | LC_ALL=C sort
)
expected_dependencies=$(printf '%s\n' lr1-modulatory-physical-return px4-lrc-lifetime | LC_ALL=C sort)
if [ "$dependencies" != "$expected_dependencies" ]; then
    echo "authority evaluator dependency surface changed" >&2
    exit 1
fi

preflight=$(mktemp)
trap 'rm -f "$preflight"' EXIT HUP INT TERM
awk '
    /^fn preflight\(\)/ { inside=1 }
    /^fn authority\(\)/ { inside=0 }
    inside { print }
' "$source" > "$preflight"
if grep -En '\b(core|replay_row|loop_trial|PlasticSubstrate|field)[[:space:]]*\(' "$preflight"; then
    echo "authority preflight can construct or execute a physical world" >&2
    exit 1
fi

if [ "$(grep -c 'replay_row(' "$source")" -ne 2 ]; then
    echo "row runner has an unregistered call surface" >&2
    exit 1
fi
if [ "$(grep -c 'PX6_LRC_CONSEQUENCE_AUTHORITY_V1_EVIDENCE_SPENT' "$source")" -ne 1 ]; then
    echo "authority evidence marker count changed" >&2
    exit 1
fi

for authority_root in 661001 661002 661003 661004 661005 661006 661007 661008; do
    if ! grep -q "${authority_root}" "$source"; then
        echo "registered authority root missing: $authority_root" >&2
        exit 1
    fi
done
if grep -En '0x6000_0000|0x6100_0000|0x7000_0000|40_000_000|42_000_000' "$source"; then
    echo "isolated PX6 identity imported into authority evaluator" >&2
    exit 1
fi

if [ -e results/px6_lrc_consequence_authority_v1.csv ] \
    || [ -e results/px6_lrc_consequence_authority_v1.md ] \
    || [ -e results/px6_lrc_consequence_authority_v1.csv.staging ] \
    || [ -e results/px6_lrc_consequence_authority_v1.md.staging ]; then
    echo "PX6 authority artifact exists during preflight" >&2
    exit 1
fi

printf 'PX6_LRC_AUTHORITY_AUDIT_OK commit=%s active_sources=2 new_active_px6=0 evaluator_sources=1 unclassified=0\n' \
    "$commit"

