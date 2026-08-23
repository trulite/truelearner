#!/bin/sh
set -eu

root=$(git rev-parse --show-toplevel)
cd "$root"

./scripts/verify_post_m8_consolidation.sh

expected_physics=6aa28a76e1362ac8dfb1d33fb68807da40e7604dfdc8cca9efa1e314e3ce4263
actual_physics=$(shasum -a 256 crates/frozen-organism-v1-physics/src/substrate.rs | awk '{print $1}')
if [ "$actual_physics" != "$expected_physics" ]; then
    echo "retained physics is not byte-identical to the frozen cleanup source" >&2
    exit 1
fi

tree=$(cargo tree -p frozen-organism-v1-physics --edges normal --prefix none)
if [ "$tree" != "frozen-organism-v1-physics v1.0.0 ($root/crates/frozen-organism-v1-physics)" ]; then
    echo "retained-physics crate acquired a normal dependency:" >&2
    echo "$tree" >&2
    exit 1
fi

if rg -n \
    'REQUEST|START|FINISH|ANSWER|CORRECT|WRONG|LEARN_HERE|MASTERED|CAPABILITY_|softmax|temperature|probability|random\(' \
    crates/frozen-organism-v1-physics/src; then
    echo "semantic or chooser vocabulary entered retained physics" >&2
    exit 1
fi

if rg -n 'organism[_-]v0|research_runtime|post_m[0-9]|ds[0-9_]' \
    crates/frozen-organism-v1-physics; then
    echo "retained physics depends on the historical research surface" >&2
    exit 1
fi

cargo fmt --all -- --check
cargo check -p frozen-organism-v1-physics --all-targets
git diff --check

echo "Frozen Organism v1 cleanup verification: PASS"
