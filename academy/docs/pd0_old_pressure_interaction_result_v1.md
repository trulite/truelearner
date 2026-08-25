# PD0 old pressure interaction characterization result v1

Status: characterization positive. No candidate mechanism or parameter was
selected. PD1 requires a separately frozen protocol.

Frozen evaluator: `a78f791`, tagged
`pd0-old-pressure-interaction-frozen-v1`.

Immutable evidence: `ee48344`, tagged
`pd0-old-pressure-interaction-characterization-v1`.

## Matrix result

```text
physical cases                              750 / 750
Reference/Production mechanics rows        1500 / 1500
same-mechanics exact reconstructions        3000 / 3000
exact Reference/Production observations      750 / 750
natural quiescence                            1500 / 1500
frozen PQLC1/PQLC0 core hashes                       exact
```

Every case serialized a sixty-tick trajectory across initial resistances
`1, 2, 4`, all ten pressure phases, and the frozen delayed-event schedules.

## The old pressure law is exactly rectangular

The eligibility-specific exception behaved as a perfect binary gate:

```text
eligible pressure epochs unchanged        310 / 310
ineligible pressure epochs reduced/died   1154 / 1154
```

Continuous participation magnitude did not soften or grade this boundary.
When a pressure epoch was covered by `eligible_until`, ordinary pressure made
no decrement. When it was not covered, ordinary pressure reduced or removed
the live weak route.

Unsupported expiry remained a separate decrement after the rectangular window
ended.

## Continuous participation extends beyond the old window

CPC1 local support was positive in every live route at delays `0..4`, and also
at later delays whenever ordinary pressure had not already removed the route:

| Delay | R1 | R2 | R4 | Total |
|---:|---:|---:|---:|---:|
| 0 | 10/10 | 10/10 | 10/10 | 30/30 |
| 1 | 10/10 | 10/10 | 10/10 | 30/30 |
| 2 | 10/10 | 10/10 | 10/10 | 30/30 |
| 3 | 10/10 | 10/10 | 10/10 | 30/30 |
| 4 | 10/10 | 10/10 | 10/10 | 30/30 |
| 5 | 0/10 | 9/10 | 10/10 | 19/30 |
| 8 | 0/10 | 6/10 | 10/10 | 16/30 |
| 12 | 0/10 | 2/10 | 10/10 | 12/30 |

At delays `5, 8, 12`, reconstructed rectangular eligibility was false in every
case. The remaining graded participation could still meet Modulation and
produce support, but only if the weak structure happened to remain physically
live. Thus current temporal plastic availability is no longer defined by the
old window; structural survival still is.

## Activity is not durable learning

The durability boundary is equally explicit:

```text
traversal-only durable resistance gains      0
CPC1-Modulation durable resistance gains     0
```

Same-path renewal was attempted while the route remained live in `167` cases;
in `43` cases pressure had already removed it. Repetition could renew transient
participation and the old rectangular bookkeeping, but never increased
durable resistance.

After activity stopped, every observed weak route in every family and initial
resistance was dead by the frozen sixty-tick horizon. This includes timely
CPC1-supported routes, because CPC1 support is deliberately not yet durable
resistance.

## Scientific diagnosis

The retained implementation currently contains three disconnected temporal
facts:

```text
eligible_until
    -> binary ordinary-pressure suppression

continuous participation
    -> graded availability for local Modulation and PQLC

plastic_support
    -> records that graded coincidence
       but does not yet alter durable structure
```

PD1 therefore has a sharply bounded task: discover whether pressure,
participation, and qualified Modulation can act on one local physical substrate
without equating activity with permanent learning and without retaining the
binary pressure exception.

## Integrity and provenance

Evidence hashes:

```text
matrix.csv
a2eff83df71f518bb1f7fa263fb75b68b2a6a59e866ef4f1bbd529214d217f99

report.md
13a4d6606f983b16369247201670773486542373dc4346dd63167426cc4d9e6f
```

E2B:

- targeted formatting/check/Clippy/preflight:
  `il7q2g6fbjh6lmrgnbudz`;
- sole matrix execution and first static audit:
  `ix4r2h8eob80mcwfztbje`;
- fresh committed-artifact verification:
  `iwp832vxf074nayg909hv`.

The matrix executable ran exactly once. The later verification executed only
the frozen shell audit and checksum validation.

## Boundary

No runtime, constant, pressure, participation, PQLC, ARC, authority, oracle,
or `arch.md` change occurred. ARC A2 did not rerun. PD1 is now scientifically
eligible but has not started.
