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

The eyes advance in half-pitch 32-unit quanta under a 128-unit velocity cap,
and the palm advances in 8-unit quanta under its 64-unit cap. Rate
limits are what make reflexes and habits able to share a joint: no summed
effort can overshoot what a reflex can correct, and no habit can move
faster than a reflex can follow.

The screen is a 2048-square display mapped onto the unchanged 0..1023 body
reach. An 8x8 mean field covers it at every gaze; signed transient quadrants
localize changes on a 16x16 lattice. A gaze-centred 17x17 field samples every
eight display pixels and supplies detail in four mirror-symmetric interleaves.
The global view orients; supported foveal detail refines gaze and reach.

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

3. **Pre-reach extension** (harness pulse, initially weak). While a salient
   target is seen, its local approach line is ready, and the palm is not in
   contact, the arm extends toward it. Contact terminates the pulse. A returned
   visual change strengthens only the open approach line; an omitted change
   temporarily inhibits that line.

4. **Pre-reach planar pull** (frame-level equilibrium-point shift). A ready
   local approach line shifts the
   arm's resting posture shifts toward the salience centroid of what the
   eyes see, with a sustained aim recruiting more drive each step — the
   infant straining toward a toy, so no fixed habit can stalemate the
   reach. It is frame-level physics, not a chosen crossing: the learner's
   habits still sum their effort against it, and its strength grows where
   a habit fights it. This path exists because arrival-level pulses were
   eaten by the learner's joint-stop inhibition — the reflex reaches
   through the equilibrium point, which learned inhibition cannot gate.

5. **Arm elasticity and transport clearance** (frame-level). In contact, the
   un-driven arm recoils by one depth quantum. After visual disengagement,
   unmatched extension is cancelled while the palm moves laterally to the new
   focus. Alignment restores extension. A held contact has not disengaged, so
   contact-preserving movement remains physically possible.

6. **Legacy ocular drift** (frame-level). Version-14 local-view samples retain
   their old dark-field return to primary position. Full-screen samples do not
   use it: a dark fovea no longer means that the only screen disappeared.

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

The reflexes deliver the first hit. Local approach readiness modulates later
touches: the dead key inhibits its hand path while remaining visible. The
course's later rungs demand sequence and drag structure. The blind controls — target drawn at zero
contrast — stay at chance because without salience every reflex is silent;
only the learner acts, and it has not yet learned to aim.

## Honest state

The body course acquires all five capabilities (gaze contingency, gaze
control, binocular fusion, hand contingency, self-world) with exact
replay. The Workstation2 ladder acquires its reflex-backed rungs — gaze,
touch, aimed tap, scan, quiet hand — with quiet blind controls and tap
rates around eleven times chance (seed 11; seeds vary in how strongly the
developed habits support the tap cycle).

Live key and dead key are acquired. Fixed-screen patches compete as coherent 4-connected
regions; one focus is retained per eye until release, disappearance, or a fresh
spatial change. Disengagement records one 32-step recent region. Fresh change,
then a non-recent peer, then brightness, then stable screen position determine
selection. A sole patch remains selectable. Fresh onsets use the existing
16x16 transient location, including repeated changes inside the retention
window. Gaze remains active when omission inhibits only the corresponding hand
line. The measured frontier is sequence; drag remains gated.
