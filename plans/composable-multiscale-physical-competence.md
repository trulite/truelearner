# Runtime-attached multiscale physical competence

```text
sensor tissue --attach at physical site--> body
     |                                      |
     +-- fires --> paths form --> action --> real sensory return
                                      |              |
                                      +-- outcome ---+
```

## Outcome

Let a running body accept a new sensor as anonymous physical tissue without
rebuilding the body or manually wiring that sensor to an action. The sensor owns
its receptors, short-lived physical trace, internal links, and input ports. The
body owns the attachment site and, after attachment, all resident physical
state. Ordinary input, path formation, output choice, actual body effects,
returned sensation, outcome, and reuse must discover which actions, if any, are
related to the sensor.

Use the same attachment operation for visual, tactile, proprioceptive, acoustic,
binocular, and held-out future sensor shapes. Then test a Levin-aligned ladder:
local regulation, same outcome by different means, adaptation, composition of
locally competent parts, and whole-body competence. Physical sensor memory is a
supporting mechanism, not the cognitive claim. No sensor type, motor map,
`CognitiveSystem`, goal, target, desired action, or evaluator result enters the
organism.

## Authority

- Project path: `arch.md`; `LANGUAGE.md`; `research/constitution.md`;
  `research/programs/learner/program.toml`; learner lessons `LP-002`, `LP-021`,
  `LP-026`, `LP-030`, `LP-032`, `LP-037`, `LP-049`, `LP-058`, `LP-059`, and
  `LP-065` through `LP-071`; the sensorimotor synthesis,
  participation-continuity, focused-receptor, binocular-alignment, and
  stable-fixation convergences; `plans/composable-embodiment-drivers.md`;
  `plans/composable-active-sensing-effects.md`;
  `plans/composable-perception-action-loop.md`.
- Scientific framing: Michael Levin, *Technological Approach to Mind
  Everywhere* (`https://arxiv.org/abs/2201.10346`) and *The Multiscale Wisdom
  of the Body* (`https://doi.org/10.1002/bies.202400196`). These motivate graded
  empirical competence and multiscale composition; they do not authorize
  project physics.
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`; `LANGUAGE.md` SHA-256
  `2b1954b161358c4a259198b0b9e4c66a93e47350d749d7c3baf3ddcef7bb8a41`;
  `arch.md` SHA-256
  `02d837a8dc205aae7b088147226c94aa08783898a653550334718bbdf0cc003f`.

## Model

The runtime operation is an attachment, not a sensor-to-motor connection.

`PhysicalComponentSpec` is a closed description of one component's relative
junctions, internal links, and exposed input-port indices. It contains no global
junction IDs, existing body IDs, output IDs, outcome sources, device nouns, or
callbacks. `AttachmentSite` supplies only the body's physical position and
region. `Harness::attach_physical` validates the complete graft, translates
relative positions to the site, allocates fresh physical identities, installs
the component atomically at the current physical tick, and returns an opaque
`PhysicalAttachment` whose ports can receive ordinary `PhysicalInput`.

Attachment is allowed only while the body is naturally quiet. Invalid local
references, non-positive thresholds, negative delay, arithmetic overflow,
duplicate ports, capacity exhaustion, and any attempted link to existing body
structure return `AttachError` and leave the Harness byte-for-byte unchanged.
The attached topology and its state enter the existing opaque core checkpoint;
the attachment handle contains only stable public port identities needed to
route later physical input. Restore must reproduce those ports and subsequent
behavior exactly.

The body does not receive a motor map. A sensor port is placed at its physical
attachment site. When that port or an internal sensor junction fires, the
existing local formation law may create ordinary paths to physically reachable
body outputs. Actual outputs change the body or world. When the new sensor
observes a change caused by one of those outputs, existing causal lineage carries
that output's physical cause through the returned input. Actual participation
then supplies consequence and reuse. If no body action ever affects the sensor,
no action relation is invented.

```text
attach sensor -> admit anonymous activity -> local paths form
      -> ordinary action -> sensor observes actual change
      -> cause returns -> participating path changes -> later reuse
