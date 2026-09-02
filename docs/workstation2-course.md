# Workstation2 Screen-Use Course

The course that must pass before ARC-AGI-3. It lives on the Workstation2
touchscreen and adds no learner state or learner law. Every rung is a claim
about what the body does on its own; the course never moves a hand, chooses a
direction, injects a touch, or tells the organism what a target is.

```text
body course (eyes, hand, self/world)
  -> gaze -> touch                       (already acquired)
  -> aimed tap -> live key -> dead key   (eyes guide the hand)
  -> scan -> quiet hand                  (look, then act)
  -> sequence -> drag                    (chains and movement)
  -> ARC-AGI-3 application
```

## One application, many levels

A single external application, the *target app*, draws on the screen:

- **Target**: one lit rectangle. Tapping inside it does something visible.
- **Decoy**: one lit rectangle of a different brightness that never reacts.
- **Blank**: no rectangle. Nothing on the screen reacts.

What "visible" means is fixed per rung (the target jumps, a bar grows, a
second target appears). All of it is ordinary pixels. Positions, sizes, and
brightness bands are regenerated for every probe so fixture memory cannot pass.

Colours: ARC uses about ten. Until the retina has colour, the application maps
each colour to a distinct brightness band. This decision is made here so the
retina does not change halfway through the course.

## Rungs

Each rung has a development phase, then fresh probes from cloned checkpoints
that discard mutation. Each has a positive claim and a killing control.

| rung | positive claim in a fresh probe | control that must stay at chance | state |
|---|---|---|---|
| Aimed tap | taps inside the target far exceed the target's share of the screen | same body, target drawn at zero contrast | Acquired |
| Live key | taps on the target exceed taps on the decoy | target and decoy swapped in the probe | Frontier |
| Dead key | after the target stops reacting, taps on it fall within a budget | target keeps reacting; tap rate must not fall | Frontier |
| Scan | gaze reaches the visible target before the first tap on it | with the same seed, the visible target is found before its zero-contrast control | Acquired |
| Quiet hand | contact rate on blank screens is far below contact rate with a target lit | no rise over time on blank screens | Acquired |
| Sequence | the rewarded order A-then-B exceeds B-then-A | order requirement reversed in the probe | Frontier |
| Drag | releases land on the goal far above its share of the screen, after starting on the target | goal drawn at zero contrast | Frontier |

The measured frontier is the live key: the reach averages both salient
rectangles and taps the empty midpoint between them, so neither key's
consequence ever presents. Selection — winner-take-all salience, reaching
the one thing the eyes hold instead of the average of everything lit — is
the named missing physics. The dead key, sequence, and drag rungs sit
behind it: each needs the learner to act on what its taps changed, which
cannot begin until a tap lands on a key at all.

"Far exceeds" means at least three times the chance rate with at least twenty
taps, on at least three seeds. A rung that passes on one seed is `Emerging`;
on all seeds it is `Acquired`. The first rung that is not `Acquired` is the
reported frontier. No rung is claimed from development alone.

## Evidence per rung

Recorded for development, each probe, and each control:

- steps, taps, taps in target, taps in decoy, gaze positions before each tap
- chance rate (target area over screen area) and the observed rate
- physical work, natural quiet, body fingerprint, exact replay
- for Dead key: taps on the dead target per hundred steps, before and after
- for Quiet hand: contacts per hundred steps, blank versus lit

Verdicts are derived from these numbers only. Failures are preserved.

## Semantic firewall

- The only organism input is one `WorldSample`: retinal light, fingertip
  contact, proprioception. Same as today.
- The session never passes causal parents, device events, target identity,
  scores, or level state to the body. The joint-stop boundary is body
  morphology and stays inside the harness.
- Target, decoy, and level are course vocabulary. The organism never sees a
  label for any of them.
- Controls change only the external drawing, never the body.

## Prerequisites and starting point

- Start from the body-course checkpoint. A fresh body appears only as the
  named cold control.
- Gaze and touch from the existing course stay as the first two rungs.
- Run every claim on three seeds. Budgets are per phase and reported.

## What we expect to find

Aimed tap is the first genuinely new physics. Today the eyes and the hand are
independent products; nothing in the body makes a lit patch on the retina
steer palm transport. When aimed tap stops honestly, that trace is the input
to the next `$dev` design. The course exists to expose that missing physics,
not to specify how the body should solve it.

## After the course

Rehost the ARC-AGI-3 adapter on Workstation2 as another application. Its
output is screen pixels; its input is generic touch events. Start ARC only
from a checkpoint that passed through Drag.

Before Drag passes, `run_with_diagnostic_checkpoint` may export the developed
frontier checkpoint for the pinned public fixture. The current reference is the
named fresh-body negative control at 256 steps and seed 11. That run is
`plumbing-negative-control` evidence only. It cannot support an ARC capability
or score claim.
