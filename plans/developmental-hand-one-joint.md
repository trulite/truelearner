# Implement the first developmental-hand rung

```text
frozen one-joint world + fixed sixteen-step schedule
                         |
              +----------+----------+
              |                     |
      monolithic protocol     recursive protocol
              |                     |
       reproduce 12/16       truthful causal origins
       negative reference           |
              |              owner-local closed loop
              +----------+----------+
                         |
                  matched step trace
                         |
          classify success or first broken transition
```

## Outcome

Add a research-only Rust experiment that executes the four preregistered
`developmental-hand-one-joint-v1` arms. It must reproduce the frozen monolithic
one-joint negative result, run the complete recursive candidate in the matched
anonymous reflected world, apply one fixed physical perturbation, and emit enough
step and physical-transition evidence to identify the earliest repaired or still
missing transition. The experiment may report a survived, falsified, or inconclusive
scientific arm; implementation success means that the frozen test was executed and
classified faithfully, not that the hand claim passed. No production learner law,
default protocol, later finger rung, adoption, or authority changes.

## Authority

- Path: `research/campaigns/developmental-hand-one-joint-v1/protocol.toml`,
  `research/campaigns/developmental-hand-one-joint-v1/campaign.toml`, its four arm
  manifests, and `research/programs/learner/forecasts/developmental-hand-v1.md`
- Revision: Git parent `b94482267e96baffecc9576ddb6878918d9a4974` plus complete
  recursive candidate tree
  `cf8f2b971d70fafff1873536b05e82f35413b20b0911e462c5eaf50f85a89178`;
  frozen protocol SHA256
  `b63f7b6ade50820d4141e66281cfcdfab07884a1774d52e7f713334038044470`;
  old one-joint artifact SHA256
  `72fb1c61e54cf71151d8fbf7f998cf9d05aba6c4ab176ccbe59342f169d89ae2`

## Model

Represent the fixture with ordinary domain types: `Arm` for the four frozen arms,
`JointWorld` for the external joint plus `Harness`, `JointStep` for one immutable
physical step, `JointTrial` for a fixed trajectory and replay evidence,
`PerturbationTrial` for the separate recovery schedule, and `TransitionStage` for the
ordered first-failure descent. Keep evaluation as a pure fold over completed traces;
it may never feed a direction, result, or stopping signal back into the learner.

One world step is the composition:

```text
(Harness, position, pending return)
  -> deliver prior physical consequence
  -> expose current anonymous position incidence and motor opportunity
  -> execute Harness
  -> integrate signed output into position with reflection
  -> schedule the actual participating directional consequence
  -> (Harness', position', pending return') + JointStep
```

Copy the frozen one-axis topology, initial position `0`, limits `[-4, +4]`, two signed
motors, local output-to-consequence wiring, tick ordering, signed integration, and
sixteen-step primary horizon into the new crate without modifying the old experiment.
`Protocol::SensorimotorSynthesis` must reproduce the existing aggregate exactly.
`Protocol::RecursiveLearnerConstruction` uses the same targets, timings, positions,
reflection, and horizon. Its physical provenance is truthful rather than semantic:
directional consequence inputs and current motor opportunity use the corresponding
anonymous outcome junction's physical id, permitting ownership to arise from the
actual causal round trip. A matched synthesis provenance-control must prove this
origin substitution alone does not alter the frozen monolithic aggregate.

Freeze the perturbation trial before implementation: begin from the same initial
world, execute exactly 8 ordinary steps, set only the external position to `+4`, and
execute exactly 16 further ordinary steps. Recovery requires leaving `+4`, observing
both signed movements after perturbation, reaching `-4`, and later leaving `-4`.
The mirrored held-out trial sets the position to `-4` at the same boundary and checks
the reflected predicates, but is verification evidence rather than a second valid
campaign run.

The locator compares completed monolithic and recursive traces in this fixed order:
external incidence; local proposal/traversal; candidate admission; output; world
change; return scheduling/admission; causal closure and deepest owner; private
consequence write; later private read; continuation; reversal; release. Each stage
is derived only from known inputs, outputs, public `Work`, and public
`PhysicalEvent`s. Missing evidence yields a typed `Unobserved` stage; simultaneous
unmatched changes yield `Ambiguous` rather than an invented unique cause.

The inherited-integrity arm composes the existing synthesis and recursive-control
probe APIs and their frozen expected classifications. The monolithic-reference arm
requires the exact old negative result. The recursive-closure arm evaluates the
primary and perturbation predicates. The localization arm succeeds only when it can
name the earliest evidence-backed differing stage, otherwise it is falsified or
inconclusive according to its frozen parents. Parent control failure gates dependent
arms to `inconclusive`; no failed reference is repaired.

## Invariants

- The old synthesis crate, old artifact, accepted core behavior, and complete
  recursive candidate remain byte-for-byte unmodified by this plan.
- The new monolithic execution reproduces 16 steps, 12 changed steps, both signs,
  neither limit reached or escaped, exact replay, and natural quiescence before any
  recursive observation is interpreted.
- Both protocol modes share topology, target junctions, timing, starting position,
  motor integration, reflection, and fixed horizons; only the selected protocol and
  anonymous truthful provenance required for causal ownership may differ.
- The synthesis provenance-control must equal the original monolithic aggregate, so
  physical-origin relabeling cannot be mistaken for learner improvement.
- Proprioception remains current physical incidence. The fixture stores external
  position for world integration but supplies no desired direction, target pose,
  joint, digit, hand, score, success, or curriculum identity to `Harness`.
