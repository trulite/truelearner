# PX3 physical event-boundary no-new-mechanism PROBE v3 protocol

Status: **PREREGISTERED MECHANICAL RETRY; EVIDENCE UNSPENT; PX3 ABSENT**.

## Frozen negative parent

PROBE v2 is permanently frozen at commit
`809a392008f17e9c2de6a1c75b097a79ef065802`, tag
`px3-physical-event-boundaries-no-new-mechanism-probe-v2-first-clause-failure`.
Its source and artifacts may not be changed, rerun, rescued, or regenerated.

V2 failed before the PX3 discriminator: the second cluster's weak direction
candidates were physically deallocated at the tick-`80` ordinary-pressure
edge after their first emission and before returned trace activity. This is
the frozen PX2 O1 law, not evidence for or against PX3 event organization.

All authority restrictions, frozen parent/source hashes, old-M3 behavioral-
reference-only restriction, organism-visible vocabulary restrictions,
no-new-mechanism topology, measurements, controls, work accounting, atomicity,
and no-rescue rules from v1 and v2 remain exact unless amended below.

## Mechanically unique amendment

The complete acquisition schedule moves exactly two ticks earlier:

- first recurrence arrival: tick `64`, rather than `66`;
- first-use delay from the last correspondence-acquisition arrival: `16`;
- all later recurrence, held-out, gapped, singleton, and post-gap arrivals move
  by the same `-2` phase shift.

Nothing else changes: 12 rounds, round spacing `18`, second cluster at `+8`,
alternating early/late order, resistance `3`, thresholds, coupling, delays,
topology, participation return, pressure, and all pass clauses remain fixed.

The shift places first-cluster trace return at tick `71` and second-cluster
trace return at tick `79`; both precede their next destructive pressure edge.
It introduces no extra opportunity, CELL, ARROW, SPIKE, update, cutoff, or
reset.

## Relation-state interpretation

V3 sharpens the already-required distinction between reusable relational
state and route-local phase residue. The evaluator serializes exact per-route
resistance and complete/permanent fingerprints, but the A/B state comparison
also normalizes only fresh physical identity and lane permutation. It may not
alter or feed back into organism state.

Relation-specific state requires both:

1. physical topology or persistent coupling/resistance that relates the two
   co-participating routes rather than merely assigning each route its own
   last-use strength; and
2. held-out physical execution that distinguishes the trained pair from a
   crossed pair after the acquisition stream is absent.

If all route marginals mature, topology remains relation-free, and trained
versus crossed held-out execution is symmetric, freeze
`NO_RELATION_SPECIFIC_STATE_IN_PX0_PX2` even if ordinary pressure leaves a
route-local early/late resistance residue. Such a residue is serialized but
cannot rescue the behavioral capability.

## Fresh identities and atomic artifacts

V3 uses no v2 namespace. Exact bases are:

```text
reference A   0x7_4300_0000
reference B   0x7_5300_0000
blocked       0x7_6300_0000
subthreshold  0x7_7300_0000
replica A     0x8_4300_0000
replica B     0x8_5300_0000
```

The sole execution command, after frozen implementation and no-CELL
validation, is:

```text
cargo run --release -p px0-physical-correspondence \
  --example px3_physical_event_boundaries_probe_v3 -- --probe
```

It emits exactly one
`PX3_PHYSICAL_EVENT_BOUNDARIES_NO_NEW_MECHANISM_PROBE_V3_EVIDENCE_SPENT`
marker and atomically publishes:

```text
results/px3_physical_event_boundaries_no_new_mechanism_probe_v3.csv
results/px3_physical_event_boundaries_no_new_mechanism_probe_v3.md
```

Staging paths use the same basenames prefixed by `.` and suffixed `.staging`.
Path pre-existence refuses execution. Any v3 outcome is frozen with no rerun.

