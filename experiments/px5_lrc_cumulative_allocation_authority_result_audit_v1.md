# PX5 LR-C cumulative allocation authority result audit v1

Status: **DEFINITIVE POSITIVE; PX5 AUTHORITY EARNED; PX6 UNCHANGED**.

## Frozen serial chain

| role | commit | tag |
|---|---|---|
| PX4 authority parent | `2348f4318e4c4ca85d6be06017e8ccd7be8b9c87` | `px4-lrc-lifetime-authority-v1` |
| PX5 authority protocol | `e553f0f73f806ab859a3651e35d8e127ac736f17` | `px5-lrc-allocation-authority-protocol-v1` |
| PX5 frozen implementation/audit | `4561cbfd4d445f04b28295fbdbfd52803c9a457c` | `px5-lrc-allocation-authority-frozen-v1` |
| immutable functional evidence | `940cc35bce7aec6195adb6862432775236da5133` | `px5-lrc-allocation-definitive-positive-v1` |

The authority protocol commit is directly parented by PX4 authority. The
isolated development evidence and all `px5-lrc-allocation-development-*` tags
remain unchanged on their original branch. No isolated evaluator commit and no
unrelated PX4 development residue was cherry-picked into the serial line.

The retained active sources remained exact:

| active source | SHA-256 |
|---|---|
| LR-C law | `7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10` |
| PX4 API | `a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71` |

PX5 added no active organism source and required no scientific or
architectural fork.

## Minimal targeted preflight

No Rust tool ran locally. Formatting-only sandbox `irv8t6cv96f5ytwi5cw3q`
canonicalized the evaluator without compiling. After all edits were batched,
fresh targeted sandbox `isrd29atat4dpi6lj18fd` ran only the changed package:

```text
format check                                             PASS
one release binary build                                PASS
one no-world matrix-definition test                 1/1 PASS
strict Clippy -D warnings                               PASS
static hash/leakage/dependency/coverage audit            PASS
no-world authority preflight                             PASS
```

No preflight constructed a world, performed exact replay, created an artifact
or spent authority evidence. No workspace-wide build or unrelated suite ran.

## One-shot definitive matrix

The sole registered command executed exactly once from clean frozen commit
`4561cbfd4d445f04b28295fbdbfd52803c9a457c` in fresh E2B sandbox
`it0n0d8swqqy7djjo4nar`, state file
`px5-lrc-authority-definitive-v1.json`:

```text
cargo run --release \
  --manifest-path arms/px5-lrc-allocation-authority/Cargo.toml \
  -- --authority-v1
```

It emitted exactly one evidence-spent marker, evaluated the complete matrix
once, emitted the registered completion line, and atomically published the
CSV/report pair. It was not rerun or regenerated.

| functional artifact | SHA-256 |
|---|---|
| `results/px5_lrc_allocation_authority_v1.csv` | `5ccfa15b6da93ac276b9474c4d501ef9c7769748c52dbf7a8882620758b1259a` |
| `results/px5_lrc_allocation_authority_v1.md` | `e96622614e4c9569f1f90d60fa0ef822072afae5e09c316b2c37344e31f194ed` |

Exact result:

```text
rows                                      24/24
row clauses                              432/432
global clauses                              4/4
total clauses                            436/436
exact complete-state replay                 true
natural quiescence                          true
maximum row work                  55186 / 100000
maximum persistent bytes            20880 / 24000
repeated-reuse memory stable                 true
PX0--PX4+LR-C conformance                    true
```

Every root `561001..561008`, load `8/32/128`, returned-site order, reflected
layout and identity replicate passed. Exact replay occurred only inside the
single definitive command.

## Physical allocation measurements

Measurements were invariant within each load:

| unsupported load | blank proposals | traversals | accepted updates | live at tick 30 | dead unsupported | live at tick 70 | reuse proposals | stable bytes |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 8 | 9 | 9 | 1 | 1 | 8 | 1 | 0 | 1680 |
| 32 | 33 | 33 | 1 | 1 | 32 | 1 | 0 | 5520 |
| 128 | 129 | 129 | 1 | 1 | 128 | 1 | 0 | 20880 |

Each returned/unsupported candidate began at resistance `4/1`. Ordinary
pressure left only the returned candidate live at resistance `1` at tick `30`,
deallocated it at tick `40`, and blocked its dead handle. The next ordinary
PX4 arrival created a distinct proposal; the unchanged downstream LR-C loop
returned qualified modulation and raised it to `4`. After matched pressure it
was again the sole live variable arrow. Four further reuse exposures created
zero proposals and left persistent bytes exact.

Return-before-proposal and late-return controls produced zero accepted updates
and no retained candidate. A queued weak generation crossing the pressure
boundary fired zero targets; a later ordinary arrival formed a distinct
proposal that alone fired. Fresh PX4 `Field` controls reproduced recurrence
resistance `4 -> 7`, zero mature-reuse proposals, physical deallocation and
distinct-generation reacquisition at resistance `4` in every row.

