# TC-DS1 trace-order negative diagnostic protocol v1

Status: frozen after the immutable TC-DS1 v1 matrix negative and before any
diagnostic implementation or execution.

## Question

Why did the first Gate B Reference and Production observations agree on path
state, contacts, plastic updates, work, body, clock, and quiescence but differ
on the ordered physical-trace hash?

This diagnostic does not repeat the TC-DS1 matrix and cannot create TC-DS1
evidence. It executes only the already identified failing geometry:

```text
root = 1100000
pressure phase = 0
Gate B return delay = 0
```

## Serialization

For Reference and Production separately, serialize every physical transition
with its original sequence index, tick, phase, event variant, and full event
fields. Also serialize:

- ordered trace hash;
- trace length;
- first ordered divergence;
- canonical `(tick, phase, full event)` multiset hash;
- per-event-variant counts;
- retained-only ordered and multiset hashes after excluding the feature-gated
  `Participation` and `ParticipationContact` observations;
- the final observation fields recorded by the failed assertion.

The canonical multiset may diagnose observational ordering only. It may not be
used to replace the frozen v1 comparator in this workflow.

## Classification

- Different transition multisets or different retained future state: physical
  mechanics/candidate counterexample; stop.
- Identical full multisets and retained histories, with divergence limited to
  ordering of simultaneous causally inert observations: measurement-order
  defect. A fresh matrix protocol may then preregister the repaired comparator.
- Any other result: unresolved; stop.

No candidate law, retained law, ARC world, pressure rule, authority, oracle, or
`arch.md` may change.
