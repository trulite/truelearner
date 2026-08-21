# RG0a pre-definitive implementation audit

Status: implementation audit completed before the single definitive command.
No definitive seed or RG0a result artifact had run when this file was written.

## Frozen boundaries

- Frozen parent: `37806c75b44108a7bc8e79afceeaec599b406cef`.
- Protocol tag: `rg0a-reflected-grounding-protocol`.
- Protocol SHA-256:
  `34c6f77ae4501a8ff916a47f2099551f54bd24ecbc9a06cf035ebd4c3ef69ee4`.
- Preexisting `src/reflected_program_discovery.rs` logic is unchanged. Its only
  substantive RG0a edit is the child-module declaration `pub mod grounding;`.
- RP0a/RP0b result and protocol files are unchanged.

Implementation hashes before this audit document:

- `src/reflected_program_discovery/grounding.rs`:
  `8b1361d3da1603a365e47ec5aa55074a4f2a41b946b3fa746533fc91f676c5a2`;
- `src/research_runtime.rs`:
  `e570b3cd0fcff759a02a38a685f22f33bc28de65e25b7beb34f77d138f3fd711`;
- `src/bin/reflected_grounding.rs`:
  `a1496f0341d5d233d7ceb1e54abd8b564b275eba457ce3e5d6dcc2d03982ae41`.

## Causal-path audit

The experimental grounded path is:

```text
opaque concrete route-source cell
    -> dense temporary cell-to-role binding read
    -> frozen ProgramLearner arrow scan
    -> one learned arrow fires
    -> dense role-to-location binding read
    -> ordinary spike enqueued at opaque lower location
    -> destination lower cell's frozen local physics
```

The following are absent from `execute_learned_grounded`, `route_spike`, and
`run_cell_machine`:

- `ProgramChoices` construction or lookup;
- a call to the frozen direct `execute` function;
- evaluator role-to-location resolution;
- `LowerRole` comparison or reflected semantic dispatch;
- operation/lookup opcode or Rust operation callback;
- concrete answer, oracle entry, direct route, or fallback.

`LowerRole` and `evaluator_locations` occur only while the evaluator constructs
the frozen lower cell registry, scores role transfer, builds the CONCRETE
baseline, constructs development fixtures, or constructs the ORACLE diagnostic.
The learned integrated router receives only `GroundMachine`, `RoleLearner`,
`ProgramLearner`, and structurally produced temporary bindings.

The ordinary lower baseline is independently checked against the frozen lower
executor at depths `1, 5, 32`. Outcomes, explicit answer, quiescence,
activity-limit behavior, and dynamic route-firing counts match.

## Harness and state audit

- Permanent RP0a state is immutable and shared; branches allocate only episode
  state.
- Every arm receives the same borrowed `GroundMachine` and immutable hash.
- Direct route targets are constructed and passed only to CONCRETE.
- Grounded arms receive no direct target table.
- Temporary bindings are explicitly erased after every branch.
- Work is recorded in a compact primitive counter and aggregated in memory;
  no per-episode CSV or filesystem write occurs.
- Definitive reconstruction and evaluation are parallel only across seed cells;
  each organism remains deterministic and single-threaded.
- Ordered collection restores seed order before reporting.

## Validation before freeze

Persistent E2B sandbox: `iv7qfq154p7ffq4xpxw0o`, left running.

- `cargo fmt --all -- --check`: pass.
- `cargo clippy --all-targets -- -D warnings`: pass.
- RG0a debug tests: `4/4` pass.
- research-runtime debug tests: `2/2` pass.
- MICRO: pass, development only, no claim.
- GATE: pass, development only, no claim:
  - CONCRETE `8/8`;
  - GROUNDED REFLECTED `8/8`;
  - NO BINDINGS `0/8`;
  - SHUFFLED BINDINGS `0/8`;
  - ACTIVITY ONLY `0/8`;
  - RANDOM PROGRAM `0/8`;
  - SHUFFLED-TERMINAL PROGRAM `0/8`;
  - ORACLE `8/8`.
- Workspaces in the final GATE check: `374/374` destroyed.
- One legacy regression pass: all `154` unchanged legacy tests and five new
  tests passed; the only failure was the subsequently fixed new MICRO
  display-seed overflow described in the development amendment.

The working tree contained no definitive CSV or Markdown result. The next
claim-eligible action is the single `--definitive` command after this audit and
the implementation are committed and tagged.
