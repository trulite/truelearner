# Replace the human prototype with a binocular workstation Harness

```text
workstation scene
  |-- left-eye light  --\
  |-- right-eye light ---+-> WorkstationHarness -> anonymous input -> learner
  `-- one-hand contact --/                              |
                                                        v
 two independent eyes + one five-digit hand <- opposing physical outputs
                                                        |
                 changed axis -> its local outcome -----+
```

## Outcome

Replace the unused development-only `HumanHarness` with a smaller
`WorkstationHarness`: two independently movable eyes, one bounded five-digit
hand, palm and fingertip touch, and complete signed proprioception. The harness
uses the authoritative connected-outcome product protocol and returns an actual
state change only to the physical axis that changed.

This establishes a truthful body surface on which a later external workstation
world can render a real keyboard, touchpad, monitor, hand, and images. It does
not implement those devices, teach their use, establish binocular depth, or
promote a new accepted body.

## Authority

- Path: `arch.md` Accepted law, Accepted body, Boundaries, and Forward design;
  `academy.md` Ownership, Body discovery, and Evidence rules; `LANGUAGE.md`;
  `algo.md`; Connected Outcome Product V1 authority artifacts
- Revision: parent commit `ec67a285e4ba8496c0b4fff8dd9a6b6ca7526974`;
  authoritative hand-ladder parent
  `hand-causal-topology-product-authority-v1@7be8bbc3009fe3131622a6ee21e9aa260d649aa1`

## Model

- Rename the development-only `truelearner-human` crate to
  `truelearner-workstation`. Migrate the only consumer, Academy Body Discovery,
  to the new one-hand and independent-eye public surface; no production crate
  depends on the old API.
- `WorkstationState` is the product of two `EyeState` values and one
  `HandState`. Each eye owns horizontal and vertical gaze. The hand owns palm
  horizontal, vertical, and depth axes, wrist, spread, thumb opposition, and
  five independent digit-flexion axes.
- `WorldSample` contains exactly two bounded raster light fields and six dumb
  contact samples: palm plus five fingertips. It contains no cursor, key,
  character, target, action, score, or evaluator field.
- `WorkstationHarness::step` validates one sample transactionally, returns each
  prior changed axis as `PhysicalIncidence::Transition` to that axis's outcome
  source, admits current light, touch, proprioception, and equal physical motor
  opportunity, folds crossings into opposing effort, and integrates every axis
  once.
- Both directions of one axis map to one local outcome source and therefore
  remain ordinary alternatives. Different axes map to disconnected outcome
  sources and compose independently under
  `RecursiveLearnerCausalTopologyProductComposition`.
- The external world owns 3-D scene rendering, collision, device state, proper
  keyboard geometry, touchpad motion, monitor pixels, and evaluation. The
  harness owns only body geometry, physical samples, physical crossings, and
  opaque checkpoints.
- `WorkstationCheckpoint` atomically preserves the opaque core checkpoint,
  body state, physical sites, sequence, and exact pending changed-axis set.

## Invariants

- Every organism-visible event enters through the public core `Harness`; every
  body change begins with an outward crossing.
- Left and right retinal fields remain distinct. No fused depth value,
  correspondence label, target depth, or correct vergence enters the learner.
- The two eyes move independently. Any vergence or shared gaze must be formed
  by ordinary composition rather than a semantic vergence motor.
- The one hand always has exactly five digits and six touch sites. Palm and
  fingertip positions derive deterministically from bounded integer state.
- Opposing outputs on one axis combine by signed addition. Equal effort is
  sensed but changes no pose and creates no transition outcome.
- Both directions of one axis share one connected outcome component; distinct
  axes use distinct disconnected components.
- Only an actual bounded pose change is returned as `Transition`; resampling an
  unchanged state is never returned as a physical consequence.
- Reading is inert. Invalid samples and checkpoints fail without partial state
  change. Save/restore preserves the exact next step.
- The core code, global default protocol, accepted authority, physical time,
  replay, and natural-quiescence rules remain unchanged.
- The representative warm regression remains strictly under 10 seconds.

## Scope

- Replace `truelearner/crates/human/` with
  `truelearner/crates/workstation/`, including crate metadata, state, harness,
  checkpoint, public tests, and physical contract.
- Update `truelearner/Cargo.toml`, `truelearner/Cargo.lock`, the root `README.md`,
  and `academy.md` to name the development workstation body and its revised
  one-hand binocular curriculum boundary.
- Update the Academy workspace dependency, lockfile, `academy-body` world,
  course, tests, and README to use `WorkstationHarness`, one hand, separate eye
  axes, and the preserved external capability firewall.
- Add law tests for eye/hand separation, opposing-axis identity, disjoint-axis
  composition, local outcome topology, explicit transition incidence,
  transactionality, checkpoint replay, and complete proprioceptive coverage.

Exclude keyboard, touchpad, monitor, image assets, collision detection, device
events, new Academy teaching policies or capability claims, grasping evidence,
learned binocular correspondence, changes to `truelearner-core`, global
protocol changes, accepted-body promotion, and compatibility with the
unreleased development-only `HumanHarness` checkpoint/API.

## Development style

TDD. Replace the public boundary tests and pure state laws first, then implement
the smallest state, harness, and checkpoint surface that makes those physical
contracts pass.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation --lib`
  establishes bounded body laws, independent eyes, one-hand geometry, sensory
  locality, and outcome-component topology.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation --test workstation_harness`
  establishes the public binocular/touch boundary, transactional failures,
  transition returns, inert reads, and exact checkpoint continuation.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --test harness_boundary`
  establishes unchanged core behavior and connected-product controls.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`
  establishes that the migrated world and course use one hand and independent
  eyes without moving evaluator knowledge across the harness.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --test body_course`
  establishes preserved probe isolation, failure evidence, and replay.
