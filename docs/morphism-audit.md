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
| Quotient | Histories that remain interchangeable under returned consequences | Exact-return membership and tentative third-instance motif reentry |

## Active laws

- Quiet is the identity and repeated stepping composes.
- A sampled junction distinguishes a repeat from a real transition.
- `RisesThrough` and `FallsThrough` make threshold crossings distinct physical
  arrows, while learned sampled paths retain identity-free `Rises` or `Falls`
  direction.
- Shifted absolute baselines with the same positive delta produce the same
  normalized drive and the same learned behavior.
- Only an exact returned consequence retains closed causal support;
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
  immediately changing choice.
- A retained two-example motif can transiently support one fresh candidate
  when both its successful and unanswered physical roles match. Several
  matching histories add support without selecting one historical parent;
  several supported fresh candidates make no motif choice.
- When several fresh first actions have equal motif support, their actual
  unique outcome-source incidence may expose further untouched morphology.
  Retained motifs can compose through those physical intermediate surfaces;
  exactly one route to a currently present condition may select its first
  action. Two reaching routes make no claim.
- Tentative motif support constructs no additional topology and strengthens
  nothing. A later exact return independently closes the fresh path; wrong
  ancestry and ambiguity retain no closed support.
- A valid retained composite consumes no unfamiliar-depth lifetime during
  transient reentry. Its exact witnessed step remains in the receipt, cycles
  and the incidence ceiling still fail closed, and inspection itself forms no
  shortcut.
- Three identical, complete reentry inspections can retain a dependent thought
  shortcut. Reuse returns the same complete route set and exact step receipts;
  it does not add a closure, strengthen a path, or create graph topology.
- Every successfully inspected suffix is rehearsed independently. The same
  exact suffix reached through different beginnings can therefore compile and
  be reused, but it cannot supply a missing beginning.
- A thought shortcut is valid only while every locally inspected physical
  dependency has the same epoch. A new branch or changed witness invalidates
  only the shortcuts that depended on it. Unchanged downstream suffixes,
  disconnected products, checkpoint replay, and attachment renaming preserve
  their compiled structure. Repeated real confirmation of the same support
  does not invalidate it.

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

### 4. Tentative fresh motif reentry

On a fresh surface, the resolver compares only physical link form, surface
law, and the presence or absence of unique outcome incidence. Two retained
examples are required. Matching witnesses are attached only to the transient
candidate receipt, and a `Reentry` warrant is available only when exactly one
fresh candidate is supported. The receipt does not create, strengthen, or
repair retained topology.

Active tests cover one-example insufficiency, changed path and outcome form,
identity and construction order, several disconnected matching witnesses,
checkpoint replay, observer verification, exact confirmation, wrong ancestry,
and ambiguous returns.

### 5. Experience-compressed foresight

Reentry has one fixed unfamiliar-step lifetime. An ordinary retained causal
step consumes one unit; a step with an existing valid automatic composite
consumes none. This does not omit the returned source or outcome witness from
the reentry receipt. A stale composite falls back to its detailed parents, a
changed intermediate consequence stops the route, and the independent
256-incidence ceiling remains absolute.

Active tests cover one-step and whole-route depth extension, unconfirmed
inspection, detailed fallback, changed consequences, identity and independent
construction, and the incidence safety ceiling.

### 6. Dependent thought compilation

Reentry now distinguishes causal evidence from derived computation. Exact
returned consequences remain the only source of closed support. Each complete
inspection rehearses both its full route set and every successfully inspected
suffix. After three rehearsals, any one of those exact compositions may become
a `ThoughtShortcut` indexed by its physical start path and present condition.
Later reuse emits the unchanged full reentry receipt, so observer verification
sees every path, returned source, and outcome witness.

Because suffixes have their own dependency sets, an unchanged causal tail can
accumulate its three rehearsals while it is reached through different
beginnings. A later real beginning that reaches that tail may reuse it. The
shortcut never creates that beginning: a path with no witnessed closure still
finds no future. Changing one beginning invalidates its dependent prefix
without erasing the unchanged tail.

The shortcut stores the local junction epochs inspected by the original
search. Adding a possible branch, changing or retiring a parent, changing an
outcome witness, or changing relevant membership makes the shortcut unusable.
The complete route set is stored, including several routes, so compilation
cannot turn ambiguity into a unique planning claim. Direct epoch lookup and
deterministic shortcut indexing keep disconnected components out of local
graph work.

Active tests cover the three-use threshold, ordinary reaction integration,
shared-tail learning across different beginnings, missing-prefix rejection,
suffix-local invalidation, unchanged causal evidence and topology, lower graph
inspection work, ambiguity, membership changes, new-branch and changed-support
invalidation, repeated identical real support, checkpoint replay, attachment
remapping, observer verification, and products containing many disconnected
compiled thoughts. Checkpoint format 8 persists this derived state; the
explicit version-7 reader restores older trained bodies with an empty thought
cache.

### 7. Transient identity-free motif routes

A fresh action's existing outcome witness names the physical surface on which
its consequence could return. When that surface has untouched local motor
morphology, the resolver can compare each possible continuation with the same
two-example motif law already used for one-step generalization. A matched
continuation may compose through its own unique outcome source. This repeats
until one route reaches a boundary condition that is physically present in the
current moment.

The route is a transient receipt, not retained planning topology. Every
proposed step records its real morphology link, proposed impulse, unique
outcome source, and retained motif witnesses. Only the first real action is
sent; the next action still waits for the real intermediate consequence. No
downstream path, return, closure, or strength is created. Sampled
downstream surfaces are currently rejected because their future rise or fall
has not yet been physically witnessed; integrating `SourceFires` surfaces do
not require that invented direction.

Route search shares the existing 256-incidence safety budget and the
16-unfamiliar-step lifetime. One downstream example is insufficient,
ambiguous outcome incidence forms no route, and two routes reaching the
present condition block a unique motif choice. Tests also cover identity and
construction-order invariance, unchanged downstream topology, checkpoint
replay, attachment remapping, independent products, and observer-side receipt
validation.

## Gated downstream ladder

These claims remain downstream rather than being implemented or inferred:

1. Refine or split caused membership or motif resemblance when an exact return
   separates histories previously treated as the same form.
2. Test whether repeated real confirmation can compile a previously transient
   motif route without allowing the proposal itself to create evidence.
3. Test goal discovery separately: a condition must become physically
   maintainable through experience rather than being named by a goal object.
4. Project frozen motif, motif-route, and dependent-thought receipts into
   observer-only Lean claims without giving the result back to the learner.
