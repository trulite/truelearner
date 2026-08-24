# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Stack

Dioxus Desktop in Rust for the first Playground, with a headless Academy core and the existing native TrueLearner body runtime. The interface should remain portable to Dioxus Web or another frontend without changing Academy or TrueLearner.

## Users

The primary users are TrueLearner researchers and teachers who need to review how a developing physical organism was taught, inspect held-out probes and controls, and verify evidence without adding semantic machinery to the organism.

## Product Purpose

Academy runs development and test curricula headlessly. The Playground is its episode review surface: it turns canonical physical evidence into watchable videos, galleries, and compact inspection without changing the evidence. Success means a person can see what the organism experienced, what appeared at its output, what changed, and whether fresh tests and controls behaved correctly.

## Positioning

The Playground is a film room for a continuously executing physical learner: Academy may understand tasks and expected outcomes, but the learner itself receives only physical arrivals and raster surfaces through its established body boundary.

## Operating Context

Academy executes development, held-out test, and negative-control episodes without UI timing. Users open a desktop gallery, play deterministic episode videos, filter collections, inspect physical measurements, and download the canonical record. Academy's captions and overlays remain external annotations.

## Capabilities and Constraints

- Dependency direction is `playground -> academy-core -> truelearner-core`; reverse dependencies are forbidden.
- Dioxus is confined to the Playground crate. Academy Core is headless.
- The organism-facing visual truth is a Rust-owned RGBA raster framebuffer.
- Text may initially use the documented direct byte/glyph embodiment affordance; it must not introduce tokenization or language concepts into TrueLearner.
- UI, WebView, file, and async timing may not silently define organism time.
- Every admitted physical input must be recordable and replayable.
- Capability states are external evidence labels, never organism state.
- Resident arena partitioning is causally inert instrumentation.
- Episode video encoding is observational infrastructure and cannot feed back
  into TrueLearner.
- R7 non-residence, storage latency, foveation, simulators, audio input,
  robotics, and distributed Playground work are out of scope for V0.

## Brand Commitments

The product names are TrueLearner, Academy, and Playground. Product language should be precise, calm, candid about evidence, and avoid anthropomorphic claims of understanding that the evidence does not establish.

## Evidence on Hand

- `playground.md` is the V0 requirements contract.
- `arch.md` is the physical-runtime oracle and forward runtime specification.
- Frozen R6 development evidence establishes resident partition invariance.
- The accepted boundary runtime already supplies bounded input/output staging, checkpoints, crossings, physical work, and production mechanics.

No user studies, testimonials, or production deployment claims are available and none should be invented.

## Product Principles

1. Human semantics stop at the physical boundary.
2. Evidence outranks labels and apparent fluency.
3. Every developmental interaction is reproducible.
4. The instrument observes the body without becoming its cognition.
5. New runtime mechanics remain replaceable beneath a stable body-level interface.

## Accessibility & Inclusion

The desktop interface must be keyboard reachable, maintain clear focus and contrast, avoid color-only state distinctions, and support reduced motion. Dense research information should remain readable without requiring hidden gestures.
