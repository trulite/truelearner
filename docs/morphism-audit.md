# Physical Morphism Audit

This audit treats a learned capability as a diagram of witnessed physical
arrows. It does not add `Shape`, `Plan`, `Goal`, or benchmark meanings to the
organism.

## Representation levels

| Level | Physical meaning | Current evidence |
| --- | --- | --- |
| Object | A junction or body/world interface | Stable IDs, attachment ports, checkpoint replay |
| Arrow | Propagation, action, returned consequence, or membership | `Link`, `Path`, exact `Return`, `Membership` |
| Weight | Delay, drive, work, strength | Link delay and impulse, normalized current drive, `Work`, closure strength |
| Path | Adjacent arrows with one real intermediate | `composition_laws` |
| Motif | A closed path or small branch history retained through its physical support | `automaticity_laws`; renamed switch-then-close composition |
| Recursive motif | A retained motif that re-enters the same composition law | `recursive_automaticity_laws` |
| Diagram | Product, branch, loop, or symmetry formed by arrows | Product, invalidation, trace, and renaming laws |
| Quotient | Histories that remain interchangeable under returned consequences | Exact-return/unique-reentry membership; third-instance transfer remains a frontier |

## Active laws

- Quiet is the identity and repeated stepping composes.
- A sampled junction distinguishes a repeat from a real transition.
- `RisesThrough` and `FallsThrough` make threshold crossings distinct physical
  arrows, while learned sampled paths retain identity-free `Rises` or `Falls`
  direction.
- Shifted absolute baselines with the same positive delta produce the same
  normalized drive and the same learned behavior.
- Only an exact returned consequence strengthens and supports a path;
  ambiguity and wrong ancestry do not.
- Separately learned steps compose only through their real returned
  intermediate.
- Three exact closures can retain a supported composite without changing its
  external effect or losing the sampled direction of its parent path.
- Retained composites recursively re-enter the same law.
- A visible branch or changed parent invalidates a shortcut before it fires.
- Independent simultaneous skills remain a product.
- Construction order, attachment renaming, checkpoints, and trace projection
  preserve the relevant structure.
- An exact return merely simultaneous with an unrelated replay creates no
  shared membership.
- An exact return that is itself the condition of one unique reentry composes
  the two participating surfaces through the existing membership law.
- Two renamed histories of “unanswered sibling, then exact closure” retain one
  sparse return-to-return composition without becoming causal members or
  changing choice.

## Closed handoffs

### 1. Signed delta retention

The event's `before` and `after` now select `Rises` or `Falls` on a sampled
path's existing physical entry link. Later candidate formation checks that
trigger, and any retained shortcut copies it. No semantic shape object or hot
memory field was added.

Active tests:

`motif_laws::a_closed_rise_motif_does_not_reuse_for_an_equal_magnitude_fall`

`motif_laws::a_shortcut_preserves_the_sampled_direction_of_its_closed_path`

### 2. Caused cross-instance membership

When the source of one exact accepted return is also the condition of the one
selected unique reentry, their two witnessed surfaces are passed to the
existing membership constructor. Unrelated, missing, passive, and ambiguous
returns remain negative controls.

Active test:

`planning_goal_laws::an_exact_return_that_is_the_reentry_condition_forms_shared_causal_membership`

### 3. Renamed branch-motif composition

When an action follows one exact, still-unanswered sibling, its temporary
return link remembers that predecessor. If the later exact closure has the
same link form as a previously closed history, the retired return witness
points to that earlier retired return witness. This uses cold fields on those
links: it adds no `Shape` object, hot memory field, membership claim, or choice
preference.

Active tests cover one-example insufficiency, renamed identity, independent
construction, changed path form, reversed experience, passive and ambiguous
returns, disconnected products, checkpoint replay, and attachment remapping.

## Gated downstream ladder

These claims remain downstream rather than being implemented or inferred:

1. Let a retained renamed motif transiently reenter a fresh local candidate
   without constructing or strengthening it.
2. Confirm or reject that tentative reuse through the fresh returned
   consequence.
3. Refine or split caused membership or motif resemblance when a return separates their
   members.
4. Admit a third causal instance only after those two lower laws survive.

The existing ignored third-instance test now fails at fresh motif reentry, not
at motif formation.
