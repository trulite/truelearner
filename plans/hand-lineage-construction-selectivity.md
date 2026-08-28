# Hand lineage construction selectivity audit

```text
frozen lineage-hand trace ----> causal contract audit ---+
                                                          |
public anonymous fixtures ----> selectivity controls -----+--> frozen verdict
```

## Outcome

Produce a research-only, replayable classification of whether the fourteen learner
constructions in the causal-lineage hand are repeated output-caused physical
boundaries or overconstruction from temporal coactivity and repeated ownership of one
unchanged boundary. Decide whether chronological-first closure identity remains a
valid hand contract and whether causal lineage itself should be narrowed. Do not
change learner physics or promote organism authority.

## Authority

- Path: `research/constitution.md`, `lessons.md`,
  `research/programs/learner/lessons.toml`, and
  `research/campaigns/hand-causal-lineage-preservation-v1/convergence.toml`
- Revision: current working tree plus immutable parent artifact
  `research/campaigns/hand-causal-lineage-preservation-v1/artifacts/hand-causal-lineage-preservation.json`

## Model

States are immutable parent trace records, typed closure keys, construction evidence
pairs, anonymous fixture observations, and one total selectivity verdict.

Transformations are:

- parent JSON -> parsed run, movement, closure, and construction records;
- closure observations -> exact evidence-one/evidence-two pairs by key;
- evidence pair + run kind + preceding physical movement -> causal-contract class;
- public Harness fixture -> construction count, ancestry, boundary identity, replay,
  and quiescence evidence;
- contract audit + controls -> survived, falsified, or inconclusive arm result.

Parsing failure and missing evidence are explicit inconclusive results. File I/O is
limited to loading the compile-time frozen artifact and writing normal experiment
artifacts at the CLI boundary. Classification functions remain pure.

## Invariants

- The causal-lineage protocol, core crate, hand topology, and frozen parent artifact
  remain byte-unchanged.
- The chronologically first closure is never accepted merely because it was first.
- A hand construction counts as output-caused only when both observations for its
  exact key arise from post-movement delivery and the closure output matches the
  physical output responsible for that movement.
- Duplicate lineage in one moment cannot supply two observations.
- Disconnected matched activity cannot construct.
- Repeating one physical surface/output boundary is reported separately from adding
  a distinct physical surface; learner depth is not treated as boundary novelty.
- Evaluator knowledge is used only to classify a completed immutable trace, never as
  learner input or runtime control.
- Every run replays exactly and settles naturally; the representative warm suite is
  strictly under ten seconds.

## Scope

- Add `research/experiments/hand-lineage-construction-selectivity/`.
- Add `research/campaigns/hand-lineage-construction-selectivity-v1/` manifests,
  frozen artifacts, results, and convergence.
- Add factory plan, candidate, and verification receipts.
- Exclude all changes under `truelearner/crates/core/`, all default-protocol changes,
  hand-world changes, authority promotion, reflected-joint retries, and mechanism
  solves.

## Development style

TDD. Encode frozen-trace classification and anonymous negative controls before the
experiment CLI so an attractive construction count cannot weaken the contract.

## Focused tests

- `cargo test --manifest-path research/experiments/hand-lineage-construction-selectivity/Cargo.toml`
  proves exact parent parsing, contract classification, temporal/disconnected/
  duplicate controls, same-boundary depth accounting, exact replay, and quiescence.
- `cargo clippy --manifest-path research/experiments/hand-lineage-construction-selectivity/Cargo.toml --all-targets -- -D warnings`
  proves the research crate is warning-free.
- `uv run research/validators/validate_campaign.py --file research/campaigns/hand-lineage-construction-selectivity-v1/campaign.toml`
  proves every frozen arm is accounted for.
- `uv run research/validators/validate_convergence.py --file research/campaigns/hand-lineage-construction-selectivity-v1/convergence.toml`
  proves the round converges without mutating the parent result.

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path research/experiments/hand-lineage-construction-selectivity/Cargo.toml --lib -- --test-threads=1`.
It must complete strictly under 10 seconds after cold bootstrap; candidate and
verification receipts record warm durations separately.

## Controls and evidence

Held-out cases are reversed distinct-surface delivery order and a longer repetition
horizon; both must preserve boundary classification. Negative controls are
disconnected matched activity, duplicate same-moment lineage,
pre-movement/current-state coactivity, unchanged frozen parent digest, replay, and
natural quiescence. Killing falsifiers are any parent mutation; any accepted
construction lacking two exact output-caused post-movement observations;
disconnected or duplicate construction; boundary novelty inferred solely from a new
learner ID or parent depth; or a result that requires learner-visible semantic
anatomy. Expected evidence is four arm artifacts, four result envelopes, one
convergence record, and validated factory candidate and verification receipts.

## Risks and rollback

The main risk is confusing evaluator-side retrospective causality with admissible
learner physics. Detect it by keeping classification post-run and excluding it from
Harness inputs. Another risk is treating intentionally recursive identical learners
as new physical boundaries; report both depth and distinct physical boundary keys.
Rollback removes only the new plan, experiment, campaign, and receipts because core
behavior is out of scope.

## Open decisions

None.
