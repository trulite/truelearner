# A0 Academy Playground V0 result

## Result

Development acceptance is positive: `20/20` gates in `playground.md` section
36 are represented by the shipped boundary, headless tests, or native desktop
smoke. A0 remains an external instrument; it changes no TrueLearner law and
does not begin R7.

## Product surface

- Native Dioxus Desktop window at 1440 × 900, minimum 1080 × 720.
- Rust-owned 640 × 360 RGBA input world with deterministic drawing.
- Display coordinates are normalized into the 640 × 360 physical raster, so
  drawing remains exact across window sizes.
- Text, image, file, and drawing admission.
- Large raster output beside the input world.
- Teach/Probe modes, capability evidence, runtime observations, history, live
  checkpoint save/restore, and exact replay.
- Spherical Desk presentation: dusk field, one split glass work deck, a bottom
  command dock, and dock-opened History and Skills galleries.
- Product copy contains direct object/action labels rather than architecture
  narration.

## Boundary and determinism

- Dependency direction is `playground -> academy-core -> truelearner-core`.
- `academy-core` is headless and has no Dioxus dependency.
- The organism runs behind bounded command/event queues, never on the UI event
  loop.
- Physical admissions and replays are recorded independently of repaint or
  host UI timing.
- Debug overlays are display-only and absent from the canonical framebuffer.
- No foveation or new organism mechanism was added.

## Validation

Local macOS validation was explicitly requested for the final native product
pass, superseding the protocol's remote-only operational preference:

- `academy-core`: `5/5` tests passed.
- `truelearner-core`: `14/14` tests passed.
- `academy-core` + `academy-playground`: strict Clippy passed with
  `-D warnings`.
- Academy workspace formatting check passed.
- Native app launched and emitted `ACADEMY_PLAYGROUND_READY`.
- Native screenshot review confirmed the equal unlabeled 16:9 split, matched
  bottom action strips, pill-shaped Teach/Probe switch, dock-owned History and
  Skills spaces, and a compact full-width runtime strip.

The compiler reported only the upstream future-incompatibility advisory for
`block 0.1.6`; it produced no project warning or failure.

## Visual artifact

`output/academy-playground-native-aspect-v1.png`

The output directory is intentionally ignored; the screenshot is review
evidence, not runtime input.
