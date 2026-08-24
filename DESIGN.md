# Academy episode viewer design

## Direction

Academy uses TrueLearner's Spherical Desk language: a full-bleed dusk field,
quiet ambient identity, glass review panels, and a centered collection dock.
The result should feel closer to Apple TV than a web dashboard.

The interface copy follows an operational rule: name the object or action and
stop. Do not narrate the architecture, explain an obvious control, or expose
research vocabulary as product copy.

## Composition

- A large video stage is the primary review surface.
- A narrow evidence inspector shows the selected episode's outcome, physical
  work, crossings, plasticity updates, quiescence, replay status, and body
  fingerprints.
- A horizontal poster gallery holds development, held-out test, and negative
  control episodes without crowding the stage.
- The centered dock filters the gallery by collection. It does not execute or
  modify the organism.
- Episode videos are derived views of canonical evidence. Selecting, playing,
  pausing, or filtering them never becomes organism input.
- World and output remain equal 16:9 surfaces inside every recorded frame.

## Visual system

- Dusk background: blue and teal through plum to ember.
- Glass is reserved for the work deck, dock, and dock-owned spaces.
- Text is near-white; muted text uses translucent white.
- Mint means ready or learned. Violet distinguishes probe activity. Amber and
  red are reserved for warning and failure.
- The main deck and gallery use a 24 px radius; gallery artifacts use 16 px.
  A thin light edge, inset highlight, and one ambient shadow establish depth.
- Section headings are large, tightly tracked, and bold; measurements remain
  compact with tabular numerals.
- Controls use 8–12 px radii. The command dock, Teach/Probe switch, and primary
  action are pills.
- Native system typography, compact labels, tabular numerals, no uppercase
  micro-labels.

## Copy

Prefer `Development`, `Tests`, `Controls`, `Outcome`, `Physical work`,
`Crossings`, `Learning updates`, `Quiescent`, `Replay`, and `Episode record`.

Avoid explanatory UI copy such as `physical development instrument`,
`causally inert runtime view`, `human semantics are rendered here`, or
`selected production mechanics`. Those constraints belong in code and
evidence, not in the interface.

## Accessibility and motion

- Keyboard focus uses a visible 3 px blue ring.
- Status remains readable as text and never relies on color alone.
- Motion is limited to short state transitions and loading placeholders.
- Reduced-motion preference removes nonessential transitions.
- The minimum window is 1080 × 720; the split view remains usable at that size.
