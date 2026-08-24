# LR1 three-factor local plasticity parallel execution protocol v1

Status: **PREREGISTERED; THREE-ARM EVIDENCE UNSPENT; PX3 NEGATIVE**.

The common scientific protocol is
`lr1_three_factor_local_plasticity_parallel_arms_protocol_v1.md`. Arm A's
additional exact geometry is frozen in
`lr1_three_factor_local_plasticity_arm_a_protocol_v1.md`.

## Isolation and commands

After a single implementation commit and implementation audit are frozen and
tagged, three new E2B sandboxes are created with distinct persistent state
files:

```text
/Users/satya/.cache/truelearner/lr1-arm-a-evidence-e2b.json
/Users/satya/.cache/truelearner/lr1-arm-b-evidence-e2b.json
/Users/satya/.cache/truelearner/lr1-arm-c-evidence-e2b.json
```

Each receives the same exact Git snapshot. It first runs its own formatting
check, release tests, strict release Clippy, artifact-absence check and
`--preflight`. All three preflights must finish before evidence begins.

The sole evidence commands are then launched concurrently, one per sandbox:

```text
cargo run --manifest-path arms/lr1-three-factor-local-plasticity-a/Cargo.toml --release -- --lr1-a
cargo run --manifest-path arms/lr1-three-factor-local-plasticity-b/Cargo.toml --release -- --lr1-b
cargo run --manifest-path arms/lr1-three-factor-local-plasticity-c/Cargo.toml --release -- --lr1-c
```

Their unique spend markers are respectively:

```text
LR1_THREE_FACTOR_LOCAL_PLASTICITY_ARM_A_EVIDENCE_SPENT
LR1_THREE_FACTOR_LOCAL_PLASTICITY_ARM_B_EVIDENCE_SPENT
LR1_THREE_FACTOR_LOCAL_PLASTICITY_ARM_C_EVIDENCE_SPENT
```

Each command must emit its marker once, publish only its own CSV/report pair,
and terminate. Artifacts are downloaded unchanged into the corresponding
registered result paths. No arm may read another arm's artifact.

## Arm A safety boundary

Development found that Arm A's content-neutral cycle qualification can turn
the simultaneous completed cycle into recurrent excitation. Its successor
propagation remains byte-identical for evidence but stops observation after
1,000 delivered spikes and returns `naturally_quiescent=false`; it does not
clear or reinterpret the pending physical process. This is evaluator safety
against another memory exhaustion, not a positive mechanism change.

Arm A therefore may lawfully produce a frozen functional negative with four
non-quiescent simultaneous rows. The other registered controls must still
serialize. B and C receive no delivery cap beyond the ordinary terminating
substrate.

## Write-once discipline

Once any evidence marker is emitted, no implementation, protocol, seed,
schedule, predicate or artifact path may change and no arm may rerun. A
technical upload/preflight failure before any marker does not spend evidence.
After execution, artifacts are audited and frozen exactly as generated,
including negative or accounting results.

This workflow is development comparison only. It does not change authoritative
PX0, select an LR1 successor, run PX0--PX2 conformance, reopen PX3 or authorize
PX4.
