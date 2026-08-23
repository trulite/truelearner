# PX6 physical consequence-credit GATE v3 implementation audit

Status: **IMPLEMENTATION FROZEN; DEVELOPMENT EVIDENCE SPENT; AUTHORITY ABSENT**.

## Exact source boundary

- authoritative parent commit/tag:
  `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`,
  `px2-physical-causal-direction-authoritative`;
- active substrate path:
  `crates/px0-physical-correspondence/src/lib.rs`;
- active substrate SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- GATE v3 implementation commit/tag:
  `da0f71ded8f6f9362328e6efd03d9437d2d768a9`,
  `px6-physical-consequence-credit-gate-v3-implementation`;
- measurement source path:
  `crates/px0-physical-correspondence/examples/px6_physical_consequence_credit.rs`;
- measurement source SHA-256:
  `2c53f8bbb4ef8c9ace4347c01433c38929f14dc73d900a896687665deaf8e030`.

`git diff` confirms zero change to the substrate source or its manifest from
the authoritative parent. `cargo tree --edges normal -p
px0-physical-correspondence` contains only the dependency-free PX package. No
old M source or parallel-lane package is linked.

## Frozen no-addition blocks

The no-new-mechanism result consists of these unchanged source blocks (line
numbers refer to the frozen substrate path above):

| physical block | lines | SHA-256 |
|---|---:|---|
| queued propagation, traversal eligibility, outward crossing, quiescence | 246--357 | `c857ea85910b1f2663d7dc7e41440ac14ae7383b6b3156a5d33d451fb11a69a5` |
| ordinary local return on eligible outgoing ARROWs | 409--424 | `dde858c9064dae68045793eb4f9437819e9a3cec539516b0223a0a614abe5f1f` |
| ordinary pressure and unsupported-use pressure | 426--454 | `9af24de6e770d1f15903740a49acd11bcc92e37ef34422b47eb1a2fa8f6092c9` |
| generic geometry-derived local proposal | 456--492 | `8aa30e34e2baca922a59e07d88ddf373430a05eafc156bf230eba48f4838961d` |
| zero-resistance deallocation/generation change | 611--619 | `895b01ac40e7a853f3008cc7e8f28d2e3102b0a150653396e1821cc2a5012d29` |

These hashes are hashes of the exact newline-terminated source slices. The
whole-file hash is the primary port invariant.

## Execution-path audit

The organism-visible object is only `PlasticSubstrate`. The external harness
constructs physical CELL/ARROW topology, injects SPIKE arrivals, calls
`propagate`/`advance_time`, and observes public physical measurements. Harness
world and stratum values are never arguments to substrate execution.

At the two block switches, the harness calls only `enter` with two external
SPIKEs. It does not call `add_arrow`. The substrate's unchanged threshold
firing and `propose_local_arrows` block discover the nearby target by position
and create the fresh reserve-1 ARROW.

Source scans found none of the frozen typed-history names, semantic outcome
terms, reinforcement terms, return-ownership fields, or evaluator credit calls
in the substrate or GATE source. The manifest audit rejects an old DS8 link.

## Artifact discipline

The implementation checks parent, protocol, predecessor negative, and source
hashes before execution. Result paths use create-new staging files plus atomic
rename. GATE v3 ran once. Its paths now exist and preclude a rerun. GATE v1 and
v2 negative artifacts remain unchanged.
