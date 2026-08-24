#!/bin/sh
set -eu

parent=f9057fe78a86db9111b0b69310d03accef3bc970
ready=20bc9ce384b74b6e5cca04f4bed2599932a34e92
law=crates/lr1-modulatory-physical-return/src/lib.rs
active=arms/px4-lrc-lifetime/src/lib.rs
runner=arms/px4-lrc-lifetime/src/main.rs
wrapper=arms/px4-lrc-lifetime/src/bin/px4_lrc_lifetime_authority_v1.rs
manifest=experiments/pxc_active_surface_manifest_v2.csv

require_hash() {
    required=$1
    path=$2
    actual=$(sha256sum "$path" | awk '{print $1}')
    if [ "$actual" != "$required" ]; then
        echo "PX4 authority frozen hash mismatch: $path $actual" >&2
        exit 1
    fi
}

audited_commit=${PX4_AUDITED_COMMIT:-}
if [ -z "$audited_commit" ] || [ "$(basename "$PWD")" != "$audited_commit" ]; then
    echo "PX4_AUDITED_COMMIT must name the exact E2B archive snapshot" >&2
    exit 1
fi
test "$parent" = f9057fe78a86db9111b0b69310d03accef3bc970
test "$ready" = 20bc9ce384b74b6e5cca04f4bed2599932a34e92

require_hash 7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10 "$law"
require_hash a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71 "$active"
require_hash 98067812bc357949af5653a115b353519bede12499804818cfaf4783c0666cbd \
    experiments/px3_lrc_physical_event_organization_authority_handoff_v2.md
require_hash a84ecf39ae1381f75edf95887aad3bcd1d7a0b623a87a1b5f874a7cb07efd4c1 \
    experiments/px4_lrc_development_readiness_handoff_v1.md
require_hash 7789fe652e39e77e8d909b2cd34ec71b8fcdc3ee6564d8f18ba1840f8fdb9d54 \
    results/px4_lrc_lifetime_gate_v1.csv
require_hash 28924746e951645047225d8d20f5c5f98d93f349f46f7c6d7019e68632ce51b9 "$manifest"
require_hash fa04de4ec43c10f3878b86d920c2a67243b84201e8759950075c069548153ba8 \
    experiments/px4_lrc_physical_lifetime_authority_protocol_v1.md
require_hash e696c8e1e50ac9504c180094daf90182d0854755a2b6289826f8de19397bfc5d "$runner"
require_hash a181fa810cef8edfe557daaf8dae9948ebd37dd429bb084d8ffedb6d84615b4c "$wrapper"

if rg -n -i '\b(lifetime|history|episode|reset|cleanup|delete)\b' "$active"; then
    echo "PX4 active mechanism contains forbidden semantic vocabulary" >&2
    exit 1
fi

if rg -n -i \
    '(struct|enum|type|fn|let)[[:space:]]+[A-Za-z0-9_]*(lifetime|history|episode|reset|cleanup|delete)|\.(lifetime|history|episode|reset|cleanup|delete)[A-Za-z0-9_]*' \
    "$runner" "$wrapper"; then
    echo "PX4 evaluator declares or accesses a forbidden semantic object" >&2
    exit 1
fi

if rg -n '\.(set_(resistance|coupling|eligibility|generation)|delete|cleanup|reset)[A-Za-z0-9_]*[[:space:]]*\(' \
    "$active" "$runner" "$wrapper"; then
    echo "PX4 authority surface contains forbidden mutation invocation" >&2
    exit 1
fi

test "$(awk -F, '$1 == "PX4" {n += 1} END {print n + 0}' "$manifest")" -eq 1
awk -F, '$1 == "PX4" {
    if ($2 != "arms/px4-lrc-lifetime/src/lib.rs") exit 1
    if ($3 != "development-candidate") exit 1
}' "$manifest"

test "$(awk -F= '/^[a-zA-Z0-9_-]+[[:space:]]*=.*path/ {n += 1} END {print n + 0}' \
    arms/px4-lrc-lifetime/Cargo.toml)" -eq 1
rg -q '^lr1-modulatory-physical-return[[:space:]]*=' arms/px4-lrc-lifetime/Cargo.toml

test -f "$runner"
test -f "$wrapper"
test -z "$(find arms/px4-lrc-lifetime/src -type f -name '*.rs' \
    ! -path "$active" ! -path "$runner" ! -path "$wrapper" -print)"

printf '%s\n' \
    'PX4 authority static audit: lineage=PASS law=PASS active_unchanged=PASS leakage=PASS dependency=PASS coverage=PASS foundation=ZERO'
