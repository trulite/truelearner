#!/bin/sh
set -eu

if root=$(git rev-parse --show-toplevel 2>/dev/null); then
    commit=$(git rev-parse HEAD)
else
    root=${PX5_SOURCE_ROOT:-$PWD}
    commit=${PX5_AUDITED_COMMIT:-}
    if [ -z "$commit" ]; then
        echo "archive audit requires PX5_AUDITED_COMMIT" >&2
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
    path=$2
    actual=$(sha "$path")
    if [ "$actual" != "$expected" ]; then
        echo "frozen input changed: $path expected=$expected actual=$actual" >&2
        exit 1
    fi
}

require_hash 7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10 \
    crates/lr1-modulatory-physical-return/src/lib.rs
require_hash a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71 \
    arms/px4-lrc-lifetime/src/lib.rs
require_hash 848c3b030824d6bc404dddad9498046b55d1f71c4d7e4ff10fda05cffb29e995 \
    experiments/px4_lrc_physical_lifetime_authority_handoff_v1.md
require_hash 050a2b489e41d13e8d8a3d55dd7d69c6e06894b85b2c172f7dc24614af09aeaa \
    results/px4_lrc_lifetime_authority_v1.csv
require_hash 445c465ba61cc12c0ece84a8ebb9a83bea1e67c1a4d640964cc7d93c0dbe4390 \
    results/px4_lrc_lifetime_authority_v1.md
require_hash 497c559f9477252195e870d2b4be8dfd38f09b163438ecce7047e2f63077c443 \
    experiments/px5_lrc_cumulative_allocation_authority_protocol_v1.md

manifest=experiments/pxc_active_surface_manifest_v3.csv
expected_manifest=${PX5_EXPECT_MANIFEST_HASH:-}
if [ -z "$expected_manifest" ] || [ "$(sha "$manifest")" != "$expected_manifest" ]; then
    echo "PX5_EXPECT_MANIFEST_HASH must match manifest v3" >&2
    exit 1
fi

source=arms/px5-lrc-allocation-authority/src/main.rs
expected_source=${PX5_EXPECT_EVALUATOR_HASH:-}
if [ -z "$expected_source" ] || [ "$(sha "$source")" != "$expected_source" ]; then
    echo "PX5_EXPECT_EVALUATOR_HASH must match frozen evaluator" >&2
    exit 1
fi

if grep -En \
    'unsafe[[:space:]]+(fn|impl|trait|\{|\()|std::cell|RefCell|UnsafeCell|thread_local|static mut|OnceLock|LazyLock|proc_macro|include!|include_bytes!|transmute|Box::leak|mem::forget' \
    "$source"; then
    echo "forbidden Rust technique in PX5 evaluator" >&2
    exit 1
fi

if grep -En \
    '\b(Allocator|TargetList|History|Episode|SemanticAdmission|LearnHere|PlasticityCommand)\b' \
    "$source"; then
    echo "forbidden semantic mechanism in PX5 evaluator" >&2
    exit 1
fi

dependencies=$(
    awk '
        /^\[dependencies\]$/ { inside=1; next }
        /^\[/ { inside=0 }
        inside && /^[A-Za-z0-9_-]+[[:space:]]*=/ { print $1 }
    ' arms/px5-lrc-allocation-authority/Cargo.toml | LC_ALL=C sort
)
expected_dependencies=$(printf '%s\n' lr1-modulatory-physical-return px4-lrc-lifetime | LC_ALL=C sort)
if [ "$dependencies" != "$expected_dependencies" ]; then
    echo "authority evaluator dependency surface changed" >&2
    exit 1
fi

expected_manifest_rows=$(mktemp)
trap 'rm -f "$expected_manifest_rows"' EXIT HUP INT TERM
printf '%s\n' \
    'layer,path,surface' \
    'PX0-PX3+LR-C,crates/lr1-modulatory-physical-return/src/lib.rs,authoritative-physical-foundation' \
    'PX4,arms/px4-lrc-lifetime/src/lib.rs,development-candidate' \
    'PX5,crates/lr1-modulatory-physical-return/src/lib.rs,shared-authoritative-physical-allocation' \
    'PX6,src/ds8_cumulative_semantic_credit_probe.rs,predecessor-target' \
    'PX6,src/ds8_cumulative_semantic_credit_gate.rs,predecessor-target' \
    'PX7,src/post_m6_ds4_arrival_initiation.rs,predecessor-target' \
    'PX8,src/post_m7_ds5_closure_emission.rs,predecessor-target' \
    > "$expected_manifest_rows"
if ! cmp -s "$expected_manifest_rows" "$manifest"; then
    echo "manifest v3 contains an unregistered row" >&2
    exit 1
fi

for path in \
    crates/lr1-modulatory-physical-return/src/lib.rs \
    arms/px4-lrc-lifetime/src/lib.rs
do
    if [ ! -f "$path" ]; then
        echo "active source missing: $path" >&2
        exit 1
    fi
done

if [ -e results/px5_lrc_allocation_authority_v1.csv ] \
    || [ -e results/px5_lrc_allocation_authority_v1.md ]; then
    echo "authority artifact exists during preflight" >&2
    exit 1
fi

printf 'PX5_LRC_AUTHORITY_AUDIT_OK commit=%s active_sources=2 evaluator_sources=1 unclassified=0\n' \
    "$commit"
