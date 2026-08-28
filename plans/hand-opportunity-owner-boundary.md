```text
donor return owner + recipient candidate owner
                    |
                    v
          actual learner ancestry
                    |
       +------------+-------------+
       |            |             |
   same owner  adjacent boundary  unrelated/sibling
```

# Classify the fresh-opportunity owner boundary

## Outcome

Core diagnostics classify every attempted fresh-opportunity transfer by actual
learner ancestry: same owner, organism-to-root, root-to-organism, parent-to-child,
child-to-parent, siblings, or unrelated. The failed hand pair can then be compared
with pure ancestry controls without changing opportunity admission or behavior.

## Authority

- Path: `research/campaigns/hand-bounded-fresh-opportunity-v1/convergence.toml`
- Revision: `sha256:e53b6b40f897d2ecdf6b757331cefb8a35e7d3acae726d1e97224f3b4e40263c`

## Model

Owner pairs and the learner parent relation map through one total pure classifier to
a typed `LearnerOwnershipRelation`. A fresh-opportunity evaluation records that
relation beside its existing donor owner, recipient owner, and rejection. The
classifier is diagnostic only; the strict owner-equality law remains frozen.

## Invariants

- Classification reads only actual learner IDs and parent links created by causal
  construction; it knows no hand, motor, position, direction, or evaluator state.
- `None -> root` is organism-to-root only when the recipient exists and has no
  learner parent; missing IDs and distinct non-ancestral owners are unrelated.
- Same-owner includes organism-to-organism and exact learner equality.
- Classification changes no selection, return lifetime, opportunity, work, replay,
  or quiescence.
- The failed strict-owner candidate remains behaviorally unchanged and unadopted.

## Scope

- Add the relation enum, pure classifier, one field to the existing diagnostic
  event, adapter evidence, focused controls, and one diagnostic experiment/campaign.
- Exclude any cross-boundary admission, protocol change, ranking, strength, memory,
  hand behavior change, default adoption, commit, or authority promotion.

## Development style

TDD. First classify a synthetic ancestry containing two roots, siblings, child, and
grandchild. Then require every failed hand opportunity pair to report its relation
while retaining exact parent behavior.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml --test harness_boundary opportunity_owner_relation`
  proves the complete relation partition and diagnostic event purity.
- `cargo test --locked --manifest-path research/experiments/hand-opportunity-owner-boundary/Cargo.toml`
  proves the frozen hand relation predicate, replay, quiescence, and parent equality.
- `cargo test --locked --manifest-path research/experiments/hand-bounded-fresh-opportunity/Cargo.toml`
  preserves the falsified predecessor.

## Development loop

`cargo test --locked --manifest-path research/experiments/hand-opportunity-owner-boundary/Cargo.toml`
is the representative warm regression suite and must remain strictly under 10
seconds. Record cold bootstrap separately.

## Controls and evidence

- Held-out cases: organism-to-root, root-to-organism, parent/child in both
  directions, siblings, two unrelated roots, same learner, and unknown IDs.
- Negative controls: unchanged transfer rejection, hand trajectory, old protocols,
  replay, quiescence, propagation, and work.
- Falsifier: any useful hand pair is not organism-to-root or the same label aliases
  sibling/unrelated controls.

## Risks and rollback

Missing learners could be mislabeled as organism adjacency; classify them unrelated.
Rollback removes the enum field, adapter projection, tests, experiment, and campaign.

## Open decisions

None.
