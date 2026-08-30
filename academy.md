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
academy-body ---------------------> truelearner-workstation -> truelearner-body
academy-workstation --------------> truelearner-workstation -> truelearner-body
academy-workstation-review -> academy-workstation
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
4. Workstation Contact: visual reach, touch/withdraw, tap/hold/release, drag,
   thumb opposition, and later keyboard, touchpad, and monitor interaction.

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
now has accepted unguided movement separation across all five digits. The next
capability frontier is local surface-contact contingency and then
contact-directed control; no pointing, clicking, typing, or image-use claim
follows from separate movement alone.

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

## ARC-AGI-3 capstone

The ARC-AGI-3 capstone is an external capability evaluation, not a teaching
episode. Its Python adapter owns the official SDK, game identity, score,
terminal state, action budget, and scorecard. A fresh Rust agent is created for
every server-selected game and receives only an owned 64×64 palette frame and
an action catalog derived from the official list of available physical actions.
The catalog contains only opaque action identities and public argument shapes.
Complete typed calls may leave the agent; descriptions, active coordinates,
scores, and evaluator state may not enter it.

The Rust agent owns `Arc3Sensorimotor`, which owns the public `Harness`. The
process boundary exposes no score, game or level identity, expected action,
babbling, support, settling, action remapping, reset, or diagnostic command.
Unknown fields and unsupported actuators fail before a Harness transition.

Official scoring remains entirely in the pinned ARC SDK. Academy records the
official scorecard beside physical work, outward crossings, learning updates,
quiescence, body fingerprints, and an exact fresh-process transcript replay.
The public fixture exercises the same semantic firewall but can never support
an official capstone claim.
