# Add a visual-touch human Harness foundation

```text
world light + contact
          |
          v
  HumanHarness sensors
 two eyes | two five-finger hands | body position
          |
          v
       Harness
          |
    outward crossings
          v
  bounded body controls
  gaze | joints | contact force
          |
          +----------> world light + contact
```

## Outcome

Add a deterministic, headless `HumanHarness` development prototype with two
foveated eyes and two five-finger hands with touch and body-position input. The
public boundary accepts only light and contact samples, sends resulting
anonymous input through the owned core `Harness`, applies outward crossings to
bounded body controls, and returns owned physical observations.

This establishes visual-touch human topology and exact closed-loop mechanics.
It does not claim realistic human biomechanics, realistic vision, learned tool
use, or accepted-body authority. ARC, Playground, and Academy curriculum
adoption require later bounded plans after this foundation survives its own
controls.

## Authority

- Path: `arch.md` Accepted law, Boundaries, Forward design, and Successor gate;
  `academy.md` Ownership and Evidence rules; `LANGUAGE.md`; `algo.md`
- Revision: parent commit `766c39c1e527fbe29c7d7445e1d6902b2de5334e`;
  `arch.md` SHA-256
  `f5b1157b8d0479cb9575980f148090932043ce7d8c9da72d70fdb887cb4b4963`;
  `academy.md` working-tree SHA-256
  `8e4b8ba126bdd726d2a2ec1c389b3c7fef329d572d2b620969447ad2e422cecd`;
  `LANGUAGE.md` SHA-256
  `2b1954b161358c4a259198b0b9e4c66a93e47350d749d7c3baf3ddcef7bb8a41`;
  `algo.md` SHA-256
  `62363a087f660caa5ea6418fc0dd1c85195ebf0a0745d19ee58894b3a160224b`

## Model

- Add `truelearner-human`, depending only on `truelearner-core` and serialization
  and hashing libraries already present in the repository. It owns one public
  core `Harness` and private `HumanState`; neither inner body nor mutable device
  state escapes.
- `HumanState` is the product of `Eyes` and `Hands`. `Eyes` contain a shared
  bounded gaze plus bounded vergence and two retinal samplers. `Hands` contain
  left and right palm poses, five named digit joint chains per hand, contact
  force, and derived fingertip positions.
- `WorldSample` contains two raster light fields and dumb contact samples for
  palm and digit surfaces. It contains no object identity, text, action offer,
  target, score, capability, or evaluator field.
- `HumanHarness::step` composes pure transformations around one effectful core
  send: validate the sample, sample retinas/touch/body position into a bounded
  ordered input batch, call `Harness::send`, decode only known outward physical
  ports into bounded control increments, update `HumanState`, and return
  `HumanObservation`. Invalid input or output fails transactionally without
  changing either state.
- Each retina uses a fixed geometric sample lattice: dense near its focus and
  progressively sparse outside it. Sensory position changes which light is
  sampled but never changes where a learned path resides.
- Touch density is highest at ten fingertips, lower on two palms, and absent
  elsewhere in this prototype. Body-position input reports bounded joint and
  pose values independently of light and contact.
- Hand controls are increments to palm translation, wrist rotation, digit
  flexion, digit spread, thumb opposition, and contact force. They do not include
  click, type, grasp, target, key, or action meanings. Fingertip positions derive
  deterministically from the bounded hand state.
- `HumanCheckpoint` atomically contains the opaque core checkpoint, complete
  human state, capacities, physical-port layout version, and pending bounded
  samples. Restore either reconstructs the exact next step or rejects the whole
  checkpoint.
- World rendering, collision/contact generation, gesture-to-API mapping, and
  evaluation are effects outside `truelearner-human`. A later screen world may
  map completed touch gestures to validated action calls only after crossings
  have moved the physical hand.
- Implement in dependency order: state and checkpoint; eyes; hands and body
  position; composed step. Each completed layer remains silent when its
  corresponding world input or output is absent.

## Invariants

- Every organism-visible event enters through the owned public core `Harness`;
  all organism output is read only from outward crossings.
- The accepted core algorithm, checkpoint, output filtering, physical time, and
  natural quiescence remain unchanged.
- Human-shaped names exist only in the wrapper and evidence; the core continues
  to use the words and story in `LANGUAGE.md` and `algo.md`.
- Body controls are bounded physical increments. No control identifies an
  action, object, coordinate target, key meaning, word, or expected result.
- Two eyes, two palms, and exactly five digits per hand are structural
  invariants. Left and right sides have stable physical identities.
- Gaze and the two hands are independent state. No hidden rule moves a hand to
  the gaze location, coordinates the hands, selects contact, or corrects an
  error.
