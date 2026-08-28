# Implement the one-joint provenance decomposition

```text
                         learner physics
                    monolithic     recursive
                 +--------------+--------------+
 synthetic       | frozen       | recursion    |
 provenance      | reference    | without truth|
                 +--------------+--------------+
 truthful        | proprio only | complete     |
 provenance      |              | composition  |
                 +--------------+--------------+
                                |
                     factorial localization
```

## Outcome

Refactor the research-only `developmental-hand-one-joint` crate to execute the six
arms of `developmental-hand-proprioception-decomposition-v1`. Measure the four fixed
factorial cells, preserve capacity and warm-runtime failures as physical results, and
attribute trajectory differences only through matched one-variable contrasts. Emit
the complete candidate first, all controls, one factorial localization, artifacts,
results, convergence, and validated implementation receipts. A negative or
over-budget scientific outcome is acceptable; production physics, prior evidence,
finger rungs, adoption, and authority remain unchanged.

## Authority

- Path: `research/campaigns/developmental-hand-proprioception-decomposition-v1/protocol.toml`,
  its campaign and six arm manifests, and
  `research/programs/learner/forecasts/developmental-hand-v1.md`
- Revision: protocol SHA256
  `9ab7a51253c03f1dc1f4f9d1dabfcc5574e174bf9d77d5978467f16b2892bb61`;
  complete recursive candidate tree
  `cf8f2b971d70fafff1873536b05e82f35413b20b0911e462c5eaf50f85a89178`;
  frozen old artifact
  `72fb1c61e54cf71151d8fbf7f998cf9d05aba6c4ab176ccbe59342f169d89ae2`

## Model

Use `Provenance::{Synthetic, Truthful}` and
`LearnerPhysics::{Monolithic, Recursive}` as the two independent axes. Their product
is `FactorialCell`; no boolean bundle may hide which variable changed. A
`FactorialEvidence` owns exactly four `TrialOutcome`s and derives six arm results by
pure evaluation, so CLI `--all` measures each cell once.

One step remains the explicit transformation:

```text
JointState
  -> deliver prior consequence
  -> expose position incidence and motor opportunity
  -> Harness::send
  -> integrate signed output with reflection
  -> schedule only the emitted direction's consequence
  -> either the next JointState and JointStep, or PhysicalStop
```

`TrialOutcome` is `Completed(JointTrial)` or `Stopped(StoppedTrial)`. `PhysicalStop`
distinguishes fixed junction capacity, fixed link capacity, and the warm-time bound.
Wrap `Harness::send` at the research boundary with `catch_unwind`; translate only the
two exact core capacity assertions into typed stops and resume every other panic.
This is sound because `Harness::send` mutates a clone and installs it only after a
successful run. Never continue a stopped world or enlarge its capacity.

All cells use 16,384 junction and 65,536 link slots, identical topology, timing,
initial position, signed integration, reflection, and sixteen-step horizon.
Synthetic provenance reproduces the original sequence-unique tags. Truthful
provenance supplies the physical id of the actual anonymous sensor, directional
outcome, or opportunity surface. Learner physics maps only to the two frozen
`Protocol` values.

Record external world state, outputs, positions, `Work`, `ExecutionCost`, public
physical events, topology counts, replay, and quiescence after every completed step.
Wall time stays at the effect boundary: research artifacts record the bound predicate
and first stopped step; candidate and verification receipts record exact command
duration. Checkpoint replay compares physical histories and canonical bytes but not
wall-clock samples.

The complete truthful-recursive cell runs before removal controls. Run the fixed
`8 -> +4 -> 16` perturbation only if its primary cell closes both limits. The mirrored
`8 -> -4 -> 16` schedule is held-out verification. The factorial evaluator computes:

- provenance main effects: truthful minus synthetic at fixed learner physics;
- recursion main effects: recursive minus monolithic at fixed provenance;
- interaction: whether the truthful-recursive change is explained by neither main
  effect alone;
- earliest incomplete transition: incidence, traversal, candidate admission, output,
  world change, return, construction, private write, private read, continuation,
  reversal, release, or physical cost.

Stopped cells retain their completed prefix and remain usable for cost localization,
but cannot satisfy a capability predicate. Integrity failure gates every scientific
arm to inconclusive. Other negative cells are never repaired and do not prevent the
factorial diagnostic from accounting for their counterexamples.

## Invariants

- Do not modify `truelearner-core`, old experiment sources, old campaign files, old
  artifacts, or the obsolete frozen v1 protocol and plan.
- Reproduce all inherited classifications and the immutable 12/16 reference before
  interpreting the matrix.
- Change exactly one typed axis per main-effect contrast; capacity, topology, timing,
  horizon, integration, and reflection are identical across cells.
- Truthful origin is anonymous physical provenance, never a direction, target,
  desired action, success, score, or semantic anatomy field.
- Only actual output schedules consequence. Perturbation changes external position
  at a fixed step and supplies no consequence.
- Execute exactly sixteen primary steps unless a declared physical stop occurs; never
  stop on success or extend a failed arm.
