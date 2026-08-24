# RC0a definitive outcome audit

Status: frozen positive compatibility result from the single preregistered
definitive matrix.

## Narrow claim

> Repeated successful grounded execution earned ordinary consolidated local
> structure that eliminated the per-transition interpretation and dereference
> overhead of a learned reflected recurrent program while preserving fresh
> binding generality and reopening the generic path after invalidation.

RC0a does not establish economic advantage, fragment substitution, elimination
of lower computation, or a second reflected level.

## Frozen execution

- Protocol: `compiled-grounded-recurrence-rc0a-v1`.
- Protocol commit/tag: `2ff4200` /
  `rc0a-compiled-grounded-recurrence-protocol`.
- Implementation commit/tag: `4119863` /
  `rc0a-compiled-grounded-recurrence-implementation`.
- Definitive command: exactly one invocation of
  `compiled_grounded_recurrence --definitive`.
- Persistent E2B sandbox: `iv7qfq154p7ffq4xpxw0o`, reused and left running.
- Release compilation: `36.71s`.
- Definitive CSV SHA-256:
  `398d0e56f9a528871b97fd00b9a24fe3a9d2c2b96253cb4268ada264fbc9faef`.
- Generated report SHA-256:
  `52d103f74e23e45e35ff957dd0e0923e6cc8e5c6174f16ea1b1fbb74f234e28f`.
- Protocol SHA-256:
  `e641aa2367e5bf504b28b22fb84268e69092577b9b6efdf5903d924b990dd2a4`.
- Shared RG0a router SHA-256:
  `3ba62a23adc36e58a68836eced7a88c571c75027f318b515ba5f7f9fb3cf6bef`.
- RC0a implementation SHA-256:
  `3274fbf2380c0db457b585d03e7ac0476daf393f3910e2ec37f0b65d3513ab55`.
- RC0a runner SHA-256:
  `2ed65a9ca3afb9c29942e904a12b84b88a4979a45adeee0b3e48cb2845c05b7a`.

The implementation hash set exactly matches the pre-definitive audit. The
definitive command was not rerun.

## Matrix integrity

The CSV contains:

- `8` acquisition rows;
- `384` runtime rows = 8 arms x 8 seeds x 6 depths;
- `13` conjunctive gate rows;
- `406` lines including the header.

All acquisition rows recorded:

- three successful generic compilation episodes;
- four compiled role-relative arrows;
- zero subthreshold compiled arrows;
- `160` persistent compiled bytes per seed.

Across all seeds, acquisition used `24` successful generic episodes, earned
`32` compiled arrows, occupied `1280` persistent bytes, and charged `9472` work.
Acquisition is reported separately from mature runtime.

Every serialized runtime row reports erased bindings, erased temporary arrows,
and matching lower effects for the required arms. The conjunctive state gate
also checks immutable hashes, persistent fingerprints, and duplicate
determinism internally. Every gate row is `PASS`. All `1,148,245` workspaces
were destroyed; maximum live workspaces per independent seed cell was `2` and
eight seed cells ran in parallel.

## Behavioral matrix

| Arm | Correct | Total | Work | Required path |
|---|---:|---:|---:|---|
| CONCRETE REFERENCE | 768 | 768 | 7,912,448 | frozen direct reference |
| GENERIC GROUNDED | 768 | 768 | 8,653,824 | reflected scan + dereference |
| COMPILED GROUNDED | 768 | 768 | 8,192,000 | temporary local arrows |
| CHANGED BINDINGS | 768 | 768 | 8,558,999 | same compiled roles, permuted cells |
| INVALIDATED TRANSITION | 768 | 768 | 8,655,192 | invalidate, resume generic |
| SUBTHRESHOLD EVIDENCE | 768 | 768 | 8,654,592 | no compile, remain generic |
| SHUFFLED EVIDENCE | 768 | 768 | 8,655,069 | invalidate, resume generic |
| NO BINDINGS | 0 | 768 | 267,264 | no usable route |

The changed-binding work is not compared economically with the unpermuted
concrete arm: permuting physical CELL positions legitimately changes location
search comparisons. It is a binding-generality control.

## Slope result

For every depth, mature per-episode work was:

