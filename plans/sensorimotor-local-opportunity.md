# Test bounded opportunity after an unanswered local return

```text
ordinary local winner
        |
        v
has return still open? -- no --> fire ordinary winner
        |
       yes
        v
one fresh neighbor? ----- no --> preserve ordinary choice
        |
       yes
        v
fire one neighbor
        |
        +-- deferral -----> retain displaced return
        `-- replacement -> close displaced return only
```

## Outcome

Implement the three preregistered choice arms for
`sensorimotor-opportunity-v1`: the unchanged physical reference, local deferral
while a chosen path still awaits outcome, and the composition that closes only
the displaced unanswered return when one fresh neighbor receives the local
opportunity.

Extend the research-only sensorimotor runner to compare these arms at exact
credit, opportunity, delayed-return, far-output, reflection, and one-joint
gates. Resume later body stages only if the replacement arm survives the
single-joint gate. This is candidate physics and discovery evidence, not an
adoption or authority change.

## Authority

- Path: `arch.md`, `LANGUAGE.md`, `algo.md`, `research/constitution.md`,
  `research/programs/learner/program.toml`, and
  `research/campaigns/sensorimotor-opportunity-v1/protocol.toml`
- Revision: authority parent
  `277fa3141a5d028f738984c0ccede76e615ad88e`; source revision
  `67ee08f2cc4b7bd05edc00a8574f484e36aa37d6`; causal-local candidate diff
  `abbe725a1159b77c3cc3c6b8b41c215f3f891e4a9e7889ea2311e301fb099f38`;
  successor protocol SHA-256
  `e6516ec58923aa51a473e5ae0501a3658a2299c49ab732c6e426fbf787f9f26e`

## Model

- `Protocol` selects one coherent choice arrow: ordinary `Physical`,
  `UnansweredReturnDeferral`, or `UnansweredReturnReplacement`. The default
  remains `Physical`; all other algorithm arrows remain identical.
- `OutputCandidate` is an internal observation of one currently ready physical
  output: incidence, position, drive, participation, and the live return links
  from that output's physically wired source to its otherwise admitted used
  path.
- Every configured outcome source holds the used path opened from that source;
  the previous single global source remains the one-source special case.
- Ordinary local ranking is a total deterministic transformation from a local
  candidate group to one winner: strongest drive, then least participation,
  then the existing clock rotation among exact ties.
- Deferral first computes the ordinary winner. Only when that winner has an
  unanswered return does it rerun the same ranking over fresh candidates in the
  same existing local radius. Exactly one winner remains; far groups are
  independent.
- Replacement composes deferral with one effect: after a fresh winner is
  selected, retire only the displaced ordinary winner's live return links.
  Deferral retains them. If no fresh neighbor exists, both protocols preserve
  ordinary choice and close nothing.
- Outcome arrival remains the only strengthening arrow. Supersession creates no
  outcome, strength, reward, or evaluator return. It records a distinct
  physical trace event and bounded deallocation work.
- The experiment maps anonymous outward crossings to neutral joint motion and
  sends back only observed local physical changes. Its stage sequence is exact
  isolation -> opportunity fixture -> single joint -> repeated axes -> digits
  -> binocular -> vocal-auditory -> composition. Failure stops the sequence.

## Invariants

- Input, formation, participation, local outcome wiring, strengthening,
  consolidation, timing, decay, and output integration remain unchanged.
- The default `Protocol::Physical` reproduces the frozen causal-local and
  shuffled traces exactly; all existing builders retain it.
- Waiting state is the actual live participation-born return, not reconstructed
  history, elapsed wall time, an evaluator verdict, or remembered action.
- Locality is existing physical position and competition radius. No action,
  axis, sign, direction, digit, eye, ear, voice, limit, target, score, or
  capability value enters organism state or choice.
- One local group admits at most one output. A group cannot clear a far group's
  inputs or returns.
- Replacement retires only return links belonging to the displaced ordinary
  winner. It cannot strengthen that route or close an unrelated return.
- A delayed outcome remains valid indefinitely within accepted physical
  lifetime when no competing local opportunity intervenes.
- Correct-local and shuffled-local controls preserve source count, timing,
  opportunity, and work except for their frozen wiring difference.
- Checkpoint round-trip preserves protocol and all open returns exactly.
- Research worlds use only the public Harness. Academy and tests never call a
  body directly.
- Exact replay, reference equality, natural quiescence, outward-only
  observation, bounded allocation, and the representative sub-ten-second warm
  loop remain intact.

## Scope

- Change `truelearner/crates/core/src/core.rs` and `physics.rs` to expose and bind
  the three coherent protocols.
- Refactor `truelearner/crates/core/src/choose.rs` around an explicit local
  candidate and shared deterministic ranking; add deferral and replacement.
- Change `truelearner/crates/core/src/hold.rs` so all configured anonymous
  outcome sources preserve their own open used paths, not only the optional
  global fallback source.
- Extend `truelearner/crates/core/src/trace.rs` only if needed to expose exact
  superseded-return closure as a causally inert observation.
- Extend `truelearner/crates/core/tests/harness_boundary.rs` with TDD fixtures
  for unchanged choice, deferral, replacement, delayed return, far-output
  isolation, reflection, and checkpoint replay.
- Extend `research/experiments/sensorimotor-emergence/src/lib.rs` and `main.rs`
  with the successor arms, exact opportunity observations, stage gates, and
  neutral results.
- Preserve one result per launched arm plus one convergence record under
  `research/campaigns/sensorimotor-opportunity-v1/`.

Exclude default-law adoption, Academy evaluator or curriculum changes, human
harness changes, Playground, ARC adapters, semantic action interfaces,
specialist learners, reward, correctness return, target injection, speech,
language, foveation, and authority promotion.

## Development style

TDD. Add focused public-Harness failures for exact deferral and replacement
before changing choice. Keep the frozen reference assertions in the same loop.
Build or run no later sensorimotor stage after an earlier scientific failure.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core unanswered_local_return`
  checks exact deferral, replacement, valid delay, far locality, reflection,
  default compatibility, and checkpoint preservation.
