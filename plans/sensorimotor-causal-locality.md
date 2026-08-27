# Test causal locality across sensorimotor worlds

```text
output participates -> local body change -> paired outcome source fires
         |                                      |
         +------ temporary used path <----------+
                              |
                        used links strengthen

global + shuffled controls -> exact isolation -> one joint -> many axes
                                                    |
                              binocular -> voice/hearing -> composition
```

## Outcome

Add the smallest generic mechanism needed to test the preregistered
`SENSORIMOTOR-LOCALITY` claim: an organism may have several anonymous physical
outcome sources, and a participating output may open its temporary return path
from the source physically wired to that output. Preserve the existing single
global source as the unchanged default and reference.

Build a research-only, stage-gated sensorimotor experiment around the public
Harness. It first compares global, correct-local, and shuffled-local return in
an exact two-output fixture. Only a surviving correct-local arm advances to an
anonymous antagonistic joint and then the preregistered multi-axis, binocular,
nonlinguistic vocal-auditory, and composition stages. A stopped ladder is a
scientific result, not an implementation failure.

This creates candidate physics and discovery evidence only. It does not adopt
the law, alter the production default, or claim speech, language, general 3D
understanding, or general motor control.

## Authority

- Path: `arch.md`, `LANGUAGE.md`, `algo.md`, `research/constitution.md`,
  `research/programs/learner/program.toml`, and
  `research/campaigns/sensorimotor-locality-v1/protocol.toml`
- Revision: clean frozen parent
  `277fa3141a5d028f738984c0ccede76e615ad88e`; protocol SHA-256
  `567013ff4fea943da4546277e3c98ec7e98d35d548a3c92a735a4e555d933db4`

## Model

- `OutcomeSources` owns an optional global source plus a deterministic partial
  function from an output junction to a physical outcome-source junction.
- `set_outcome_source` defines the existing constant function: every used path
  returns from one source. `set_outcome_source_for_output` adds one point to the
  partial local function and rejects no valid existing construction.
- When a complete path participates, its second link identifies the physical
  output junction. Looking up that junction selects its local outcome source;
  absence falls back to the global source. The selected source opens the same
  temporary modulatory return path used by accepted physics.
- Firing an outcome source remains ordinary `Input`. Existing outcome return,
  strengthening, deep propagation, decay, time, and quiescence are unchanged.
- Checkpoint save and restore preserve the ordered local wiring exactly and
  reject missing output/source junctions or duplicate output mappings.
- The research experiment owns three topologies: global reference, correct
  local wiring, and a reflected/shuffled local control. It owns body/world
  integration and observer verdicts; the learner receives anonymous physical
  input only.
- Experiment stages form a stopping sequence. Each stage consumes a fresh
  initial organism and produces either `Passed(observations)` or
  `Stopped(falsifier)`. Only `Passed` composes with the next stage.

## Invariants

- The learner still receives only input, forms and chooses paths, fires output,
  receives outcome, strengthens used links, and later reuses paths.
- Local wiring is physical topology, not an action, axis, body-part, capability,
  target, score, or evaluator identifier. No such value enters core state or
  input.
- A return can strengthen only a path that actually participated and opened a
  link from the fired source. The experiment never reconstructs probable paths.
- Correct-local and shuffled-local arms have equal counts, timing, impulses,
  capacities, opportunity input, and work bounds; only physical source wiring
  differs.
- Global-only builders produce the exact parent physical behavior. No existing
  Harness or HumanHarness call site changes.
- Blocked, cancelled, passive, disconnected, unrelated, and external-sound
  controls cannot be reclassified or weakened.
- Checkpoint round-trip, reference equality, reflection, natural quiescence,
  bounded allocation, and boundary-buffer behavior remain exact.
- Research-only worlds remain outside the production dependency graph. Academy
  and tests continue to interact with bodies only through public harnesses.
- Voice is a bounded nonlinguistic physical surface: airflow, amplitude, pitch,
  and tract shape produce delayed pressure/frequency input at two ears. No text,
  phoneme, word, speaker, or correctness state exists.
- The representative warm regression remains strictly under ten seconds.

## Scope

