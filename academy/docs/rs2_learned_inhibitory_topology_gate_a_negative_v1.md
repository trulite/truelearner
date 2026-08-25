# RS2 learned inhibitory topology Gate A negative v1

Status: immutable static negative; Gate B not constructed.

## Result

The accepted ordinary local variation path contains exactly one structural
proposal constructor. For every eligible nearby CELL it constructs:

```text
mode        Drive
coupling    +1
resistance  1
```

There is no sign choice, negative-coupling branch, candidate distribution,
seed-dependent sign, or alternate ordinary proposal mechanism. The accepted
runtime can execute manually constructed negative Drive ARROWs, as RS1 proved,
but its variation operator cannot create them.

Therefore:

```text
ordinary variation
→ positive Drive candidates only
→ useful inhibitory candidate absent from variation support
→ consequence learning has nothing inhibitory to select
```

Gate A is negative under the frozen RS2 stop rule. No RS2 evaluator, candidate
world, negative proposal law, coupling-sign preference, Gate B matrix, Rust
compilation, or learning run was created.

## Integrity

The exact source audit ran in fresh E2B worker `i5y85noru41lo3sjf8d8m` from
protocol commit `60043bc`.

```text
core lib.rs  45dd6af368776d68574ff2b00dd4db109d469bfeedc99b57eb76ad6b26ca111c
core Cargo   5d794eae058f5cdd896064b0a37a6dfb124d9d7b6d03f8cfa9c53651e58460ef
audit output 675b2eba79dba4e1df0f47a87fbd8c7d37ecf64e6e862173c28cc8f05c6d8bab
```

The audit proves the proposal constructor is unique in the accepted core,
constructs exactly one `ArrowSpec` form, emits the ordinary Proposal event,
and fixes coupling/mode to `+1/Drive`.

## Classification

RS2 does not falsify consequence-based selection. It establishes a prior
support failure:

> The accepted variation law cannot propose the physical sign required by the
> RS1 stabilizing topology.

This negative independently identifies the next missing question, but does not
authorize or choose its answer: what general local physical variation law can
produce both excitatory and inhibitory causal possibilities without being told
which sign is useful?
