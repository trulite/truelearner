# Academy

```text
Academy Body / Workstation -> WorkstationHarness -> truelearner_body::Body
          |                         |
          +---- teach, probe -------+
                record, compare
```

## Purpose

Academy teaches and measures development through one production organism path.
TrueLearner remains the physical learner described by [arch.md](arch.md).

Academy may use meaning. TrueLearner may not receive capability names, expected
answers, correctness bits, loss, reward, or evaluator state.

## Ownership

- **TrueLearner** owns input, junctions, links, paths, output, outcome, strength,
  physical time, and durable body state.
- **Academy** owns teaching cases, probes, capability claims, schedules,
  evidence, and developmental records.
- **Review tools** own observer-only rendering and inspection.

Keep the runtime dependency direction:

```text
academy-body ----------> academy-workstation-course -> academy-workstation
      |                                                   |
      +---------------------------------------------------+--> truelearner-workstation -> truelearner-body
academy-workstation-review -> academy-workstation
academy-formal -------------------> truelearner-workstation -> truelearner-body
       |
       +--------------------------> pinned Lean checker (frozen evidence only)
```

The removed core, embodiment, semantic harness, episode runner, and gallery are
historical only and do not participate in this graph.

## Development loop

For each external capability claim:

1. Generate teaching experiences.
2. Generate fresh probes independently.
3. Add transfer and negative controls.
4. Admit each experience as ordinary physical input.
5. Let the organism run to natural quiescence.
6. Record input, output, outcome, body identity, physical work, and replay data.
7. Change the external capability state only from recorded evidence.

Separate teaching, probing, and transfer. A probe must not teach unless its
protocol says so. Regenerate identities, positions, timing, context, and
distractors so fixture memory cannot pass.

## Formal checks

Academy may submit a frozen, versioned causal trace to the pinned Lean checker
after an episode. Lean checks whether the claimed closure resolution follows
from the explicit causal ancestry in that trace and returns an immutable
receipt. A theorem or receipt is observer evidence, never organism input.

```text
headless run -> frozen evidence -> capability receipt
                    |
                    +-----------> Lean receipt
                    |
                    +-----------> optional render
```

The checker may accept or falsify an Academy claim. It cannot create missing
causal ancestry, choose an action, alter a checkpoint, or return any result to
the learner. The ordinary Academy and production paths therefore do not depend
on Lean at runtime.

The closed-return projector accepts only a complete frozen chain: verified
choices, the physical output transition, the accepted return naming its
resolved path, both path-link strengthening events, and natural quiet. The body
trace also retains every live contender of an ambiguous return. Academy can
project ambiguity only when every contender has its own earlier output event in
trace order and none was strengthened. Lean then proves that the competing
explanations persist nothing. Explicit ancestry can order an output and return
inside one body tick; equal timestamps alone still establish no causal relation.

## Capability evidence

Treat each capability as an external claim with prerequisites and falsifiers.
Measure:

- acquisition cost;
- generalization to fresh worlds;
- retention after unrelated experience;
- automaticity through lower physical cost;
- robustness to noise, timing, and context;
- reversibility after a changed relation;
- durable body growth.

Use these evidence states:

| State | Meaning |
|---|---|
| Unknown | no useful evidence |
| Emerging | works inside teaching |
| Acquired | passes fresh same-structure probes |
| General | passes changed worlds |
| Stable | survives spacing and interference |
| Automatic | stays stable while physical cost falls |

These names never enter TrueLearner.

## Scheduling

- Teach near the frontier between stable paths and genuine novelty.
- Reopen prerequisites when probes fail.
- Space retention by intervening experience or physical work, not wall time.
- Interleave capabilities to prevent order memory.
- Begin with deterministic scheduling.
- Do not add a learned curriculum policy until evidence requires one.

Track whether comparable new capabilities require fewer experiences and less
physical work. This is the main learning-to-learn measure.

## Body discovery

Before interface or benchmark curricula, the headless Body Discovery curriculum
lets the learner discover the visual-touch body exposed by
`WorkstationHarness`: two independently movable foveated eyes and one
five-finger hand with palm and fingertip touch plus signed position, velocity,
opposing effort, and joint-limit input.
Development experience may commit learning. Fresh probes, transfers, retention
checks, and controls run from cloned checkpoints and discard mutation.

