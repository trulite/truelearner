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

check_hash 45dd6af368776d68574ff2b00dd4db109d469bfeedc99b57eb76ad6b26ca111c \
  truelearner/crates/core/src/lib.rs
check_hash 5d794eae058f5cdd896064b0a37a6dfb124d9d7b6d03f8cfa9c53651e58460ef \
  truelearner/crates/core/Cargo.toml

core=truelearner/crates/core/src/lib.rs
proposal_body="$(sed -n '/fn propose_local_arrows(/,/^    fn decay_cell(/p' "$core")"

test "$(grep -c 'propose_local_arrows' "$core")" -eq 2
test "$(printf '%s\n' "$proposal_body" | grep -c 'self.add_arrow(ArrowSpec')" -eq 1
test "$(printf '%s\n' "$proposal_body" | grep -c 'PhysicalEvent::Proposal')" -eq 1
printf '%s\n' "$proposal_body" | grep -Fq 'coupling: 1,'
printf '%s\n' "$proposal_body" | grep -Fq 'resistance: 1,'
printf '%s\n' "$proposal_body" | grep -Fq 'mode: TransmissionMode::Drive,'

if printf '%s\n' "$proposal_body" | grep -En \
  'coupling:\s*-|coupling:\s*(sign|candidate|variation)|TransmissionMode::Modulatory|negative|inhibit|random|seed|permut|choice|choose'; then
  printf 'unexpected sign choice or non-Drive proposal path found\n' >&2
  exit 1
fi

printf 'gate_a=negative\n'
printf 'proposal_coupling=1\n'
printf 'proposal_mode=Drive\n'
printf 'negative_drive_candidate=false\n'
printf 'RS2_LEARNED_INHIBITORY_TOPOLOGY_GATE_A_STATIC_NEGATIVE_V1\n'
