# PX8 LR-C cumulative closure authority v3 coverage audit v1

Status: **FROZEN CLASSIFICATION; AUTHORITY-V3 EVIDENCE UNSPENT**.

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
No active file, Cargo dependency, retained law, topology, schedule, behavioral
predicate, threshold, work ceiling, or memory ceiling changed.

## V3 evaluator-only repair

`arms/px8-lrc-closure-authority-v3/src/main.rs` is evaluator-only and depends
directly only on active PX8 and retained PX7. It preserves the v2 physical
worlds and behavioral predicates under fresh roots `865001..865016` and fresh
physical-origin identities.

The sole authority measurement repair is clause 12. The six mature retained
fixtures (primary, uninterrupted, incomplete, duplicate, blocked, and
cumulative PX7) still require exact same-body before/after byte equality. The
stale/reproposal fixture is classified separately: no outward crossing or
stale-route execution, memory at most 8192 bytes, empty queue and natural
quiescence, exact replay, with fresh lawful proposals permitted. Its before,
after, signed delta, capacity, outward count, route-execution count, derived
fresh-proposal count, queue/quiescence, and replay observations are published
unconditionally before aggregate assertions.

The evaluator owns only identities, observations, predicates, exact replay,
resource accounting, input hashes, the v3 execution firewall, and write-once
result publication. It exports no organism API and changes no physical input.

## Complete mechanism and separation coverage

Coverage is complete for recursive formation, complete/incomplete reuse,
blocked/stale/open controls, duplicate recursive and physical routes,
zero-length/branch/cycle topologies, exactly-once outward crossing,
downstream physical return/update, pause/resume, all five compact forms,
bounded work/memory, queue exhaustion, natural quiescence, exact replay, and
fresh PX7 cumulative maturation/held-out initiation.

The v3 evaluator contains no v1/v2 authority or diagnostic execution
mode/marker/path, semantic mechanism object, terminal object, Episode/Query,
begin/reset, manual temporary cleanup, unsafe/interior mutable global state,
generated organism code, or measured-result feedback.

Active sources: **4 unique files**. Active changes: **0**. V3 evaluator
sources: **1**. Unclassified candidate sources: **0**.
