# ARC3 A2 eligibility-aware pressure successor protocol v1

Status: frozen before implementation or execution.

## Question

Should ordinary periodic pressure distinguish dormant structure from an ARROW
that actually traversed and is still inside its existing physical eligibility
window?

The current law applies ordinary pressure to every live ARROW and later applies
the existing unsupported-use pressure when unmodulated eligibility expires.
The ARC3 phase-9 negative showed a resistance-1 route dying under the first
pressure while its caused outward effect and lawful return were still in
flight.

## Candidate law

Change only ordinary pressure:

```text
ordinary pressure epoch
    live ARROW with live eligibility -> no ordinary decrement
    dormant/noneligible live ARROW   -> ordinary decrement as before

eligibility expires without modulation
    -> existing unsupported-use pressure applies unchanged
```

No timer, first-effect deadline, return label, curriculum exception, proposal
strength change, or new persistent state is permitted. The rejected
`first_effect_due` candidate is excluded from this successor.

## Permanent paired regressions

Each physical world is defined once and executed under both:

- `MechanicalConfig::REFERENCE`, the permanent slow oracle;
- `MechanicalConfig::PRODUCTION`, the selected physical organism mechanics.

Both executions must independently satisfy the behavioral predicate and match
on crossings, Drive and Modulatory deliveries, plasticity updates, proposals,
deallocations, clock and pressure phase, canonical durable body, pending
activity where observed, natural quiescence, and exact replay.

Focused worlds:

1. A resistance-1 ARROW traverses immediately before an ordinary pressure
   epoch. Its eligibility remains live. It must survive that epoch.
2. Modulation within the eligibility window must strengthen exactly once and
   consume eligibility.
3. Without modulation, eligibility expiry must invoke the existing
   unsupported-use pressure and remove the resistance-1 ARROW.
4. A noneligible resistance-1 ARROW must still die at ordinary pressure.
5. Modulation after expiry must produce zero update.

## Academy integration

Run the official A2-only `ls20`, seed 205, `[1,4,2,3]` curriculum with the
continuous-admission schedule at initial pressure phases 0 and 9. Each phase
is run independently with reference and production mechanics and exact replay.

Require in both phases and both mechanics:

- actions `[1,4,2,3]` occur through ordinary physical crossings;
- all four official rasters change;
- qualified update sequence is `[0,1,1,1,1]` including the final return;
- all transitions quiesce naturally;
- reference and production observations are physically identical.

## Decision

- Any focused failure freezes this successor as negative before Academy.
- Any reference/production divergence freezes a mechanical-equivalence
  negative, even if one implementation satisfies the behavior.
- Phase 0 pass plus phase 9 fail freezes a phase-dependent negative.
- Only all focused and both Academy phase rows passing permits broader retained
  PXR0/PX-C and physical-body regression replay. No oracle or `arch.md` update
  occurs before that replay passes.
