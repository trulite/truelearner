# CORE1-E13-D — composition depth and lifetime boundary result v1

## Result

The frozen CORE1-B composition mechanism is positive across every tested
depth, including depth 128. Its first observed boundary is not causal depth;
it is whether the ordinary Drive and QLP material remains alive long enough
for consequence to close twice.

This gate changed no organism/runtime law and did not run ARC. It used the
frozen protocol at `d3d42f5` and evaluator at `dd768d6`.

## Definitive matrix

The single definitive run completed 34 cases, each with two fixed supported
experiences:

| Family | Positive | Tested | Observed boundary |
|---|---:|---:|---|
| Depth | 8 | 8 | Full closure through depth 128 |
| Initial resistance | 4 | 6 | R2 partial; R4 is the lowest full positive at delay 12 |
| Return delay | 9 | 12 | Delay 32 positive; delay 48 negative |
| Signed variation load | 8 | 8 | Full closure through 64 signed pairs |
| **Total** | **29** | **34** | 5 recorded negatives, no repairs |

Every case was naturally quiescent and had zero pending activity. Exact
Reference replay and Reference/Production equality were both `34/34`.

## Depth composition

For configured depths `1, 2, 4, 8, 16, 32, 64, 128`, consequence traversed
and consolidated the complete route in both experiences. The measured
backward supported depths were respectively:

```text
2, 3, 5, 9, 17, 33, 65, 129 links
```

The extra link is the source-to-first-contact link. No case introduced a path
representation or depth-dependent organism rule.

Physical work was exactly:

```text
PhysicalWork = 10 * configured_depth + 20
```

for the complete two-experience world. Thus the tested composition cost grew
linearly, from 30 at depth 1 to 1,300 at depth 128.

## Lifetime boundary

At fixed depth 8 and return delay 12:

- R1 died before the first consequence and could not re-execute.
- R2 completed and consolidated all nine desired Drive links in the first
  experience, but only one link in the second; its QLP relay material no
  longer survived as a complete chain.
- R4, R8, R16, and R32 completed both experiences exactly.

The lowest full positive among the preregistered values is therefore R4. The
result is a bracket, not a tuned lifetime recommendation.

## Return-delay boundary

At fixed depth 8 and resistance 8, delays through 32 completed both
experiences. At delays 48 and 64, the first closure completed, but the QLP
relay material was gone before the second consequence. At delay 96, the
desired route was gone before the first consequence and could not re-execute.

The observed two-experience boundary is therefore:

```text
delay 32  full closure
delay 48  closure stops before the second experience
```

This is ordinary material lifetime, not a supplied credit-depth limit.

## Signed variation load

The desired depth-8 route completed both experiences with `0, 1, 2, 4, 8,
16, 32, 64` symmetric signed variation pairs. No tested load displaced or
interrupted the desired live route.

All noise branches also physically participated in the local consequence and
were consolidated. This family therefore demonstrates execution/composition
under increasing symmetric competing topology; it does **not** demonstrate
selection against unrelated noise. Physical work rose from 100 with no pairs
to 35,684 with 64 pairs, exposing a substantial mechanical cost even though
the physical result remained exact.

## Claim

The frozen E13-B mechanism is a genuine recursive local composition process
over the tested range:

> Backward consequence composition remains exact through depth 128 with
> linear physical work. Its observed failure boundary is the finite lifetime
> of the participating local material, not a limit in causal depth.

E13-B remains frozen. CORE1-A and CORE1-C remain documented boundary cases,
not bugs. No authority claim is advanced by this characterization.

## Evidence

- Matrix: `experiments/results/core1_e13d_boundary_v1/matrix.csv`
- Generated report: `experiments/results/core1_e13d_boundary_v1/report.md`
- Matrix SHA-256:
  `8c951528f06bf0ff291dbc9f3a5f210a9efae041cd21047eaf7e076bce5ea1fa`
- Report SHA-256:
  `598dd158168aaa1594ff2e4d469953b9d79affdd1a5396e9360b2973d881bc33`

ARC E14 remains unchanged and unrun in this gate.
