#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../.."

check_hash() {
  local expected="$1"
  local path="$2"
  local actual
  actual="$(sha256sum "$path" | awk '{print $1}')"
  test "$actual" = "$expected" || {
    printf 'hash mismatch: %s\nexpected %s\nactual   %s\n' \
      "$path" "$expected" "$actual" >&2
    exit 1
  }
}

check_hash b6b7f2a47818d84ac2fd69aab466f5f917e6d3ba7cfc8f8c5db4ce91b97fbae5 \
  truelearner/crates/core/src/lib.rs
check_hash 4cb6d665d738cdea61f928975fa34ddf89d62aa9150420748d94d574ed731aeb \
  truelearner/crates/core/Cargo.toml

core=truelearner/crates/core/src/lib.rs
proposal_body="$(sed -n '/fn propose_local_arrows(/,/^    fn decay_cell(/p' "$core")"

test "$(grep -c 'propose_local_arrows' "$core")" -eq 2
test "$(printf '%s\n' "$proposal_body" | grep -c 'self.add_arrow(ArrowSpec')" -eq 1
test "$(printf '%s\n' "$proposal_body" | grep -c 'self.add_cell')" -eq 0
test "$(printf '%s\n' "$proposal_body" | grep -c 'CellSpec')" -eq 0
printf '%s\n' "$proposal_body" | grep -Fq 'self.cells'
printf '%s\n' "$proposal_body" | grep -Fq '.values()'
printf '%s\n' "$proposal_body" | grep -Fq 'let proposal_couplings: &[i32] = &[1, -1];'
printf '%s\n' "$proposal_body" | grep -Fq 'event: PhysicalEvent::Proposal {'

proposal_variant="$(sed -n '/    Proposal {/,/    },/p' "$core" | head -n 6)"
printf '%s\n' "$proposal_variant" | grep -Fq 'arrow: ArrowId'
printf '%s\n' "$proposal_variant" | grep -Fq 'from: CellId'
printf '%s\n' "$proposal_variant" | grep -Fq 'to: CellId'
if printf '%s\n' "$proposal_variant" | grep -Eq 'cell:|CellSpec|position|threshold'; then
  printf 'unexpected CELL-construction data in proposal event\n' >&2
  exit 1
fi

printf 'gate_a=negative\n'
printf 'variation_adds_arrows=true\n'
printf 'variation_adds_cells=false\n'
printf 'contact_compartment_creation=false\n'
printf 'runtime_gates_constructed=false\n'
printf 'SV1_COMPARTMENTALIZED_SIGNED_VARIATION_GATE_A_STATIC_NEGATIVE_V1\n'
