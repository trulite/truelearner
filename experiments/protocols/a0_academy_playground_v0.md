# A0 Academy Playground V0 protocol

## Parent and boundary

A0 begins from frozen R6 tag `r6-partition-invariance-development-v1`.
It is an external instrument, not an organism-law successor. R7 is forbidden.

Dependency direction is fixed:

```text
playground -> academy-core -> truelearner-core
```

`truelearner-core` must not depend on either Academy crate. `academy-core` must
not depend on Dioxus.

## V0 implementation

- Rust-owned RGBA `VisualSurface` with deterministic drawing and rendering.
- Headless capability graph, evidence modes, developmental records, and replay.
- A bounded worker command/event boundary around `BoundaryRuntime` using
  `MechanicalConfig::PRODUCTION`.
- Dioxus Desktop conversation/world surface, image and file input, shared
  drawing surface, Teach/Probe control, capability evidence, physical inspector,
  checkpoint/replay controls, and development timeline.
- Debug overlays are display-only and never alter the canonical framebuffer.
- Text is admitted through the explicitly documented byte/glyph embodiment
  affordance. It is not tokenized or interpreted by TrueLearner.

## Acceptance

The twenty gates in `playground.md` section 36 are the acceptance matrix. In
addition:

- all existing TrueLearner tests remain green;
- the Academy worker never runs the organism on the UI event loop;
- command/event queues are bounded and report backpressure;
- replay compares the exact admitted spike stream, crossings, physical clock,
  work, and final durable body fingerprint;
- resident-arena observations remain causally inert;
- no simulator/foveation/world-spike work begins in A0.

All Rust formatting, compilation, tests, Clippy, and app launch smoke checks run
in E2B. No project Rust command runs locally.
