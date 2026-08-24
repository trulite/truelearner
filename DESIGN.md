# Academy Playground design

## Direction

Academy uses TrueLearner's Spherical Desk language: a full-bleed dusk field,
quiet ambient identity, glass work panels, and a centered command dock. The
result should feel closer to Apple TV than a web dashboard.

The interface copy follows an operational rule: name the object or action and
stop. Do not narrate the architecture, explain an obvious control, or expose
research vocabulary as product copy.

## Composition

- The world and evidence are a functional split view.
- The input world is the primary surface. Output is a smaller picture-in-picture
  surface until it needs more room.
- History sits above the command dock and never overlaps it.
- The command dock owns text admission and the primary Teach or Probe action.
- Runtime values are read-only observations. They never become organism input.

## Visual system

- Dusk background: blue and teal through plum to ember.
- Glass is reserved for the two work panels, history, and command dock.
- Text is near-white; muted text uses translucent white.
- Mint means ready or learned. Violet distinguishes probe activity. Amber and
  red are reserved for warning and failure.
- Panels use a 16 px radius, a thin light edge, an inset highlight, and one
  ambient shadow.
- Controls use 8–12 px radii. Only the command dock and its primary action are
  pills.
- Native system typography, compact labels, tabular numerals, no uppercase
  micro-labels.

## Copy

Prefer `World`, `Input`, `Output`, `Skills`, `Runtime`, `History`, `Teach`,
`Probe`, `Save`, `Restore`, and `Replay`.

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
