# Add ARC-AGI-3 as the Academy capstone

```text
server-selected ARC world
       | frame + available physical actions
       v
Python world adapter <-> JSONL capstone protocol <-> Rust agent -> Harness
       ^                                                    |
       |                     outward crossing --------------+
       |
       +-> official scorecard + inert transcript -> capstone receipt
```

## Outcome

Academy has one first-class ARC-AGI-3 capstone command. It runs the frozen
organism through a teaching-free, harness-only agent against the official
server-selected game suite, closes the official scorecard, replays the recorded
physical inputs through fresh agents, and writes one immutable JSON receipt.

The receipt reports the official ARC score beside actions, crossings, physical
work, learning updates, memory, quiescence, body fingerprints, replay, and the
first observed failure. A public `ls20` smoke uses the same boundary but is not a
capstone result. Runtime performance measurement remains a separately named
engineering lane.

## Authority

- Path: `academy.md`; `arch.md` Boundaries; `.agents/skills/academy/SKILL.md`;
  `.agents/skills/benchmark-climb/SKILL.md`; official ARC-AGI-3 toolkit and
  scoring documentation at `https://github.com/arcprize/ARC-AGI` and
  `https://docs.arcprize.org/methodology`
- Revision: clean pre-capstone commit
  `b0e61e9e591fdff648a8394be6300e6f05494103`; `academy.md` SHA-256
  `90358f599caf4baa7fcf92c8c11cb01d8d192656dc27aef5726759422eddc730`;
  `arch.md` SHA-256
  `f5b1157b8d0479cb9575980f148090932043ce7d8c9da72d70fdb887cb4b4963`;
  official toolkit source revision
  `f12822c4d550121c35a275008d964afbbed47d2f`, `arc-agi==0.9.8`, and
  `arcengine==0.9.3`

## Model

- `CapstoneRequest` is only a validated 64×64 palette frame plus the official
  available-action boundary. It has no game ID, score, level count, expected
  action, teaching flag, action map, or evaluator state.
- `CapstoneAgent` owns `Arc3Sensorimotor`, which owns the public `Harness`.
  `observe` maps one request to zero or one outward crossing plus an owned,
  causally inert physical observation. Unknown fields and teaching controls fail
  before a harness transition.
- `WorldSession` is an effectful Python adapter around the pinned official SDK.
  It owns game IDs, server state, scorecards, action execution, budgets, and
  recordings. It sends only `CapstoneRequest` values to the Rust process.
- `CapstoneProtocol` is checked-in TOML naming SDK versions, server-discovered
  official selection, fresh-per-game initialization, organism seed, action
  budget, receipt schema, and replay requirement. Its digest identifies a run.
- Each game is the composition `fresh agent -> repeated observe/action -> stop`.
  Games compose only through inert receipt aggregation; body state never flows
  between games, so order cannot change a result.
- `CapstoneReceipt` contains the source revision, protocol digest, agent binary
  digest, toolkit versions, official scorecard, per-game transcript summary,
  physical totals, initial/final fingerprints, stop reason, and transcript-replay
  verdict. The full raster transcript is a separate content-addressed artifact.
- Errors are typed as configuration, dirty-source, SDK, protocol, invalid frame,
  unsupported actuator, agent exit, ambiguous crossing, budget, scorecard-close,
  receipt integrity, or replay divergence. Official mode closes the scorecard and
  preserves partial evidence on every recoverable failure.

## Invariants

- The organism receives only raw palette frames and physical actuator
  availability through the capstone process protocol and public Harness.
- Score, game identity, levels, tags, baselines, terminal state, expected action,
  and capstone verdict remain evaluator-only.
- Capstone mode exposes no babbling, support, pressure-settling, action remapping,
  reset-body, diagnostic, or direct body command.
- Every official game starts from the same frozen initialization in a new Rust
  process; game ordering and residual process state are irrelevant.
- Official mode requires a clean named commit and a registered `ARC_API_KEY`,
  discovers the complete accessible suite from the server, and forbids a caller
  supplied game subset.
- `ls20` smoke is visibly labeled non-capstone and cannot produce a supported
  capstone verdict.
- The official SDK owns RHAE scoring. Academy never recomputes or alters the
  score and never sends it into the agent.
- A scored transcript replayed through a fresh agent produces identical actions,
  physical observations, work, quiescence, and fingerprints or the capstone is
  rejected.
- Unsupported complex or surplus actuators are recorded as a boundary failure;
  the adapter does not invent coordinates, hide actions, or teach a workaround.
- Existing ARC development scripts, frozen results, organism law, and archived
  protocols do not become capstone authority.
- The representative warm regression remains strictly under 10 seconds.

## Scope

