# CORE1 adoption code-consolidation result v1

## Decision

The adoption gate passes.

The earned route laws are ordinary CORE1 defaults, the original E14 behavioral
frontier is closed, and learned routes execute autonomously on a later encounter.
No ARC task was run.

Baseline: `3cc6aca98fbfc147bf0ea9d08efd723c55b55e01`.

## Adopted physical laws

The normal execution path in `truelearner/crates/core/src/lib.rs` now expresses:

1. Co-occurring source activity and local opportunity form a complete temporary
   route in the same event.
2. Opposite-signed generated routes compete locally, while motor incidence is
   held and integrated over its causal wave.
3. Route participation immediately creates its temporary physical consequence
   return, and delivered consequence consumes that return.
4. Successful local credit both preserves the complete used route and makes it
   executable by ordinary later source activity.

PQLC's update rule was not changed. Route consolidation occurs only after a
successful existing update.

There are no evaluator enable calls, candidate flags, pending-route metadata,
post-hoc return construction, action identities, episode identities, timeouts,
benchmark names, ARC assumptions, or Academy-specific policy behavior in the
CORE1 implementation.

Academy now supplies only the physical body and the consequence-return endpoint.
The historical E15-E27 binaries are excluded from Cargo target discovery. The
retained adoption gate invokes no experimental arm.

## Focused regression gate

| Check | Result |
| --- | --- |
| Default and CORE1 strict clippy, all targets | pass |
| Experiment package strict clippy, all retained targets | pass |
| Atomic route formation plus motor participation | pass |
| Signed local competition | pass |
| Causal-wave motor integration and cleanup | pass |
| Delayed consequence return, PQLC, return cleanup, and consolidation | pass |
| Five-context ordinary autonomous re-expression | pass, `1|4|2|3` |
| Reference replay | exact |
| Reference versus Production | exact |
| Natural quiescence | pass |

Focused test names:

- `generated_routes_compete_and_integrate_at_the_motor`
- `local_competition_admits_the_physically_stronger_alternative`
- `motor_integration_holds_staggered_incidence_and_clears`
- `consequence_consumes_return_and_consolidates_the_used_route`
- `core1_defaults_form_and_participate_in_a_complete_route`
- `core1_consequence_makes_used_routes_autonomously_executable`

## Unchanged E14 plus autonomous revisit

The frozen E14 teaching and closing observations ran unchanged. A source-only
revisit was then applied to the resulting organism without an enable call,
teaching shortcut, or policy lookup.

| Observable | Result |
| --- | --- |
| Teaching actions | `1|4|2|3|none` |
| Autonomous revisit | `1|4|2|3` |
| PQLC updates | `0|2|2|2|2` |
| Modulatory deliveries | `0|1|1|1|1` |
| Live temporary returns | `1|1|1|1|0` |
| Behavioral frontier closed | `true` |
| Legacy one-update predicate | `false` |
| Replay / Production / quiescence | exact / exact / true |

The legacy predicate is stale: it expects one update per learned action, while
the complete physical route has two participating arrows and correctly produces
two updates.

Evidence is in
`experiments/results/core1_adoption_gate_e14_v1/report.md` and
`experiments/results/core1_adoption_gate_e14_v1/matrix.csv`.

## Legacy contract classification

All failures below reproduce at the frozen baseline. No new failure was found.

| Suite or contract | Failure | Classification |
| --- | --- | --- |
| Frozen E14 acceptance predicate | Expects `0|1|1|1|1`, observes the correct two-arrow `0|2|2|2|2` | stale expectation |
| CORE1 library | `quiescent_checkpoint_preserves_clock_phase_and_future_behavior` | unsupported CORE1 lifetime-test boundary |
| CORE1 library | `reused_identity_rejects_stale_generation` | unsupported CORE1 lifetime-test boundary |
| Academy default | `changed_raster_supports_one_motor_and_probe_needs_no_babble` | unsupported legacy learner boundary |
| Academy default | `action_meaning_follows_the_external_map` | unsupported legacy learner boundary |
| Academy default | `spatial_body_learns_distinct_contexts_without_changing_a1_sensor` | unsupported legacy learner boundary |
| Academy CORE1 | The same three failures above | unsupported legacy learner boundary |
| Academy CORE1 | `four_context_pressure_regimen_matches_reference_and_physical_organism` | unsupported Profile A boundary |

Counts:

- CORE1 library: `17/19` pass, with the same two failures at baseline.
- Academy default: `5/8` pass, with the same three failures at baseline.
- Academy CORE1: `6/10` pass; its four legacy failures match the baseline
  CORE1 suite, and both newly adopted-law tests pass.
- Real regressions: `0`.

## Adoption conclusion

Removing the experiment arms does not change the adopted behavior. The active
implementation reads as four local physical laws rather than a staged experiment:

`source -> participation -> consequence return -> PQLC -> executable route`.

The code-consolidation gate is therefore complete. ARC remains the next separate
evidence phase.