Academy arranges only light and contact worlds. It never chooses or injects a
gaze, hand, finger, direction, pressure, or action output. If ordinary sensory
experience produces no unguided outward exploration, the course preserves
`MissingExploration` as its first failure instead of teaching an action route.
Development may include a visible external demonstrator acting on a separate
world object. The demonstrator never moves the learner's body. Its events carry
external causal ancestry, cannot satisfy a learner capability claim, and are
absent from fresh probes.
Opposing outputs are combined by the body's fixed force law, and Academy counts
only the resulting net pose change as movement.
`GazeContingency` additionally requires repeated net gaze changes that change
light sampled at the before/after focus in the same recorded world. One
accidental movement, passive light change, or visually inert movement cannot
establish the capability.

Body Discovery is split into four courses:

1. Eye Control: gaze contingency, gaze control, and binocular depth.
2. Hand and Finger Control: hand contingency and independent digits.
3. Eye-Hand Coordination: distinguishing and coordinating self and world.
4. Workstation Contact: visual reach, touch/withdraw, tap/hold/release,
   contact drag, thumb contact, pinch drag, and later keyboard, touchpad, and
   monitor interaction.

Body Discovery experiences contain twelve ordinary world steps by default.
`Contact` contains sixteen because its observed physical displacement cannot
reach the external surface in twelve; this is an Academy exposure horizon, not
an action, direction, or success signal supplied to the organism. Extending all
capabilities to sixteen is rejected because it changes earlier binocular
evidence. `ContactDrag` contains thirty-two steps so the same uninterrupted
physical contact can include motion and its eventual release; the learner sees
neither the horizon nor the verdict. For deterministic reference seed `31_001`,
the current schema-14 course acquires eight bounded Body Discovery claims.
`VisualReach` is acquired; `TapHoldRelease` is the first failure, and its
prerequisite-gated successors are not reached. Acquisition is not treated as
the end of the evidence ladder: changed-world transfer and retention are
recorded separately when those lessons are reached.
That lesson retains the course's compact visual field while using the physical
workstation's key geometry, contact threshold, press/release hysteresis, and
device events. A key held for two physical steps now produces one visible
`LongPressActivated` world event. The capability requires an organism-caused
press, that held consequence, and release of the same key. Contact, depth
motion, or a demonstrated event alone cannot pass. Exact replay regenerates the
sensory samples, causal event origins, device events, and world fingerprint.

The TapHoldRelease lesson separates demonstration, imitation, boundary
diagnosis, shaped practice, and transfer. First, a separate external finger
visibly presses, holds until the consequence appears, and releases; an unaided
normal-key control then tests observational imitation. If that control fails,
Academy restores the exact pre-demonstration checkpoint and runs a causally
inert press-depth ladder. Every rung clones that same checkpoint, uses the same
world seed and fixed release depth `608`, changes only the press depth through
`640, 656, 672, 688, 704, 720`, and stops at the first missing organism-caused
press. A rung records evidence but cannot mutate the durable learner or satisfy
the capability. Academy then restores the same lesson checkpoint for
self-caused practice on the `640/608` key, followed by a fresh normal-key probe
at `720/660`.

For reference seed `31_001`, all six frozen depth controls now pass from the
same pre-practice checkpoint. The ladder shows the same retained palm-depth
increase composing through contact at `640, 656, 672, 688, 704, 720`; its seven
steps are exposure time, not a desired-action hint. Self-caused practice on the
`640/608` key closes press, holds through the distinct long-press event, and
releases. After repositioning only the external body pose to the frozen lesson
pose, a fresh normal `720/660` probe performs the same sequence and replays
exactly. The demonstration remains insufficient by itself, the immediate
imitation control remains failed, and every capability event must still have
organism ancestry.

The decisive separation is now explicit. Ordinary proprioceptive movement may
close an ordinary movement return without claiming that the world boundary is
finished. Fresh contact progress continues only when the world supplies the
exact motor parent and the retained path's boundary instance remains open. A
press or release event closes that boundary instance, clears transient action
recency, and inhibits the completed output. Morphological competition then
releases only inside the same actuator's ordinary outcome component, so
palm-depth increase yields to palm-depth decrease instead of an unrelated
finger. If ancestry or locality is ambiguous, no continuation or local release
claim is made.

### TapHoldRelease implementation contract

This is the accepted TapHoldRelease mechanism for the current Academy evidence.
Preserve the complete boundary square:

