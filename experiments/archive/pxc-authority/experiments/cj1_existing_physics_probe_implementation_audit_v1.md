# CJ1 unchanged-physics PROBE implementation audit v1

Status: **FROZEN IMPLEMENTATION; PROBE EVIDENCE UNSPENT**.

## Exact inputs

- protocol commit: `77e954d7c17379228b1fa18f06b3adcefe00e7ac`;
- protocol SHA-256:
  `8c31387f3337c3ad38d83e030dd6a43d4fce8f2e146d93596c9c7231e8b8a6ad`;
- authoritative PX0 law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- evaluator source SHA-256:
  `d23b30d09f384d184588a681e644f3ea3d8184067091cb34cf01da9d9643a6dd`;
- isolated manifest SHA-256:
  `d75dc57c3a32dafc563784e5a1bb30dcd59243fe4d33f9a9388939fa334d7dbe`;
- isolated lockfile SHA-256:
  `a6e2480dcdcd66f805915c3c177b13b267c17c976d06a2e8fe99b51191f4d70e`.

The isolated evaluator depends directly on the authoritative PX0 crate. It
adds no wrapper physics and no substrate state. All authoritative and CJ0
paths are unchanged.

## Frozen execution surface

The sole accepted command is:

```text
cd arms/cj1-distinct-path-coincidence
cargo run --release -- --existing-probe
```

It expands exactly 13 ordered rows and reconstructs each row twice. Physical
entry always fires threshold-2 source CELLs, then traverses explicit ARROWs to
the local threshold-2 CELL. The two-origin/shared-path control converges before
one final path; the one-origin/two-path control diverges after one source CELL.
External origin values affect only deterministic queue order.

The evaluator records entered impulse, source firings, cross-region path
traversals, eligibility writes/closures, local arrivals/firings, effects,
held-out effects, deallocation, complete/permanent fingerprints, native work,
persistent storage, a temporary lower bound, replay and quiescence.

## Pre-evidence validation

- focused formatting: pass;
- all-target check: pass;
- pure evaluator tests: `2/2` pass;
- strict all-target Clippy: pass;
- wrong-argument refusal: exit `2` before evidence construction;
- exact protocol/PX0 hashes: pass;
- PROBE and staging artifact paths: absent;
- result dimensions/names: 13 and unique;
- forbidden substrate/evaluator representation leak scan: clean;
- definitive, authority and later-stage command surfaces: absent.

No CJ1 physical world executed during implementation validation. Generated
build storage was removed; the arm-local ignore prevents its publication.
