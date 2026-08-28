# Workstation return-bearing opportunity composition

## Outcome

Add one research-only workstation mode that submits an actual returned body
transition and its anonymous component-local motor opportunities in one core
physical run. Preserve production behavior and the existing falsified
`LocalAfterTransition` mode exactly.

## Invariants

- Only `pending_transitions` create composed local opportunity.
- Both directions of the returned component receive equal adapter treatment.
- The adapter never records or reconstructs the prior direction.
- The transition input retains `PhysicalIncidence::Transition`; opportunity
  inputs remain `Sample` and use a separate anonymous origin.
- The existing core path lineage must identify any current-transition winner.
- Production checkpoint schema and GenericOnly behavior do not change.
- All runs naturally quiesce and replay exactly.

## Change

1. Add `ResearchTransitionOpportunity::ComposedWithReturn`.
2. In that mode, append two sample-incidence motor opportunities per returned
   component to the same `send_physical` batch as the actual outcome return.
3. Do not append those inputs to the later ordinary sample batch.
4. Retain choice diagnostics and add a focused fixture for incidence, replay,
   exact-path executability, current-transition choice, and no cancellation.
5. Add one complete 120-step evidence runner that retains continuation,
   first-break choice evidence, contact projection, replay, quiet, and work.

## Verification

- Formatting, check, clippy, focused tests, production workstation regression,
  core diagnostic purity, and Academy semantic firewall.
- One fresh frozen candidate run only after the targeted fixture passes.
- The candidate is falsified at the first failed ladder rung; no authority or
  default promotion follows from discovery success.