```text
motor crossing ---------> world consequence
      |                          |
      | opens witness            | retains causal parents
      v                          v
open return path -------> closed / ambiguous / no claim
```

Implement and test it in this order:

1. Let the world retain zero, one, or several exact motor-crossing parents for
   both boundary consequences and contact progress. A demonstrator has external
   ancestry and no organism parent. Time adjacency alone supplies no parent.
2. Return only the opaque crossing and cause through the workstation boundary.
   Do not expose device event, key identity, threshold, capability, direction
   target, or verdict to the organism.
3. Close one live boundary instance only when exactly one witnessed crossing
   explains the consequence. Missing ancestry makes no claim; several live
   explanations are ambiguous and strengthen nothing. Ordinary movement
   closure does not mark the boundary instance closed.
4. Before an `Exploration` warrant can release competition, continue only one
   executable, retained, boundary-open path carrying fresh world-witnessed
   progress from its own motor cause. Recompute this condition at every choice;
   do not store a permanent continue command.
5. Boundary closure clears transient outcome selection and inhibits the exact
   completed output. The next release is confined to the same ordinary outcome
   component; an unrelated finger or eye cannot win that local reversal.
6. Remove continuation after closure, absent progress, ambiguity, physical
   limit, or expiry. Commanded movement with no world-witnessed consequence is
   no progress.

Keep implementation ownership separated:

- `academy-workstation` derives boundary and progress parents from one physical
  before/after world transition;
- `truelearner-workstation` maps returned opaque parents to existing boundary,
  progress, and ordinary outcome incidence;
- `truelearner-body` resolves instance closure, retained progress, and local
  antagonist release;
- `academy-body` owns only the paired discriminator and capability evidence;
- `academy-formal` projects explicit frozen arrows, and `formal/closure` proves
  the claimed closure resolution; Rust's frozen verifier checks the choice
  contract.

The paired discriminator remains mandatory. With the same checkpoint, a deeper
world must compose the progressing palm path until its own boundary closes,
while a shallower world must return press ancestry, close that instance, and
release to the local antagonist. Core controls establish retained open progress,
no progress, boundary-closed progress, ambiguous progress ancestry, local
release, and simultaneous-component ambiguity before the normal-depth lesson.

Category theory specifies the composition boundary: objects are crossings,
world effects, witnesses, and resolutions; arrows are act, affect, return,
close, and continue. Identity is natural quiet, independent body/world parts
compose as products, and renaming physical identities must not change
resolution. Rust must record the actual arrows. The frozen-trace projector must
reject an absent parent rather than manufacture one. Lean proves that one
explanation closes exactly one witness and several explanations persist
nothing; Rust's frozen choice verifier checks retained progress and local
release. Lean remains observer-only and its receipt never enters the learner or
the hot path.

### ContactDrag, ThumbContact, and PinchDrag implementation contract

The former `DragOpposition` label is three separate falsifiable claims:

1. `ContactDrag`: organism-caused lateral motion during contact moves the
   cursor. A cursor change is ongoing progress, not boundary completion. When
   the drag ends, the terminal event closes only its exact current lateral
   crossing; a stale earlier crossing is not acceptable.
2. `ThumbContact`: a real thumb-opposition crossing changes the thumb from no
   contact to contact. Passive contact, another digit, palm motion, missing
   ancestry, or several possible parents cannot establish the claim.
3. `PinchDrag`: thumb and another digit must jointly maintain contact while an
   object moves. The object's thumb and finger surfaces translate only when both
   contacted tips undergo the same nonzero horizontal, vertical, or depth
   displacement. Squeezing changes their separation and therefore cannot move
   the object. An organism-caused object move counts only with one exact
   palm-transport parent and maintained joint contact in the adjoining samples.

The Academy world supplies two distinct physical surfaces. A rigid horizontal
pad produces maximal support pressure; workstation morphology converts that
pressure into an equal and opposite palm-depth reaction, canceling only inward
effort. It cannot pull the hand outward or stop lateral motion. A light vertical
patch begins immediately beside the current thumb, is insensitive to palm
depth, and produces contact only after the thumb crosses into it. The pinch
object begins with a selected non-thumb digit already on its own firm surface
and the thumb immediately outside its light surface. After the thumb makes real
contact, coherent hand transport moves the two surfaces and the visible object
together. The thumb and its flexion form one local competition component; the
rest of the hand remains independent. This is morphology, not a desired
movement or reward.