- Retinal, tactile, and body-position encoders have fixed declared capacities
  and deterministic ordering. Input size cannot increase the number of admitted
  signals beyond those capacities.
- Reading an observation is causally inert. A failed step changes neither the
  core checkpoint nor human state.
- Save and restore preserve the exact next inputs, crossings, human state, work,
  clock, body fingerprint, and natural quiescence.
- Reference runs, replay, negative controls, and the representative warm
  regression remain unchanged and strictly under ten seconds.
- This candidate remains development evidence. Promotion into the accepted body
  requires the successor gate in `arch.md`; software success alone cannot grant
  authority.

## Scope

- Add `truelearner/crates/human` and include it in `truelearner/Cargo.toml` as
  package `truelearner-human`.
- Add small modules for bounded geometry/state, retinal sampling, hand
  kinematics and touch, physical-port construction, composed stepping, owned
  observations, and atomic checkpoints.
- Add public-boundary tests under `truelearner/crates/human/tests/`; tests may
  construct and operate `HumanHarness` but may not construct or inspect the core
  body.
- Add a concise physical contract to `truelearner/crates/human/README.md` and
  label the crate as a development prototype in the root overview.

Exclude realistic 3-D dynamics, balance, smell, taste, skin beyond palms and
fingertips, pain, temperature regulation, biological growth, ears, hearing,
voice, speech, semantic attention, hidden local controllers, DOM or
accessibility input, gesture recognition, action schemas, ARC integration,
Academy teaching cases, Playground media I/O, changes to `truelearner-core`, and
accepted-authority updates.

## Development style

TDD. Specify the public states, bounded transformations, transactional failures,
channel-separation laws, and exact checkpoint replay before implementing each
physical layer. Add one layer at a time and keep incomplete layers physically
silent rather than filling them with semantic shortcuts.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-human --lib`
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-human --test human_harness`
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --test harness_boundary`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-core -p academy-arc3 --lib`
- `cargo fmt --all --manifest-path truelearner/Cargo.toml -- --check`
- `cargo clippy --workspace --all-targets --locked --manifest-path truelearner/Cargo.toml -- -D warnings`
- `cargo test --workspace --locked --manifest-path truelearner/Cargo.toml`

These establish bounded and separate visual/touch senses, independent gaze and
hand motion, five-digit topology, touch/body-position feedback,
transactionality, exact restart, unchanged core behavior, and compatibility
with the current Academy boundary.

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-human --lib`

Its measured warm budget is strictly under 10 seconds. Record cold bootstrap
separately. The pre-change Academy boundary regression remains green at `5.23
seconds` including a one-crate rebuild; remeasure a fully warm baseline before
implementation and do not use compilation time as physical time.

## Controls and evidence

- Held-out cases: unseen raster dimensions and focus positions, left/right image
  disagreement, unseen valid hand poses, crossed-hand positions, and contact on
  a previously unused digit.
- Negative controls: moving an eye never moves a hand; moving one hand never
  moves the other; proximity without supplied contact produces no touch input;
  contact on one fingertip does not fire another; reading does not alter a
  checkpoint; invalid samples and unknown output ports leave the complete
  checkpoint unchanged.
- Laws: left/right reflection commutes with channel reflection; zero control is
  identity; bounded increments saturate without wraparound; sampling is
  deterministic; independent controls commute when they touch disjoint state;
  save then restore is identity on the next complete step.
- Falsifiers: any semantic field or decoder, hidden eye-hand coupling, supplied
  target location, unbounded signal count, floating-point replay divergence,
  direct body access, changed core run, partial mutation on failure, restart
  divergence, non-quiescent leak, or warm regression at or above 10 seconds
  rejects the candidate.
- Evidence: validated plan; per-layer public tests; exact checkpoint replay
  transcript; capacity and work measurements; unchanged core and Academy
  regressions; candidate receipt; and independent verification receipt.
- Not applicable because this plan establishes a development body rather than a
  learned capability: no capstone score or capability verdict is produced.

## Risks and rollback

- A body wrapper can become a hidden controller. Prevent this by allowing only
  fixed bounded sampling and control increments, and reject any semantic action
  or automatic coordination in review.
- Full rasters can overwhelm the core. Fixed retinal lattices and measured
  input/work capacities expose and limit that cost.
- Floating-point geometry can break exact replay. Use checked integer and
  fixed-point transformations with canonical serialization.
- A checkpoint can split core and body state. Serialize and validate one
  versioned envelope and commit a step only after every transformation succeeds.
- Simplified anatomy can be mistaken for human fidelity. Keep the claim limited
  to topology and physical channel shape; record departures in the crate README.
- Roll back by removing `truelearner-human` from the workspace and deleting its
  crate and overview reference. The accepted core checkpoint and Academy data
  require no migration because they remain unchanged.

## Open decisions

None.
