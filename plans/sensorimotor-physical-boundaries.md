# Test bounded local sensorimotor physics

```text
physical input -> propagated causal origin -> ready local paths
                                             |        |
                              recent consequence    durable drive
                                             |        |
                                             v        v
                                      bounded selection
                                             |
                   finite return <- actual output -> distinct local surfaces
                         |                           |
                         +------ learned cohort ----+
                                             |
                                 active topology only ages
```

## Outcome

Add one explicitly selected experimental protocol and an eleven-arm research-only
ladder. It tests whether recent consequence, finite return lifetime, active-topology
indexing, physical-origin competition, and learned consequential cohorts repair the
five frozen counterexamples and their stage-gated compositions. Default protocols and
adopted authority remain unchanged.

## Authority

- Path: `research/constitution.md`, `research/campaigns/sensorimotor-composition-sweep-v1/convergence.toml`, and `research/campaigns/sensorimotor-physical-boundaries-v1/protocol.toml`
- Revision: `67ee08f2cc4b7bd05edc00a8574f484e36aa37d6`, predecessor convergence `774b159ae1e48f6602eceee426fce2534a2bf24b06272068e3903977fd054af6`, and dirty-tree verification `5772448996a5b4a27ca5d93a12940eae532b2cd0e0ad4f436df99f0f63b995bf`

## Model

`Protocol::SensorimotorCandidate` selects a coherent experimental interpretation.
A firing carries one anonymous physical causal origin. A drive link records its latest
local consequence without replacing durable strength. A temporary modulatory return
has finite resistance and records distinct accepted physical origins. An arena indexes
only links whose resistance is physically aging. Selection maps ready candidates to a
bounded subset: recent same-tick consequence preserves a learned cohort; otherwise one
unlearned alternative per causal origin survives. I/O, time, fixture translation, and
artifact writing remain at `Harness` and experiment boundaries.

## Invariants

- `Physical`, `UnansweredReturnDeferral`, and `UnansweredReturnReplacement` retain their behavior.
- No learner state contains action, direction, body-part, target, score, episode, answer, or evaluator identity.
- Durable link strength is not overwritten by recent eligibility.
- Delay 20 remains creditable while delay 200 is stale.
- Stable dormant topology remains resident but does not enter the candidate aging frontier.
- Independent causal origins do not suppress one another; a learned same-consequence cohort is not split.
- One return accepts a distinct local physical origin at most once and expires naturally.
- Every experiment calls only public `Harness`, replays exactly, and quiesces naturally.
- A body composition is inconclusive after an actual prerequisite failure.

## Scope

- Extend candidate state and bindings in `truelearner/crates/core/src/` and checkpoint serialization.
- Add focused public-boundary regressions in `truelearner/crates/core/tests/harness_boundary.rs`.
- Add `research/experiments/sensorimotor-physical-boundaries/`.
- Complete `research/campaigns/sensorimotor-physical-boundaries-v1/`, program status, and evidence-backed lessons.
- Exclude default-protocol changes, Academy changes, benchmark changes, authority adoption, and frozen predecessor edits.

## Development style

TDD. Add public `Harness` tests for each local law and its negative control before the
candidate implementation, then add the eleven research classifications.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core` establishes default preservation, candidate lifetime, selection, cohort, scaling, replay, and quiescence.
- `cargo test --manifest-path research/experiments/sensorimotor-physical-boundaries/Cargo.toml --lib` establishes all eleven preregistered classifications.
- `cargo run --quiet --manifest-path research/experiments/sensorimotor-physical-boundaries/Cargo.toml -- --all --output-dir research/campaigns/sensorimotor-physical-boundaries-v1/artifacts` produces the frozen batch.
- `uv run research/validators/validate_campaign.py --file research/campaigns/sensorimotor-physical-boundaries-v1/campaign.toml` validates arm lineage.
- `uv run research/validators/validate_convergence.py --file research/campaigns/sensorimotor-physical-boundaries-v1/convergence.toml` validates complete fan-in.

## Development loop

The representative warm regression is
`cargo test --manifest-path research/experiments/sensorimotor-physical-boundaries/Cargo.toml --lib`.
It must remain strictly under 10 seconds; cold bootstrap is recorded separately.

## Controls and evidence

Held-out cases are incumbent ratios, delay 200, 1024 dormant outputs, eight independent
origins, an untrained shared fanout, an either-alone world, duplicate and unrelated
surfaces, reversed order, reflected joint limits, and stage-gated compositions.
Negative controls are unchanged default protocols, no-consequence release, stale
delay, independent origins, untrained fanout, either-alone output, duplicate surface,
unrelated surface, and dormant rather than active body topology.
Falsification is a valid result. Evidence is one immutable JSON artifact and result
envelope per arm, one convergence record, a content-addressed candidate receipt, and an
independent verification receipt.

## Risks and rollback

The main risks are using arbitrary external origin labels as semantics, letting a
candidate index change default decay, or calling a supplied topology learned. Detect
them with physical-origin reflection, unchanged-default controls, trained-versus-
untrained contrasts, exact replay, and artifact reproduction. Rollback removes the
experimental protocol, its extra local state/index, and the successor experiment only.

## Open decisions

None.