Planar palm transport and palm depth are distinct workstation competition
components with distinct contact incidence: pressure is normal to depth, while
slip is tangential to planar transport. An external clock exposes one component
surface at a time without selecting an action or direction. This factorization
lets the reference learner discover a real lateral reach without weakening the
body's same-surface locality law. Thumb opposition and thumb flexion remain one
local component.

Development and probes begin from the same frozen external pose while retaining
only development's learned topology. All three claims must pass a fresh
immutable probe, exact replay, choice-law verification, natural quiet, and
negative controls. Transfer changes only an external relation: ContactDrag uses
a smaller shifted pad, ThumbContact a smaller side patch, and PinchDrag a
different ordinary grip digit. After later committed body experience, retention
restores only the external pose and probes the still-current learner. Lean
checks the already-general unique/ambiguous closure law on frozen Rust evidence;
no theorem or receipt enters the learner.

ContactDrag retention also records a one-step external lateral displacement
before the immutable probe. The pre-setup checkpoint remains the durable
reference, replay applies the same displacement, and the displacement changes
only the physical pose: it does not change learned topology, causal time, or
history. Because it is not learner output, it receives no motor parent or
credit. The retention claim still requires a later organism-caused lateral
crossing, cursor progress with that exact parent, and terminal drag closure.

The receipt reports an explicit evidence state rather than collapsing every
passing lesson into one claim: `Acquired` means development plus a fresh probe,
`General` additionally requires changed-world transfer, and `Stable`
additionally requires retention after subsequent committed learning.
`Automatic` requires a separate repeated low-cost-use contract. In the
current reference run, eight body claims are acquired and TapHoldRelease is the
first failure. Its manipulation successors remain gated. A workstation branch
can emit a separate `RepeatedUseEvidenceState::Automatic` receipt only after
the body reaches that branch; it does not silently upgrade any body capability.

### Repeated-use automaticity successor

Automaticity is not merely stronger reuse. It requires the same externally
observable physical result with less internal physical work after repeated
closed use. The compact body now establishes the generated evidence ladder in
constructed worlds: three exact closed uses retain one transparent pair, that
retained link re-enters the same law to form an eight-link hierarchy, and a
changed-light workstation probe traverses a retained link while preserving its
two outward effects and reducing the action wave from `640` to `635` work
units. Formation, closure maintenance, and reuse are counted separately; the
eight-link fixture reaches a finite break-even use count and then holds steady
without further retained growth. The generated body mechanism evidence remains
separate from Academy capability claims.

The reference Body Course now exercises that law through the ordinary generic
workstation boundary. Seven uniquely returned key-to-screen uses form one
observed screen-closed composite, and a later normal-depth probe is observed
traversing it. The exact device-event and returned-parent timeline is unchanged;
physical work falls from `9,658` to `9,486` units after a passive intervening
screen disturbance. Formation costs two automaticity-work units, so the
measured break-even is one later use. Demonstration and action-without-screen
controls form no such claim. The passive disturbance has no returned screen
ancestry, creates no composite, changes no automaticity work, and the retained
link is still traversed afterward. Exact replay and checkpoint-discarded probing
remain mandatory. This supports the separate repeated-use receipt, not general
typing, pointing, application understanding, or spontaneous action.

Historical experiments constrain the successor:

- RC0b earned one role-relative motif after three successful uses, removed
  genuine relay and route firings, preserved the complete declared trace, and
  fell back when a parent or formerly transparent context changed.
- FFS0 let a learned pair become an ordinary participant in the same pairing
  law. It produced useful recursive execution depth `0 -> 3 -> 5 -> >=6` as
  reusable workload depth increased. Its own audit still marked learning,
  retrieval, and decision as unavailable through that common process.
- RE0 showed that lower execution work is not yet lower total work. Depending
  on depth and seed, the old compiled hierarchy needed roughly `1,177` to
  `90,803` later uses to repay acquisition; shallow compiled runs could cost
  more than their concrete parents.
- G2c1 formed reusable temporal chunks and predictive groups, but supplied a
  boundary rule, maximum chunk length, downstream grouping law, and a
  discovery/probation split.
- CORE1 E26 retained re-entry topology that remained behaviorally inert. E27
  passed only when the retained route also carried enough ordinary signed
  efficacy to cross its existing local thresholds.
