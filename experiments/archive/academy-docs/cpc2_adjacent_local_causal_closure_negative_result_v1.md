# CPC2 adjacent local causal closure negative result v1

Status: immutable development negative; no rescue arm authorized.

## Result

```text
physical cases                              360 / 360
Reference/Production mechanics rows         720 / 720
same-mechanics exact reconstructions        1440 / 1440
exact ordered cross-mechanics histories     360 / 360
local-Modulation predicate cases            120 / 180
adjacent-relay predicate cases              120 / 180
complete unchanged-law arm                    0 / 2
runtime or substrate-law changes                  0
```

The two arms failed for complementary physical reasons.

## Arm M: local but not chain-capable

Local Modulation preserved the CPC0+CPC1 attribution boundary:

```text
one contact          pass
broken chain         pass
unused intermediate pass
branch both/one      pass
temporal break       pass
```

But Modulation did not fire its contact or emit any ordinary upstream
activity. At depths two and three, only the last contact received support:

```text
two contacts   support = 0 | 4026531840
three contacts support = 0 | 0 | 4026531840
```

The parallel main chain therefore also failed to support its earlier contact.

## Arm R: chain-capable but not participation-qualified

The adjacent relay repeated ordinary Drive and local Modulation and therefore
reached every contact at depths one, two, and three:

```text
three contacts support = 4026531840 | 4026531840 | 4026531840
```

It also preserved branch honesty because CPC1 participation at an untraversed
branch remained zero. But relay propagation itself did not depend on forward
participation:

- `broken_chain` delivered Modulation to all three contacts and supported the
  isolated upstream contact;
- `unused_intermediate` crossed the unused middle contact and supported the
  earlier participating contact;
- `temporal_break` continued delivering Modulation through both expired
  upstream contacts even though their support remained zero.

Thus an explicit ordinary return topology can move activity, but it does not
constitute local causal closure. It is blind to whether each adjacent forward
contact actually maintained the causal episode.

## Exact world signatures

World order:

```text
one, two, three, broken, unused, distractor, branch-both, branch-one, temporal
```

Observed complete signatures:

```text
local Modulation [T, F, F, T, T, F, T, T, T]
adjacent relay   [T, T, T, F, F, T, T, T, F]
```

Every signature was invariant across two fresh identity roots, pressure phases
`0..9`, Reference, Production, and exact reconstruction.

## Scientific classification

CPC2 identifies a precise missing physical affordance:

> Local attribution and continuous local participation exist, but current
> ordinary dynamics cannot make consequence influence cross an adjacent
> contact only when that contact remains part of the causal episode.

This is not evidence for a stored path, backward credit packet, or depth loop.
Those remain forbidden. CPC2 is stopped before mechanism invention.

## Evidence

```text
matrix.csv
2914030b92c8d6b0960b61c8107016a2b3afefaf641ae024511cfbd40489cc9d

report.md
56150936c94bcbfd041b473701c4c81a1118f1c9f04320ac14a8d25d090c6e0c

SHA256SUMS
5b4e07dcbc9354aba99c94085f4525cafc5ace2612844e2d40997b72b31f1f58
```

Sole matrix and static audit E2B sandbox: `ic6wbb4i3jv376ljz4eiq`.
Fresh exact artifact replay: `i7jh90vvetzjeushpz3jo`.

## Boundary

No CPC3, pressure integration, eligibility deletion, ARC A3-A5, authority,
oracle, or `arch.md` work starts from this result. A separately preregistered
diagnostic or new physical hypothesis is required before continuing causal
closure.