- Add a strict teaching-free protocol and `academy-arc3-capstone-agent` binary to
  `academy-arc3`; reuse the current sensorimotor/Harness transformation without
  changing its physical geometry or accepted law.
- Add `academy/capstones/arc3/` with a pinned `uv` project, frozen protocol,
  official SDK adapter, atomic receipt/transcript writing, replay, and offline
  fake-world/fake-agent tests.
- Repair the obsolete mechanics argument in the living A2-A5 development bridge
  so it matches the current agent, but keep that scaffolded ladder explicitly
  separate from capstone scoring.
- Update `.gitignore`, `academy.md`, `academy/README.md`, and
  `plans/academy-fast-run-and-bench.md`; reserve “capstone/benchmark” for the ARC
  capability evaluation and rename the proposed speed executable/lane
  `academy-perf`.

Exclude changes under `truelearner/`, new learner physics, new sensory
compression, coordinate actuators, curriculum changes, result rewriting,
Playground rendering, ffmpeg, model-provider agents, the official LLM benchmark
repository, and execution of the credentialed official capstone before the code
candidate is independently verified and committed cleanly.

## Development style

TDD. First specify the strict Rust command boundary and pure Python fake-world
contract, including leakage, dirty-tree, unsupported-action, partial-failure,
atomic-output, and replay-divergence tests. Then add the process and official SDK
effects without changing the frozen assertions.

## Focused tests

- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-arc3 capstone`
- `uv lock --check --project academy/capstones/arc3`
- `uv run --locked --project academy/capstones/arc3 -m unittest discover -s academy/capstones/arc3/tests -v`
- `cargo build --release --locked --manifest-path academy/Cargo.toml -p academy-arc3 --bin academy-arc3-capstone-agent`
- `uv run --locked --project academy/capstones/arc3 academy/capstones/arc3/capstone.py --mode fixture --agent academy/target/release/academy-arc3-capstone-agent --output output/arc3-capstone-fixture`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-core -p academy-arc3 --lib`
- `cargo fmt --all --manifest-path academy/Cargo.toml -- --check`
- `cargo clippy --workspace --all-targets --locked --manifest-path academy/Cargo.toml -- -D warnings`

These establish a teaching-free serialized boundary, deterministic orchestration,
content integrity, honest unsupported-action failure, release buildability,
unchanged physical behavior, formatting, and lint cleanliness without opening an
official scorecard.

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path academy/Cargo.toml -p academy-core -p academy-arc3 --lib`

Pre-change warm baseline: `3.42 seconds`, strictly under 10 seconds. The clean
cold run was `12.16 seconds`; record it separately and never treat it as the warm
gate or capstone score.

## Controls and evidence

- Held-out case: official mode receives its game set only after a clean committed
  candidate starts with a registered key. Do not list, inspect, or tune on that
  set during implementation.
- Negative controls: identical frames with different game IDs, scores, levels,
  tags, and baselines yield identical agent requests; teaching fields are
  rejected; complex-action-only worlds fail without a harness mutation; a dirty
  worktree cannot start official mode; transcript tampering and wrong binary or
  protocol digests fail closed.
- Laws: fresh-per-game execution is permutation-invariant; transcript replay is
  identity on agent responses; receipt aggregation changes no game execution;
  scoring projection cannot reach an agent request.
- Falsifiers: any evaluator field in serialized agent input, scaffolded action in
  scored mode, direct body access, cross-game state, caller-selected official
  subset, altered official score, fabricated coordinate, replay divergence,
  changed existing physical test, or warm regression at or above 10 seconds
  rejects the candidate.
- Evidence: validated plan; candidate and independent verification receipts;
  pinned `uv.lock`; fake fixture transcript; dependency/source leakage scan;
  release binary digest; and, only after a clean post-verification commit, the
  separately generated official scorecard and capstone receipt.
- Production/reference equality is not a capstone configuration axis because
  the accepted core exposes no alternate `MechanicalConfig`; do not reintroduce
  the obsolete historical switch.

## Risks and rollback

- SDK/API drift can silently change selection or scorecard shape; pin the SDK,
  validate required fields, record versions and environment metadata, and fail
  before scoring on mismatch.
- Closing a scorecard can fail after games run; preserve the transcript and card
  ID atomically, report an inconclusive receipt, and never manufacture a score.
- A wrapper can leak semantics even when Rust is blind; test the exact JSON line
  sent to the agent and reject unknown fields.
- A fresh process per game increases startup cost; keep startup outside official
  action scoring and report wall time separately.
- Roll back by deleting the capstone binary/module, Python project, protocol, and
  docs, then restoring the prior performance-plan terminology. The frozen
  organism and historical results need no migration.

## Open decisions

None.
