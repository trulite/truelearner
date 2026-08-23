# CJ0 ARM CJ-B GATE v2 development handoff

Status: **DEVELOPMENT-READY; TERMINAL GATE COMPLETE; LANE CLOSED**.

## Outcome

The separately preregistered GATE v2 mechanically corrects one mirrored
schedule phase and passes `8/8` rows and `88/88` claims. The candidate law is
byte-identical to GATE v1. GATE v1 remains an immutable frozen terminal
development negative and is not reclassified.

The correction is exactly `changed_offset = 2` for both alternating changed
routes, all four fresh normal/mirror/allocation/permutation variants, and both
timing strata. There is no route-specific branch.

## Physical consume/produce contract

The law decays and inspects current destination CELL state when a source CELL
fires through a live ARROW. It adds ordinary ARROW coupling. Below threshold it
suppresses without return eligibility. At threshold it consumes the current
destination state, emits one generation-bound ordinary SPIKE carrying the
available impulse, and makes only the traversed ARROW return-eligible.

Timely ordinary return strengthens the same ARROW; ordinary pressure weakens
and may deallocate it; stale generation-bound SPIKEs cannot execute; generic
local reproposal permits bootstrap and relearning. Learned output is an
ordinary CELL firing and recursively uses the identical law.

No persistent variable, relation identifier, participant list, logical
operator, level marker, invalidation signal, or history resurrection path was
added.

## Evidence

| stage | outcome |
|---|---|
| PROBE | `4/4`, `40/40`, positive and frozen |
| MICRO | `4/4`, `40/40`, positive and frozen |
| GATE v1 | `4/8`, `76/80`, frozen terminal development negative |
| GATE v2 | `8/8`, `88/88`, positive terminal development gate |

GATE v2 repeats matched flat discrimination, self-evidence, reversal,
historical non-resurrection, recursion through X/Y/Z, ordinary convergent
inclusive alternatives, all five temporal cases, exact replay, finite
recurrence, and natural quiescence.

Its boundary control shows in every row that a fresh weak proposal due on tick
10 is physically deallocated by ordinary pressure before its stale generation
can fire. Ordinary activity at tick 11 creates a replacement, return at tick 13
matures it to coupling/resistance `2/4`, and held-out activity at tick 14
delivers at tick 16 and crosses once.

## Exact artifacts

| artifact | SHA-256 |
|---|---|
| physical module | `ef0de37a9ac54b632b991f0d4647a5ee78c23810084d61497c88d6f757ec2188` |
| GATE v2 protocol | `2f34832522086b393577bfb430c8517a4c9e487a7273a2e1f056f2909aba6539` |
| GATE v2 evaluator | `4733bc8d1cc06881102e0d36067649d22381be554b71056debf36a306b83bb86` |
| implementation audit | `03c524207441f1662f8ef62df7e3c4a49e639f1759805b38a56f0788b634c9b0` |
| GATE v2 CSV | `4c3d2e63765f5eb70f0ab0e1d673462a9bc32a7bf45fc51603a115e900bfed69` |
| GATE v2 report | `12c8a59897709a0c0927e1cfe5f96dadf881179ad40f58ed02dd82a789b980bd` |
| result audit | `b724a2f5c5d22b4e6994c09ab89bb3d8cf5b7b46baa58fc8bfce6dabc739ef45` |
| development readiness | `964f3ec7611e0757cb8bf1f221408b419d92a5c384e31b0e1ea4195c5be0c5a5` |
| compare-ready port contract | `b055bf1cd6ab14b3dfa8dd7bc8f47c2a5e5fcf6c67156ddd7e6d38b923d9315d` |

## Validation and ledger

- focused candidate unit tests: `2/2` pass;
- strict all-target Clippy: pass;
- format, zero-dependency, no-cell preflight, refusal, artifact, staging,
  physical-source vocabulary, source-isolation, and exact-hash checks: pass;
- broad historical suites: operationally canceled and not claimed;
- GATE v2 physical work: `93,880` operations;
- cumulative CJ-B PROBE/MICRO/GATE v1/GATE v2 work: `254,748` operations;
- GATE v2 atomic result storage: `8,291` bytes;
- cumulative CJ-B atomic result storage: `20,643` bytes.

Authoritative PX0--PX2 bytes, PX3/PX3-R negatives, and the complete GATE v1
chain remain unchanged. PX3, PX-C, and PX4--PX8 do not advance. This handoff is
for development comparison only; no later scientific surface exists in this
lane.
