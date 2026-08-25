#!/usr/bin/env bash
set -euo pipefail

root=${1:-.}
output=${2:-/tmp/ah0-retained-gate-v1}
mkdir -p "$output"

cd "$root"
bash experiments/tools/audit_ah0_handle_ordering_removal_v1.sh .

cargo test --release --manifest-path truelearner/Cargo.toml \
    -p truelearner-core r1_r5_mechanical_prefixes_preserve_physics

cargo run --release --manifest-path experiments/verification/r6-partition-invariance-ah0/Cargo.toml \
    -- --output "$output/r6"

cargo run --release --manifest-path experiments/arms/si0-simultaneous-local-incidence/Cargo.toml \
    -- "$output/si0"
cargo run --release --manifest-path experiments/arms/pc0-cpc0-successor-conformance/Cargo.toml \
    -- "$output/cpc0"
cargo run --release --manifest-path experiments/arms/cpc1-local-temporal-participation/Cargo.toml \
    -- "$output/cpc1"
cargo run --release --manifest-path experiments/arms/pqlc0-participation-qualified-local-closure/Cargo.toml \
    -- "$output/pqlc0"
cargo run --release --manifest-path experiments/arms/pqlc1-depth-composition/Cargo.toml \
    -- "$output/pqlc1"
cargo run --release --manifest-path experiments/arms/fd0-phase-free-local-forgetting/Cargo.toml \
    -- "$output/fd0"
cargo run --release --manifest-path experiments/arms/fd1-consequence-consolidation/Cargo.toml \
    -- --v3 "$output/fd1"
cargo run --release --manifest-path experiments/arms/j0-junction-derived-lifetime/Cargo.toml \
    --features junction-model -- "$output/j0"
cargo run --release --manifest-path experiments/arms/cv0-bounded-local-contact-genesis/Cargo.toml \
    -- full "$output/cv0"

find "$output" -type f ! -name SHA256SUMS -print0 \
    | sort -z \
    | xargs -0 sha256sum > "$output/SHA256SUMS"
printf 'AH0_RETAINED_DIFFERENTIAL_GATE_POSITIVE_V1\n'
