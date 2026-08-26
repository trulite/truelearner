# CORE1 adoption code-consolidation result v1

## Decision

The adoption gate passes.

The earned path laws are ordinary CORE1 defaults, the original E14 behavioral
frontier is closed, and learned paths fire on a later encounter.
No ARC task was run.

Baseline: `3cc6aca98fbfc147bf0ea9d08efd723c55b55e01`.

## Adopted physical laws

The normal execution path in `truelearner/crates/core/src/lib.rs` now expresses:

1. A firing input and a local opening form a complete temporary path in the
   same event.
2. Opposite paths are choices, and an output cell holds their signals until the
   current wave finishes.
3. Using a path opens a physical return path, and its later outcome closes it.
4. A successful outcome strengthens both used links so a later input can reuse
   the path.

PQLC's update rule was not changed. Links strengthen only after a successful
existing update.

There are no evaluator enable calls, candidate flags, pending-path metadata,
post-hoc return construction, action identities, episode identities, timeouts,
benchmark names, ARC assumptions, or Academy-specific policy behavior in the
CORE1 implementation.

Academy now supplies only the physical body and the outcome source.
The historical E15-E27 binaries are excluded from Cargo target discovery. The
retained adoption gate invokes no experimental arm.

## Focused regression gate

| Check | Result |
| --- | --- |
| Default and CORE1 strict clippy, all targets | pass |
| Experiment package strict clippy, all retained targets | pass |
| Form and use a complete path | pass |
| Choose between opposite paths | pass |
| Hold and clear output input | pass |
| Return outcome, apply PQLC, close return, and strengthen used path | pass |
| Five-context learned action | pass, `1|4|2|3` |
| Reference replay | exact |
| Reference versus Production | exact |
| Natural quiescence | pass |

Focused test names:

- `paths_compete_and_fire_output`
- `stronger_path_is_chosen`
- `output_holds_then_clears_input`
- `outcome_closes_return_and_strengthens_used_path`
- `core1_defaults_form_and_use_a_complete_path`
- `core1_outcome_strengthens_paths_for_later_reuse`

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
the complete physical path has two used links and correctly produces
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

`input -> use path -> return outcome -> PQLC -> reuse path`.

The code-consolidation gate is therefore complete. ARC remains the next separate
evidence phase.