- Only actual emitted output schedules directional consequence. No movement means no
  consequence; imposed perturbation itself supplies none.
- Primary execution is exactly 16 steps. Perturbation occurs after exactly 8 steps at
  `+4`, followed by exactly 16 recovery steps. Evaluation never terminates a run.
- Checkpoint replay includes learner state and the external `JointWorld` state and
  produces identical histories and canonical Harness bytes.
- Every step records output, position, limit and escape flags, relevant public
  physical events, quiescence, work, and execution cost without direct private-state
  access.
- Parent or sibling isolation is inherited from the frozen recursive-control arm;
  this one-joint fixture does not fabricate a sibling or claim sibling composition.
- Scientific failure is retained. No core change or solve mechanism is admitted in
  this campaign after observing the trajectory.
- All dependent hand rungs and downward ablations remain stopped.

## Scope

- Add `research/experiments/developmental-hand-one-joint/Cargo.toml`, lockfile,
  `src/lib.rs`, and `src/main.rs`. Depend on `truelearner-core`,
  `sensorimotor-synthesis-ladder`, and
  `recursive-learner-proprioceptive-control`; reuse their public frozen arm results
  and implement only the matched research world and evaluator locally.
- Add immutable discovery artifacts and arm-result envelopes under
  `research/campaigns/developmental-hand-one-joint-v1/`, followed by one convergence
  manifest after all four arms are accounted for.
- Add candidate and independent verification receipts under `factory-artifacts/`.
- The candidate receipt may include the already-present recursive-candidate working
  tree in its lineage, but new Rust edits are limited to the new experiment crate.
- Exclude changes to `truelearner/crates/core`, old experiment sources or evidence,
  public learner APIs, protocols, thresholds, lifetimes, Academy, benchmarks,
  production adoption, authority evidence, two-joint fingers, digits, and hands.

## Development style

TDD. First encode the exact monolithic aggregate and the provenance-equivalence
control. Then test deterministic primary/replay and fixed perturbation scheduling
without asserting scientific success. Add arm-classification tests that assert the
reported outcome is the total function of the frozen predicates, so a genuine
negative result still passes the software suite. Finally add transition-localization,
CLI artifact emission, and immutable result envelopes.

## Focused tests

- `uv run factory/validators/validate_plan.py --file plans/developmental-hand-one-joint.md`
  validates this decision-bearing plan before Rust edits.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml monolithic_reference_reproduces_frozen_result`
  establishes exact 12/16 reference reproduction and synthesis provenance equality.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml fixed_perturbation_schedule_is_external_and_deterministic`
  establishes the `8 -> +4 -> 16` schedule, no perturbation consequence, and replay.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml arm_classification_matches_frozen_predicates`
  establishes total survived/falsified/inconclusive classification without requiring
  a positive research outcome.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml one_joint_transition_localization`
  establishes ordered evidence localization and explicit ambiguous/unobserved cases.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml`
  runs every new fixture and CLI-facing probe.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`
  preserves all active core behavior.
- `cargo run --quiet --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml -- --all --output-dir research/campaigns/developmental-hand-one-joint-v1/artifacts`
  emits one deterministic artifact for each frozen arm.
- `uv run research/validators/validate_campaign.py --file research/campaigns/developmental-hand-one-joint-v1/campaign.toml`
  and `uv run research/validators/validate_convergence.py --file research/campaigns/developmental-hand-one-joint-v1/convergence.toml`
  validate lineage and complete four-arm fan-in after execution.
- `cargo fmt --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml -- --check`,
  `cargo check --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml --all-targets`,
  and `cargo clippy --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml --all-targets -- -D warnings`
  establish strict Rust hygiene.

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml --lib`.
Its measured warm execution must remain strictly under 10 seconds; cold dependency
bootstrap is recorded separately.

## Controls and evidence

Held-out cases are the mirrored `8 -> -4 -> 16` perturbation trial, truthful-origin
substitution under the synthesis protocol, replay from a checkpoint containing both
Harness and external world state, first preference just inside and outside the recent
window as exposed by inherited probes, and changed dormant capacity without changed
active topology. Negative controls are the exact old monolithic result, fresh owner
before consequence, global consequence reference, absent/shifted/stale/disconnected/
unrelated proprioception, blocked output, withheld/duplicate/unrelated/dead-generation
return, reflection, reversed order, and parent/sibling before-after state inherited
from the complete candidate. Killing falsifiers are reference drift, a changed
schedule or physical law, success without truthful participation, missing replay or
quiescence, semantic or evaluator leakage, unlocalizable simultaneous changes, and
any repair after observation. Expected evidence is four deterministic artifacts,
four immutable arm-result envelopes, one convergence, one validated candidate
receipt, and one independent verification receipt. No authority evidence is
produced.

## Risks and rollback

The main risk is an unfair comparison: recursive ownership needs truthful physical
origins while the old fixture used synthetic per-step origin tags. The separate
synthesis provenance-control must prove origin substitution preserves the exact old
aggregate before recursive improvement can be interpreted. Other risks are copying
the frozen topology incorrectly, treating imposed position as consequence, allowing
the evaluator to stop on success, mistaking aggregate oscillation for a localized
transition, losing external world state during replay, or asserting research success
in a software test. Exact parity assertions, fixed step counts, no-consequence
perturbation, complete trace ordering, ambiguous/unobserved locator states, and
classification-by-predicate tests detect these failures. Rollback removes only the
new experiment, its new artifacts/results/convergence, and its receipts; the existing
recursive candidate and every prior artifact remain intact.

## Open decisions

None.
