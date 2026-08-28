# Hand same-lineage closure renewal diagnostic

```text
successful first closure                        hand first closure
          |                                             |
fresh action -> live return -> surface          fresh movement -> live return -> surface
          |                                             |
          +---- compare actual transition identities ---+
                                |
                     first mismatching transition
```

## Outcome

Add a research-only successor experiment and five-arm diagnostic campaign that compare
the established scripted second closure with the truthful reflected hand after its
first closure. Record actual participation deltas, output, return link and
generation, link lifetime at consequence, admitted origin, reverse consolidation,
closure identity, evidence, and construction. Freeze the first differing physical
transition without changing learner physics, construction thresholds, prior
experiments, or prior evidence.

## Authority

- Path: `lessons.md`, `research/programs/learner/lessons.toml`, and
  `research/campaigns/developmental-hand-construction-admission-v1/convergence.toml`
- Revision: `lessons.md` SHA256
  `7f557e4b11fbc9f1bf404e333d93cb15378b69f66917ef34dee94690f6131fb5`;
  program lessons SHA256
  `e3076d1f3c156eb8ce744763ae943f36d203f9d3f62167a96b4ce4a6cd26e3ad`;
  convergence SHA256
  `778bbfd7bcc2d7d8428ff93bf7d1d39cbf9bc98f9f352d5f37da53e888b3f808`;
  frozen hand artifact SHA256
  `058723a7dc7e7b88087d4572d2452cdee7af9daee82aa9e59117a5df85f2464e`;
  return-reentry artifact SHA256
  `08fe6a67c04a9aa5d35d348f3ba5b3ff4b547ada3d1837a2203eadca6c3f3e34`;
  source revision `b94482267e96baffecc9576ddb6878918d9a4974`

## Model

`ClosureKey` is the observable `(parent, surface, output)` identity.
`ReturnToken` is the observable `(owner, link, generation)` identity within one
return-event role. `ReturnScheduling` names the return edge considered during output
choice, while `ReturnOriginAdmission` names the live modulatory edge struck by an
origin. Core emission sites establish that these are distinct roles and must not be
joined merely because both expose a link and generation.
`RoundTrace` is a pure record derived from an action run, the harness immediately
before its consequence, and the consequence run. It contains output participation,
participation deltas, return-scheduling decisions, live modulatory state at consequence,
admissions of the actually delivered surface origins, reverse consolidations, closure observations, and
construction events.

The scripted reference performs one already-established closure, checkpoints, then
performs exactly one fresh `action + motor -> surface` round. The hand reference uses
the complete symmetric-path plus physical-surface world unchanged. It checkpoints
immediately after its first closure, skips only zero-displacement action attempts,
then captures the first later physical movement and the actual pending surface
delivered for that movement. Skipped attempts remain in the observation and budget;
they are not cleared or treated as episode boundaries.

```text
WorldBeforeFirstClosure
  -> FirstClosureCheckpoint
  -> FreshActionParticipation
  -> ScheduledReturn
  -> ReturnStillLiveAtConsequence
  -> DeliveredSurfaceOriginAdmission
  -> ReverseConsolidation
  -> SameClosureKey
  -> EvidenceTwo
  -> Construction
```

`TransitionStage` orders those predicates. `first_divergence` is the first stage that
survives in the scripted reference and fails in the hand. If an earlier reference
predicate fails, the comparison is inconclusive rather than reinterpreted. Harness
mutation, clock progression, and physical integration remain effect boundaries;
trace extraction and stage comparison are pure functions.

Five arms share one measured evidence object: inherited integrity, scripted two-cycle
reference, hand first-closure reference, the preregistered same-lineage renewal
hypothesis, and first-transition localization. Renewal survives only if the hand
advances the same closure to evidence two and constructs. Localization survives when
both references are intact and exactly one earliest divergence is reported, regardless
of whether renewal survives.

## Invariants

- Do not modify `truelearner-core`, existing experiment crates, existing campaigns,
  existing artifacts, program claims, construction evidence thresholds, or defaults.
- Reproduce the scripted fixture's second closure and construction under
  `RecursiveLearnerConstruction` before interpreting the hand.
- Reproduce the hand's first closure evidence one and the frozen parent arm result
  before interpreting renewal.
- Use only actual output events, participation deltas, return events, link observations,
  and actual post-movement anonymous surfaces. Do not infer a path from intended motion.
- Keep scheduling and origin-admission identities separate because they describe
  distinct return-edge roles. Count renewal admission only when the admitted physical
  origin is one of the anonymous surfaces actually delivered for the selected movement;
  do not count an outcome-origin or unrelated admission.
- Compare closure identity by junction and learner identities produced by the physical
  run; never inject direction, target, score, episode, anatomy, or expected identity.
- Do not clear return history, reset the harness, extend the construction threshold,
  or train the hand on the evaluator's behalf.
- Bound the hand search to the unchanged sixteen-step horizon. A stop or absent later
  movement is a recorded failure, not permission to extend the run.
- Checkpoint replay must cover the first-closure prefix and captured renewal suffix,
  including external position and pending surfaces.
