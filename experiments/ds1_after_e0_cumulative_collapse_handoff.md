# Unchanged DS1-on-DS-E0 cumulative composition handoff

Protocol: `ds1-after-e0-cumulative-composition-attempt-v1`

Outcome: **CUMULATIVE DS1 DEVELOPMENT COLLAPSE AT 4. actual anonymous
boundary-ordering/action alternatives available from current substrate**.

This is a development-only first-collapse freeze. It is not definitive, not
claim eligible, not a cumulative negative or positive, and not an M1 result.
No definitive/result artifact was created.

## Authority and lineage

- **M0 authoritative**:
  `1d74c0ed0b515446161a63a6d43ecbe27514dc85`;
- **enabling parent `d154fde`**:
  `d154fde5632c0ba9d76fc2d1d1a700276045adc8`;
- enabling tag: `ds-e0-cumulative-anonymous-event-formation-readiness`;
- preregistration commit/tag:
  `002ba648c849c713903fae3394232a3bdcf7c076` /
  `ds1-after-e0-cumulative-composition-attempt-protocol`;
- **M1 does not exist**.

M0, DS-E0 E0-A/E0-B, the serializer, and the frozen DS1 learner are read-only
and byte-identical to the enabling parent.

## Ordered first-collapse report

| Stage | Development status |
|---|---|
| 0. exact lineage and hash/fingerprint proof | READY |
| 1. DS-E0 event formation | READY |
| 2. format-only E0-B serialization | READY |
| 3. frozen DS1 `Neighborhood` consumption/invocation | READY: unchanged `frozen_choice` invoked read-only on one E0-produced representation per seed |
| 4. actual anonymous boundary-ordering/action alternatives available from current substrate | **COLLAPSE: absent** |
| 5. selected DS1 action physically executes | BLOCKED |
| 6. route-contingent ordinary consequence naturally returns | BLOCKED |
| 7. unchanged DS1 acquisition/consolidation | BLOCKED |
| 8. fresh/relabel/layout transfer | BLOCKED |
| 9. contradiction invalidation and generic reopening | BLOCKED |
| 10. cumulative functional recovery | BLOCKED |

The frozen learner consumed the serialized representation and returned no
mature choice, as expected for an unacquired default learner. The harness did
not call `choose`: the current substrate supplies E0 candidate proposals and
observed propagation relations, but no pair of actual anonymous DS1
boundary-ordering actions. Treating the learner's indices as routes or
correctness would manufacture the missing prerequisite, so the attempt freezes
at stage 4 without rescue.

The parent-audit amendment replaces the original literal absence report with a
source-derived `ActionSurfaceInventory`. Exact frozen source/type counts are:

```text
DS1 choose definitions/calls                 1 / 0
DS1 apply_consequence definitions/calls      1 / 0
DS1 frozen_choice definitions/read-only calls 1 / 1
E0 candidate proposal sites                  1
E0 propagation-observation surfaces          2
E0 formation-only boolean callbacks           1
owned composition report surfaces             2
exported action-pair values                    0
M0 correspondence execution signatures        3
M0 DS1-compatible execution signatures        0
DS1 choice-to-physical-execution paths         0
natural post-action consequence paths          0
```

`actual_anonymous_actions_available` is now computed from these values. It is
false only because the required reachable `choose`, exported pair, compatible
execution signature, and choice-to-execution path are not jointly present.
Focused tests fail if a compatible action pair, mapping, or consequence path
appears while stage 4 still reports absent. The scientific first-collapse
therefore remains stage 4 with a mechanically supported absence proof.

## MICRO/GATE and work

Release MICRO seed `100` and release GATE seeds `100..104` passed the audit
harness. For every GATE seed, E0-A formed 16/16 presentations, E0-B made 96/96
exact copies, and the existing read-only DS1 probe consumed one additional
formed/serialized `Neighborhood`. The first collapse was identical for all
seeds.

Per GATE seed, available E0 work is 661,423 operations: 583,632 raw relation
comparisons; 10,808 enumerated triples; 64,848 canonical permutations; 1,749
persistent-shape comparisons; 64 proposals; 64 E0 physical propagations; 64
ordinary E0 consequence updates; 97 temporary formations; and 97
serializations. E0 persistent storage is 130 bytes and temporary peak storage
is 40 bytes. There is one frozen read-only DS1 invocation.

DS1 observation comparisons, candidate evaluations, proposals, route firings,
credit, selected-action execution, route-contingent consequence, persistent
storage, maintenance, and carrying are blank because stage 4 and every later
causal stage are unavailable. They are not reported as zero-cost competence.

## Local validation

The following targeted checks passed:

```text
cargo fmt --all -- --check
cargo clippy --release --lib --bin ds1_after_e0_cumulative_composition -- -D warnings
cargo test --release -q --lib ds1_after_e0_cumulative_composition
cargo test --release -q --bin ds1_after_e0_cumulative_composition
cargo run --release --quiet --bin ds1_after_e0_cumulative_composition -- --micro
cargo run --release --quiet --bin ds1_after_e0_cumulative_composition -- --gate
```

Five focused library tests passed after the parent-audit amendment.
`--definitive` rejected before calling the
harness with exit status 2. The digest of every existing file under `results/`
was identical before and after rejection:
`491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4`.

Broad legacy regression was not run because frozen/shared behavior source did
not change.

## E2B validation

Targeted E2B validation passed against exact clean implementation snapshot
`bd856322cbb447cbec79dac0cd1122177ff5a2db` using only dedicated state
`/Users/satya/.cache/truelearner/ds1-after-e0-cumulative-e2b.json`. Remote
marked-learner hash verification, format, strict release Clippy, three focused
library tests, the runner target, release MICRO, and release GATE passed. The
remote `--definitive` rejection also exited 2 before the harness and preserved
the exact results digest
`491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4`.

Persistent sandbox `ij04oormcxmaks8eoz06i` was reused, reset to an
86,400-second timeout, never killed, and left running.

## Artifacts

- preregistration:
  `experiments/ds1_after_e0_cumulative_composition_attempt_protocol.md`;
- fingerprints: `experiments/ds1_after_e0_cumulative_fingerprints.md`;
- serializer continuity:
  `experiments/ds1_after_e0_cumulative_serializer_continuity.md`;
- dependency manifest:
  `experiments/ds1_after_e0_cumulative_dependency_manifest.csv`;
- leak audit: `experiments/ds1_after_e0_cumulative_leak_audit.md`;
- physical ledger:
  `experiments/ds1_after_e0_cumulative_physical_ledger.csv`;
- parent-audit amendment:
  `experiments/ds1_after_e0_cumulative_parent_audit_amendment.md`;
- harness: `src/ds1_after_e0_cumulative_composition.rs`;
- runner: `src/bin/ds1_after_e0_cumulative_composition.rs`.
