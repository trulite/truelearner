# TC-DS1 continuous path participation matrix v1 negative

Status: immutable technical negative. The frozen matrix was not rerun or
relabeled. No result artifact was published.

## Execution

The frozen candidate at commit
`0aae849d0af446e898c1a5240ec8184e4e79acdc` executed once in fresh E2B sandbox
`iydrzoqpwa7eq41p05b73`.

Gate A and decay assertions completed in memory. The executable stopped at the
first Gate B Reference/Production equality assertion, root `1100000`, pressure
phase `0`, return delay `0`.

Both observations agreed on every serialized field except `trace_hash`:

```text
A participation                         4294967296 / 4294967296
B participation                         4294967296 / 4294967296
A contacts                              [4294967296] / [4294967296]
B contacts                              [4294967296] / [4294967296]
A plastic updates                       1 / 1
B plastic updates                       1 / 1
causal work                             7 / 7
final tick                              0 / 0
pressure phase                          0 / 0
durable body hash                       equal
quiescent                               true / true
Reference trace hash                    bddffbb2552f6a9ea2d93c7208ab3f15cd59c740ac9ee76eb362dd1c58f7fa6b
Production trace hash                   5082be4fb5b480c2be8a970b18598976dab2cdf1c252abf0548350951797476e
```

The source-local discriminator already visibly contacted and updated both A
and B in this first case, but the frozen workflow cannot claim the complete
Gate B matrix because execution stopped before serialization.

## Classification boundary

This record does not infer whether the trace difference is a physical-history
counterexample or an ordering/measurement defect. A separately frozen
diagnostic must serialize the first differing transitions without changing the
candidate law or repeating this matrix.

ARC, pressure, retained eligibility, authority, oracle, and `arch.md` remain
unchanged.
