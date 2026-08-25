# CPC1 local continuous temporal participation result v1

Status: development positive; non-authoritative.

## Result

```text
physical curve cases                         440 / 440
Reference/Production curve rows              880 / 880
physical control cases                       180 / 180
Reference/Production control rows            360 / 360
total physical cases                         620 / 620
total mechanics rows                        1240 / 1240
same-mechanics exact fresh replays          2480 / 2480
exact ordered cross-mechanics histories      620 / 620
graded positive delays                         0 .. 20
naturally relaxed zero delay                       1024
runtime/substrate authority changes                    0
```

Within the fixed CPC0 contact topology, actual ARROW traversal produced a
path-local physical magnitude. Ordinary elapsed time relaxed that magnitude by
the same arithmetic law at every contact:

```text
traversal:
    participation += 2^32

each physical tick:
    participation = floor(participation * 15 / 16)

Modulation at the contact:
    plastic_support += participation
```

The measured single-traversal response was graded rather than rectangular:

```text
delay  0     4294967296
delay  1     4026531840
delay  2     3774873600
delay  3     3538944000
delay  4     3317760000
delay 10     2252540587
delay 20     1181368504
delay 1024            0
```

There is no deadline, remaining-ticks field, accepted-delay comparison,
positive-state branch, or boolean eligibility interpretation in this plastic
response.

## Locality and renewal

All preregistered controls passed across two fresh identity roots, every
pressure phase, and both mechanics:

- prompt return at `Ca` supported only the `Ca` contact;
- return at `Cb` supported only the `Cb` contact;
- unrelated activity did not renew either path;
- repeated actual traversal of A renewed A while B continued relaxing;
- repeated traversal of B did not maintain A;
- source activity without contact traversal changed neither path;
- equal activity on the wrong path did not maintain A;
- one contact with two traversed outputs supported both, preserving CPC0's
  physically honest compartment granularity;
- return at delay 1024 produced zero support while the structure remained live,
  deterministic, and quiescent.

Thus CPC1 establishes temporal interaction at the same physical resolution
earned by CPC0. It does not introduce finer hidden attribution.

## Representation independence

Reference and Production matched on the complete ordered retained
`PhysicalTransition` vector, candidate participation/support state, physical
work, durable body, clock, pressure phase, liveness, and quiescence in every
case. Candidate state was compared separately and did not add mechanics-
dependent trace events.

## Evidence hashes

```text
curve.csv
2a23c68a37d8b5c7fc08067a924b25af081884c3d7334db43d80fe4c661af9ca

controls.csv
5112043ff9e646e5c5d59558bbdff8b74b9ab8a856223041d49e0026df9b2338

report.md
19d6a034f9c21e4f4db919cff1489963cdcf6f390aad250f0d62a76b04a82692

SHA256SUMS
628f428dca216ae24eea25f47b32b787f3b44f5802bb65df06fcee179def900b
```

## E2B provenance

- targeted default/feature validation: `i45f1g5a6ob5ww5x6ngke`;
- sole primary matrix: executed once from frozen commit `8153198`; its console
  identifier was lost to active-turn compaction, while all four downloaded
  artifacts remain checksum-valid;
- fresh artifact replay: recorded after the positive evidence commit.

## Boundary

CPC1 establishes only a continuously relaxing, path-local participation state
and its graded arithmetic intersection with Modulation at the same contact.
The candidate does not alter pressure, durable resistance, checkpoints, ARC,
or authority. The retained `eligible_until` pressure bookkeeping still exists.

CPC2 becomes scientifically eligible as a separate chained-local-closure
workflow. Pressure de-supply and ARC A3-A5 remain paused. The oracle,
authority, and `arch.md` remain unchanged.

