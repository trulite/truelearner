# CPC0 contact-compartment spatial attribution protocol v1

Status: frozen before evaluator implementation or execution.

Parent: TC-DS0 characterization at commit
`0a14c7eb588cff199bc4d219fa3ce28468bdb16e`.

TC-DS1 remains a separately tagged stopped negative. CPC0 does not inherit its
feature state or trace instrumentation.

## Narrow question

Can ordinary CELL/ARROW topology provide spatial credit specificity under the
unchanged LR-C law, such that Modulatory activity need not identify an ARROW?

The compared geometries are:

```text
source-local

P --A--> X
P --B--> Y

return at P

contact-compartment

P --> Ca --A--> X
P --> Cb --B--> Y

return at Ca or Cb
```

`Ca` and `Cb` are ordinary CELLs. A contact compartment has no type, label,
port, or special transition rule. Existing Modulatory transmission reaches a
CELL and existing LR-C considers recently traversed outgoing ARROWs from that
CELL.

## Frozen physics

- `eligible_until` and `LOCAL_WINDOW = 4`: unchanged;
- pressure and resistance: unchanged;
- LR-C Drive and Modulatory behavior: unchanged;
- proposal, refractory, timing, and ordering laws: unchanged;
- active core hashes:

```text
lib.rs       d49a6b98081cee65c3e7a5f64e9cf6356fa06ef456a9cf78a41d0cd1187bcd58
mechanics.rs ba81648a0318aedfbf90fe968ca51bdcb7efaddf844c0967887fb35a3f6d69be
Cargo.toml   aff8989aa31a503eecd38c9d6632817819f35456f97f1ebef064a27bdc3afe42
```

CPC0 may add only an experiment-only evaluator and evidence.

## Matrix

Each scenario runs under two fresh identity roots, initial pressure phases
`0..9`, and both `MechanicalConfig::REFERENCE` and `PRODUCTION`.

1. `old_source_local`: both direct branches traverse; return at P; A and B
   both strengthen.
2. `c0_contact_a`: both contact branches traverse; return at Ca; only A
   strengthens.
3. `c1_contact_b`: both contact branches traverse; return at Cb; only B
   strengthens.
4. `c2_a_not_traversed`: A does not traverse; return at Ca; neither candidate
   strengthens.
5. `c3_drive_at_ca`: A traverses; later ordinary Drive reaches Ca and may
   execute it; no candidate receives plastic credit.
6. `c4_late_modulation`: A traverses; Modulation reaches Ca after retained
   eligibility expires; zero positive credit.
7. `c5_dense_distractors`: both branches traverse amid 32 unrelated Drive
   paths around the Ca-side activity; return at Ca cannot credit B.
8. `c6_swapped_identity`: physical identities and construction order are
   mirrored; return at the physical A contact still credits only A.
9. `c6_reverse_order`: the B-side topology is constructed/traversed before the
   A side; return at Ca still credits only A.
10. `c6_timing_offset`: branch traversals have different ordinary delays but
    both remain within the retained window; return at Ca still credits only A.
11. `c7_contact_fanout`: Ca has two recently traversed outgoing candidates;
    Modulation at Ca strengthens both.

The exact matrix is `2 roots * 10 phases * 11 scenarios = 220` physical cases
and `440` mechanics rows.

## Required observations

For every run serialize:

- complete ordered `PhysicalTransition` sequence and hash;
- Drive and Modulatory delivery counts;
- Fire, Eligible, Resistance, Proposal, Deallocate, and Crossing counts;
- candidate-specific positive resistance updates;
- initial and final candidate resistance/coupling/live state decoded from the
  canonical durable body;
- physical work, clock, pressure phase, final body hash, quiescence;
- exact fresh-run replay under the same mechanics.

Reference and Production must match on the complete ordered physical
transition sequence, physical work, durable body, clock, pressure phase,
quiescence, and scenario predicates. `ExecutionCost` and raw checkpoint bytes
are excluded.

The comparator is frozen before evidence. The first mismatch stops the matrix;
no ordering normalization or comparator repair is allowed inside CPC0 v1.

## Decision

- Any functional predicate failure or Reference/Production history mismatch:
  CPC0 negative; stop.
- All 220 physical cases and fresh artifact replay pass: CPC0 development
  positive for the narrow topology claim.

The strongest allowed claim is:

> Given ordinary contact-like CELL/ARROW topology, unchanged LR-C provides
> spatial attribution whose resolution is limited by compartment granularity.

CPC0 does not establish autonomous contact formation, temporal de-supply,
chained credit, pressure de-supply, ARC capability, or authority. CPC1 and
CPC2 remain blocked.