```

The sensor's short-term memory is ordinary attached tissue. The embodiment
builder creates a bounded threshold-trace subgraph using the same junction,
link, signed coupling, phase, refractory, decay, and physical-time machinery as
the body. A first sample initializes it; equal resampling is quiet; crossed
thresholds emit sparse rise or fall activity; rewrite follows comparison; and
decay causes honest forgetting. No adapter keeps a previous value and no new
core memory field is added.

The categorical shape remains plain. A component and a body are open physical
processes. Attachment joins compatible physical boundaries. Serial composition
passes activity onward; parallel composition preserves independent components;
feedback closes an action through its actual sensory consequence. Identity is
no attachment, no input, or no physical effect. Reordering and mirroring must
only reorder or mirror the same physical behavior.

Competence is an observed property, never a Rust type. A local loop qualifies
only when it returns to and holds one externally measured physical region after
disturbances from both sides. Stronger competence requires a different live path
to regain the same region after the used path is blocked. Collective competence
requires attached local units to maintain a relation no single unit receives,
and removal of their coupling to break that relation while local controls remain
interpretable.

Initial attachment is monotonic, matching bodily development: a graft becomes
body tissue and is not surgically deleted by this change. Temporary absence is
represented honestly by unavailable sensing or by sending no event; its trace
decays naturally. Reconnection uses the same handle. Replacement attaches fresh
tissue and leaves old tissue dormant, so execution must follow the active
frontier rather than accumulated resident anatomy. Destructive topology removal,
learner cleanup, identity reuse, and compaction are a separate successor only if
evidence shows monotonic attachment is insufficient.

Evidence status is explicit:

- **Established:** runtime input into fixed receptors, ordinary dynamic path
  formation, actual effect return, recursive learner construction around
  controllable surfaces, product composition, focused fields, bounded binocular
  alignment, and bounded stable fixation in their declared fixtures.
- **Constrained:** current `HarnessBuilder` is pre-run only; `Harness` exposes no
  runtime topology transaction; external `ChangeDetector` cannot count as body
  memory; dense focused wiring cost 2.6 MB; forward action-to-surface association
  did not imply reverse execution.
- **Predicted:** one atomic graft operation plus one physical trace subgraph is
  sufficient for an unknown sensor to enter existing formation and outcome
  physics without a motor map.
- **Unknown:** automatic regulation after attachment, alternative-route
  recovery, order-independent multisensor assimilation, replacement transfer,
  collective repair, goal-relation rewriting, natural-image reach, and keyboard
  use.
- **Retired:** prewiring each sensor to an actuator; every-sensor by
  every-actuator products; sensor-specific controllers; external previous-value
  memory; semantic setpoints; evaluator-selected attachment effects; and calling
  attachment, memory, learner divergence, or movement alone cognitive.

## Invariants

- The caller chooses a physical attachment site, not an action, desired result,
  learner, path, or output.
- A component may reference only its own relative junctions. It cannot inspect
  or name resident body junctions, links, learners, outputs, or outcomes.
- Attachment changes topology atomically at natural quiescence. Failure changes
  neither body state, clock, physical identities, capacities, nor checkpoint
  bytes.
- Fresh physical identities are body-owned, durable, generation-safe, and
  independent of resident storage slots.
- Attachment itself emits no input, output, outcome, strength change, learner
  construction, or physical-time advance.
- The first sensor event enters through an attachment port as ordinary anonymous
  physical incidence. It does not receive special formation or ranking.
- All sensor memory that affects behavior lives in attached checkpointed
  junction or link state. Adapters retain no previous measured value.
- No fixed attachment link crosses into an existing motor or learner. Useful
  sensor-action relations arise only through ordinary formation and real
  participation.
- An action-caused sensory change preserves the real action origin; unchanged
  resampling cannot create, refresh, or erase an action outcome.
- Silence or unavailability creates no fabricated transition. Reconnection after
  trace expiry initializes rather than comparing with stale state.
- Independent attachments do not share trace, suppress one another, or create a
  dormant cross-product. Registration order may change private IDs but not
  externally observable behavior for symmetric physical layouts.
- Resident bytes may grow with attached tissue; warm execution cost must follow
  active ports, live traces, and touched paths rather than dormant attachments.
- Exact checkpoint replay preserves attachments, port routing, local trace,
  learned paths, outputs, work, and later behavior.
- A competence claim requires a closed loop and declared perturbations. Memory,
  attachment, path formation, learner change, or movement alone is insufficient.
- Default core construction, accepted body, accepted hand, accepted workstation
  opportunity, Production/reference equality, natural quiescence, and semantic
  firewall remain unchanged.

## Scope

- `truelearner/crates/core/src/core.rs` and a small attachment module: neutral
  component specification, site, handle, typed errors, atomic quiescent Harness
  attachment, checkpoint round-trip, and no new propagation or learning law.
- `truelearner/crates/core/src/snapshot.rs` and checkpoint validation only as
  required to preserve runtime-attached topology; prefer existing arena snapshot
  representation and avoid a schema field when topology already carries the
  complete fact.
- `truelearner/crates/embodiment/src/lib.rs`: lagom builders for anonymous sensor
  components and physical threshold-trace tissue; no new driver framework.
- `truelearner/crates/embodiment/tests/runtime_attachment.rs`: attachment laws,
  trace laws, modality transfer, order, reconnection, replacement, checkpoint,
  quiet, and cost.
- `truelearner/crates/workstation/` and Academy: conditional real sensor grafts
  after generic attachment and assimilation gates pass.
- `research/campaigns/runtime-attached-multiscale-physical-competence-v1/`: one
  frozen staged campaign, retained causal traces, controls, and convergence.
- `factory/receipts/`: candidate and independent verification receipts for code;
  receipts establish no competence or authority.
- Excludes sensor-to-motor maps, new learner law, a `CognitiveSystem` trait,
  semantic sensor or body-part IDs, external history, callbacks into the body,
  adaptive evaluator wiring, destructive detach, identity reuse, compaction,
  object or glyph detectors, desired actions, authority promotion, and Production
  adoption.

## Development style

TDD. First write atomic attachment, checkpoint, symmetry, dormant-cost, and
physical-trace tests. Implement one neutral Harness transaction and one
embodiment component builder. Stop if a new component cannot enter existing path
formation without naming a resident motor. Only after the generic laws pass,
author the complete staged competence campaign with lossless diagnostics enabled
on its first run.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core runtime_attachment`
  proves quiescent atomic grafting, invalid-spec rollback, capacity rollback,
  fresh identity, ordinary first incidence, checkpoint restore, and unchanged
  default construction.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment attachment_`
  proves a component cannot name body structure, attaches at runtime after prior
  learning, forms no fixed motor link, reconnects honestly, and preserves
  symmetric behavior under registration order.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment trace_`
  proves internal initialization, equal resampling, rise, fall, rewrite, expiry,
  reconnection, quiet, and exact replay.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment modality_`
  attaches the identical construction for luminance, local contrast, pressure,
  slip, depth, sound-band energy, position, velocity, effort, availability, and
  a held-out odd-shaped field.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment composition_`
  proves independent sensors, mirrored sites, different attach orders,
  simultaneous modalities, dormant replacement, and fixed-active cost without a
  cross-product.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment`
  preserves existing driver, spatial-field, active-effect, interaction, and
  wiring laws.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation --features research --test focused_vision_body`
  conditionally proves a sensor attached after the body is already running can
  alter ordinary paths through actual eye-caused return while default body state
  remains unchanged.