- FD0, FD1, and CC0 separated traversal, forgetting, and consolidation: use
  alone did not earn persistence; a local qualified consequence did. The
  compact body now preserves silent probation, removes it only when a later
  exact closure exposes invalid local support, and demonstrates bounded steady
  reuse. General finite-resource forgetting remains a separate question.
- RI0 found a real numeric-identity and insertion-order leak in an older
  simultaneous scheduler. Fresh identities, resident-layout shifts, and
  construction-order products therefore remain mandatory controls rather than
  cosmetic variants.
- H0 and I0 showed that separately positive mechanisms were not yet one blank,
  continuously connected organism. Experiment-built structs, episode
  boundaries, terminal supervision, and opaque Rust mutation remained between
  them.

These are historical constraints, not current behavior or authority. The new
contract must combine their surviving lessons with the current body's exact
physical return law:

1. Only repeated uniquely attributed returned closure may retain a cheaper
   route. Wrong, missing, ambiguous, failed, or shuffled return evidence retains
   none.
2. A cheaper route may remove only internally transparent propagation. It may
   not skip a world crossing, ordered external effect, current returned cause,
   or boundary closure.
3. Before consolidation, behavior and work remain the ordinary parent route.
   After consolidation, the complete external trace and physical timing remain
   equal while measured body work falls.
4. A changed parent or newly observable intermediate effect invalidates the
   cheaper route before it fires and exposes the nearest valid parent route.
   Returning to a compatible context may reuse still-retained history.
5. A retained composite counts as an ordinary physical participant only when
   it reaches the same anonymous interface as its parents. No level, macro,
   routine, task, or capability tag may enter the organism.
6. Independent components consolidate independently, and renaming junctions,
   links, insertion order, or resident layout changes no result.
7. Checkpoint restart preserves the exact retained structure, fallback, work,
   replay, and natural quiet.
8. Formation and retention cost are reported separately from cheaper reuse. A
   lower-cost execution is not an `Automatic` capability until its acquisition
   cost is physically counted and amortized under fresh workloads.

The generated evidence ladder is:

```text
exact repeated path
  -> lower-work reuse
  -> context invalidation and parent fallback
  -> two independently learned paths compose through a real world event
  -> retained composites participate again through the same physical law
  -> overlapping, branching, and independent composites
  -> continuous retrieval and action without episode or semantic control
  -> changed-world workstation transfer
```

The compact body now has direct tests at every rung of this ladder. Retained
links carry no level or task tag; the same adjacent-pair law forms two-, four-,
and eight-link compositions. A visible branch, changed parent, insufficient
support, or pending input at an omitted interior prevents use before the stale
composition fires. The real motor crossing and a separately returned world
intermediate remain explicit. Independent simultaneous components can share
one cause and still consolidate locally through their participated boundary
links, with construction-order equality. Checkpoint restart and attachment
preserve parent identities. Wrong-cause, ambiguous, missing-return, and
untrained-continuation controls retain nothing. A heterogeneous
`1,3,2,4,1,2,3,1` delay chain preserves the same nineteen-tick terminal time
before and after recursive compaction.

The workstation transfer is deliberately smaller than a body-course receipt.
Seven closed generic workstation experiences produce two retained paths. A
fresh changed-luminance probe restored from the developed checkpoint traverses
one retained link, reproduces the same unordered motor effects with the same
relative timing, uses less work, and exactly replays. Repeating the same
physical input without returning the motor parent forms no composite. No task,
action label, expected answer, or evaluator verdict enters the harness.

Each rung uses fresh identities, layouts, timings, delays, and distractors.
Subthreshold, wrong-cause, ambiguous, missing-return, stale-parent,
same-endpoint/missing-effect, gapped-chain, and disconnected-product controls
are mandatory. Stop at the first rung whose required physical transition is
absent; do not let a later application score hide it.

Eye and hand foundations may develop independently. Coordination and
manipulation are not reached until their actual capability prerequisites have
been acquired. A failure closes only its course, so evidence can preserve both
an eye frontier and a hand frontier without teaching around either one. Failed
lesson evidence is preserved, but its unacquired development checkpoint is not
carried into another independent course.

