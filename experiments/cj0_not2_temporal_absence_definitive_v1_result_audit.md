# CJ0-NOT-2 temporal-absence definitive v1 result audit

Status: **DEFINITIVE POSITIVE CLASSIFICATION FROZEN; PX2 REMAINS AUTHORITATIVE**.

The sole definitive execution emitted
`CJ0_NOT2_TEMPORAL_ABSENCE_DEFINITIVE_V1_EVIDENCE_SPENT`, exited zero, and
atomically published:

| artifact | SHA-256 |
|---|---|
| definitive CSV | `f66b5f591533a53b1ad3f17a7c9a362e5881a202ff87999d7359850655b0e414` |
| definitive report | `90b4d6f8f0f7b23d7d9c33ebb786c781f6abf736bf1098afeaa2819f9c5d29ea` |

Exactly `112/112` rows passed: sixteen fresh seed/layout strata in each of seven
worlds. Every world contributed `16/16` passes.

## Independent physical findings

- In every row, trigger-only propagation naturally quiesced and its complete
  state fingerprint differed from the initial fingerprint before B or closure
  was entered.
- B absent through closure: closure reached transient state at tick `2`, and
  transient/output fired once at tick `2`.
- B at tick `1` after trigger: negative activity changed the transient state
  and prevented closure/output firing.
- B at tick `2` before closure: ordering at the physical closure boundary
  prevented closure/output firing.
- B at tick `2` after closure: closure threshold crossing and output emission
  occurred first; later-ordered negative activity could not erase them.
- B at tick `3`: output at tick `2` remained physically irreversible.
- Blocked and pressure-staled B paths delivered no negative activity, so the
  closure/output path operated.

Initial/post-trigger state changed in `112/112`; trigger and final propagation
each naturally quiesced in `112/112`; replay matched in `112/112`. Work was
`46..71`, storage was exactly `496` bytes per fixture, and the sixteen stale
rows recorded sixteen physical deallocations. Signed arrival ticks, role
firings, complete/permanent fingerprints, layout controls, and every row result
are serialized in the CSV. Staging paths were absent after publication.

## Classification

NOT-2 temporal absence is **definitively positive within the tested physical
boundary**: existing transient CELL state, time/decay, pressure, firing/closure,
signed coupling, and natural quiescence implement the measured trigger/B/closure
relation. No absence symbol, timeout label, evaluator-selected branch, or new
persistent variable was used.

This result does not classify NOT-1, reinterpret PX3, modify PX0-PX2, or advance
authority. The PX0 source and PX2 definitive evidence hashes remain exact.