- `cargo fmt --all --manifest-path truelearner/Cargo.toml -- --check`,
  `cargo check --locked --manifest-path truelearner/Cargo.toml`, and
  `cargo clippy --locked --manifest-path truelearner/Cargo.toml --all-targets --all-features -- -D warnings`
  enforce workspace gates.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core runtime_attachment && cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment`.
Its measured warm duration must remain strictly under 10 seconds. Record cold
bootstrap, conditional workstation integration, and research campaign time
separately.

## Controls and evidence

The complete upward ladder is:

1. **Attachment identity.** Attach no component, then attach one silent component
   to an already experienced body. The body clock, prior paths, strengths,
   learners, and outputs remain unchanged; only declared resident tissue grows.
   Killing falsifier: attachment itself behaves like input, learning, or action.
2. **Internal continuity.** Send samples through attached physical trace tissue.
   It initializes, detects only actual threshold crossings, rewrites, forgets,
   replays, and quiesces without adapter history. Killing falsifier: it needs a
   modality-specific comparator or new core memory primitive.
3. **Automatic assimilation.** Fire the new sensor at a real physical site. The
   unchanged formation law creates only bounded local alternatives; no fixed link
   reaches an existing motor. Actual action-caused return can later make a path
   consequential and reusable. Killing falsifier: the fixture supplies a motor
   mapping, no path can form, or every actuator must be scanned or wired.
4. **Closed local regulation.** Real visual, tactile, and proprioceptive grafts
   face disturbances from both sides and return to one externally measured
   bounded region through different physical outputs, then hold by identity or
   learned quiet. Killing falsifier: a fixed open-loop command sequence performs
   equally or the region is admitted to the body.
5. **History and alternate means.** From the same current sample, actual prior
   consequence changes later reuse. Block the used route without naming a
   replacement; a distinct participating path must regain the same region from
   novel disturbances. Killing falsifier: current input alone decides behavior,
   stale history persists, or the fixture selects the alternate.
6. **Unknown-sensor transfer.** Attach visual, touch, depth, sound, proprioceptive,
   availability, and held-out field shapes using the same operation and local
   trace construction. Killing falsifier: any sensor needs a private controller,
   global ID, motor link, goal field, or attach branch.
7. **Runtime plurality.** Attach two sensors in both orders after different body
   histories; silence and reconnect one; attach a replacement while the old one
   remains dormant. Active behavior remains local and execution cost follows
   active tissue. Killing falsifier: registration order changes symmetric
   behavior, reconnection fabricates change, histories leak, or work follows all
   accumulated sensors.
8. **Collective competence.** Couple already competent attached parts so the
   whole maintains a relation unavailable to any one part: binocular alignment
   and fixation, then multi-digit surface contact. Perturb one member while
   retaining interpretable local controls. Killing falsifier: local successes are
   merely counted as a collective result or removing coupling changes nothing.
9. **Scale expansion and plasticity.** Mirror, reorder, silence, or replace one
   participating unit and change the world's physical consequence relation.
   Whole-system consequence must reach only participating local paths; the body
   must find another composition and later release the old relation. Killing
   falsifier: repair requires semantic body identity, evaluator reassignment,
   broad coactivation, direct goal writing, or stale credit.
10. **Whole body and keyboard.** Only after one through nine pass, compose natural
    images, clutter, occlusion, binocular gaze, visible hand, proprioception,
    touch, depth-directed reach, proper-key contact, visible consequence, and
    release under separate frozen capability protocols.

One lossless trace is retained from the first run of every launched rung:

```text
attach transaction -> input port -> local trace -> formed paths -> choice
  -> output -> actual effect -> returned sensor change -> outcome -> later reuse
