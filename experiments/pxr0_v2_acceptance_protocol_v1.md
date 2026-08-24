# PXR0 v2 acceptance protocol v1

Status: **PREREGISTERED; ACCEPTANCE UNSPENT; PX-C UNSPENT**.

The exact development-ready parent is commit
`141adae6e6512a5ca9a7f7ae72907f9c190c4f0d` / tag
`pxr0-single-file-physical-runtime-development-ready-v2`. The canonical kernel
must remain byte-identical at SHA-256
`f6989555f5a43dff91b39a5c7f79038168f39142fdbecca7e5e40938a72785cb`.
Acceptance may add only review, audit, and evidence artifacts. It may not edit
the kernel, retained authority sources, `arch.md`, or any future-runtime code.

## Five-question review gate

The acceptance review must bind every conclusion to the exact source and
resolve all five parent-architecture questions before execution:

1. classify the `external_arrival` proposal gate as copied physical boundary
   law or residual orchestration using the retained PX0-R/LR1 authority;
2. enumerate every `region` read and prove that it cannot affect firing,
   ordering, plasticity, pressure, proposal choice, or propagation, except for
   deciding whether to serialize a `Crossing`;
3. enumerate `Work` and `resident_bytes` reads/writes, prove they cannot affect
   a transition or queue choice, and decide whether they belong in a later
   production hot path;
4. enumerate CELL `resistance`, `generation`, and `live` initialization,
   reads, and mutations; distinguish causal state from immutable/redundant
   scaffolding and decide retain/remove without changing v2;
5. freeze pressure phase as substrate physical state: `pressure_tick` begins at
   zero, advances only in integral ten-tick epochs, aligned translations
   preserve relative phase, and noncongruent construction may lawfully differ.

Any discovered semantic/fixture selector, hidden module, evaluator dependency,
causal observer counter, region-dependent transition, or mismatch with copied
authority law is a hard negative. Removing scaffolding, changing instrumentation,
or broadening proposal formation would change bytes or law and therefore
requires a separately preregistered successor rather than an acceptance repair.

## Exhaustive source review

The review must list all 13 types and 15 functions/methods and give each a
physical or observer-only responsibility. It must search the one active file
for cognitive, fixture, named-world, reset/cleanup, typed-handoff, mechanism
selector, generated-code, module, cfg, and test surfaces. Zero hidden semantic
or fixture logic is required.

## Acceptance evidence

One fresh E2B sandbox may execute one static acceptance audit. No Rust build,
test, Clippy run, development matrix, or broad suite is authorized. The audit
must require:

- exact kernel, manifest, one-page specification, retained LR1 source, v2
  invariance CSV, phase-control CSV, result Markdown, static gate, inventory,
  taxonomy, and harness-audit hashes;
- the frozen development result `466/466`, exact replay, quiescence, maximum
  work `15039`, maximum memory `6000`, and authority flags false;
- exact 28-entry inventory, one active file, zero dependency, zero banned or
  hidden surface, and taxonomy primary/semantic/evaluator counts zero;
- all five review conclusions present and source-reconciled.

On success, freeze the audit and tag the unchanged kernel as the accepted PXR0
v2 parent for PX-C. This accepts the existing scientific kernel; it does not
retroactively alter PX0-PX8 authority and does not itself start PX-C.
