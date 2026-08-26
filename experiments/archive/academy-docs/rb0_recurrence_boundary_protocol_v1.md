# RB0 — recurrence boundary under continuous material physics

## Status and lineage

This protocol is frozen before any RB0 fixture API, evaluator, or execution
change. Its exact parent is CORE0 v3 result `17e5881`. RB0 is
characterization-only development evidence. It cannot change learning,
variation, organism authority, `arch.md`, or any CORE0 result.

## Question

What first physical difference makes the RS1 recurrent circuit settle while
CORE-B E11 continues, when both are given identical fixed topology, timing,
thresholds, and initial incidence?

The prior results already establish a likely boundary condition: RS1 observed
`H1/H2` periodic and `H3+` quiescent for excitation two and threshold two,
while CORE0 E11 supplied inhibition magnitude two. RB0 characterizes that
geometry under both integer and continuous material execution; it does not
select or learn a magnitude.

## Frozen bodies

Two bodies use the current CORE0 runtime and differ only in material
representation:

- **RS1-style:** `Core0Profile::A`, whole integer activation and coupling;
- **CORE-B:** `Core0Profile::B`, signed Q32 activation and coupling.

There is no Modulation, participation-dependent update, structural proposal,
deallocation, learning, variation, ARC, or Academy input in a valid row.
All ARROW resistance is `1_000_000`, enough that local forgetting cannot end
the observation.

## Fixed topology

```text
A -- +E --> B -- +E --> A
|            |
+1           +1
v            v
Ia           Ib
|            |
-H -> A      -H -> B
```

`A/B` have threshold `T`; `Ia/Ib` are ordinary threshold-one CELLs. All links
are ordinary SourceFires Drive ARROWs. A single external incidence of magnitude
`T` starts at A. Positions are separated so this boundary probe cannot create
local proposals.

## Material sweep

All values below are frozen before execution. Fractions are exact Q32 halves.

1. **Historical pair:** `(E,T,H) = (2,2,2)` and `(2,2,16)` at delay `1+1`.
2. **Efficacy plane:**
   - `E = 1, 1.5, 2, 2.5, 3`;
   - `H = 0, 1, 1.5, 2, 2.5, 3, 4, 8, 16`;
   - `T = 2`, reciprocal delays `1+1`.
3. **Threshold section:**
   - `(E,T) = (1,1), (2,2), (3,3)`;
   - `H = 0, 1, 1.5, 2, 2.5, 3, 4, 8, 16`;
   - reciprocal delays `1+1`.
4. **Delay section:**
   - `(E,T) = (2,2)`;
   - `H = 2, 2.5, 3, 4, 16`;
   - reciprocal delays `0+1`, `1+1`, `2+2`, `3+3`.

Duplicate parameter tuples are canonicalized before execution. Profile A can
represent only whole material values; fractional rows are CORE-B-only and are
labelled accordingly rather than rounded.

## Observation

Observe at most 256 scheduled deliveries, then continue a still-active body
for 32 more. Classify each row:

- `quiescent`: the queue empties naturally;
- `periodic`: activity survives both ceilings with a repeated firing period;
- `persistent_nonperiodic`: activity survives both ceilings without a proven
  period;
- `growing`: second-segment firing density exceeds the first by the frozen
  diagnostic criterion.

Record:

- wave-level signed Drive incidence and CELL firing sequence;
- A/B and Ia/Ib firing counts;
- excitatory, relay, and inhibitory traversals;
- per-wave Q32 activation after local incidence;
- initial and final Q32 material coupling/resistance;
- PhysicalWork, scheduled deliveries, final tick, pending count, and
  quiescence/ceiling state;
- Reference replay and Reference/Production equivalence.

The first divergent physical transition between RS1-style and CORE-B is
reported only for parameter tuples both can represent. Different observer
labels caused solely by Q32 versus integer serialization do not count as a
physical divergence; firings, incidences, traversals, body state, clock, work,
and quiescence do.

## Hard controls

- `(2,2,2)` reproduces the CORE0 weak-inhibition boundary;
- RS1-style `(2,2,16)` settles as in RS1;
- uninhibited executable recurrence remains active;
- a one-way acyclic control with the same total intended forward activity
  quiesces and is never classified as instability;
- no coupling/resistance changes, proposal, plasticity update, Modulation,
  QLP traversal, or deallocation occurs;
- exact replay and Reference/Production equivalence hold for every row.

## Decision

- If a finite ordinary inhibitory region preserves the intended first
  traversal and settles CORE-B, RB0 concludes that E11 exposed a material
  boundary, not a missing substrate law. The later developmental question is
  whether existing consequence learning can reach that region.
- If no frozen finite inhibitory value settles CORE-B while preserving useful
  execution, or stability exists only on a razor-thin representation-specific
  point, RB0 records a genuine continuous-physics frontier.

RB0 stops after characterization. It cannot run E12, CE1, FD2, ARC, scarcity,
or variation work.

