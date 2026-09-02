# Reflexes and Foveation in the Attachment

> Shipped design. Builds on `docs/body-laws.md`, `docs/body-structures.md`,
> and the Workstation2 course design in `docs/workstation2-course.md`.

A baby is born with reflexes that act before any learning exists: bright
light pulls the eyes, a seen object invites an arm swipe, a touched palm
closes. Learning does not build these arcs; it refines them. The body now
carries its reflexes as birthright structure, built while attaching, and the
Workstation2 course climbs them to aimed tapping.

## The body

Two eyes and one undifferentiated hand: a single pointer with a planar
position and a depth, one contact surface, seven axes. No wrist, spread,
thumb, or finger axes — a chord or a second pointer has no rung that needs
it, and each was a standing risk to the one-component-at-a-time movement
budget. If a future rung honestly demands fingers, they grow then.

The eyes are rate-limited to one receptor pitch per step however efforts
sum, and the palm's planar transport to four palm steps per step. Rate
limits are what make reflexes and habits able to share a joint: no summed
effort can overshoot what a reflex can correct, and no habit can move
faster than a reflex can follow.

The retina is full-field: one receptor per 128 world units, so a centered
gaze sees the whole screen. A narrow view would hide most of the world from
a resting body — the gaze probe once failed exactly there, eyes anchored at
center staring at a quarter of the screen.

## The salience floor

One shared signal of "what stands out": retinal light above 129 (of 255)
feeds the tonic salience cells. The floor sits above the rendered hand of
the body course (palm 96) and every background, below every application's
target bands. One floor replaces the per-reflex magic numbers; the deadzone
and the hand mask are geometry, not brightness.

The salience cells are tonic: they fire every step their receptor is lit
above the floor, so a resting body on a static scene still sees what
matters. The old change-only retina was blind at rest, and the binocular
probe failed on exactly that blindness.

## The reflexes

All are body machinery, computed from the organism's own surfaces —
retina, touch, proprioception — never application state. Reflex firings are
ordinary physical incidence; the learner's return, choice, and
strengthening laws act on them.

1. **Foveation** (attach-time arcs). Each salience cell drives the eye
   opportunities that move gaze toward its receptor. The arc's impulse
   reaches the motor threshold alone, so a seen target wins the crossing
   the step it appears; only summed effort above it can outvote. A centered
   stimulus drives both sides equally, so balance — not a constant —
   terminates the pull.

2. **Yoked vergence** (harness pulse). When both eyes see salient pulls in
   opposing horizontal directions — the vergence geometry — both eyes'
   horizontal opportunities are pulsed in the same step, every step, until
   each target sits on its fovea. Real vergence is brainstem-yoked: the
   eyes converge together or not at all, and the choice machinery would
   otherwise serialize them one eye per step.

3. **Pre-reach extension** (harness pulse, impulse at threshold). While a
   salient target is seen and the palm is not in contact, the arm extends
   toward what is seen. Contact terminates it, exactly as balance terminates
   orienting.

4. **Pre-reach planar pull** (frame-level equilibrium-point shift). The
   arm's resting posture shifts toward the salience centroid of what the
   eyes see, with a sustained aim recruiting more drive each step — the
   infant straining toward a toy, so no fixed habit can stalemate the
   reach. It is frame-level physics, not a chosen crossing: the learner's
   habits still sum their effort against it, and its strength grows where
   a habit fights it. This path exists because arrival-level pulses were
   eaten by the learner's joint-stop inhibition — the reflex reaches
   through the equilibrium point, which learned inhibition cannot gate.

5. **Arm elasticity** (frame-level). In contact, the un-driven arm recoils
   toward its resting length: press, lift, press again. The tap cycle is
   the equilibrium of extension and recoil, not a learned trick.

6. **Ocular drift** (frame-level). The gaze integrator leaks: when the
   whole retina is dark, the eyes drift back toward the primary position
   at an effort no habit noise can stalemate. Any salience hands the eye
   to the foveation reflex. A lost gaze never stays lost.

Terminations are physical: balance for foveation, fusion for vergence,
contact for extension, the deadzone for the planar pull, the resting length
for recoil, and primary position for drift.

## The hand renders dark

A hand between the eyes and the screen occludes light. Workstation2 draws
the palm as a dark silhouette (8) — below every background pixel and below
the salience floor, visible to the learner as contrast, invisible to the
reflexes. The body course already rendered its hand below the floor; the
bright hand (250) was backwards physics that also captured every light
reflex ever built.

## What the learner still owns

The reflexes deliver the first hit and the tap cycle; the course's later
rungs demand the learner's modulation: the dead key (stop hammering what
stopped reacting), the quiet hand (no contact on blank screens), scan
before act, sequences, and drag. The blind controls — target drawn at zero
contrast — stay at chance because without salience every reflex is silent;
only the learner acts, and it has not yet learned to aim.

## Honest state

The body course acquires all five capabilities (gaze contingency, gaze
control, binocular fusion, hand contingency, self-world) with exact
replay. The Workstation2 ladder acquires its reflex-backed rungs — gaze,
touch, aimed tap, scan, quiet hand — with quiet blind controls and tap
rates around eleven times chance (seed 11; seeds vary in how strongly the
developed habits support the tap cycle).

The learning rungs — live key, dead key, sequence, drag — are the
measured frontier, and the live key names the missing physics precisely:
the reach averages every salient thing, so between two keys it taps the
empty midpoint and neither consequence presents. Selection —
winner-take-all salience, reaching the one thing the eyes hold — is the
next body change, and behind it the learner must act on what its taps
changed: stop hammering the dead key, follow the sequence order, and slide
the drag to its goal.
