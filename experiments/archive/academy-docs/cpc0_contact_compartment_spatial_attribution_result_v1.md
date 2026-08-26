# CPC0 contact-compartment spatial attribution result v1

Status: development positive; non-authoritative.

## Result

```text
physical cases                              220 / 220
Reference/Production mechanics rows         440 / 440
same-mechanics exact replays                 880 / 880
exact ordered cross-mechanics histories     220 / 220
old source-local alias controls               20 / 20
contact-specific controls                    120 / 120
contact fan-out granularity controls          20 / 20
natural quiescence                           440 / 440
fresh artifact replay                         exact
runtime or substrate-law changes                  0
```

The same unchanged LR-C law produced two different attribution resolutions:

```text
P --A--> X        return at P       A and B strengthen
P --B--> Y

P --> Ca --A--> X return at Ca      only A strengthens
P --> Cb --B--> Y
```

Identity swaps, construction/traversal order changes, ordinary timing offsets,
and 32 unrelated Drive distractors did not change the contact-local result.
Attribution followed the physical CELL at which Modulatory activity arrived.

The deliberately coarse contact control also passed:

```text
Ca --A--> X
Ca --B--> Y

both traverse + Modulation at Ca
    -> A and B both strengthen
```

Thus a contact CELL is not magically synonymous with one synapse. Specificity
is exactly limited by the topology's physical compartment granularity.

## Representation independence

Reference and Production matched on the complete ordered public
`PhysicalTransition` vector—not a sorted or normalized comparator—as well as
physical work, durable body, clock, pressure phase, quiescence, and all
scenario predicates in every case.

The fresh replay worker regenerated `matrix.csv`, `report.md`, and
`SHA256SUMS` byte-for-byte.

## Audit correction

The primary matrix was positive and checksum-valid. Its frozen v1 static audit
could not execute the semantic source scan because the E2B image lacked `rg`;
the shell continued because the missing command occurred as an `if` predicate.

A separately preregistered audit-v2 replaced only that invocation with
portable `grep -E`. It passed in a fresh worker without compiling Rust or
reconstructing a physical world. The v1 script and matrix remain immutable.

## Evidence hashes

```text
matrix.csv
56d2236f92cb959094e5016054b4d2c8464bf6e92da065300bf02448513e711a

report.md
307c86f989ddf6fb95dfddea0d3f3990c61a97d9174eb4f538d0eb565e72ea59

SHA256SUMS
348e4bb96733bee763d215b7d4a179101ae0f89b127950440dedcfdcc573b288

exact_replay.txt
ca5f3d13eef5531ba3016f2a1226f97f43c37291b60ae77823610d5283c66e00

portable audit-v2
6e385b11db9011a967da5a52dcd941d0c53b070b32aa2d215263bf26c08d3e29
```

## E2B provenance

- targeted formatting and strict Clippy: `inlgs9h8g1992uxva5t86`;
- sole primary physical matrix: `ifkzazwbrnie733ltd5vz`;
- portable audit-v2: `i1q7aedonns4aa78e60av`;
- fresh exact artifact replay: `iscumuwbq4uqivwlliezy`.

## Boundary

CPC0 establishes spatial attribution given supplied contact topology. It does
not establish autonomous contact formation, temporal de-supply, chained local
closure, pressure de-supply, ARC capability, or authority.

CPC1 is scientifically eligible as a separate workflow. ARC A3-A5, pressure,
the oracle, authority, and `arch.md` remain unchanged.