```

On failure, descend this sequence once and stop at the first missing arrow. A
diagnostic fixture may supply a prior physical condition but may not supply the
action, replacement route, stable-region identity, coupling result, or verdict.

Held-out cases include attachment after learning, minimal remaining capacity,
invalid relative references, odd component shape, opposite site mirrors,
irregular time, expiry around its exact bound, low noise, unavailable sensing,
reconnection, replacement, two simultaneous sensors, blocked route, actuator
saturation, changed surface stiffness, changed lighting, one unavailable eye,
one immobilized digit, clutter, partial occlusion, and large dormant attachment
sets with fixed active input.

Negative controls are no attachment, silent attachment, failing atomic attach,
equal resampling, removed trace, external previous-value comparison, direct
sensor-to-motor link, shuffled action origin, missing physical return, supplied
stable region, selected alternative, shared trace between attachments, collapsed
binocular placement, removed collective coupling, evaluator-directed repair,
default body equality, semantic firewall, replay, quiescence, and
Production/reference equality.

Only after the complete candidate passes through its highest launched rung,
freeze it and run downward ablations. Separately remove runtime attachment,
internal trace, actual-effect return, causal origin, path change, alternate
route, inter-unit coupling, identity effect, and active-frontier indexing.
Restore the positive reference between removals and rerun the complete upward
ladder. A removal supports necessity only when the intended competence fails
while substrate controls remain interpretable; a surviving removal indicates
redundancy.

The strongest inherited counterexamples remain constraints: action-to-surface
association failed to execute in reverse; a moving one-joint loop reached neither
limit; dense focused receptors changed learner state but did not prove behavior
and cost 2.6 MB; centered vision did not close hand reach; binocular alignment
departed until the common effect boundary became identity; and dormant topology
once made work rise from 78 to 3,138. Reproducing any one stops the ladder there.

Expected evidence is one compact artifact per completed rung, one convergence,
one candidate and one independent verification receipt for code, and no
competence or authority claim from software tests alone. The post-convergence
frontier is the earliest predicted transition that fails or, after a complete
survivor, the first downward removal that distinguishes necessity from
redundancy.

## Risks and rollback

Runtime topology mutation could bypass normal development, corrupt identity,
partially apply at capacity, change tie-breaking, or make checkpoints depend on
attachment order. Preflight plus clone-and-commit, relative-only specs, fresh
body-owned identities, symmetric order controls, and exact restore are killing
gates. If core topology already serializes completely, do not add duplicate
attachment state to the checkpoint.

The existing local-radius formation law may be too narrow to assimilate an
attached port or may still scan too much dormant topology. That failure
localizes the first real attachment boundary; it does not authorize a motor map,
global cross-product, semantic port, or larger search. Existing activation may
also fail the trace truth table; preserve that primitive failure rather than add
external history.

Monotonic replacement consumes resident capacity even when execution stays
local. Bound attachment count and resident bytes in this experiment. If later
use requires true removal, plan generation-safe incident-link retirement,
learner and causal-closure cleanup, handle invalidation, and compaction as a
separate lifecycle change with its own checkpoint proof.

Rollback removes the unadopted Harness attachment transaction, component and
trace builders, tests, conditional body mode, and campaign while retaining
existing driver infrastructure, accepted body behavior, frozen negative
evidence, and all prior authority.

## Open decisions

None.
