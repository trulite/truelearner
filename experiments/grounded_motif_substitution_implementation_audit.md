# RC0b implementation and development audit

Protocol: `grounded-motif-substitution-rc0b-v1`

Status: implementation and development gates frozen; definitive scientific
matrix not run.

## Frozen references

- protocol tag: `rc0b-grounded-motif-substitution-protocol`
- protocol commit: `4ae4378eb5df95c9f1e829db58f3c7ba5c341b9b`
- protocol SHA-256:
  `92e08d219c18d5ee6abe88e19e327014c9b973d5f6780f6dc8fd2c8551a0b687`
- implementation source commit:
  `9d0813fa15a10736eb7a4e8738cfac0fe9b256d2`
- shared grounding source SHA-256:
  `104e37d2967809cc1b38bae95ee4ffbe7be8939ade26a09a10b89bdd772010f8`
- RC0a source SHA-256 after passive trace integration:
  `bbd6dc434a1f12dc6c9df6f657ec7fbb17b99b04a1c142d7cab45ae35269c7ed`
- RC0b source SHA-256:
  `6a21c62e406e68d47a442839938b457505cd195bbb891929f2b9b6c7f44e1987`
- RC0b binary SHA-256:
  `7621e0ffed14cc2443e82e838d0531d2209e0132dae80f8cc537638a2a60eeaa`

No `results/rc0b_*` artifact exists. Development modes refuse artifact paths;
the definitive binary refuses to overwrite either frozen result filename.

## Implementation boundary

There is one lower executor body. Existing `run_cell_machine` calls
`run_cell_machine_traced(..., false)`; RC0b calls the same body with passive
trace recording enabled. Trace recording does not increment or redefine any
RG0a/RC0a work counter.

The persistent RC0b structure contains only:

- a canonical cycle of learned integer role identities;
- parent compiled-arrow identities;
- emitter/relay/target learned-role triples;
- strength.

Its Rust types contain no `OpaqueId`, concrete location, query, answer,
episode, depth, context marker, or observation boundary. The learner receives
only translated local role occurrences, recurrence, local route-source
transparency, compiled adjacency, and terminal success/failure. It receives no
`ObservableTrace`, work delta, counterfactual result, effect marker, or
evaluator boundary.

The single motif is materialized in temporary episode state. Current bindings
join its role triples to fresh emitter, relay, and target cells. Three eligible
relay destinations are redirected in a cloned temporary `GroundMachine`; the
same CELL/ARROW/SPIKE loop then runs the substitute. Lookup, current mutation,
terminal answer delivery, no-result routing, and context effects remain in the
ordinary executor and are charged. No RC0b opcode, direct transformation
executor, depth macro, supplied fragment boundary, answer cache, or second
motif exists.

Compatibility validates the learned compiled parent, role endpoints, relay
physics, and absence of a local relay effect before installing a shortcut. A
local effect invalidates RC0b and resumes RC0a. Replaced learned-arrow
identities invalidate RC0b and RC0a, then resume RG0a. The forced-stale arm is
an evaluator-only deliberately broken control and is never offered as an
organism path.

## E2B clean-snapshot validation

Persistent sandbox: `iv7qfq154p7ffq4xpxw0o`

Validated the clean implementation commit above with:

```text
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -q
cargo run --release --bin grounded_motif_substitution -- --gate
cargo run --release --bin compiled_grounded_recurrence -- --micro
```

The remote command chain exited `0`.

The single legacy regression was run because passive tracing changed shared
RG0a/RC0a executor code:

- library: 163 passed, 0 failed, 2 ignored;
- main binary: 10 passed, 0 failed;
- reviewer API: 4 passed, 0 failed;
- all binary test targets: no failures.

No further legacy regression is required unless shared or frozen machinery
changes again.

## RC0a compatibility result

RC0a MICRO reproduced the frozen signature exactly:

```text
generic overhead slope   3.000000
compiled overhead slope  0.000000
all RC0a gates           PASS
workspaces               119/119 destroyed
```

The tracing extension therefore changes neither frozen RC0a behavior nor its
logical work accounting.

## RC0b development result

E2B GATE used one development seed, acquisition depths `3, 4, 6`, evaluation
depths `5, 8, 13`, four fresh episodes per depth, and all eleven arms.

```text
arm                                correct   trace equality   work     motif fires
concrete reference                  12/12       12/12         10,160       0
full RC0a                           12/12       12/12         11,936       0
motif substitute                    12/12       12/12         10,304     324
changed surroundings                12/12       12/12         10,739     324
interruption/re-entry               12/12       12/12         10,304     324
context-effect invalidation         12/12       12/12         12,256       0
forced stale same endpoint          12/12        0/12         10,304     324
RC0a parent invalidation            12/12       12/12         12,620       0
subthreshold evidence               12/12       12/12         11,948       0
shuffled recurrence evidence        12/12       12/12         11,972       0
no bindings                          0/12       12/12          1,584       0
```

All fourteen RC0b-A qualitative/accounting gates passed. Every one of the 324
shortcut firings corresponds to one eliminated relay activation and one
eliminated ordinary route firing. Mature substituted work was 1,632 lower than
FULL RC0a, a 13.6729% development reduction after motif validation,
installation, and shortcut-firing charges.

The same-endpoint control is intentionally endpoint-correct but trace-wrong:
12/12 final answers and 0/12 exact observable traces. The validated context arm
instead fired zero shortcuts, invalidated the motif, resumed RC0a, and retained
12/12 exact traces. This separates observational equivalence from answer
equality.

The balanced shallow development workload has motif work 144 above concrete,
or +1.4173%. This is a non-claim diagnostic only. It neither establishes nor
rejects RC0b-B; the frozen definitive distribution includes substantially
deeper held-out invocations and has not been executed.

All 569 E2B GATE workspaces were destroyed, maximum live workspace per cell was
one, duplicate evaluation was deterministic, permanent fingerprints were
stable, and temporary bindings/routes were erased.

## Frozen status

```text
RG0a     functional grounding                         positive, frozen
RC0a     compiled recurrent dispatch                  positive, frozen
RC0b-A   implementation + development gates           positive, frozen
RC0b-A   definitive scientific outcome                pending
RC0b-B   definitive economic prerequisite             pending
RE0      blocked on definitive RC0b-B
F1       blocked on RE0
```

No RC0b definitive command was executed while preparing this audit.