- Change `truelearner/crates/core/src/body.rs`, `core.rs`, `reuse.rs`,
  `outcome.rs`, and `snapshot.rs` to represent, select, count, validate, and
  persist global plus output-local outcome sources.
- Extend `truelearner/crates/core/tests/harness_boundary.rs` with exact global,
  correct-local, shuffled-local, reflection, duplicate, and checkpoint controls.
- Add `research/experiments/sensorimotor-emergence/` as an independent Rust
  package using only the public `truelearner-core` Harness. Its tiny suite
  records exact participation and strengthening; its full runner contains the
  stage-gated neutral joint, repeated-axis, raster/binocular, touch, acoustic,
  vocal, and composition worlds declared by the protocol.
- Record one neutral arm result per executed topology and a convergence record
  that preserves every stopped or falsified stage.

Exclude changes to accepted default physics, `truelearner-human`, Academy
evaluators or worlds, Playground, ARC adapters, semantic action interfaces,
specialist learners, reward, loss, correctness return, target injection,
language, speech recognition, and authority promotion.

## Development style

TDD. First add failing core tests showing that one global outcome credits both
participating paths and that an unavailable local mapping cannot isolate them.
Then implement the typed local mapping until correct-local isolation passes
while the global and shuffled controls retain their preregistered results.
Build later experiment stages only while the preceding scientific gate passes.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core causal_local_outcome`
  establishes exact source-to-participating-path selection, reflection, global
  compatibility, shuffled failure, and checkpoint preservation.
- `cargo test --locked --manifest-path research/experiments/sensorimotor-emergence/Cargo.toml`
  establishes the three-arm tiny fixture, authority firewall, replay, and stage
  stopping rules through the public Harness.
- `cargo test --workspace --locked --manifest-path truelearner/Cargo.toml`
  preserves all accepted core and human-body boundaries.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`
  preserves the fast external body oracles without changing their learner.
- Format, check, and Clippy commands for both Rust manifests establish canonical
  warning-free code.

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core causal_local_outcome`

Its measured warm budget is strictly under 10 seconds. Record cold compilation
and every full sensorimotor stage separately.

## Controls and evidence

- Held-out cases: reflected output positions, delayed local return, two causal
  outputs in one moment, either physical limit, unseen joint scale, unseen
  binocular disparity, changed acoustic delay, ear reflection, and external
  sound identical to self-produced sound.
- Negative controls: global return, shuffled local return, passive change,
  blocked output, equal opposing cancellation, disconnected sensor, unrelated
  moving axis, silent voice, accepted core suite, Academy semantic firewall,
  exact replay, and natural quiescence.
- Laws: global lookup is constant; local lookup is deterministic; local then
  global fallback is left-biased; adding an unrelated mapping cannot change a
  used path; save/restore preserves lookup; reflection commutes with source and
  output reflection; firing source A cannot retire source B's return.
- Falsifiers: any noncausal path strengthens; shuffled wiring passes isolation;
  the reference changes; semantic identity enters the organism; the candidate
  only improves its construction fixture; a later stage is run after an earlier
  stop; replay/reference/quiescence differs; or the warm regression reaches 10
  seconds.
- Evidence: validated research program, protocol and campaign; validated Rust
  plan; exact tiny traces; neutral per-arm result envelopes; stopped-stage
  counterexamples; convergence; candidate receipt; and independent verification
  only if a Rust candidate survives its declared gates.
- Authority evidence is not applicable during this discovery campaign. A
  survivor would require a separately frozen one-shot protocol, adjudication,
  and explicit user authorization before adoption.

## Risks and rollback

- Multiple sources may accidentally become semantic channels. The core stores
  only junction identities and physical topology; research names stay in the
  external observer.
- A local-return candidate may isolate credit yet fail to expose an alternative
  at a limit or suppress simultaneous axes. That falsifies sufficiency while
  preserving the useful causal counterexample; do not add selection machinery
  inside this campaign.
- Checkpoint shape changes even though the default history does not. This
  pre-release repository has no public persistence schema; round-trip and exact
  same-candidate replay remain mandatory.
- Roll back local outcome-source storage, lookup, snapshot fields, public builder
  method, and research experiment together. The global source path remains the
  accepted parent behavior.

## Open decisions

None.
