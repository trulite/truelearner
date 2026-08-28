# Give recursive learners local proprioceptive consequence control

```text
current physical proprioception ----+
                                    v
executable owned candidate -> motor threshold -> actual output
                                    |
                                    v
                         accepted physical return
                                    |
                                    v
                 private recent consequence record
                                    |
                          later same-owner read
                                    v
                         local candidate preference
```

## Outcome

Let a constructed child or grandchild combine current anonymous proprioception with
its own bounded recent consequence history. Construction creates empty control
memory; an actual accepted owner-local return records the participating path
generation and tick; later current same-owner proprioception reads that record to
prefer the consequential alternative. Parent-global and other learners' preference
is neither copied nor read, and stale preference becomes inert. This establishes a
bounded learner-local proprioception/write/read/release transition only; complete
bidirectional limit control, recruitment, nested consequence, sibling coalitions,
body composition, adoption, and authority remain excluded.

## Authority

- Path: `research/campaigns/recursive-learner-proprioceptive-control-v1/protocol.toml`,
  `research/campaigns/recursive-learner-proprioceptive-control-v1/campaign.toml`, and
  `research/campaigns/recursive-learner-fresh-memory-v1/convergence.toml`
- Revision: supported fresh-memory candidate tree
  `b970cd48b76c61ea6413b15510896ef639e9869dba67848ab429d95366177f3c`;
  frozen proprioceptive-control protocol SHA256
  `ff8ad9ce27fefe9876f7cb4d9cfd4ce47983531e3d47793c65e26663dd1dbbc2`

## Model

Keep proprioception and learned consequence as different states. Proprioception is a
`Firing` in the current `Moment`; it is never copied into `LearnerState`. Add a
private sorted `LearnerConsequenceMemory` entry containing only `LinkId`, live
`Generation`, and `last_consequence_tick`. Ownership is implicit in the containing
learner, and construction always creates an empty entry set.

For an output candidate, partition current firings into executable path drive and
non-path proprioceptive opportunity. Resolve the path owner from the truthful causal
origin junction and deepest learner membership. An organism candidate retains the
existing integration and ranking exactly. An owned candidate may use only positive
non-path incidence whose truthful origin resolves to the same owner in the current
moment; held, absent, shifted, disconnected, and other-owner incidence contributes
no owned opportunity. Physical path drive still establishes executability and motor
threshold, but it is not an inherited preference tie-break for an owned learner.

An accepted return already resolves its deepest owner and exact participating local
links. Preserve existing physical strengthening and reverse consolidation, then
append or update that owner's sorted `(link, generation, tick)` consequence entry
for only the live paths actually updated. Missing owner retains existing global
`LinkState.last_consequence_tick` behavior. Duplicate, stale, invalid-origin,
nonparticipating, and blocked returns write nothing.

Owned selection maps each candidate's executable links to the newest owner-local
consequence tick within `RECENT_ELIGIBILITY_TICKS`. One uniquely newest candidate is
preferred. With no recent private entry, candidates use deterministic physical
rotation only, ignoring global consequence recency, global learned drive magnitude,
and inherited participation as preference. Historical entries remain serialized but
become inert after the window. The transformations compose as:

```text
(current incidence, causal membership) -> owned opportunity
(accepted return, participating links)  -> private consequence write
(owned candidates, private memory)      -> local preference read
(tick beyond recent window)             -> neutral owned selection
```

Wrong protocol, missing or ambiguous owner, mixed-owner opportunity, stale link
generation, absent live path, insufficient threshold, invalid return, and physical
resource exhaustion remain no-ops. Emit owner-bearing trace events for current
proprioceptive admission, private consequence write, and private selection read; no
public operation may name, read, or write learner memory.

## Invariants

- CORE1, every accepted protocol, sensorimotor current-opportunity behavior, and the
  supported fresh-memory construction candidate retain exact behavior, replay,
  quiescence, and cost outside recursive owned selection.
- Every learner has the same control and construction law. A child inherits causal
  membership and executable physical topology but no consequence entry, recent
  preference, confidence, credit, or parent selection history.
- Proprioception remains current physical incidence. No position, direction, body,
  target, desired action, or sensor value is stored as learner memory.
- Owned opportunity requires same-moment, same-owner, positive non-path incidence;
  held, shifted, stale, disconnected, and unrelated incidence cannot substitute.
- Only actual accepted consequence over a participating live link generation writes
  private memory. One return cannot write another owner or a nonparticipant.
- Once an owner exists, its candidate ranking reads only that owner's recent entries;
  global link recency, learned magnitude, parent memory, and sibling memory cannot
  act as preference.
- With no recent local entry, owned selection is physically deterministic but
  preference-neutral; after the bounded window the same neutral rule resumes.
- Private entries are sorted, unique by live link generation, bounded by actual
  consequence participation, checkpointed byte-exactly, and included in recursive
  memory accounting.
- Current proprioception, consequence write, selection read, output, and release are
  separately observable without evaluator routing or direct learner access.
- Public Harness-only tests preserve absent, shifted, stale, disconnected,
  unrelated, withheld-consequence, duplicate-return, reflection, replay,
  quiescence, parent-isolation, sibling-isolation, and dormant-surface controls.