| Depth | Concrete | Generic grounded | Compiled grounded | Generic excess | Compiled excess |
|---:|---:|---:|---:|---:|---:|
| 5 | 468 | 690 | 832 | 222 | 364 |
| 8 | 756 | 1,038 | 1,120 | 282 | 364 |
| 16 | 1,700 | 2,142 | 2,064 | 442 | 364 |
| 32 | 4,356 | 5,118 | 4,720 | 762 | 364 |
| 64 | 12,740 | 14,142 | 13,104 | 1,402 | 364 |
| 128 | 41,796 | 44,478 | 42,160 | 2,682 | 364 |

The exact fitted excess slopes were:

```text
generic k  = 515212800 / 77281920 = 6.666666...
compiled k'=         0 / 77281920 = 0
```

Thus `k'/k = 0`, exceeding the preregistered requirement `k'/k <= 0.20`.
Generic excess grows with all `3*d+2` recurrent transitions; compiled excess is
exactly `364` work per invocation at every depth.

The fixed `364` consists of fresh recognition/binding plus parent-topology
validation and temporary-arrow installation. It is not hidden or omitted. At
depths 5 and 8 that fixed cost makes compiled execution more expensive than
generic interpretation; at depths 16 and above the eliminated recurrent tax
outweighs it. This crossover is descriptive only, not an RC0a economics claim.

Aggregate compiled runtime remained `279,552` work (`3.533066%`) above the
concrete reference. Compared with generic grounding, compilation removed
`461,824` runtime work, or `62.292818%` of RG0a's total excess on this mixed-depth
matrix. RC0a therefore approaches concrete through a fixed overhead; it does
not beat concrete.

## Causal-path evidence

GENERIC GROUNDED recorded:

- `459,520` reflected arrow evaluations;
- `98,688` reflected arrow firings;
- `197,376` per-step binding reads;
- `98,688` binding deliveries;
- zero compiled or direct firings.

COMPILED GROUNDED recorded:

- `98,688` local compiled evaluations and `98,688` local compiled firings;
- `173,568` parent-topology validation comparisons, paid per invocation;
- `6,144` installation-time binding reads;
- `12,288` installation comparisons and `3,072` temporary-arrow installations;
- zero reflected evaluations/firings;
- zero per-step binding reads/deliveries;
- zero direct executor, direct route, pre-resolved persistent route, fallback,
  oracle, or generic-resumption calls.

The two successful arms performed the same `98,688` dynamic recurrent
transitions and identical lower effects. RC0a changed how each transition was
dispatched, not what lower computation executed.

## Binding and invalidation controls

CHANGED BINDINGS reused the same role-only persistent compiled state across
fresh identities and non-identity physical CELL permutations. It fired all
`98,688` expected compiled transitions correctly with zero reflected fallback.
This rejects fixed destination and fixed cell-index caches.

INVALIDATED TRANSITION replaced parent learned-arrow identities while
preserving their role endpoints. Across eight seeds:

- all `32` compiled arrows invalidated before firing;
- compiled firings were zero;
- generic resumption occurred in all `768` episodes;
- the generic path fired all `98,688` expected reflected transitions;
- correctness remained `768/768`.

SUBTHRESHOLD EVIDENCE earned zero compiled arrows and used the generic path in
all `768` episodes. SHUFFLED EVIDENCE invalidated all `32` incompatible arrows
before firing and also resumed generic execution in all `768` episodes.

NO BINDINGS attempted no successful compiled transition and answered `0/768`,
confirming that persistent role-relative arrows do not contain future concrete
destinations.

## Conjunctive outcome

All preregistered gates passed:

1. frozen ancestry and RP0a reconstruction parity;
2. earned three-episode compilation;
3. role-relative persistent structure;
4. fresh and changed binding generality;
5. compiled local dispatch only;
6. unchanged lower effects;
7. pre-fire invalidation and generic resumption;
8. subthreshold non-compilation;
9. shuffled-evidence rejection;
10. binding necessity;
11. state isolation and duplicate determinism;
12. at least 80% per-step slope reduction;
13. RC0b source exclusion.

RC0a is therefore a positive compatibility result.

## Frozen interpretation and sequencing

The result establishes the reflected-level version of the project's recurring
developmental pattern:

```text
expensive generic learned execution
        -> repeated successful use
        -> consolidation
        -> cheap local recurrent dispatch
```

It does not show that reflection eliminates lower work. RC0b remains absent and
blocked pending a separate protocol decision; RE0 and F1 remain blocked. No
RC0b implementation was created during or after the definitive RC0a run.
