# TrueLearner workstation Harness

This development crate wraps the public core `Harness` with the smaller body
needed for a workstation: two independently movable foveated eyes and one
five-digit hand with palm, fingertip, and proprioceptive input.

It is a physical body boundary, not a workstation simulator. It contains no
cursor, click, key, character, object, target, word, score, or evaluator
meaning. A later external world owns the real keyboard geometry, continuous
touchpad, monitor pixels, images, collision, and device state.

Each eye receives its own raster and owns horizontal and vertical movement.
There is no built-in vergence control and no fused depth channel. Any useful
relation between the two views must form from separate pixels, separate eye
position, and their shared physical consequences.

The hand owns palm horizontal, vertical, and depth movement, wrist, spread,
thumb opposition, and five independent flexion axes. Its six touch sites are
the palm and five fingertips. Fingertip positions are bounded deterministic 3-D
geometry; they carry no knowledge of keys or devices.

Decrease and increase are opposing effort on each of fifteen axes. The body
adds each pair, applies the bounded net impulse once, and returns signed
position, actual velocity, both effort magnitudes, and limits as ordinary input.
Equal effort is felt but does not move the body.

Both directions of one axis share one physical outcome component. Different
axes have disconnected outcome components and compose under
`RecursiveLearnerCausalTopologyProductComposition`. On the next step, only an
axis that actually changed returns a physical transition. Merely sampling the
same state again is not an outcome.

Every input reaches the learner through `WorkstationHarness::step`; callers
receive owned observations and opaque `WorkstationCheckpoint` values. The
private core body and mutable workstation body state never escape.
