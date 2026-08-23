# CJ0 ARM CJ-B locally gated ARROW PROBE implementation audit

Status: **IMPLEMENTATION FROZEN; PROBE EVIDENCE UNSPENT**.

## Exact implementation

| path | SHA-256 |
|---|---|
| `arms/cj-b-locally-gated-arrow/Cargo.toml` | `142ef92f1c331e0462b2f64531539fe1bd442b80beec3bf994fb4d92610bce4c` |
| `arms/cj-b-locally-gated-arrow/Cargo.lock` | `d778299980b40a5a5a1d5451cb5aa493828bfea8ecc3a4209350a9c50396f397` |
| physical module `src/lib.rs` | `ef0de37a9ac54b632b991f0d4647a5ee78c23810084d61497c88d6f757ec2188` |
| evaluator/serializer `src/bin/probe.rs` | `9e76b996182f75700654377fe1e9a1b5b3861bdac000fc8feacd7c7491fb6bc1` |

The crate is standalone and has zero dependencies. It adds no root-workspace
member and changes no pre-existing source path.

## Frozen parent and isolation

- HEAD ancestry contains exact authoritative commit
  `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`;
- authoritative law SHA-256 remains
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- authoritative PX2 execution source SHA-256 remains
  `c47d605371d5787cffc7d456f1d9e38168b4b203063fb9dcdeefcf630fa4aed5`;
- the diff from the frozen start contains added CJ-B paths only;
- no PX3 or PX3-R artifact is copied, modified, rerun, or reclassified.

## Physical rule audit

The physical module adds only ordinary numeric CELL/ARROW/SPIKE matter and the
preregistered local consume/produce rule. Its persistent state consists of
the pre-existing physical quantities: identity, position, region, threshold,
transient CELL state/time/refractory state, generation, resistance, liveness,
ARROW endpoints/delay/phase/coupling/local eligibility, queued SPIKEs, local
time, and serial order.

No gate flag, contributor identity, ownership, relation key, list, semantic
record, lifecycle label, or evaluator value is stored. A case-insensitive
forbidden-vocabulary scan of `src/lib.rs` is empty. The evaluator binary can
schedule external physical arrivals and observe discarded clones, but cannot
name or select an ARROW update inside the physical module.

## Pre-evidence validation

- focused formatting: pass;
- focused unit tests: `2` pass, `0` fail;
- strict all-target Clippy: pass;
- no-argument refusal: exit `2`;
- wrong-argument refusal: exit `2`;
- no-CELL `--preflight`: pass, `cells_entered=0`, `artifacts_written=0`;
- final and staging artifact absence: pass;
- exact frozen hashes: pass;
- dependency/source/isolation audit: pass.

No PROBE, MICRO, GATE, definitive, authority, PX3, or PX-C cell has executed.
The committed/tagged source is the only implementation eligible for the sole
PROBE v1 command.