- Translate only known capacity assertions. Unexpected panics, serialization errors,
  and invariant violations remain software failures.
- Replay includes Harness and external world state; completed prefixes and stops must
  reproduce exactly apart from wall-clock samples.
- A stopped or over-budget cell is scientifically falsified for bounded control, not
  infrastructure-failed or silently omitted.
- Software tests assert total classification and controls, not a desired scientific
  verdict.
- Keep the representative warm suite strictly under 10 seconds; a slower loop fails
  implementation even if correctly reported by a scientific arm.
- Do not run finger, hand, downward-ablation, adoption, or authority work.

## Scope

- Replace the disposable preflight implementation in
  `research/experiments/developmental-hand-one-joint/src/lib.rs` and update its CLI in
  `src/main.rs`; retain its manifest and lockfile unless dependency changes are needed.
- Add artifacts and neutral arm-result envelopes under
  `research/campaigns/developmental-hand-proprioception-decomposition-v1/`, then add
  one convergence accounting for all six arms.
- Add candidate and verification receipts under `factory-artifacts/`.
- Candidate lineage includes the already-present recursive learner working tree and
  research manifests; new Rust behavior remains confined to the experiment crate.
- Exclude core changes, new public APIs, new learner laws, capacity changes after a
  run, old evidence mutation, Academy, benchmarks, later hand rungs, and authority.

## Development style

TDD. First test the two typed axes and exact common fixture. Then establish immutable
reference reproduction and known-capacity-to-`PhysicalStop` conversion. Add one
software test for each factorial cell that accepts its evidence-bearing scientific
classification, followed by pure main-effect/interaction localization tests. Finish
with fixed perturbation gating, CLI all-arm reuse, and evidence envelopes. Never make
a unit test require the complete candidate to survive.

## Focused tests

- `cargo test --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml factorial_cells_change_exactly_one_axis`
  checks the product model and common fixture.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml synthetic_monolithic_reference`
  checks inherited integrity and exact 12/16 reproduction.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml known_capacity_exhaustion_is_a_physical_stop`
  checks typed conversion while unexpected panics remain failures.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml factorial_arm_classifications_follow_frozen_predicates`
  checks all four cells and complete-candidate gating without asserting success.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml provenance_memory_factorial_localization`
  checks main effects, interaction, first-transition ordering, and stopped-prefix use.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml fixed_perturbation_is_conditional_and_external`
  checks the primary gate and no-consequence `8 -> +4 -> 16` schedule.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml --lib -- --test-threads=1`
  runs the complete new software suite with one shared factorial measurement.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`
  preserves every active core regression.
- `cargo run --quiet --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml -- --all --output-dir research/campaigns/developmental-hand-proprioception-decomposition-v1/artifacts`
  executes the complete candidate first and emits all six arm artifacts.
- `uv run research/validators/validate_campaign.py --file research/campaigns/developmental-hand-proprioception-decomposition-v1/campaign.toml`
  and `uv run research/validators/validate_convergence.py --file research/campaigns/developmental-hand-proprioception-decomposition-v1/convergence.toml`
  validate frozen lineage and complete fan-in.
- `cargo fmt --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml -- --check`,
  `cargo check --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml --all-targets`,
  and `cargo clippy --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml --all-targets -- -D warnings`
  enforce Rust hygiene.

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path research/experiments/developmental-hand-one-joint/Cargo.toml --lib -- --test-threads=1`.
It must execute under 10 seconds after cold bootstrap; the candidate and independent
verification receipts record their own warm durations.

## Controls and evidence

Held-out cases are mirrored `8 -> -4 -> 16` perturbation, checkpoint replay after a
completed prefix, capacity exhaustion in each provenance mode, and factorial
localization with one synthetic stopped cell. Negative controls are the immutable old
artifact, synthetic provenance at both protocols, monolithic physics at both
provenances, inherited global/fresh-owner/shifted/stale/disconnected/unrelated/
blocked/withheld/duplicate/dead-generation cases, reflection, fixed capacity,
quiescence, and runtime. Killing falsifiers are reference drift, more than one changed
factor, semantic leakage, hidden success termination, capacity enlargement, swallowed
unexpected panic, replay inequality, unreported cost, and attribution without both
matched contrasts. Expected evidence is six artifacts, six results, one convergence,
and validated candidate and independent verification receipts. No authority evidence
is produced.

## Risks and rollback

The main risk is confusing a capacity assertion with a software defect. Match only
the core's exact capacity messages, preserve the completed prefix, and rethrow every
other payload. Another risk is exceeding the development budget before a stop can be
observed; common fixed capacity, one measurement per process, complete-candidate-first
ordering, and the serial warm suite bound it. Other risks are wall time breaking
artifact replay, duplicated cell execution, a hidden fifth treatment, perturbation
acting as consequence, and scientific assertions in unit tests. Keep timing outside
state equality, derive all arms from one matrix, encode axes as enums, retain fixed
step schedules, and test classification as a pure function. Rollback removes only
the new crate refactor and successor artifacts/results/convergence/receipts; all prior
physics and evidence remain intact.

## Open decisions

None.
