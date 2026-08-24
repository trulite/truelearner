# PX8 LR-C cumulative closure authority v2 coverage audit v1

Status: **FROZEN CLASSIFICATION; AUTHORITY-V2 EVIDENCE UNSPENT**.

## Active closure unchanged

The complete unique active Rust closure remains exactly:

| source | classification |
|---|---|
| `crates/lr1-modulatory-physical-return/src/lib.rs` | authoritative PX0--PX3+LR-C law and shared PX5/PX6 reduction |
| `arms/px4-lrc-lifetime/src/lib.rs` | authoritative PX4 physical API |
| `crates/px7-lrc-arrival/src/lib.rs` | authoritative PX7 physical-arrival surface |
| `arms/px8-lrc-physical-closure/src/lib.rs` | byte-identical PX8 physical-closure/emission surface |

Active PX8 SHA-256 remains
`8623cae7dc1b14e666140a192c49e59ae72df681bc14f0359f0c4465e0d11e8f`.
No active file, Cargo dependency, retained law, topology, schedule, threshold,
physical predicate, or work/byte ceiling changed.

## V2 evaluator-only repair

`arms/px8-lrc-closure-authority-v2/src/main.rs` is evaluator-only and depends
directly only on active PX8 and retained PX7. It preserves every v1 physical
world and predicate under fresh roots `863001..863016`.

The sole behavioral difference in evaluator observation is clause 12:
seven explicitly serialized same-body byte pairs replace the invalid
cross-fixture equality. Measurements bracket the primary, uninterrupted,
incomplete, duplicate, blocked, stale, and cumulative held-out operations.
They do not affect physical input, schedule, propagation, or later
observation. Every pair and every observed size participates conjunctively.

The evaluator owns identities, observations, predicates, exact replay,
resource accounting, hashes, the v2 execution firewall, and write-once result
publication. It exports no organism API.

## Complete mechanism and separation coverage

Coverage remains complete for recursive formation, complete/incomplete reuse,
blocked/stale controls, duplicate recursive input, pause/resume/fingerprints,
all five compact forms, exact outward/return/update counts, bounded work,
bounded/stable memory, queue exhaustion, natural quiescence, and fresh PX7
cumulative maturation/held-out initiation.

The v2 evaluator contains no v1 or diagnostic execution mode/marker/path,
semantic mechanism object, manual cleanup, unsafe/interior mutable global
state, generated code, or measured-result feedback.

Active sources: **4 unique files**. Active changes: **0**. V2 evaluator
sources: **1**. Unclassified candidate sources: **0**.
