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
academy-body ----------> academy-workstation
      |                         |
      +-------------------------+--> truelearner-workstation -> truelearner-body
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
all twelve bounded Body Discovery claims are acquired and there is no
acquisition failure. Acquisition is not treated as the end of the evidence
ladder: changed-world transfer and retention after later committed learning are
recorded separately. `ContactDrag`, `ThumbContact`, and `PinchDrag` are
`Stable`.
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
4. Before `UntriedOutputRelease`, continue only one executable, retained,
   boundary-open path carrying fresh world-witnessed progress from its own
   motor cause. Recompute this condition at every choice; do not store a
   permanent continue command.
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
  the claimed closure resolution; Rust's frozen verifier checks choice laws.

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
`Automatic` is reserved for a future repeated low-cost-use contract. In the
reference run, all twelve claims are acquired and all three manipulation
capstones pass both transfer and later retention. The course makes no
automaticity claim.

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
defined manipulation stability rung. Repeated low-cost automaticity and richer
workstation use remain separate future contracts. No general pointing,
clicking, typing, grasping, or image-use claim follows from these sequences.

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
checkpoint emitted by the completed Body Discovery course. A fresh body is a
negative control and cannot support a post-course claim.

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
invalid frames, unsupported application actions, and corrupt body checkpoints
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