## Engineering gates

The authority evaluator uses ordinary explicit Rust structs, arrays, vectors
and direct loops. It contains no unsafe code, interior mutability, hidden
global/thread-local state, proc macro, code generation, semantic allocator or
admission adapter, artificial lifetime/leak workaround, or measured-outcome
feedback into later organism input. Serialization occurs after the verdict and
does not participate in mechanism behavior.

The fixed work ceiling and byte ceiling passed with substantial margin. Memory
stability counts the existing authoritative generation-safe tombstone rather
than hiding it. All propagations naturally quiesced.

## Serial PX-C taxonomy and comparator

After functional evidence was committed, fresh E2B sandbox
`ikaimtk44x8fwij9on523`, state file `px5-lrc-authority-pxc-v1.json`, ran the v2
taxonomy over exact manifest v3 hash
`32fda2d86f6c836438fdd01fc5433c8731f4ec92e3e052e5d1a64fe751d15388`.
It then compared the raw output directly against PX4's immutable authority
taxonomy and manifest hash
`28924746e951645047225d8d20f5c5f98d93f349f46f7c6d7019e68632ce51b9`.
No Rust or functional replay ran in this sandbox.

Taxonomy result:

```text
primary seams                      283
unique source lines                221
semantic guard                     119
evaluator guard                    477
PX0--PX3+LR-C foundation seams       0
PX4 seams                            0
PX5 seams                            0
```

Exact comparator delta:

| metric | PX4 parent | PX5 authority | delta | passed |
|---|---:|---:|---:|:---:|
| primary seams | 297 | 283 | -14 | true |
| semantic guard | 162 | 119 | -43 | true |
| evaluator guard | 559 | 477 | -82 | true |
| new seam kinds | 0 | 0 | 0 | true |
| new guarded surfaces | 0 | 0 | 0 | true |

Exact primary-kind delta:

| kind | before | after | delta |
|---|---:|---:|---:|
| typed representation | 85 | 85 | 0 |
| explicit mechanism invocation | 66 | 62 | -4 |
| episode reset boundary | 1 | 1 | 0 |
| seed history synthesis | 9 | 5 | -4 |
| semantic condition | 38 | 38 | 0 |
| manual temporary cleanup | 1 | 1 | 0 |
| typed handoff | 90 | 84 | -6 |
| evaluator-derived input | 7 | 7 | 0 |

Exact layer delta:

| layer | before | after | delta |
|---|---:|---:|---:|
| PX0--PX3+LR-C | 0 | 0 | 0 |
| PX4 | 0 | 0 | 0 |
| PX5 | 14 | 0 | -14 |
| PX6 | 37 | 37 | 0 |
| PX7 | 136 | 136 | 0 |
| PX8 | 110 | 110 | 0 |

| PX-C artifact | SHA-256 |
|---|---|
| taxonomy inventory | `ddcb8f071c1dcdc6dde6b3a3500d824f30504bcd61c14970d09be1c5c279f5a0` |
| guard inventory | `4f51212d5935104fe28fea9897b9e166428b628612a4df23f33a2310806667b7` |
| taxonomy summary | `c6e5ebc2bf9ca33387a52fa2e7e900bb2946cc357bcfe2ff08820022a2bf9e37` |
| taxonomy report | `93927f76beb607ea531bb444b75ee1ba105a8fa84d05702105f3872660cb1416` |
| readiness delta CSV | `2be1cd2cde1d3639f2f973d2812b7f7d65e6bc107576002e23b4d9a6ba3de888` |
| readiness delta report | `e570e1c3770db539e28874c9a9ca7bb7e5ebd8a9b496c7a90d51ad04601e51f8` |
| kind delta | `b571d1e599c3e623072efd4a1721c75b5f2c54e49c020edfcc306959cfd949f0` |
| layer delta | `b7368f0557f6eeb6b9b1aa33217d56d9e212d8790d14338759039df173266963` |
| new kinds | `7e5ecf41e673f27bfc5957420ba466da02c700c15e982bfeed4727058ce3c0de` |
| new guarded surfaces | `d5033ae75b748d89a215895d25406b7ab5155f622e42dcd59ec72db19a3f7ca9` |

The new-kind and new-surface files contain headers only. Manifest v3 changes
only the two PX5 predecessor rows; every other layer row remains exact.

## Authority decision

All preregistered functional, cumulative-conformance, engineering, coverage,
leakage and serial PX-C conditions passed. Physical plasticity allocation
therefore collapses onto existing local proposal/traversal, eligibility,
qualified LR-C modulation, resistance, ordinary pressure, generation-safe
deallocation and ordinary reacquisition on the exact authoritative PX4 parent.

PX5 authority is earned. A separate clean authority handoff may now be frozen.
This audit does not execute, prepare, reinterpret or advance PX6.
