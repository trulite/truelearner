# CJ0-NOT-1 active-inhibition definitive v1 result audit

Status: **DEFINITIVE POSITIVE CLASSIFICATION FROZEN; PX2 REMAINS AUTHORITATIVE**.

The sole definitive execution emitted
`CJ0_NOT1_ACTIVE_INHIBITION_DEFINITIVE_V1_EVIDENCE_SPENT`, exited zero, and
atomically published:

| artifact | SHA-256 |
|---|---|
| definitive CSV | `f9c85e70afe840b68e2610bc9e2b03101a6f258a14abc6335562cad6bafc21d1` |
| definitive report | `1ebe360231fc95a5370048a1ac7949bfe46b524802da9de09835b236d0b4e04b` |

Exactly `112/112` rows passed: sixteen fresh seed/layout strata in each of seven
worlds. Every world contributed `16/16` passes.

## Independent physical findings

- A absent: B integration/output operated.
- A one tick early: negative CELL state survived with ordinary decay and
  prevented B threshold crossing.
- A coincident before B: the negative impulse arrived first and suppressed the
  integration/output path.
- A coincident after B: B crossed and emitted output first at the same physical
  tick; later-ordered negative activity did not erase it.
- A one tick late: output tick `1` preceded negative arrival tick `2`.
- A blocked: A fired, the non-live path delivered nothing, and B operated.
- A stale: A fired, ordinary pressure deallocated the weak delayed path, its
  queued generation delivered nothing, and B operated.

All `112` rows naturally quiesced and replayed exactly. Work was `28..50`,
storage was exactly `384` bytes per fixture, and the sixteen stale rows recorded
sixteen physical deallocations. Signed arrival ticks, role firings, crossings,
complete/permanent fingerprints, layout controls, and every row result are
serialized in the CSV. Staging paths were absent after publication.

## Classification

NOT-1 active inhibition is **definitively positive within the tested physical
boundary**: existing ordinary negative coupling can make timely A suppress a B
path; absence of A leaves B operational; activity after B's threshold crossing
has no retroactive effect. The boundary is order-sensitive and does not claim
a Boolean complement or add a logical NOT primitive.

This result does not classify NOT-2, reinterpret PX3, modify PX0-PX2, or advance
authority. The PX0 source and PX2 definitive evidence hashes remain exact.
