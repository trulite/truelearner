# CJ0-C plasticity-only conjunction PROBE v2 cadence amendment

Status: **PREREGISTERED MECHANICAL TIMING CORRECTION; NOT CLAIM ELIGIBLE**.

This document amends only the primary and reversal cadence in
`cj0_c_plasticity_conjunction_probe_v1_protocol.md`. Every mechanism clause,
world, control, pass condition, serialization requirement, stop rule, and
authority boundary in v1 remains frozen and applies unchanged.

## Reason frozen before implementation

Static schedule inspection found that v1's resistance-`1` proposal at one
occurrence and twelve-tick recurrence mechanically guarantees deallocation
before a second occurrence: ordinary pressure steps at tick `10`, while the
only newly used incident ARROW also loses unsupported-use pressure when its
four-tick eligibility expires. That cadence cannot test whether the exact
candidate bootstraps; it deterministically removes the proposal first.

No candidate source existed and no organism execution or result artifact was
run. V1 remains immutable. The law and all numeric structure are unchanged.

## Corrected cadence

- Use `16` acquisition rounds at round spacing `8`.
- Within each round, place one two-source cluster at `base` and the other at
  `base+4`.
- Alternate which organization is early every round.
- Consequently every individual route occurs exactly once per round and each
  organization's recurrence intervals alternate `12,4,12,4,...` in opposite
  phase. Counts, total activity, traversal, eligibility, pressure, effects,
  identity frequency, and aggregate timing marginals remain exact across all
  four routes.
- Use the same corrected cadence for the `40` reversal rounds and every
  matched replica. Too-late and spacing controls remain separately frozen at
  gaps that cannot sum at the target.

The first occurrence may expose structure; the next four-tick recurrence may
exercise it inside the existing eligibility/pressure opportunity. No special
replay, top-up, reset, or proposal-time emission is added.
