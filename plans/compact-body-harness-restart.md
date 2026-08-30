# Express the retained organism contract as body laws

```text
old compatibility test
        |
        v
underlying physical claim
        |
        v
Body input -> physical composition -> observable effect
```

## Outcome

Replace the copied old-core compatibility suites with one body-law suite. Each
test states a physical transformation, composition law, or negative control
using `truelearner-body` directly. No test requires `Harness`, a historical
`Protocol`, old learner sidecars, an old diagnostic event name, a public memory
inspector, attachment, reading, or checkpoints.

The rewrite defines the contract the body implementation must satisfy. It is
not evidence that the current implementation satisfies it until the user
authorizes execution and verification.

## Authority

- Path: `LANGUAGE.md`, `lessons.md`,
  `truelearner/crates/core/tests/harness_boundary.rs`, and
  `truelearner/crates/core/tests/runtime_attachment.rs`
- Revision: Git `bedcbe54208bd8dc2df9d1bdde976f6c28c4ea7a`;
  authority digests `2b1954b161358c4a259198b0b9e4c66a93e47350d749d7c3baf3ddcef7bb8a41`,
  `5b50453e4895e5a25c337555af167894cbfd4625d89837976815914aa21e1bb0`,
  `b2ea8c33a6a6c4f8466ddb439b4eff7e736fea4e40c268c0db77284ccb7097c8`,
  and `6737498da8a80cfc190eb972da825e8d96a05a1f7d7a33f1c567610f62696c55`

## Model

The objects under test are `Body`, its junctions and links, physical events,
local reaction views, and explicit edits. The main arrows are `input`, `step`,
`react`, and `apply`. Repeated `step` must equal `run`; quiet is identity;
observation is inert; disconnected causal components compose independently.

Historical protocol stages that tested successive approximations are folded
into the final law plus its counterexamples. Tests may inspect stable domain
observations such as emitted physical events, applied structural additions, and
requested physical edits, but not memory storage, attachment shape, checkpoint
format, or a specific implementation module.

## Invariants

- Every retained physical claim has a positive case or a negative control in
  the new suites.
- Tests import `truelearner-body` directly and contain no old-core dependency.
- Tests do not select a historical protocol or reproduce an old implementation
  inside a fixture.
- Input order, reflection, ambiguity, stale evidence, unrelated causes, and
  dormant-body size remain controlled where relevant.
- Only actual participation receives consequence; repeated evidence produces
  one effect; outcomes are consumed at their first valid choice.
- Learner construction requires new physical membership and same-moment
  outcome composition requires an exact live witness.
- Attachment and checkpoint contracts are deferred until their boundaries are
  agreed.
- No test is executed during this rewrite, as explicitly requested.

## Scope

Replace `truelearner/crates/new-harness/tests/harness_boundary.rs` with
`truelearner/crates/new-harness/tests/body_laws.rs`, remove the provisional
runtime-attachment suite, and update the test target in
`truelearner/crates/new-harness/Cargo.toml`.

Update this plan so the old copied tests remain authority material rather than
the new acceptance interface. Do not change `truelearner-body`, the old core,
embodiment, workstation, Academy, lessons, research evidence, or production
workspace selection.

## Development style

Use TDD: write the complete accepted body contract first, with controls beside
their law. Do not run or repair the implementation in this change. Missing body
interfaces and failing laws become the implementation work for the next
authorized change.

## Focused tests

When execution is later authorized:

- `cargo test -p truelearner-new-harness --features candidate --test body_laws`
  checks physical memory, formation, choice, return, consequence, construction,
  composition, observation purity, and active-work scaling.
- `cargo test -p truelearner-body` remains the held-out physical-kernel suite.

## Development loop

The future representative warm regression suite is
`cargo test -p truelearner-new-harness --features candidate --test body_laws`.
Its measured warm wall time must remain strictly under 10 seconds. No regression
suite is run while producing this test-only candidate.

## Controls and evidence

The held-out cases are the existing `truelearner-body` tests and the old core
suites at the authority revision. Negative controls are co-located with each
law: unchanged samples, expired sensor memory, mixed causes, distant outputs,
boundary reentry, ambiguous returns, preopening outcomes, duplicate outcomes,
ambiguous cycles, repeated membership, and stale construction witnesses.

Evidence for this rewrite is source inspection, formatting, changed-path
inspection, test counts, and test-content digests. Runtime results, pass counts,
and performance claims are explicitly absent until execution is authorized.

## Risks and rollback

The risk is losing a valid physical claim while removing old implementation
details. The authority files remain untouched, and each implementation phase
must compare new law coverage against them before deleting any old production
code. Another risk is accidentally defining convenience APIs that recreate a
harness; direct imports and body-owned observations make that visible.

Rollback is restoration of the untracked copied tests and their Cargo test
targets. Production behavior is unaffected because this change edits only the
candidate tests, their target declaration, and the plan.

## Open decisions

None.
