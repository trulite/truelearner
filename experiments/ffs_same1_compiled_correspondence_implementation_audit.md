# FFS-SAME1 compiled correspondence implementation audit

Protocol: `identity-desupply-ladder-v1/ffs-same1`

Status: implementation frozen after MICRO/GATE validation and before the
single definitive execution.

## Frozen ancestry

FFS-SAME1 consumes these frozen positive parents without regenerating their
definitive matrices:

- FFS0: tag `ffs0-full-fractal-scaling-positive`, commit
  `2fd56376e32a403b5e1fac6dbdbc6f21e4f2645d`;
- FFS-SAME0: tag `ffs-same0-learned-correspondence-positive`, commit
  `2d96d8d603834ddb67e1b883d6a656b16d58a549`;
- CS0a: tag `cs0a-compiled-correspondence-positive`, commit
  `c341f113e0d2b42bb105a6693610bbef71f316e6`;
- umbrella protocol SHA-256:
  `493cc50ac67e7c3985b61e7fbe249f19388fed45b026d29dc81ad70f20c9ccbe`.

The runner checks the frozen FFS0, FFS-SAME0, CS0a, CS0b-trigger, and umbrella
protocol artifacts by SHA-256 before any mode can pass. The preregistered
CS0b trigger remains frozen as absent: grounding accounts for only `2/6` of
the positive residual slope, below the required one-half threshold. No CS0b
code or result was created.

## Source freeze

- source commit:
  `0d302d3041755507586bdc09cece096fc52b7ed7`;
- reintegration kernel SHA-256:
  `26bcbc9bfc999977b070b9ab5411a6cf1706ed33bdc21cb50e8135b639e128dd`;
- runner SHA-256:
  `85e396dbd2badcf4c727d0d253b017f8524a36a5cb665fac813a17c78adc8292`.

The only changes to frozen CS0a source are Rust visibility changes from
private to `pub(super)` for the exact compiler store, acquisition function,
execution function, work accumulator, and source audit. No expression,
constant value, persistent field, branch, work charge, or lifecycle behavior
changed. Focused CS0a tests remain positive.

## Capability boundary

FFS-SAME1 adds no capability. It reintegrates:

```text
frozen FFS-SAME0 generic correspondence
        -> frozen CS0a consolidation and compiled route
        -> frozen FFS-SAME0 recursive arrow acquisition/execution
```

The same `CompiledCorrespondenceStore` and
`execute_compiled_or_generic` code is used during recursive motif acquisition,
parent evaluation, child evaluation, transfer, invalidation, and historical
return. There is no L0-only fixture, `MetaCorrespondence`, recursive-level
branch, process-specific executor, economic signal, or evaluator fallback.

The recursive arrow learner is copied mechanically only to replace its call
to generic correspondence execution with the frozen CS0a
compiled-or-generic lifecycle. Candidate generation, credit, threshold,
pruning, pair substitution, storage, and parent-relative economics are
unchanged.

## Persistent-state and leak boundary

Persistent compiled correspondence remains exactly the CS0a representation:

```text
correspondence asset identity
source role
target role
context and support atoms
parent dependency fingerprint
strength and local route identity
```

It contains no past/future occurrence, evaluator filler identity, concrete
destination, stable filler-derived token, level, task depth, population,
process label, answer, work, price, or horizon. Current concrete occurrences
exist only in invocation-local temporary routes.

FFS-SAME1 runs the complete inherited FFS-SAME0 identity leak controls plus:

- fresh and bijectively relabeled occurrence transfer;
- allocation-order and memory-order perturbation;
- changed binding transfer;
- subthreshold and shuffled compilation controls;
- stale parent dependency invalidation and generic reopening;
- historical compatible-route reuse;
- persistent fingerprint stability during use;
- source audits over both the frozen CS0a persistent region and the new
  level-blind reintegration region.

## Frozen matrix and outcomes

The definitive runner uses seeds `0..7`, sixteen fresh held-out episodes, and
the preregistered cells:

| Cell | Depth | Identity population |
|---|---:|---:|
| S0 | 8 | 16 |
| S1 | 32 | 64 |
| S2 | 128 | 256 |
| S3 | 512 | 1,024 |
| depth-only | 128 | 64 |
| population-only | 32 | 1,024 |

It reports independently:

- A1 functional hierarchy preservation;
- B1 parent-relative computational recursion;
- C1 parent-relative economic recursion;
- D1 mature identity-tax reduction versus FFS-SAME0;
- E1 adaptive invalidation and historical reuse;
- P1 process availability, non-blocking.

The exact anchor depth signature `0, 3, 5, >=6`, zero over-retention, and zero
under-retention are frozen compatibility requirements. Every claimed edge is
tested against its immediate retained parent.

## Work accounting

Every mature invocation reports four read-only references:

1. anonymous generic search;
2. frozen FFS-SAME0 learned correspondence (`+18/use`);
3. current FFS-SAME1 compiled learned correspondence;
4. frozen supplied-SAME FFS0.

The unchanged CS0a residual attribution is:

| Component | Work/use |
|---|---:|
| compiled local activation | 1 |
| context/support/dependency validation | 3 |
| ambiguity handling | 1 |
| temporary route installation and binding | 1 |
| **total** | **6** |

Compilation acquisition, generic correspondence acquisition, persistent
bytes, recursive arrow acquisition, parent/child mature work, invalidation,
generic reopening, and recovery work remain separate fields. Economic
information is evaluator-only.

## Development-only validation

Development seed `70000` is disjoint from definitive seeds. GATE produced:

| Scale | Structural / justified / realized depth | FFS-SAME0 | FFS-SAME1 | supplied SAME | mature tax |
|---|---|---:|---:|---:|---:|
| S0 | 0 / 0 / 0 | 54 | 42 | 36 | 6 |
| S1 | 3 / 3 / 3 | 70 | 58 | 52 | 6 |
| S2 | 5 / 5 / 5 | 124 | 112 | 106 | 6 |
| S3 | 6 / 6 / 6, right-censored | 335 | 323 | 317 | 6 |
| depth-only | 5 / 5 / 5 | 124 | 112 | 106 | 6 |
| population-only | 3 / 3 / 3 | 70 | 58 | 52 | 6 |

Development also produced `23/23` controls, `4/4` adaptive rows, exact
transfer, zero retention disagreement, deterministic duplication, and passing
source/ancestry audits. These results establish implementation readiness only;
they are not claim-eligible.

## Verification

The exact source commit was validated in persistent E2B sandbox
`iv7qfq154p7ffq4xpxw0o` with:

- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- ten focused FFS-SAME0/CS0a/SAME1 kernel tests;
- two FFS-SAME1 runner tests;
- release MICRO;
- release GATE.

All passed. The sandbox was left running. No legacy definitive experiment was
rerun because no frozen behavior expression or artifact changed.

## Definitive lock

The definitive command remains unexecuted:

```text
cargo run --release --bin ffs_same1_compiled_correspondence -- --definitive
```

The write-once definitive artifacts do not exist. They may be created exactly
once from the frozen implementation tag. No tuning, CS0b implementation, or
rescue gate is allowed before that run.
