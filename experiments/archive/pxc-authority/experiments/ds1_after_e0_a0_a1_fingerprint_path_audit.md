# Unchanged DS1 after A1 fingerprint and path audit

## Frozen fingerprints

| item | SHA-256 |
|---|---|
| DS-E0 source | `fc5d426cc8a5116dbd2749b914e6c30db88529d3070a844a20fc76ac88782615` |
| accepted DS-A0 source | `3eb802f394a225a4ad7f0938b4a672723da2c1303ff95e805423de8161057527` |
| DS-A1 source | `b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1` |
| marked frozen DS1 slice | `adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e` |
| M0 source | `50cf169bb293177a35270adde656f28f98e68c83a4d39d2876399261b7ee697c` |
| compiled M0 correspondence | `430cd2206c8baa7106c4de7f203d4d0c48b544290e6266596ebcdb91d02655c9` |
| A1 readiness handoff | `5798387bd30558ed86fa092453dd5aafee29983486be3d75af56d2cd18e54676` |
| retry protocol | `66cdf1872e433dc6860745672ef9bfd9efdd09193253ad6562bc291648e9a0de` |
| retry mechanism | `3b96de98a8f91ca9f7338d1184d4d2e6c10e6528783820030d6ae74dae81d08e` |
| retry runner | `68ff04965f66efc05b69fa948289ac946d320bdeffced1c7efea3466d67d7739` |
| build fingerprint wiring | `b9e040b2ee0c9104c91ab431ac3f6573236e284d78fe41d5d32f729022a06f40` |

All frozen mechanism files remain byte-identical. Stage 0 reuses their frozen
controls by fingerprint; no legacy matrix was rerun.

## Derived source and runtime path inventory

| path | source count | runtime count per seed |
|---|---:|---:|
| frozen DS1 `choose` definitions | 1 | — |
| composition-to-frozen-`choose` edges | 1 | 1 |
| choice-to-A1-live-execution edges | 1 | 2 including permutation control |
| post-choice observer edges | 0 | 0 |
| natural A1 execution-to-evidence paths | 0 | 0 |
| frozen DS1 `apply_consequence` definitions | 1 | — |
| composition-to-`apply_consequence` edges | 0 | 0 |
| effect-to-choice edges | 0 | 0 |

The zero observer, natural-evidence, and update paths are independent. Source
mutations add each forbidden edge separately and make its corresponding audit
count nonzero. No runtime zero is a literal placeholder.

The frozen DS1 learner was invoked only after A1 had installed and bridged two
roots. The selected index was then mapped one-to-one to an already-existing
opaque handle. Execution injects a root SPIKE and follows live adjacency; no
token-to-operation interpreter exists.
