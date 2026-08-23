# Developmental Harness v0 implementation plan

Status: implementation plan; no Harness v0 scientific evidence has been
spent.

H0 update: extraction PROBE v1 is frozen negative. The authoritative cumulative
mechanism has no mechanically extractable continuous anonymous-arrival path;
its blank construction, update, and transient lifecycle remain fixture-bound.
I0 continuous-integration PROBE v1 is also frozen negative (`I0-C`): the
mechanisms do not consume the retained physical substrate and require
experiment-specific structured inputs that an anonymous bus/scheduler cannot
derive without a new representation. H1 and all world work remain blocked
pending separately authorized mechanism-physicalization or representation
research. See
[the H0 negative](../results/h0_frozen_organism_extraction_probe_v1_negative.md)
and [the I0 negative](../results/i0_continuous_physical_integration_probe_v1_negative.md).

## Fixed scientific boundary

The starting organism is Frozen Organism v1: authoritative M8 plus the
independent SSA0.3 deterministic causal-window law. SSA1 Classification C,
SSA1-C2, SSA1-S2/S3, and SSA2-P remain frozen observations and limitations.

This program creates no M9, DS9, or new cognitive mechanism. A developmental
failure first changes experience, world richness, contrast, curriculum order,
teacher policy, developmental duration, capacity, or internal work. A proposed
organism change stops this program and requires separately authorized
architecture research.

## Cleanup gate before harness construction

The historical root crate is a scientific archive, not a production
dependency. Cleanup proceeds without rewriting authoritative mechanisms:

1. Keep all authority tags, protocols, results, positives, and negatives
   immutable.
2. Isolate retained CELL/ARROW/SPIKE execution physics in the dependency-free
   `frozen-organism-v1-physics` crate.
3. Mechanically extract the cumulative M0--M8 learner into a separate
   `frozen-organism-v1` crate. First extraction preserves the frozen mechanism
   structure; deduplication happens only after fingerprint equivalence.
4. Remove development matrices, fixed seeds, report rendering, evaluator
   semantics, and definitive runners from the active learner build. They stay
   in the archive and conformance package.
5. Expose a narrow continuous physical interface: anonymous external arrivals,
   deterministic advancement until quiescence or a work bound, outward
   physical crossings, capacity configuration, and snapshot/restore.
6. Do not expose an episode reset or semantic task boundary to the harness.
   Transient clearing must occur only through the frozen organism's ordinary
   dynamics.
7. Put internal fingerprints and structure inspection behind an evaluator-only
   observer surface. The teacher and world crates must be unable to import it.
8. Replay the M0--M8 development fingerprints and SSA0.3 law after every
   extraction/refactor step. Never invoke a spent definitive runner.

The standalone retained-physics crate exists now. The complete reusable blank
M0--M8 host is the next cleanup deliverable and is a prerequisite for world
episodes. A bare substrate is not to be mistaken for the complete organism.

## Compile-time architecture

Harness v0 will be a workspace of narrow crates with one-way dependencies:

```text
harness-external-types
    ↑          ↑
  world      teacher
    ↑          ↑
    └──── runner ──── organism-host ──── frozen-organism-v1
             │                                  │
             ├──── social-actor                 └─ frozen-organism-v1-physics
             │
             └──── append-only observation log ───→ evaluator ───→ UI
```

Rules enforced by manifests and audits:

- `teacher` depends only on external world/behavior evidence and opaque family
  identifiers. It cannot depend on `frozen-organism-v1`, its observer API, the
  evaluator, or UI.
- `world` implements simulator physics and receives only outward physical
  emissions chosen by the organism-host boundary adapter.
- `organism-host` translates physical world activity to anonymous boundary
  arrivals and crossings back to world activity. It contains no lesson or
  capability semantics.
- `evaluator` may read the append-only external log and evaluator-only organism
  snapshots. It returns no value to the runner, teacher, world, or organism.
- `UI` is a read-only projection of teacher-visible and evaluator-only streams.
  Its controls may pause, resume, checkpoint, or choose a predeclared run; they
  may not alter an active teacher decision.

Mechanical separation tests:

1. Cargo dependency audit rejects teacher-to-organism/evaluator/UI edges.
2. Source audit rejects internal-state types and semantic outcome fields from
   teacher inputs.
3. Null-evaluator replay must produce an exact teacher schedule, world trace,
   organism input stream, and organism emission stream.
4. Mutating evaluator labels, metrics, and UI state must leave those same
   causal traces exact.
5. Teacher schedule replay from its external evidence log must be exact.

## Milestones

### H0 — frozen organism host

Deliver a genuinely blank reusable M0--M8 instance with:

- fresh physical namespace and capacity;
- continuous anonymous arrival stream;
- outward boundary-crossing stream;
- deterministic work-bounded advancement;
- exact snapshot/restore and complete/permanent fingerprints;
- evaluator-only structure and activity snapshots;
- conformance to every frozen cumulative fingerprint and SSA0.3.

Stop if extraction requires a new representation, changes a frozen mechanism,
or needs a semantic episode/start/finish path.

### H1 — deterministic world and causal loop