- Preserve natural quiescence and keep the representative warm suite strictly under
  10 seconds.

## Scope

- Add `research/experiments/hand-same-lineage-closure-renewal/` as an isolated Rust
  crate depending on the two immutable parent experiment crates and `truelearner-core`.
- Add protocol, campaign, five arm manifests, artifacts, results, and convergence under
  `research/campaigns/hand-same-lineage-closure-renewal-v1/`.
- Add candidate and independent verification receipts under `factory-artifacts/`.
- Exclude production physics, prior source or evidence mutation, candidate solves,
  hand/finger advancement, Academy, benchmarks, adoption, and authority promotion.

## Development style

TDD. First encode trace identity and pure stage comparison tests. Then implement the
scripted and hand effect boundaries, frozen references, replay, arm classification,
CLI artifacts, campaign envelopes, and convergence. Tests require total scientific
classification and frozen controls; only the positive scripted reference is required
to construct.

## Focused tests

- `cargo test --locked --manifest-path research/experiments/hand-same-lineage-closure-renewal/Cargo.toml transition_order_is_total`
  checks the frozen physical ordering and pure first-divergence function.
- `cargo test --locked --manifest-path research/experiments/hand-same-lineage-closure-renewal/Cargo.toml scripted_second_round_reaches_evidence_two`
  checks fresh output, truthful surface-origin admission, reverse consolidation, same closure,
  and construction in the positive reference.
- `cargo test --locked --manifest-path research/experiments/hand-same-lineage-closure-renewal/Cargo.toml hand_first_closure_matches_frozen_reference`
  checks the evidence-one checkpoint and immutable parent outcome.
- `cargo test --locked --manifest-path research/experiments/hand-same-lineage-closure-renewal/Cargo.toml hand_round_uses_only_actual_movement_and_surface`
  checks that the renewal sample is the first later nonzero displacement plus its
  actual pending anonymous surface, with skipped attempts retained.
- `cargo test --locked --manifest-path research/experiments/hand-same-lineage-closure-renewal/Cargo.toml admission_requires_an_actually_delivered_surface_origin`
  checks that an admitted outcome or unrelated origin cannot satisfy the surface-origin stage.
- `cargo test --locked --manifest-path research/experiments/hand-same-lineage-closure-renewal/Cargo.toml renewal_and_localization_follow_frozen_predicates`
  checks total outcome classification without forcing hand renewal to survive.
- `cargo test --locked --manifest-path research/experiments/hand-same-lineage-closure-renewal/Cargo.toml --lib -- --test-threads=1`
  runs the representative new suite.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`
  preserves active core regressions.
- `cargo run --quiet --locked --manifest-path research/experiments/hand-same-lineage-closure-renewal/Cargo.toml -- --all --output-dir research/campaigns/hand-same-lineage-closure-renewal-v1/artifacts`
  emits all five artifacts once from shared evidence.
- `uv run research/validators/validate_campaign.py --file research/campaigns/hand-same-lineage-closure-renewal-v1/campaign.toml`
  and `uv run research/validators/validate_convergence.py --file research/campaigns/hand-same-lineage-closure-renewal-v1/convergence.toml`
  validate preregistration and complete fan-in.
- `cargo fmt --manifest-path research/experiments/hand-same-lineage-closure-renewal/Cargo.toml -- --check`,
  `cargo check --locked --manifest-path research/experiments/hand-same-lineage-closure-renewal/Cargo.toml --all-targets`, and
  `cargo clippy --locked --manifest-path research/experiments/hand-same-lineage-closure-renewal/Cargo.toml --all-targets -- -D warnings`
  enforce Rust hygiene.

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path research/experiments/hand-same-lineage-closure-renewal/Cargo.toml --lib -- --test-threads=1`.
It must finish strictly under 10 seconds after cold bootstrap; candidate and
verification receipts independently record warm durations.

## Controls and evidence

Held-out cases are an admitted outcome origin, an admitted unrelated physical origin,
and the next zero-displacement hand attempt retained outside the selected movement round.
The positive control is the established scripted second closure. Negative controls
are the frozen failed hand arm, admitted outcome or unrelated origins, zero-displacement
attempts, unrelated admitted origins, unchanged duplicate/stale/disconnected parent
controls, fixed horizon, replay, and quiescence. The killing falsifiers are parent
drift, failure of the scripted reference, semantic identity leakage, evaluator-selected
movement, history reset, conflating scheduling and admission link roles, replay
inequality, over-budget execution, or failure to account for every arm. Expected
evidence is five artifacts, five result envelopes, one convergence record, and
validated candidate and independent verification receipts. No solve, adoption, or
authority evidence is produced.

## Risks and rollback

The hand can emit outputs without external displacement because opposing effects
cancel. The diagnostic therefore records those attempts but samples the first later
actual displacement and its own surface, preventing an intended-output inference.
Multiple returns may be admitted in one run; strict delivered-origin membership
prevents unrelated admission from hiding the break. Copying the frozen physical world
can drift, so the parent artifact and first-closure predicates gate interpretation.
Rollback removes only the new experiment, campaign, and receipts.

## Open decisions

None.