- `cargo test --locked --manifest-path research/experiments/sensorimotor-emergence/Cargo.toml`
  checks all successor arm fixtures, stage stopping, replay, quiescence, and the
  public Harness boundary.
- `cargo test --workspace --locked --manifest-path truelearner/Cargo.toml`
  preserves accepted core and human-body behavior.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`
  preserves the fast Academy body boundary without changing its learner.
- `cargo fmt --all --check`, `cargo check --workspace --locked`, and
  `cargo clippy --workspace --all-targets --locked -- -D warnings` run for the
  TrueLearner manifest; equivalent format, check, and Clippy commands run for
  the independent experiment manifest.

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core unanswered_local_return`

Its measured warm duration must remain strictly under 10 seconds. Record cold
bootstrap and full research stages separately.

## Controls and evidence

- Held-out cases: left-right reflection, reversed creation order, a third far
  output, immediate outcome, delayed outcome without a competitor, delayed
  outcome after physical supersession, both starting limits, changed joint
  scale, checkpoint replay with an open return, and fresh origins.
- Negative controls: unchanged causal-local saturation, shuffled wiring,
  deferral without closure, no fresh neighbor, far ready output, blocked and
  disconnected outputs, equal opposition, passive change, unrelated axis,
  exact replay, and natural quiescence.
- Laws: default choice identity; deterministic local ranking; deferral changes
  choice only when the ordinary winner waits and a fresh neighbor exists;
  replacement equals deferral plus closure of the displaced return; reflection
  commutes with ranking and closure; disjoint local groups compose independently;
  checkpoint round-trip preserves protocol and waiting topology.
- Falsifiers: any frozen trace changes; more than one or a far output receives
  the opportunity; stale or unrelated credit appears; a valid delay is lost
  without competition; either direction or either limit recovery fails; a later
  stage runs after failure; semantic authority enters; replay, reference,
  quiescence, cost, or the warm budget fails.
- Evidence: validated program, forecast, protocol, campaign and Rust plan;
  focused traces; neutral per-arm results; convergence; implementation receipt;
  and an independent verification receipt. Factory receipts remain software
  evidence and are not scientific or authority evidence.

## Risks and rollback

- Treating every open return as failure would break valid delayed outcomes.
  Defer only at an actual later competing local opportunity; test a delay with
  no intervening competitor.
- Filtering all waiting candidates could change choice when the ordinary winner
  is fresh. Compute the ordinary winner first and branch only if that winner
  waits.
- Closing every waiting return in a group would erase unrelated causal state.
  Carry exact return IDs on the displaced winner and close only those IDs.
- The composition may bounce near one limit or accumulate strength without
  reaching the reflected limit. Preserve that trace as a negative result and
  stop the ladder rather than adding persistence inside this campaign.
- Adding protocol variants changes checkpoint shape. This pre-release runtime
  has no public persistence schema; same-candidate exact round-trip remains
  mandatory.
- Roll back the protocol variants, candidate refactor, supersession trace,
  focused tests, and successor experiment together. The default physical and
  causal-local reference paths remain unchanged.

## Open decisions

None.
