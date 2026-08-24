# PX2-O1 post-forgetting opportunity-window diagnostic protocol

Status: **PREREGISTERED; DIAGNOSTIC EVIDENCE UNSPENT; PX2 NON-AUTHORITATIVE**.

## Frozen basis

- PX2-H1 implementation SHA-256:
  `59db8ed88e70e02570d45902c4f480bf9843e352004d47f032f1c805742c1adc`;
- PX2-H1 summary SHA-256:
  `4eeef4681fba0a5c41dbe42d8911b9f8f285be0c897cb6a5d33c4a8d7da46005`;
- PX2-H1 trajectory SHA-256:
  `78afd070081eb17ce23f7b88da3d0d7d6187b126c8f4f76c111b24651c2f0323`;
- PX2-H1 result audit SHA-256:
  `c1c3c6dd80265d5e864fa7c346b8effde8e4a522a0826067c3e30dcbba202dd9`;
- immutable PX2 GATE-negative CSV SHA-256:
  `ef63c70d3ce980d71cbe1e085174b654bd4dcc4505d3e308e2ed59a34abeaec5`.

The PX0 substrate, PX1 participation trace, reserve `3`, ten-tick ordinary
pressure, return gain, thresholds, and causal-direction opportunities remain
byte-identical.

## Question

> After true forgetting and fresh PX0 reacquisition, is successful opposite
> direction learning determined by the finite physical interval between fresh
> opportunity formation and first use?

## Arms

After the same old-direction history, full deallocation, fresh PX0
reacquisition, and fresh direction proposal, wait exactly:

```text
0, 5, 10, 20, 30 ticks
```

before the first contemporary opposite-direction experience.

Normalize proposal creation to an ordinary-pressure boundary in every cell.
This is evaluator-measured timing and is not exposed as metadata to the
substrate.

Run both mirrors (forward-to-reverse and reverse-to-forward) in all four fresh
layout/traversal-delay strata, with one opportunity per direction and no
distractor load: `5 * 2 * 4 = 40` cells, each duplicate-exact. Namespaces begin
at `0x3_4200_0000` and are fresh.

## Serialized stages

Record independently:

1. initial-direction held-out execution;
2. complete old-direction and PX0-correspondence deallocation;
3. old generations stale and stale execution absent;
4. fresh PX0 correspondence identities;
5. fresh direction opportunity identities;
6. proposal tick and first-use tick;
7. opportunity live state and resistance immediately before first use;
8. first contemporary continuation firing;
9. first consequence firing;
10. trace firing;
11. local returned activity;
12. resistance immediately after first experience;
13. final mature direction and held-out/post-gap execution;
14. source refiring, quiescence, work, fingerprint, duplicate equality.

## Classification

- **A — finite opportunity window:** pre-use resistance is non-increasing with
  wait; live opportunities learn through traversal/trace/return; sufficiently
  delayed dead opportunities cannot do so;
- **B — delay-independent reacquisition:** all waits learn equivalently;
- **C — immediate use also fails:** the GATE lifecycle failure is not explained
  by waiting;
- **D — non-monotonic or ambiguous boundary:** freeze without mechanism change.

The diagnostic does not repair GATE v1, advance PX2, or authorize PX2 authority.
Run once without tuning or rescue.