The initial binocular claim is deliberately small. Academy presents one target
at equal-and-opposite horizontal disparities in the two eye fields, with larger
disparity for a nearer external depth band. The capability requires repeated
coordinated horizontal movement of the independent eyes with a recorded light
consequence in both eyes. There is no vergence motor or fused depth input. The
learner receives only separate pixels and proprioception: target, depth,
disparity, and verdict stay outside the harness. Passing does not establish
distance estimation, depth-directed reaching, or general 3D understanding.

Every claim requires fresh generated evidence, exact replay, natural
quiescence, unchanged negative controls, and bounded physical work.

The headless `academy-workstation` world now supplies the next external surface:
a standard ANSI 104-key keyboard, continuous touchpad, monitor photograph,
visible cursor and text, binocular scene rendering, and physical collision. It
now has accepted unguided movement separation across all five digits, one
bounded tap/hold/release sequence, contact dragging, thumb contact, and one
two-contact object transport that transfers to a different grip digit and
survives later learning. The bounded body course is complete through the
defined manipulation stability rung. Repeated-use automaticity now has separate
constructed-body and generic-workstation mechanism evidence. The Body Course
emits a separate automatic repeated-use receipt for the workstation branch;
the twelve body capability states remain distinct. Richer workstation use also
remains a separate future contract. No general pointing, clicking, typing,
grasping, or image-use claim follows from these sequences.

### Generic workstation causality

`academy-workstation-course` asks one smaller question before any application
benchmark: can an organism-caused workstation device event be followed by a
changed monitor presentation through the same witnessed causal boundary? The
application frames are arbitrary generated luminance patterns. No application
identity, correct action, task rule, score, or semantic label enters the body.

The course keeps four separations explicit:

1. An external key-to-screen demonstration is visible but has no organism
   parent and cannot establish the claim.
2. A passive screen change cannot produce later device use.
3. Self-caused key motion without a screen response cannot produce later
   screen-directed use.
4. Only self-caused key motion followed by a changed screen may consolidate;
   the returned screen consequence substitutes the exact application parent,
   not every simultaneously moving body axis.

For the repeated-use receipt, the course additionally requires seven closed
uses, an observed screen-closed composite link, later traversal of that exact
link at normal key depth, equal external event/ancestry timing, lower physical
work with finite break-even, survival across an unrelated passive screen
disturbance, checkpoint retention, and exact replay. The disturbance may
coincide with body motion, but it must receive no motor ancestry, form no
composite, and change no automaticity work. This tests causal attribution rather
than demanding artificial bodily stillness.

The completed cycle is press, changed screen, release, returned release, and a
settling step with no new opportunity. Reference seed `31_001` learns this at a
shallower key depth and passes fresh normal-depth probes with new generated
frames. A one-step horizontal pose transfer fails, so the evidence is
`Acquired`, not `General`.

This lesson branches from the checkpoint immediately after TapHoldRelease,
where the keyboard path is still physically accessible. The ordinary body
course currently stops honestly at TapHoldRelease, before this branch is
available. When reached, Academy emits separate opaque body-course and taught
workstation artifacts rather than pretending they are one body. Recombining
those branches remains a future compositional-retention problem.

### Workstation1 is disabled by default

Workstation2 is the primary workstation path. `BodyCourse::run` and the
`academy-body-course` binary teach only eye control, hand and finger control,
and eye-hand coordination; the Workstation1 contact course and generic screen
course run only through `run_with_workstation_course` or `--with-workstation`.
`academy-workstation`, `academy-workstation-course`,
`academy-workstation-review`, and `academy-arc3` remain workspace members but
are outside the default build. Their tests in `academy-body` are ignored with a
stated reason rather than deleted. With the joint-stop boundary (lesson 111) the
Workstation1 course stops at VisualReach.

### Workstation2 touchscreen course

`academy-workstation2` is a separate tablet-like external world used to test a
smaller body boundary. One luminous surface is also touch-sensitive. A virtual
keyboard and scalable object are ordinary application pixels on that surface;
they are not body sensors, motors, outcomes, or task labels.

The only values entering `WorkstationHarness` are one `WorldSample` containing
retinal light, fingertip contact, and the harness's ordinary proprioception.
The session never calls a causal-parent API and never supplies a device event,
application state, touch identity, virtual key, expected action, or verdict to
the organism. It observes the body's public before/after physical pose, derives
generic touch-start/move/end events outside the organism, applies those events
to the external application, and renders the later application state back as
light.

```text
hand -> touchscreen contact -> generic touch event -> application
 ^                                                  |
 |                                                  v
 +---------------- eyes <- changed screen light ----+
```

