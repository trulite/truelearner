# Continue an action while its physical transition returns

## Outcome

Add one opt-in cumulative protocol that resolves local motor competition by retaining
one uniquely identifiable incumbent when it has both an unanswered return and a
current owner-local physical-transition lineage. A sample cannot retain it. The law
uses no position, direction, boundary, hand, or desired-action knowledge.

## Observed counterexample

At hand step 8, both motor paths and candidates were executable. The previous
movement's transition selected the opposite motor before its consequence write;
later in the same external cycle the fresh write selected the original motor. Both
effects fired and canceled. The first missing step is competition, not path, memory,
ownership, or output.

## Model and invariants

- Extend `RecursiveLearnerRootFreshOpportunity`; keep every earlier protocol frozen.
- Mark an executable candidate transiently when at least one path lineage contains a
  transition no older than the existing recent window and its physical origin is
  owned by the candidate owner.
- Prefer continuation only when exactly one local candidate is so marked and still
  has an unanswered return; otherwise use ordinary recent consequence, fresh
  replacement, and deterministic ranking unchanged.
- Add trace evidence for every continuation admission/rejection.
- Store no new persistent memory and alter no paths, strengths, returns, or owners.
- Samples, stale transitions, mismatched owners, answered returns, ambiguous
  candidates, repeated returns, and nonlocal outputs remain controls.

## Verification

- Focused tiny competition tests cover transition versus sample, unique versus
  ambiguous continuation, exact return lifetime, reflection, and old protocols.
- The unchanged hand must remove the first opposing coactivation, reach and leave
  both limits, recover from perturbation, replay exactly, quiesce naturally, and
  exhaust no propagation budget.
- The representative warm suite remains strictly under ten seconds.
