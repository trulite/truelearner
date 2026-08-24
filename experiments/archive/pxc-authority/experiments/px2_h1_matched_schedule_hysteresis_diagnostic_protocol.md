# PX2-H1 matched-schedule hysteresis diagnostic protocol

Status: **PREREGISTERED; DIAGNOSTIC EVIDENCE UNSPENT; PX2 NON-AUTHORITATIVE**.

## Frozen basis

- PX2 GATE implementation SHA-256:
  `8cdd72cff084c6a85d65629fd6504f5ca96f14d281a7a5ac518fd9c4754579ec`;
- immutable GATE-negative CSV SHA-256:
  `ef63c70d3ce980d71cbe1e085174b654bd4dcc4505d3e308e2ed59a34abeaec5`;
- immutable negative audit SHA-256:
  `2e04bb306d6426181461357d4caeb8a10b9a8c7499ac2bac65f2f04efe6ac943`;
- immutable negative handoff SHA-256:
  `98865d55579e5ecdbe9981415227601990834c3187720a72053f3fc2f070a814`.

The PX0 substrate and PX1 participation-trace law remain byte-identical. No
threshold, reserve, pressure, trace, return, or plasticity change is permitted.

## Question

> With exactly 12 forward and 12 reverse physical participation histories, does
> schedule order determine the mature direction because one weak structure
> becomes physically protected before its competitor receives equivalent use?

## Schedules

Every schedule has exactly 24 equally spaced experiences, 12 per direction:

1. `F*12,R*12`;
2. `R*12,F*12`;
3. strict alternating, forward first;
4. strict alternating, reverse first;
5. all six circular rotations of `(F,F,R,F,R,R) * 4`.

The six rotations preserve the same circular multiset and inter-event-gap
pattern while changing the linear starting phase. Nothing identifies a
preferred direction to the substrate.

Run all ten schedules in four fresh layout/timing strata derived from G0..G3,
but hold opportunity abundance at one path and distractor load at zero so the
diagnostic isolates history. Use exact duplicate execution: `40` cells and `80`
developments. All namespaces begin at `0x2_4200_0000` and are fresh.

## Per-experience serialization

After every experience, record separately for both directions:

- scheduled physical participant;
- continuation firing;
- trace firing;
- local-return coincidence;
- candidate live state;
- candidate resistance;
- inferred resistance gained from local return;
- resistance spent by ordinary pressure;
- first experience at which resistance exceeds the initial reserve `3`;
- physical deallocation experience, if any.

Also record final live paths, final resistance, held-out direction execution,
post-gap execution, source refiring, quiescence, work, complete fingerprint, and
duplicate equality.

`pressure_spent` is evaluator-side accounting from the same arrow's prior
resistance, physical return gain, and subsequent resistance. It is not supplied
to the substrate.

## Classification

- **A — protection-first hysteresis:** schedule rotations systematically change
  the winning basin, and the trajectory shows the eventual winner gained
  protection before the competitor deallocated or fell behind;
- **B — matched histories remain symmetric:** both directions remain reusable
  across schedules;
- **C — ordering matters but protection timing does not explain it:** freeze the
  unexplained boundary;
- **D — no stable classification:** freeze the negative/ambiguous result.

No classification advances PX2, repairs GATE v1, or authorizes authority
evidence. Run once; preserve every result without tuning or rescue.