Implement the closed loop:

```text
world state
→ physical experience stream
→ Frozen Organism v1
→ outward physical emission stream
→ world transition
→ next physical experience
```

World physics initially supports anonymous objects, containment, adjacency,
movement, contact, occlusion, ordering, grouping, quantity, delay, actors,
actor actions/emissions, and persistent external devices. These names exist
only in simulator/evaluator code. Organism input is event/activity structure,
never relation labels.

Start with exact replay, fresh-identity, layout-permutation, occlusion, stale
object, blocked action, and persistent-device controls.

### H2 — compositional experience space

Build a small set of parameterized relation composers rather than lesson
files. Approximately 12 clean generators should yield roughly 20 initial
family configurations through composition, including containment/movement,
containment/delay, containment/occlusion, quantity/grouping, ordering/movement,
actor/object movement, actor emission/object state, delayed actor consequence,
persistent device/retrieval, two-actor signaling, and distractor-rich
persistence.

Every generated situation varies fresh identities, layout, timing, ordering,
distractors, appearance, actors, delay, number, and nesting where lawful. No
stored question bank or target answer is allowed.

### H3 — contrast and social signaling

Add two first-class generator operations:

- invariance variation: preserve a physical relation while changing lawful
  surface facts;
- relational contrast: preserve surfaces where possible while changing one
  physical relation.

Add a physical social actor whose byte emissions are ordinary world events.
Begin with arbitrary grounded byte sequences, then allow familiar words and
short phrases without token labels, language loss, parser output, or target
completion. Support actor emission → organism emission → actor/world response
and record a live conversation transcript as observational evidence only.

### H4 — external-only teacher

Teacher v0 stores, per opaque family, only external behavioral evidence:
repeatability, fresh-identity transfer, surface/delay/distractor robustness,
contrast sensitivity, composition evidence, recent exposure, and rate of
behavioral change.

It uses a soft advisory family graph and chooses mostly uncertain/reachable
neighbors, with smaller consolidation, composition, contradictory, and broad
nearby samples. Nothing is hard-unlocked. Avoid `MASTERED`, capability labels,
`CORRECT`, `WRONG`, internal confidence, or direct organism state.

Implement T3 through T0 as separate scheduler policies:

- T3 selects family and contrast form;
- T2 selects a frontier neighborhood;
- T1 selects only a broad world region;
- T0 supplies environment dynamics without pedagogical family selection.

Scheduler proportions remain configurable development parameters, not claims.

### H5 — evaluator, frontier, and live UI

The evaluator shows world state, physical input/emission streams, teacher
choice and external evidence, developmental history, permanent structure,
temporary activity, affordance multiplicity, internal fingerprints, work
accounting, and conversation transcript.

The UI visibly separates teacher-visible evidence from evaluator-only
internals. A broad fixed bank of fresh generated families/compositions measures
the unaided learnable frontier at checkpoints. Frontier results are written to
the evaluator log only and have no direct scheduler path.

### H6 — 3--5-organism harness development

Run only 3--5 genuinely blank instances while debugging Harness v0. Use
PROBE/MICRO/GATE discipline for harness mechanics and freeze every meaningful
positive and negative. Allowed changes are world design, distributions,
curriculum order, contrast, teacher policy, duration, capacity, and internal
work bounds—not organism architecture.

Harness v0 reaches development-freeze readiness only if repeated evidence
shows all ten required properties:

1. some fresh-identity transfer;
2. robustness increases under generated variation;
3. at least one delayed relation;
4. at least one novel relation composition;
5. useful persistent external state/device interaction;
6. grounded social-signal behavior;
7. measurable teacher weakening on earlier families;
8. teacher/evaluator causal separation;
9. exact frozen-organism fingerprints;
10. no new cognitive architecture.

If these do not emerge, freeze a Harness v0 developmental negative. Do not
repair the organism.

### H7 — separate authority readiness, then 20 childhoods

Only after Harness v0 development freezes may a separate authority workflow
preregister the first 20-organism experiment. It measures capability growth,
transfer, delay, composition, permanent structure, learning/inference work,
curriculum path, cross-history functional and structural variance, affordance
multiplicity, hysteresis, teacher dependence, and unaided frontier.

The target claim is developmental, not a named-capability count:

> A fixed blank Frozen Organism v1 repeatedly develops increasingly
> transferable and compositional behavior under different generated
> childhoods, while pedagogical dependence decreases and the experience
> frontier from which it can learn expands.

Call convergence a developmental attractor only if the data supports it.
Freeze permanent curriculum divergence if that is what occurs.

## Immediate implementation order

1. Finish H0 extraction and exact conformance packaging.
2. Land the workspace dependency/audit skeleton before implementing world
   semantics.
3. Build H1 with two or three relations and exact replay controls.
4. Add generators and contrasts incrementally; prove that every behavior is
   mediated by organism emissions rather than harness shortcuts.
5. Add the teacher only after external evidence can be computed without it.
6. Add evaluator/UI through the append-only, causally inert observation path.
7. Begin 3--5-organism development only after all separation audits pass.

No definitive Harness v0 evidence is authorized by this plan.
