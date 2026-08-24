#!/bin/sh
set -eu

root=$(git rev-parse --show-toplevel)
cd "$root"

sha256_file() {
    shasum -a 256 "$1" | awk '{print $1}'
}

require_hash() {
    expected=$1
    path=$2
    actual=$(sha256_file "$path")
    if [ "$actual" != "$expected" ]; then
        echo "fingerprint mismatch: $path" >&2
        echo "expected $expected" >&2
        echo "actual   $actual" >&2
        exit 1
    fi
}

base=4ba88b6ed03b8e012231363fa6e3c29ea41308bb
git merge-base --is-ancestor "$base" HEAD

unexpected=$(
    git diff --name-only --diff-filter=DMRTUXB "$base" -- src experiments results |
        awk '
            /^src\/lib.rs$/ { next }
            /^src\/organism\// { next }
            /^src\/bin\/post_m8_consolidation.rs$/ { next }
            { print }
        '
)
if [ -n "$unexpected" ]; then
    echo "frozen research surface changed:" >&2
    echo "$unexpected" >&2
    exit 1
fi

require_hash 7883f71918d48c4c622d7cd2d9dd7561f5954f7287f8bc6abb535f5a9f994a55 results/ffs_same1_compiled_correspondence.csv
require_hash fede145a50bc059ffcd19a26dc65763843a83b1644c89bd44a3b27e8cd7cea27 results/ds1_boundary_role_cumulative_definitive.csv
require_hash 68d6dd31ca15e206b382f3ef6592804882eecfe09efd6696b8ed403dc6304159 results/ds2_cumulative_causal_direction_definitive.csv
require_hash ac8c0a6c9b7badfa263ceb054ffe59c11162b1ca256c56cc6df5f0d378179401 results/ds3_cumulative_event_boundary_definitive.csv
require_hash 5c4a2e2b021a26a4cc2161202dd9a62205d426ba361f90a69d00ceb3df470a83 results/ds6_cumulative_lifetime_definitive.csv
require_hash 86d9f6e3a8ab4ad5c242e0d7c619d8eda99e0da47faff623f26c8c6835b9a99a results/ds7_cumulative_plasticity_allocation_definitive.csv
require_hash 0cb9ba779fca1899cf030d30358fe9354cfb7b2cccf87f32df3f6ea9ddfe91e4 results/ds8_cumulative_semantic_credit_definitive.csv
require_hash 13619c786471b34f5dc9da914c4a0f454bab8d95a87142ce6c9e35808f3dd91a results/post_m6_ds4_arrival_initiation_definitive.csv
require_hash 20b052cd513e12c8b5873289647dda95f7991026671b4c309c60bd481900705b results/post_m7_ds5_closure_emission_definitive.csv

ssa_commit=$(git rev-parse deterministic-affordance-causal-window-authoritative^{commit})
if [ "$ssa_commit" != ce735bb7b3dab0d17ede176f863257e19c42900a ]; then
    echo "SSA0.3 authority moved: $ssa_commit" >&2
    exit 1
fi
ssa_hash=$(
    git show deterministic-affordance-causal-window-authoritative:results/ssa0_3_precommit_support_definitive_v1.csv |
        shasum -a 256 |
        awk '{print $1}'
)
if [ "$ssa_hash" != 50c46962c2388359a46b2a12ce74f8bcba4bcbb33c651f2c908fcd35e16ee631 ]; then
    echo "SSA0.3 authority fingerprint mismatch: $ssa_hash" >&2
    exit 1
fi

cargo fmt --all -- --check
cargo test -p frozen-organism-v1-physics
cargo test -p organism-v0 --lib organism::
cargo run -p organism-v0 --quiet --bin post_m8_consolidation

if [ "${1:-}" = "--full" ]; then
    cargo run -p organism-v0 --release --quiet --bin post_m8_consolidation -- --m8-gate
fi

echo "post-M8 consolidation verification: PASS"
