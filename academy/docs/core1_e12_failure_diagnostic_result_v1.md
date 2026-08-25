# CORE1-E12D — E12 failure diagnostic result v1

## Outcome

CORE1 E12 is an **evaluator fixture defect**, not a failure of continuous
efficacy learning.

All three frozen profiles (`CORE1-A/B/C`) produced exactly the same result
under Reference, exact Reference replay, and Production:

1. both inhibitory routes exist and remain live;
2. both routes traverse during every teaching experience;
3. nonzero participation remains after the fixed 12-tick delay;
4. Modulatory incidence reaches the correct local contact;
5. coupling and resistance change continuously after consequence;
6. inhibitory magnitude crosses the RB0 `2.5` boundary after experience 2;
7. the probe never initiates, so neither inhibitor can re-execute;
8. no probe is classified settled.

Replay equality and Reference/Production equality are exact for all three
profiles.

## Exact learning curve

The two inhibitory routes are symmetric. Representative route 0 in CORE1-A:

| Experience | Participation after traversal | Participation at consequence | Coupling before | Coupling after |
|---:|---:|---:|---:|---:|
| 1 | 4294967296 | 1979772000 | -2.000000000 | -2.460951589 |
| 2 | 6151003546 | 2835314858 | -2.460951589 | -3.121099772 |
| 4 | 7299683818 | 3364800855 | -3.867329193 | -4.650757942 |
| 8 | 7554255691 | 3482146164 | -7.066166116 | -7.876916449 |
| 16 | 7563443341 | 3486381225 | -13.558330359 | -14.370066744 |
| 32 | 7563454530 | 3486386383 | -26.546129614 | -27.357867199 |

Resistance also increases at every qualified consequence. Ordinary QLP relay
count is zero because consequence is delivered directly to each local contact;
no multi-hop relay is required in this world.

## Exact probe failure

At every preregistered count `0,1,2,4,8,16,32`:

```text
A fires = 0
B fires = 0
inhibitory route re-executes = false
natural quiescence = true
observation ceiling = false
```

The coupling is already `-3.121099772` at probe 2 and reaches
`-27.357867199` by probe 32. Therefore insufficient learning is excluded.

The frozen E12 body constructs both recurrence junctions with threshold `100`,
then the probe supplies an initiating Drive impulse of `2`. The probe does not
construct the separate executable recurrence world required by the E12
protocol. The initiating junction cannot fire, so the learned inhibitory route
has no physical opportunity to execute.

This localizes the first broken link to diagnostic item 6:

```text
learned inhibitor re-executes on probe = false

because

probe initiating activity = absent
```

It is not classification D (maturation too weak) and does not justify new
physics.

## Evidence

- Protocol: `d85987c`, tag `core1-e12d-protocol-v1`
- Frozen evaluator: `290da9d4992285659c6abd6f7be2e3296700dd75`, tag
  `core1-e12d-frozen-v1`
- Sole execution marker: `CORE1_E12D_V1_EVIDENCE_SPENT`
- Completed rows: 702 data rows plus one CSV header
- Probe counts: `0|1|2|4|8|16|32`
- CSV SHA-256:
  `1f3f7558bd5d914aa64949599d30991bcf3b73a860f9ac8fce27cf272bd5af57`
- Generated summary SHA-256:
  `6e9af1b254ede6239e2b6e19582c70fbd025f3c4bb504d8b1a82e625ff2acad8`

The generated summary's coarse class `E` is retained verbatim. This audit
provides the finer fixture diagnosis using the recorded causal chain; no row
was edited and the diagnostic was not rerun.

## Boundary

CORE1's immutable negative remains unchanged. This diagnostic does not make
E12 positive, repair its probe, run ARC, touch CORE1-D, or advance authority.
A future E12 rerun requires a separately frozen evaluator correction whose
probe uses an ordinary executable recurrent body while preserving the learned
inhibitory material.

