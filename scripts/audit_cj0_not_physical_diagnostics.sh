#!/usr/bin/env bash
set -euo pipefail

base=2fbee861a0aeed335d3ffa8f9095ca28f2ac6129

fail() {
    echo "CJ0-NOT audit failure: $*" >&2
    exit 1
}

require_hash() {
    local expected=$1
    local path=$2
    local actual
    actual=$(sha256sum "$path" | awk '{print $1}')
    [[ "$actual" == "$expected" ]] || fail "$path hash $actual != $expected"
}

git merge-base --is-ancestor "$base" HEAD || fail "PX2 is not an ancestor"
[[ "$(git rev-parse "$base^{tree}")" == "3e69c15c5a9f7259d8617aa23ffb9083064f53a1" ]] \
    || fail "PX2 tree mismatch"

unexpected=$(git diff --name-status "$base"..HEAD | awk '$1 != "A" {print}')
[[ -z "$unexpected" ]] || fail "non-additive path changes detected: $unexpected"

require_hash 3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d \
    crates/px0-physical-correspondence/src/lib.rs
require_hash c47d605371d5787cffc7d456f1d9e38168b4b203063fb9dcdeefcf630fa4aed5 \
    crates/px0-physical-correspondence/examples/px2_physical_causal_direction.rs
require_hash da356bc46a9d83d0cd749bcaa697cba66393b7d694de500e2208565806d680d1 \
    results/px0_physical_correspondence_definitive.csv
require_hash 6613ff0a96bb3a60fbe7afeb92cd64edced3c6df5dcc04fe47518db158dd88f6 \
    results/px1_physical_boundary_roles_definitive.csv
require_hash 921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18 \
    results/px2_physical_causal_direction_definitive.csv

git diff --quiet "$base" -- Cargo.toml Cargo.lock \
    crates/px0-physical-correspondence/Cargo.toml \
    || fail "dependency surface changed"

require_hash 4f3ad19bea689a60641852ef038e7ba5d8938e8dcdba802f0019dea8df68dedb \
    results/cj0_not1_active_inhibition_probe_v1.csv
require_hash 365f665e609b50ec6b35b4d3768f7a78f8199f9f645fcbb16319c9abee1bd5df \
    results/cj0_not1_active_inhibition_probe_v1.md
require_hash 07cb0d4ccbd817c6de56166f89d4e5719a4d645bfab9d78718169538d36cad7d \
    results/cj0_not2_temporal_absence_probe_v1.csv
require_hash 0f4b0c554275a820dfc6a3de7799736edebf2f5aa272dc211b7228a66c3fd05b \
    results/cj0_not2_temporal_absence_probe_v1.md
require_hash f9c85e70afe840b68e2610bc9e2b03101a6f258a14abc6335562cad6bafc21d1 \
    results/cj0_not1_active_inhibition_definitive_v1.csv
require_hash 1ebe360231fc95a5370048a1ac7949bfe46b524802da9de09835b236d0b4e04b \
    results/cj0_not1_active_inhibition_definitive_v1.md
require_hash f66b5f591533a53b1ad3f17a7c9a362e5881a202ff87999d7359850655b0e414 \
    results/cj0_not2_temporal_absence_definitive_v1.csv
require_hash 90b4d6f8f0f7b23d7d9c33ebb786c781f6abf736bf1098afeaa2819f9c5d29ea \
    results/cj0_not2_temporal_absence_definitive_v1.md

awk -F, 'NR > 1 { n++; pass += ($18 == "true"); replay += ($17 == "true"); q += ($12 == "true"); storage += ($16 == 384) } END { exit !(n == 10 && pass == 10 && replay == 10 && q == 10 && storage == 10) }' \
    results/cj0_not1_active_inhibition_probe_v1.csv \
    || fail "NOT-1 PROBE matrix invariant"
awk -F, 'NR > 1 { n++; pass += ($24 == "true"); replay += ($23 == "true"); q1 += ($16 == "true"); q2 += ($17 == "true"); storage += ($22 == 496) } END { exit !(n == 12 && pass == 12 && replay == 12 && q1 == 12 && q2 == 12 && storage == 12) }' \
    results/cj0_not2_temporal_absence_probe_v1.csv \
    || fail "NOT-2 PROBE matrix invariant"
awk -F, 'NR > 1 { n++; pass += ($24 == "true"); replay += ($23 == "true"); q += ($18 == "true"); storage += ($22 == 384) } END { exit !(n == 112 && pass == 112 && replay == 112 && q == 112 && storage == 112) }' \
    results/cj0_not1_active_inhibition_definitive_v1.csv \
    || fail "NOT-1 definitive matrix invariant"
awk -F, 'NR > 1 { n++; pass += ($28 == "true"); replay += ($27 == "true"); q1 += ($20 == "true"); q2 += ($21 == "true"); changed += ($8 != $9); storage += ($26 == 496) } END { exit !(n == 112 && pass == 112 && replay == 112 && q1 == 112 && q2 == 112 && changed == 112 && storage == 112) }' \
    results/cj0_not2_temporal_absence_definitive_v1.csv \
    || fail "NOT-2 definitive matrix invariant"

if find results -maxdepth 1 -name '.cj0_not*.staging' -print -quit | grep -q .; then
    fail "staging artifact remains"
fi

if rg -n '^(pub[[:space:]]+)?struct[[:space:]]+(Cell|Arrow|Spike)\b' \
    crates/px0-physical-correspondence/examples/cj0_not*.rs; then
    fail "replacement substrate type found"
fi

for tag in \
    cj0-not1-active-inhibition-probe-v1-protocol \
    cj0-not1-active-inhibition-probe-v1-implementation \
    cj0-not1-active-inhibition-probe-v1-positive \
    cj0-not2-temporal-absence-probe-v1-protocol \
    cj0-not2-temporal-absence-probe-v1-implementation \
    cj0-not2-temporal-absence-probe-v1-positive \
    cj0-not1-active-inhibition-definitive-v1-protocol \
    cj0-not1-active-inhibition-definitive-v1-implementation \
    cj0-not1-active-inhibition-definitive-v1-positive \
    cj0-not2-temporal-absence-definitive-v1-protocol \
    cj0-not2-temporal-absence-definitive-v1-implementation \
    cj0-not2-temporal-absence-definitive-v1-positive
do
    git rev-parse -q --verify "$tag^{tag}" >/dev/null || fail "missing annotated tag $tag"
done

echo "CJ0_NOT_PHYSICAL_DIAGNOSTICS_AUDIT_OK"

