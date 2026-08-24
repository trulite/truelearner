# Unchanged DS1 after DS-D2 cumulative composition collapse

Status: **CUMULATIVE DEVELOPMENT COLLAPSE AT STAGE 11 — HELD-OUT
BOUNDARY-ROLE RECONSTRUCTION**

No definitive experiment was run. No result artifact was created. M0 remains
authoritative, the E0+A0+A1+R0+C0+D2 stack remains enabling-only, and M1 does
not exist.

## Frozen lineage

- exact parent: `9b2c9f27f4e6e9c8d0e4f79f3225106de0344faf`
- protocol: `bd7cdc2f3f6f59b90ae17310799e905baa96e449`
- MICRO amendment: `c61ceaac2661975b20cbe4bbf2a869f16a820519`
- validated implementation: `b8e511a1309dede70905651dc86994a981812300`
- composition source SHA-256:
  `141f7ca6beeb34e11d8d0d4d3b5e60158db903bb35b208ba6342fcac88bd71f6`
- runner SHA-256:
  `0b225b52ad27d1ed2de74e447ee01be851a52f96c754d0dd439d3e8b2a87e6b9`
- frozen D2 SHA-256:
  `ac257b53e28b0bdbcfd4cbcb7ca855086d1de5812a07029f4b2405fda2a6da8f`
- frozen C0 SHA-256:
  `5c8d00189593ca2f7efb47165efddf85111259f90433a016e5822b5b9578aed2`
- frozen DS1 learner SHA-256:
  `adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e`

## Ordered result

Stages 0 through 10 passed. Stage 11 failed identically in MICRO and every
GATE seed.

```text
8c  D2 relation reaches DS1       32/32 per GATE seed
9   DS1 update fires              32/32 per GATE seed
10  strengths diverge              4/4 patterns per GATE seed
11  correct mature patterns        2/4
    held-out reconstruction        8/16
```

There were zero held-out abstentions. The learner confidently selected a role
on every held-out episode, but half of those roles were wrong.

## Scientific interpretation

DS-D2 is physically and interface sufficient. Its temporary directional ARROW
reaches the byte-identical frozen DS1 consequence surface, triggers every
expected update, and creates stable strength asymmetry.

It is not sufficient for boundary-role learning. The D2 relation answers:

> Which live route produced activity structurally compatible with this
> returned trace?

Because the returned trace is emitted by the selected route, this relation
attributes evidence to—and therefore reinforces—the route that was selected.
It does not answer whether that route realizes the boundary role demanded by
the current interaction. The resulting strength divergence is real but only
half aligned with the evaluator boundary relation.

The narrow corrected conclusion is:

> Non-semantic differential compatibility supplies action attribution and can
> drive plasticity, but self-consistency with the executed affordance is not by
> itself discriminative evidence of boundary-role adequacy.

This freezes a generalization/credit-content dependency, not an interface or
plasticity dependency. Any later enabling gate must reconstruct a
non-semantic difference in consequences that distinguishes the adequacy of
competing affordances; it may not add evaluator correctness, polarity, or the
expected boundary role.

## Validation

Local and exact-commit E2B validation passed:

- formatting;
- strict release Clippy;
- 46 focused release tests;
- release MICRO and GATE;
- `--definitive` rejected before the harness with status 2;
- results digest remained
  `491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4`.

E2B sandbox `iqk0wu89ozqdg9vi1ebac` uses only
`/Users/satya/.cache/truelearner/ds1-after-d2-e2b.json` and was left running.

## Authority

```text
M0                         authoritative
E0+A0+A1+R0+C0+D2         enabling-only
M1                         absent
```

No rescue mechanism or next gate was implemented.