- Stop the campaign at the first failed primitive; do not interpret recursive or
  sibling composition after proprioceptive gating, consequence write, selection
  read, or release fails.

## Scope

- Extend private learner state and owner queries in
  `truelearner/crates/core/src/learner.rs` with empty-at-construction recent
  consequence memory and total owner-local read/write helpers.
- Partition owned path drive from current same-owner opportunity and route recursive
  candidate ranking through private consequence recency in
  `truelearner/crates/core/src/choose.rs`; leave organism and other protocols exact.
- Route only accepted participating consequence updates to private owner memory in
  `truelearner/crates/core/src/outcome.rs`, preserving existing global strengthening,
  reverse consolidation, and no-owner behavior.
- Validate and restore sorted entries in
  `truelearner/crates/core/src/snapshot.rs`, include their capacity in memory
  accounting, and add owner-bearing physical events in
  `truelearner/crates/core/src/trace.rs`.
- Add public-boundary TDD regressions in
  `truelearner/crates/core/tests/harness_boundary.rs`.
- Add `research/experiments/recursive-learner-proprioceptive-control/` with the four
  frozen arms, controls, gating, and deterministic artifact output; after execution,
  add immutable arm results and convergence plus factory candidate and independent
  verification receipts.
- Preserve every fresh-memory and earlier negative artifact unchanged. Exclude new
  public learner-addressing APIs, semantic controllers, direct memory observation,
  closure or return threshold changes, global link representation changes, Academy,
  benchmarks, default adoption, and authority evidence.

## Development style

TDD. First add one public-Harness test that constructs a child, proves a current
same-owner proprioceptive incidence completes a threshold that path drive alone
cannot, and proves absent, shifted, and unrelated incidence do not. Then freeze a
checkpoint, give one physical alternative an actual child-owned consequence, and
require later matching current proprioception to select it while a matched untrained
replay remains neutral. Finally require bounded release, empty grandchild preference,
parent and opposite-trained sibling isolation, exact replay, and local cost before
building campaign probes.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --test harness_boundary recursive_learner_proprioceptive`
  establishes current same-owner gating, invalid-timing controls, private write/read,
  neutral birth, release, parent and sibling isolation, depth symmetry, replay, and
  quiescence through public APIs.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`
  preserves every active core regression and accepted protocol.
- `cargo test --manifest-path research/experiments/recursive-learner-proprioceptive-control/Cargo.toml`
  establishes all four arm entrypoints, frozen controls, first-failure gating, and
  deterministic classifications.
- `cargo run --quiet --manifest-path research/experiments/recursive-learner-proprioceptive-control/Cargo.toml -- --all --output-dir research/campaigns/recursive-learner-proprioceptive-control-v1/artifacts`
  emits one immutable discovery artifact per declared arm.
- `uv run research/validators/validate_campaign.py --file research/campaigns/recursive-learner-proprioceptive-control-v1/campaign.toml`
  and `uv run research/validators/validate_convergence.py --file research/campaigns/recursive-learner-proprioceptive-control-v1/convergence.toml`
  establish frozen lineage and complete fan-in after execution.
- `cargo clippy --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --all-targets -- -D warnings`
  and both affected-manifest formatting checks establish strict Rust hygiene.

## Development loop

The representative warm regression suite is
`cargo test --manifest-path research/experiments/recursive-learner-proprioceptive-control/Cargo.toml --lib`.
Its measured execution must remain strictly under 10 seconds; record cold bootstrap
separately.

## Controls and evidence

Held-out cases are current versus one-tick-shifted proprioception, child memory across
checkpoint, preference just inside and outside the recent window, child and
grandchild training in reversed order, parent behavior before and after child
learning, two independently owned surfaces trained oppositely, reflection, and 4,
64, and 1024 dormant surfaces. Negative controls are global-link preference, fresh
child before consequence, path drive without proprioception, proprioception without
path drive, unrelated and mixed-owner incidence, blocked output, withheld
consequence, duplicate and stale return, nonparticipating alternative, and direct
learner access. Killing falsifiers are inherited preference at birth, shifted or
unrelated opportunity admission, private writes without participation, selection
that ignores the write or reads another owner, permanent preference lock, replay
inequality, quiescence loss, global work growth, or a dependent arm running after a
primitive failure. Expected evidence is four artifacts and results, one convergence,
and validated candidate and independent verification receipts. No authority evidence
is produced.

## Risks and rollback

The primary risk is disguising global link history as learner-local control. Matched
fresh-child, parent-before-after, sibling, and global-reference traces must show the
owner-local write and read while the other owners retain their classifications.
Other risks are treating held activation as current proprioception, allowing mixed
owners to complete threshold, recording nonparticipating paths, copying memory at
construction, permanent recency lock, ambiguous owner resolution, corrupt checkpoint
order, and changing accepted sensorimotor behavior. Current-moment partitioning,
truthful causal-origin ownership, exact live generations, sorted private entries,
bounded reads, frozen references, and negative controls detect them. Rollback removes
private consequence memory and owned gating/ranking while retaining the supported
fresh-return-memory candidate and all preserved evidence.

## Open decisions

None.
