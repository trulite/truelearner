# SI0 simultaneous local incidence implementation audit v1

Status: frozen candidate; no SI0 physical evidence has run.

Parent protocol: `ff28373` (`si0-simultaneous-local-incidence-protocol-v1`).

## Candidate surface

- `truelearner-core` feature `si0` selects a Drive-only execution path.
- A scheduled Drive arrival carries `causal_wave`, initialized to zero for an
  admitted arrival and incremented only by zero-delay, same-phase physical
  transmission caused by a firing.
- Scheduling drains every arrival sharing the minimum `(tick, phase, wave)`
  before a junction is evaluated.
- Same-target arrivals are grouped by `CellId` equality only. The candidate
  uses neither ordered handles nor physical identity for grouping.
- Every junction receives one signed incidence sum, updates activation once,
  evaluates threshold/refractory once, and fires at most once.
- All junction incidences in a wave update before any firing emits the next
  wave. Independent same-wave processing order therefore cannot create a
  same-wave causal edge.
- Reference and Production use the same candidate law. Scheduler, traversal,
  frontier, layout, and batching remain mechanical choices.

`origin_physical`, target handle, and serial remain in the mechanical queue key
only to make storage/inspection deterministic inside an already fixed physical
wave. The candidate drains that complete wave before applying physics, so none
of those values selects an incidence member or determines its result.

## Frozen evaluator

The evaluator contains ten preregistered families and six mechanical
permutations: arrival reversal, CELL insertion reversal, ARROW insertion
reversal, two physical-name bijections, and identity. It compares logical
partial-order traces, future-causal CELL state, durable body, PhysicalWork,
clock, pending activity, replay, and natural quiescence across Reference and
Production.

Independent junction chunks inside the same `(tick, phase, wave)` are
canonically normalized only after execution. This is an observation rule for a
physical partial order, not an execution tie-break.

## Scope exclusions

- No Modulatory incidence or Drive/Modulatory ordering claim.
- No cycle detector, TTL, maximum-wave law, predecessor, path, or route state.
- No RS2, CE1, FD2, ARC, authority, oracle, or `arch.md` advancement.
- Accepted non-SI0 runtime behavior is unchanged because the candidate is
  feature-gated.

Targeted E2B formatting, release check, and strict Clippy passed in reusable
development worker `ifk44bxtlfjlci644r63m`. The evidence matrix has not run.
