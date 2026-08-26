# Academy

```text
person -> Playground -> physical input -> TrueLearner -> physical output
                           |                    |
                           +------ Academy -----+
                              teach, probe,
                              record, compare
```

## Purpose

Playground lets people live with the organism. Academy teaches and measures its
development. TrueLearner remains the physical learner described by
[arch.md](arch.md).

Academy may use meaning. TrueLearner may not receive capability names, expected
answers, correctness bits, loss, reward, or evaluator state.

## Ownership

- **TrueLearner** owns input, junctions, links, paths, output, outcome, strength,
  physical time, and durable body state.
- **Academy** owns teaching cases, probes, capability claims, schedules,
  evidence, and developmental records.
- **Playground** owns human interaction, rendering, and inspection.

Keep the runtime dependency direction
`playground -> academy-core -> truelearner` for live interaction. Keep
`academy-core` headless. The causally inert episode reviewer instead depends
only on the portable review catalog:

```text
playground -> academy-review <- academy-episodes -> academy-core -> truelearner
```

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
the official list of available physical actions.

The Rust agent owns `Arc3Sensorimotor`, which owns the public `Harness`. The
process boundary exposes no score, game or level identity, expected action,
babbling, support, settling, action remapping, reset, or diagnostic command.
Unknown fields and unsupported actuators fail before a Harness transition.

Official scoring remains entirely in the pinned ARC SDK. Academy records the
official scorecard beside physical work, outward crossings, learning updates,
quiescence, body fingerprints, and an exact fresh-process transcript replay.
The public fixture exercises the same semantic firewall but can never support
an official capstone claim.
