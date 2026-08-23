# CJ0-OR ordinary convergence implementation audit

Status: **IMPLEMENTATION READY TO FREEZE; ALL CJ0-OR EVIDENCE UNSPENT; AUTHORITY UNCHANGED**.

## Frozen implementation surface

- protocol commit: `eefdf4ff7edb744ebdd3cc734932de0a41117ed9`, tag
  `cj0-or-ordinary-convergence-protocol-v1`;
- protocol SHA-256:
  `8780e9b4ceb521c53052e3f3e63e8f096c6e5d116352d922f78faf1d880df009`;
- sidecar source:
  `crates/px0-physical-correspondence/examples/cj0_or_ordinary_convergence.rs`;
- sidecar source SHA-256:
  `fc16f376d6ba34ef59add84bddace1d9a3360f237cf0019866a2632c58bcef43`;
- numeric organism-visible block SHA-256:
  `63b6d370e156f9087d944c10c39e1826b1d3b7e5ffd9e3b2c7633e050863e95c`;
- authoritative substrate-law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

The implementation is a dependency-free example importing only the frozen
public substrate API. `crates/px0-physical-correspondence/src/lib.rs`, every
authoritative PX0-PX2 source/artifact, Cargo manifests, and `Cargo.lock` are
unchanged. No PX3 file is imported into the branch.

At execution time the example proves authoritative ancestry, verifies the
substrate-law and protocol hashes, verifies the two frozen PX3 negative tags
still resolve to their recorded commits, and compares its current source bytes
exactly with the implementation tag. Evidence commands additionally require a
clean worktree and write-once empty result/staging paths.

## Numeric physical block and evaluator isolation

The organism-visible block is delimited by
`BEGIN/END NUMERIC PHYSICAL BLOCK`. It contains only numeric specifications,
ordinary CELL/ARROW/SPIKE construction, physical handles, propagation,
traces, crossings, work, persistent storage, and natural queue drain.

A case-insensitive whole-word scan of that block found zero occurrences of:

```text
A B C OR disjunction logical scenario expected pass fail alone joint branch
trained crossed evaluator semantic
```

The block receives two numeric activation bits, numeric route kinds, a numeric
threshold, timing, layout, identity, and allocation parameters. It never
receives a stage name, success expectation, logical label, evaluator result,
or prior execution outcome. Logical names, clause evaluation, result
serialization, and classification occur only after the end marker and have no
causal path into propagation.

## Accounting and artifact closure

Every row constructs and executes exactly `23` independent fixtures and `23`
inert no-input follow-ups:

- four candidate cases and four byte/execution-identical replays;
- four coupling-`0` blocked-route controls;
- four absent-ARROW controls;
- four stale-route controls;
- three threshold-`2` controls.

This is exactly `92` CELL instances and `65` ARROW instances per row. The
externally entered source ledger is exactly `120` SPIKEs producing exactly
`30` source CELL firings per row. Four stale fixtures must each report exactly
one physical ARROW deallocation. Incidental local structural proposals must be
zero.

Two separately written aggregation functions must agree on total work,
persistent bytes, external SPIKEs, source/convergence/downstream firings,
crossings, deallocations, proposals, CELL instances, and ARROW instances.
Both include every candidate, control, exact replay, and idle follow-up. The
CSV serializes the aggregate fields per row; the Markdown report independently
sums all rows. This closes the known discarded-clone/accounting hazard before
evidence is spent.

Both result files are created at fresh staging paths with `create_new`, fully
written, and synced before their sequential atomic renames. Existing final or
staging paths refuse execution. No result path exists at implementation
freeze.

## Physics/control distinctions

- Candidate convergence threshold is `1`, so each isolated coupling-`1`
  route is independently sufficient.
- The simultaneous both-route case is allowed exactly one downstream firing;
  it is explicitly marked as refractory suppression.
- Positive skew must produce exactly two bounded downstream firings.
- The separate threshold-`2` control must produce `0,0,1` and is classified as
  conjunction/saturation, never as OR evidence.
- Coupling-`0`, absent, and stale controls must suppress only their physical
  route while the intact route remains sufficient.
- Every idle follow-up must have empty trace/crossings, zero work, unchanged
  fingerprint, and natural quiescence.

Thus a positive result cannot be supplied solely by threshold saturation or
unreported refractory behavior. It requires isolated physical sufficiency of
both symmetric routes and bounded downstream use.

## Pre-evidence validation

- focused formatting: pass;
- focused `cargo check`: pass;
- focused example tests: `3/3` pass;
- focused strict Clippy with `-D warnings`: pass;
- no-argument refusal before source audit/cells: exit `2`, pass;
- wrong-argument refusal before source audit/cells: exit `2`, pass;
- authoritative law diff from PX2 commit: empty;
- Cargo manifest/lock diff from PX2 commit: empty;
- dependency additions: none;
- forbidden-token block scan: zero matches;
- final and staging result paths: absent;
- evidence-spent markers during validation: zero.

The focused tests use only `0xcf00_0000_0000` development identities. They
cover simultaneous refractory suppression, positive skew, isolated route
sufficiency, blocked/absent/stale controls, threshold saturation, exact replay,
natural quiescence, source bounds, and complete accounting. No evidence-stage
namespace was executed.

## Freeze and next action

Freeze this source and audit together at tag
`cj0-or-ordinary-convergence-implementation-v1`, then run the preregistered
PROBE exactly once. A PROBE failure must be frozen and stops MICRO/GATE and
definitive evidence. No implementation change is permitted after any evidence
marker without separately freezing the result and preregistering a correction.
