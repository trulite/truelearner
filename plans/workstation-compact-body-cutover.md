# Cut Workstation and Academy over to the compact Body

```text
Academy Body Discovery -> WorkstationHarness -> compact Body
Academy Workstation ----^          |
                                   +-> private sensor/motor handles
```

## Outcome

Make the runnable Academy Body Discovery and physical Workstation paths use a
`WorkstationHarness` that owns `truelearner_body::Body` directly. Remove the old
core, old Harness API, core-dependent Embodiment wiring, compatibility exports,
research-only Workstation variants, and Academy packages whose live organism
path constructs the old Harness. Preserve ordinary physical input, outward
effects, natural quiescence, transactional transitions, exact replay, corrupt
checkpoint rejection, and the Academy semantic firewall. This is an execution
and external-world adapter cutover, not new learner physics.

## Authority

- Path: `academy.md`, `arch.md`, `LANGUAGE.md`, `algo.md`, the 28 compact-body
  black-box laws, and the existing non-research Workstation and Academy Body
  Discovery tests.
- Revision: Git `ded2e725622e270ad0d414dc433d1ee965f8145d`; authority document
  digests are recorded in `plans/academy-workstation-harness-boundary.md`.

## Model

`WorkstationHarness` owns a compact `Body`, private sensor/motor/effect handles,
the external `WorkstationState`, physical time, pending returned axes, and the
ordinary sample history needed for a durable deterministic checkpoint. Build
attaches motors, then sensors near those motors, then one outcome component.
Step lowers a `WorldSample` and one generic movement opportunity to anonymous
arrivals, runs to quiet, maps outward effects to opposed actuator effort,
integrates the external body state, and records actual changed axes for return
on the next step. Transition is clone-step composition, so failure leaves the
source unchanged.

A checkpoint is an opaque checksummed encoding of the initial rule and admitted
sample history. Restore rebuilds the same morphology and replays that history
through the real Body. Fingerprints hash the same owned history and state.
Rendering, devices, capability verdicts, and evaluator state stay outside.

The production graph retains `body`, `checkpoint`, `behavior-contract`, and
`workstation`. Academy retains Body Discovery, Workstation world/review,
portable review, and Playground. Historical research files are not rewritten
or used as production dependencies.

## Invariants

- Workstation depends directly on `truelearner-body` and not on
  `truelearner-core` or `truelearner-embodiment`.
- Academy production crates depend on Workstation only and never on Body.
- Workstation's Body and all junction handles remain private.
- Construction is motors, sensors, nearness, and outcome components only; no
  old protocol, resistance, region, builder, or semantic link mode survives.
- One generic movement opportunity contains no action choice or evaluator
  knowledge. Academy never injects a direction or selected motor.
- Only actual outward effects become effort, and only actual pose changes are
  returned as outcomes.
- Every successful step reaches natural quiescence; invalid samples and run
  failures are transactional.
- Save/decode/restore reproduces the exact next observation and checkpoint;
  corruption, truncation, and trailing bytes fail closed.
- Existing compact-body behavioral laws remain green.
- Existing non-research Workstation, Academy Body Discovery, and Academy
  Workstation semantic-firewall/replay controls remain green without weakened
  expectations.
- Archived research evidence is not edited. Removed source remains recoverable
  from Git history.
- The representative warm regression is strictly under 10 seconds.

## Scope

- Rewrite `truelearner/crates/workstation` production source, manifest, and
  non-research tests around `truelearner-body`.
- Move the small opposed-effort value into Workstation state.
- Replace Workstation checkpoint versions with one clean history-replay format.
- Remove `truelearner/crates/core` and `truelearner/crates/embodiment`; remove
  them from the TrueLearner workspace.
- Remove old-only Academy packages `academy-core`, `academy-arc3`,
  `academy-episodes`, and `academy-runner`; remove their workspace edges and the
  unused `academy-core` dependency from Academy Body.
- Remove research-only Workstation/Academy feature surfaces and tests.
- Update lockfiles, current architecture wording, and factory receipts.
- Exclude compact Body physics changes, capability expectation changes,
  historical research artifact deletion, rendering/UI behavior changes, and
  new capability claims.

## Development style

Use TDD against the already-passing compact-body laws and unchanged public
non-research Workstation/Academy tests. Replace the private Workstation runtime,
make checkpoint/replay pass, then remove unreachable old crates and require
zero old production dependencies.

## Focused tests

- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --test behavior_contract`
  preserves all shared compact-body behavioral laws.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-workstation`
  checks exploration, transactional transition, state integration, checkpoint
  replay, corruption, and physical morphology.
- `cargo test --manifest-path academy/Cargo.toml -p academy-body -p academy-workstation`
  checks Body Discovery, semantic firewall, session replay, world interaction,
  and checkpoint composition.
- `cargo check --manifest-path truelearner/Cargo.toml --workspace` and
  `cargo check --manifest-path academy/Cargo.toml --workspace --exclude academy-playground`
  check both surviving production graphs.
- Zero-match manifest/source scans for `truelearner-core`, `truelearner_core`,
  `truelearner-embodiment`, old `Protocol`, and research Harness constructors
  establish removal.

## Development loop

The representative warm regression is
`cargo test --manifest-path truelearner/Cargo.toml -p truelearner-workstation --lib`.
Run it after one bootstrap and require elapsed wall time strictly under 10
seconds. Record cold bootstrap separately.

## Controls and evidence

Held-out controls are Academy Body Discovery exact replay, Academy Workstation
session replay, unchanged-sample world behavior, and dormant/corrupt input
cases. Negative controls are evaluator-field serialization scans, invalid world
samples, corrupt/truncated/trailing checkpoints, reads before and after repeated
observation, and a changed replay sample. The cutover is falsified by an old
production dependency, any weakened existing expectation, non-quiescent
success, replay mismatch, evaluator leakage, compact-body law failure, or warm
regression at 10 seconds. Evidence is validated candidate and independent
verification receipts; no learner authority or capability promotion is made.

## Risks and rollback

The new Body may expose the first real Workstation behavioral mismatch once the
old scaffolding is removed. Preserve that failure rather than adding an action
choice or hidden teaching route. Checkpoint replay is initially linear in
history length; record that limitation and replace it only with an opaque Body
snapshot in a later measured change. Git contains every removed source file;
rollback restores the removed workspace members and manifests.

## Open decisions

None.
