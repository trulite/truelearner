# Core Learning Mechanism Tests

This matrix records the unit- and crate-level physical laws that support each
learning mechanism. The names are shorthand for observable behavior, not new
objects inside the learner.

Every mechanism needs four kinds of evidence:

1. a positive case;
2. a counterexample that must not learn or act;
3. an invariance, replay, or transfer case;
4. an integration case in which its result becomes ordinary input to another
   law in the same body.

## Evidence matrix

| Mechanism | Positive law | Counterexample | Invariance / integration |
| --- | --- | --- | --- |
| Persistent state | `integrating_memory_holds_subthreshold_input`; `sampled_memory_preserves_a_short_change_across_u32_wrap` | `expired_sensor_memory_does_not_invent_a_transition`; `sampled_memory_forgets_across_a_whole_u32_epoch` | `checkpoint_restores_the_exact_next_wave`; `recursive_automaticity_survives_checkpoint_and_continuous_time` |
| Causal credit | `a_later_consequence_credits_only_the_action_that_physically_happened`; `one_completed_physical_cycle_strengthens_its_participating_action` | `preopening_and_repeated_samples_close_nothing_and_credit_nothing`; `an_ambiguous_return_does_not_strengthen_a_path`; `topological_but_wrong_cause_returns_do_not_earn_automaticity` | `a_return_matches_only_the_component_that_returned_it`; `independent_simultaneous_skills_consolidate_as_a_product` |
| Intervention | `directional_links_turn_sensor_change_into_distinct_actions`; `physical_exploration_returns_only_actual_changed_axes` | `a_surface_without_a_nearby_output_changes_no_later_action`; `distant_action_is_rejected_on_the_compact_body` | `a_previously_successful_action_without_a_new_return_releases_to_an_alternative`; black-box changed-contingency replay |
| Hypothesis formation | `a_local_surface_forms_one_reusable_choice_without_duplicate_growth`; `learner_construction_requires_new_physical_membership` | `formation_is_local_and_does_not_cross_distance_three`; `an_outward_effect_does_not_form_a_reentry_choice` | `connected_choice_is_independent_of_construction_order`; reversed-construction and dormant-subsystem behavior contracts |
| Composition | `separately_learned_steps_compose_through_one_physical_intermediate`; `retained_links_reenter_the_same_law_and_form_a_recursive_hierarchy` | `an_untrained_second_step_is_not_invented`; `a_wrong_cause_cannot_close_a_step_while_starting_the_next`; `automatic_internal_paths_do_not_skip_the_real_world_intermediate` | `trace_composition_has_identity_and_associativity`; `independent_simultaneous_skills_consolidate_as_a_product` |
| Prediction | `latest_available_consequence_precedes_old_strength`; `a_returned_consequence_is_available_until_one_choice_then_consumed` | `ambiguous_current_returns_do_not_create_a_false_preference`; a consumed old success cannot suppress a newer unanswered intervention | `offline_verifier_checks_unanswered_output_release`; changed-contingency relearning and exact checkpoint replay |
| Consolidation | `three_exact_closed_uses_make_the_same_effect_with_less_internal_work`; `retained_links_reenter_the_same_law_and_form_a_recursive_hierarchy` | `wrong_cause_returns_never_consolidate_a_transparent_chain`; `ambiguous_returns_preserve_the_full_parent_path`; `a_changed_leaf_invalidates_every_dependent_level_before_it_fires` | `formation_cost_is_finite_and_reuse_can_amortize_it`; checkpoint and attachment remapping laws |
| Reuse | `learned_action_is_reused_on_the_compact_body`; `repeated_closed_workstation_experience_compacts_and_transfers` | `changing_a_parent_invalidates_the_stale_composite_before_it_fires`; `a_new_effect_of_the_omitted_middle_forces_the_full_path`; workstation no-return control | changed luminance, exact replay, attachment, recursive reuse, and lower-work probes |

Here, “prediction” has a deliberately narrow physical meaning: a returned
consequence changes which later path persists or is selected. The body does not
yet emit a symbolic forecast or a probability distribution.

## Required regression runs

The compact mechanism suite is:

```text
cargo test -p truelearner-body
cargo test -p truelearner-behavior-contract
```

The composition and transfer evidence additionally requires:

```text
cargo test -p academy-body --test automaticity_transfer
cargo test -p academy-body --test body_course
```

Goal discovery is intentionally absent. Passing this matrix establishes strong
local and compositional learner physics; it does not establish that the body can
invent, rank, or pursue its own goals.

## Planning and goal-discovery unit-test frontier

These claims begin inside `truelearner-body` as deterministic unit tests. They
do not begin as Academy lessons or benchmark scores.

The active prerequisites are:

- `separately_closed_steps_compose_only_after_the_real_intermediate_returns`:
  learned steps reactively compose, but the world crossing remains explicit;
- `one_unique_self_caused_closure_changes_later_choice_on_the_same_surface`:
  unique closure can persist through later selection;
- `a_passive_closure_sample_creates_no_later_action_preference`: an external
  state change without organism ancestry creates no goal-like preference.

The first unsupported transitions are frozen as ignored frontier tests:

- `an_open_condition_selects_the_learned_route_that_can_close_it`: the same
  already-participating first-step paths must choose the route whose learned
  continuation closes a recurring physical condition; the dead branch has an
  ordinary immediate consequence but leaves that condition open;
- `a_discovered_closure_transfers_to_a_fresh_equivalent_surface_and_output`:
  a closure relation must transfer across fresh surface and output identities
  that participate in the same physical outcome component.

Run the active unit suite with:

```text
cargo test -p truelearner-body --lib
```

Run either unsupported frontier explicitly with `--ignored --exact`. Frontier
tests are not marked `should_panic`: when learner physics eventually satisfies
one, it must pass as an ordinary assertion and can be promoted into the active
suite. Replanning, closure refinement, and combined discovered-goal planning
come only after these first two arrows commute.