- `cargo fmt --all --manifest-path truelearner/Cargo.toml -- --check`
- `cargo check --workspace --all-targets --locked --manifest-path truelearner/Cargo.toml`
- `cargo clippy --workspace --all-targets --locked --manifest-path truelearner/Cargo.toml -- -D warnings`
- `cargo test --workspace --locked --manifest-path truelearner/Cargo.toml`
- `cargo test --workspace --locked --manifest-path academy/Cargo.toml`

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-workstation --lib`

Its measured warm budget is strictly under 10 seconds. Record cold bootstrap
separately; rendering and device-world execution are outside this loop.

## Controls and evidence

- Held-out cases: different left/right raster contents, an unused fingertip,
  disjoint eye and hand axes, movement into each bound, and checkpoint restore
  before multiple pending axis returns.
- Negative controls: equal opposing effort changes no pose and produces no
  transition; an unchanged sample is admitted only as a sample; one eye never
  moves the other or the hand; one digit never moves another; invalid raster or
  contact data leaves the complete checkpoint unchanged.
- Laws: zero/equal effort is identity on pose; disjoint axes commute; actuator
  folding is order invariant; each axis pair shares exactly one outcome source;
  distinct axes have distinct outcome sources; save/restore is identity on the
  next complete step.
- Falsifiers: a fused depth or device-semantic field crosses the boundary; an
  unchanged state is marked as transition; outcomes remain global; one eye or
  digit silently controls another; checkpoint replay differs; core regressions
  change; a run fails to quiesce; or the warm suite reaches 10 seconds.
- Evidence: validated plan, candidate receipt, focused tests, unchanged core
  regression, exact checkpoint replay, and an independently validated
  verification receipt.
- Not applicable because this change provides a development body rather than a
  learned capability or capstone: no capability verdict or external score is
  produced.

## Risks and rollback

- Fifteen independent axes may increase causal work. Keep every surface bounded,
  measure the warm loop, and preserve natural-quiescence assertions.
- A workstation wrapper can become a hidden controller. Keep all device meaning
  in the future external world and expose only light, contact, effort, pose,
  limits, and anonymous crossings.
- Independent eyes may be harder to coordinate than a built-in vergence axis.
  That is intentional: a built-in coordination motor would assume the relation
  the learner is meant to develop.
- The old development checkpoint is deliberately incompatible. Roll back by
  restoring `crates/human`, its workspace member, and its two documentation
  references; accepted core checkpoints and authority artifacts are untouched.

## Open decisions

None.