The screen emits several independent contact tracks, so two contacts may
compose into a pinch while one contact cannot. Eye position selects a local
retinal view; the complete tablet is not injected as a fixed panoramic image.
The virtual keyboard may move during a fresh probe without changing the body.

`academy-workstation2-course` accepts an opaque body checkpoint. It records one
development phase, one shifted-keyboard probe, exact replay, natural quiet,
physical work, and the first unsupported rung among gaze, touch, virtual-key
use, and pinch. A fresh body is used only by named cold-control tests. A visible
event in development alone is `Emerging`; `Acquired` requires the corresponding
event in the shifted probe as well. The course reports failure rather than
moving a hand, selecting a direction, or injecting a touch.

Reference result after the joint-stop boundary (lesson 111): a fresh body
acquires all four rungs at `256` steps per phase; the body-course checkpoint
acquires gaze, touch, and virtual key at `96` steps and leaves pinch
`Emerging`. Key taps arrive by sweeping, not by aimed reaching.

## First curriculum

Start with a small set that can be inspected completely:

- interaction and turn taking;
- fresh symbol binding and replacement;
- sequence continuation;
- visual distinction and position change;
- composition of two learned relations;
- visible-context conversation;
- navigation across rendered document pages.

Expand only after prerequisites survive fresh probes and controls.

## Evidence rules

- Keep expected answers and capability state outside physical input.
- Do not accept self-report as evidence.
- Preserve every failed probe and negative control.
- Make each claim traceable to an independently replayable experience.
- Distinguish organism-visible frames, the shared world, and causally inert
  observer annotations.
- Keep UI timing and evaluator scheduling out of physical time.
- Persist body checkpoints and experience records across restart.

## V0

V0 must:

- accept text, images, files, and drawing through declared physical surfaces;
- show text and raster output;
- teach at least one fresh relationship;
- probe it with fresh identities, transfer, and negative controls;
- show capability evidence and the active frontier;
- record and replay the exact admitted experience;
- report physical work, body change, retention, and acquisition cost.

V0 does not require broad language, general document understanding, foveation,
distributed storage, robots, audio, video input, or a large ontology.

Stop if Academy supplies an answer, action, route, capability, or correctness
signal to the learner.

## ARC-AGI-3 development probe

ARC-AGI-3 is first used as an external falsification environment, not a
teaching episode or an official capability claim. Before the run, Academy pins
a clean body parent, adapter revision, SDK versions, seed, action budget, and
one named public development environment. Server-selected holdouts remain
uninspected.

The Python adapter owns the SDK, game identity, score, terminal state, action
budget, and official action catalog. Before the run it loads the opaque
`workstation-body-checkpoint-*` emitted by the generic workstation course. The
protocol rejects initialization as an ordinary body-course checkpoint. A fresh
body is a negative control and cannot support a post-course claim.

The Rust process owns `Arc3Sensorimotor`, which attaches that developed body to
an ordinary `WorkstationSession`. The 64×64 palette frame is converted to
distinct monitor luminances and rendered inside the workstation scene; it does
not replace the body's optics or enter as a benchmark-native sensor. The
workstation may take up to 32 ordinary physical steps while the external
application is paused. ARC receives a call only from an arrow-key, Space,
Escape, or touchpad-click `DeviceEvent`. The action catalog can reject that
external application input, but it cannot choose, suppress, or reinterpret an
internal movement.

The workstation retains the exact motor parent of each device event. The next
ARC frame is installed before the following workstation step, so its displayed
consequence composes through that retained physical return path. No
`BodyControl`, `MotorEffect`, or arbitrary crossing is visible to the ARC
adapter.

The process boundary exposes no score, game or level identity, expected action,
babbling, support, settling, reset, or diagnostic command. Unknown fields,
invalid frames, unsupported application actions, and corrupt workstation checkpoints
fail before a harness transition.
Academy records outer episode state separately from the request, the complete
physical trace, work, crossings, body fingerprints, and an exact fresh-process
transcript replay.

Diagnosis walks the first failed physical transition in order: perception,
route genesis, participation, outward consequence, returned ancestry,
consolidation, and autonomous reuse. A public-game improvement alone cannot
change learner physics. Any candidate must first survive a benchmark-blind
fixture, negative controls, transfer, cost gates, and the ordinary production
regressions.
