# Academy Playground design

## Direction

Academy uses TrueLearner's Spherical Desk language: a full-bleed dusk field,
quiet ambient identity, glass work panels, and a centered command dock. The
result should feel closer to Apple TV than a web dashboard.

The interface copy follows an operational rule: name the object or action and
stop. Do not narrate the architecture, explain an obvious control, or expose
research vocabulary as product copy.

## Composition

- World and evidence form one translucent split deck with an internal divider.
- Input and Output are equal physical surfaces. The split carries no hierarchy,
  and redundant panel headings are omitted.
- Both surfaces retain the native 16:9 raster geometry. Their bottom action
  strips occupy matching space and never change the coordinate transform.
- Runtime is a thin, full-width instrumentation strip across the top of the
  work deck.
- Skills and History are dock-owned spaces. Both are hidden by default and open
  as large galleries above the dock without changing organism state.
- History cards lead with each admitted artifact. Skills use a card gallery and
  a focused evidence pane.
- The command dock owns History, Skills, Teach/Probe mode, text admission, and
  Run.
- Runtime values are read-only observations. They never become organism input.

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
