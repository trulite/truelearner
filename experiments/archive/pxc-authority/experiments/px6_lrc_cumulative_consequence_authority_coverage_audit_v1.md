# PX6 LR-C cumulative consequence authority coverage audit v1

Status: **FROZEN SOURCE/DEPENDENCY CLASSIFICATION; AUTHORITY EVIDENCE UNSPENT**.

## Exact serial parent

The authority protocol commit `5d0ce8aebfcc7656bdd4089d06205a047a036b3a`
is directly parented by exact PX5 authority
`7392505c26edfe9fa5d9d74dc42fed4a0cb7b902`. Isolated PX6 development is
referenced only by immutable tag and artifact hashes; none of its commits or
unrelated files is present in this serial branch.

The current active manifest remains PX5 authority manifest v3, SHA-256
`32fda2d86f6c836438fdd01fc5433c8731f4ec92e3e052e5d1a64fe751d15388`,
until functional evidence is frozen. The preregistered v4 transformation will
replace only:

```text
PX6,src/ds8_cumulative_semantic_credit_probe.rs,predecessor-target
PX6,src/ds8_cumulative_semantic_credit_gate.rs,predecessor-target
```

with:

```text
PX6,crates/lr1-modulatory-physical-return/src/lib.rs,shared-authoritative-physical-consequence
```

No PX0--PX5, PX7, or PX8 row may change.

## Complete active dependency closure

The serial PX6 authority evaluator declares exactly two direct dependencies:

```text
px4-lrc-lifetime
lr1-modulatory-physical-return
```

`px4-lrc-lifetime` declares only `lr1-modulatory-physical-return`, and LR-C
declares no dependency. The unique active Rust dependency closure is exactly:

| source | classification | SHA-256 |
|---|---|---|
| `crates/lr1-modulatory-physical-return/src/lib.rs` | authoritative PX0--PX3+LR-C law and shared PX5/PX6 physical reduction | `7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10` |
| `arms/px4-lrc-lifetime/src/lib.rs` | authoritative PX4 public physical API | `a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71` |

PX5 and PX6 add no active Rust source. There is no root-crate, DS7/DS8,
isolated evaluator, optional feature, build script, dynamic module, generated
source, or PX7 dependency in the candidate closure.

## Evaluator-only classification

| source | exclusion reason |
|---|---|
| `arms/px6-lrc-consequence-authority/src/main.rs` | fresh authority geometry, public physical observations, fixed predicates, embedded exact replay, bounded resource accounting, and write-once publication only; exports no organism API |
| `arms/px6-lrc-consequence-authority/Cargo.toml` | dependency/build metadata only |
| `arms/px5-lrc-allocation-authority/src/main.rs` | frozen PX5 evaluator, not a dependency |
| `arms/px4-lrc-lifetime/src/main.rs` and PX4 evaluator wrappers/tests | frozen evaluators, unreachable from the library dependency closure |
| `experiments/px6_lrc_cumulative_consequence_authority_*` | protocol/audit prose only |
| `results/px6_lrc_consequence_authority_v1.*` | future serialized authority evidence only |
| `scripts/audit_px6_lrc_authority_v1.sh` | static observational tooling only |

The authority evaluator source SHA-256 is
`3b9477d63d13e80ee0e50328d42a10f458e43b80fbd607d0cacc893e6312e1a2`;
its Cargo manifest SHA-256 is
`ce46ecec4237431600859ba090346fcbf821e8c8df8c7e906b02c33cb6a5908b`.
The substrate cannot call or depend on either.

## Execution, memory, and leakage boundary

The evaluator uses ordinary structs, arrays, vectors, direct loops and public
read-only measurements. It contains no unsafe code, interior mutability,
global/thread-local state, proc macro, generated include, semantic mechanism
object, artificial leak, or measured-result feedback into physical input.

Complete per-row work is accumulated from every propagation and pressure
advance across the positive geometry, all controls, and fresh PX0--PX5
conformance. Maximum persistent bytes are observed across every world.
Repeated stable return, PX4 reuse, and PX5 reuse each require unchanged ARROW
count and persistent bytes. The fixed ceilings are `150000` ledger operations
and `32000` bytes; natural quiescence is conjunctive.

The no-world preflight checks only frozen hashes, fixed matrix constants,
namespace disjointness and artifact absence. Static audit confirms that it has
no call to a physical constructor or row runner. The sole row-runner call is
inside the definitive authority function after the unique evidence marker.

Active sources: **2 unique files**. New active PX6 sources: **0**. Evaluator
sources: **1 new file**. Unclassified candidate sources: **0**.
