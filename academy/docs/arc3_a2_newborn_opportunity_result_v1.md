# ARC3 A2 newborn-opportunity pressure diagnostic result v1

Status: immutable development negative. The candidate successor law is not
accepted as retained physics, and no architectural oracle changed.

Protecting a locally proposed ARROW only until its first emitted effect was
resolved fixed the original tick-10 collision and allowed the phase-0 A2 row
to pass. The phase-9 row exposed a later collision: an actually traversed and
eligible route can still die under ordinary pressure while its physical
consequence is returning.

## Core discriminators

All four focused E2B controls passed:

- a newborn proposal survived pressure through its first effect;
- resolving a stale first effect ended protection;
- protection did not act as modulatory eligibility;
- exact live-checkpoint round-trip preserved the transient deadline;
- the pre-existing resistance-1 ordinary-ARROW pressure control still died.

## Official phase-0 row

Phase 0 passed A2 exactly:

| Turn | Context | Action | Update | Candidate | Clock |
|---:|---:|---:|---:|---|---:|
| 0 | 699 | 1 | 0 | R1 / C1 / live | 3 |
| 1 | 290 | 4 | 1 | R1 / C1 / live | 6 |
| 2 | 552 | 2 | 1 | R1 / C1 / live | 9 |
| 3 | 524 | 3 | 1 | R1 / C1 / live | 12 |
| final return | 124 | none | 1 | R1 / C1 / live | 15 |

All four expected actions changed 52 official raster cells, the update sequence
was `[0,1,1,1,1]`, every transition quiesced, and the complete row replayed
exactly.

## Official phase-9 row

The phase-9 row produced all four correct actions and all four changed rasters,
but failed the final qualified update:

| Turn | Context | Action | Update | Candidate | Clock |
|---:|---:|---:|---:|---|---:|
| 0 | 699 | 1 | 0 | R1 / C1 / live | 12 |
| 1 | 290 | 4 | 1 | R1 / C1 / live | 15 |
| 2 | 552 | 2 | 1 | R1 / C1 / live | 18 |
| 3 | 524 | 3 | 1 | R0 / C1 / dead | 21 |
| final return | 124 | none | 0 | R1 / C1 / live | 24 |

The decisive sequence was:

```text
context 524 route traverses       tick 18
first route effect resolves       tick 19
route remains eligible for return
ordinary pressure                 tick 20
candidate R1 -> R0 / dead
already-emitted downstream action tick 21 -> action 3
official consequence returns      tick 21+
modulation reaches source         candidate already absent -> zero update
```

So the route really participated and caused the correct outward action. It was
not merely a newborn proposal waiting to execute. The missing distinction is
between unsupported dormant structure and recently participating structure
that is still lawfully awaiting downstream return.

## Classification

The candidate law was physically meaningful but incomplete. It must remain a
frozen development negative because it is phase-dependent.

The next hypothesis is smaller than extending an arbitrary grace timer:

```text
ordinary pressure
    dormant/noneligible live ARROW -> ordinary decrement
    ARROW with live participation eligibility -> do not decrement yet

eligibility expires without modulation
    -> existing unsupported-use pressure applies
```

That would reuse the already-authoritative participation/eligibility window
instead of adding another semantic return label or curriculum-specific delay.
It was not implemented or tested here.

## Evidence

- E2B sandbox: `in4tr1yesxq16xxjqm5xy`;
- phase-0 A2: pass; exact replay: true;
- phase-9 A2: fail on final update; exact replay: true;
- A3-A5: not executed;
- phase-0 suite SHA-256: `daeef92f59385c6718642c891f6b1390b0e4ce862c2ddc00e2d6db2717106574`;
- phase-0 report SHA-256: `cd4e9e6fa8e8ba6588339b4f7f12c0f722a589e0ca60b15ea0243bb3efc6a7b5`;
- phase-0 video SHA-256: `99c0d05887a7fa3a7f00fc7fdb870a18c2f6e5c8e6c464668c6b9c9adb0b5bdd`;
- phase-9 suite SHA-256: `8c29313209f44e9561933f03d68e587bab01e0f33286a5408d3acc37a271d17f`;
- phase-9 report SHA-256: `5152fa31966321d9ce29eb449fec2f800063491747130b5fd736c38a7d6417f0`;
- phase-9 video SHA-256: `841eeb44e871c1c6170d57574a0119e06003f8732fc4d95c257e56f84e34b367`.

Review videos:

- `results/arc3_a2_newborn_opportunity_v1/phase0/gallery/episodes/arc3-a2-four-actions/episode.mp4`
- `results/arc3_a2_newborn_opportunity_v1/phase9/gallery/episodes/arc3-a2-four-actions/episode.mp4`
