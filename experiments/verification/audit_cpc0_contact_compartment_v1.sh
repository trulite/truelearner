#!/usr/bin/env bash
set -euo pipefail

test "$(sha256sum truelearner/crates/core/src/lib.rs | cut -d' ' -f1)" = \
  d49a6b98081cee65c3e7a5f64e9cf6356fa06ef456a9cf78a41d0cd1187bcd58
test "$(sha256sum truelearner/crates/core/src/mechanics.rs | cut -d' ' -f1)" = \
  ba81648a0318aedfbf90fe968ca51bdcb7efaddf844c0967887fb35a3f6d69be
test "$(sha256sum truelearner/crates/core/Cargo.toml | cut -d' ' -f1)" = \
  aff8989aa31a503eecd38c9d6632817819f35456f97f1ebef064a27bdc3afe42

test "$(grep -c '^' results/cpc0_contact_compartment_v1/matrix.csv)" = 441
grep -q -- '- physical cases: `220/220`' results/cpc0_contact_compartment_v1/report.md
grep -q -- '- exact Reference/Production transition histories: `220/220`' \
  results/cpc0_contact_compartment_v1/report.md
grep -q -- '- runtime or substrate-law changes: `0`' \
  results/cpc0_contact_compartment_v1/report.md

if rg -n 'ArrowId.*modulat|modulat.*ArrowId|credit.*id|targeted.*reward' \
  experiments/arms/cpc0-contact-compartment-attribution/src/main.rs; then
  printf 'semantic attribution surface detected\n' >&2
  exit 1
fi

(cd results/cpc0_contact_compartment_v1 && sha256sum -c SHA256SUMS)
printf 'CPC0_STATIC_AUDIT_OK physical_cases=220 mechanics_rows=440\n'
