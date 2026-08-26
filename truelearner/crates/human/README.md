# TrueLearner human Harness

This development crate wraps the public core `Harness` with a bounded
visual-touch body: two foveated eyes and two five-finger hands with fingertip,
palm, and proprioceptive input.

It models topology and a deterministic flat interaction surface, not realistic
human anatomy. Its controls are physical increments only. It contains no click,
key, object, target, word, score, or evaluator meaning. Ears and voice are out
of scope.

Every control receives the same subthreshold physical readiness pulse. It
cannot move the body by itself; a path formed by ordinary sensory input must
meet it before an output can fire. No caller selects a control or direction.

Decrease and increase outputs are opposing effort on one of 25 physical axes.
The body adds each pair, applies the bounded net impulse once, and returns
signed position, actual velocity, both effort magnitudes, and joint limits as
ordinary input on the next step. A dedicated neutral-position receptor makes a
still joint physically present before it first moves; it is silent off neutral,
where the existing signed position receptors take over. Equal opposing effort
is felt but does not move the pose or produce a movement outcome.

Receptors have a fixed anatomical locality rather than an ordinal motor map.
Retina is local only to eye axes, palm and fingertip touch to the matching hand
axis, and every proprioceptor to its source axis. Each receptor site is equally
distant from its decrease and increase motors, so locality supplies no desired
direction or hidden reflex.

Every input reaches the learner through `HumanHarness::step`; callers receive
owned observations and opaque `HumanCheckpoint` values. The private core body
and mutable human state never escape.
