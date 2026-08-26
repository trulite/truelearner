# CORE1 E16 — First Perturbation Result v1

## Status

Development result complete. The accepted organism/runtime was not changed, no
authority was advanced, and ARC was not rerun.

Protocol: `core1_e16_first_perturbation_protocol_v1.md`

Frozen evaluator: `core1-e16-first-perturbation.rs`

Evidence: `experiments/results/core1_e16_first_perturbation_v1/`

## Result

| Arm | First motor action | Full consequence-learning chain |
|---|---:|---:|
| Zero initiation | 0/8 | 0/8 |
| S — deterministic self-trigger | 8/8 | 2/8 |
| V — blind spontaneous variation | 8/8 | 8/8 |

Every initiated first action occurred after four physical ticks. All 48
Reference/replay/Production arm rows naturally quiesced. Replay equality and
Reference/Production equality were exact on all 48 rows.

## What happened

The zero-initiation control reproduced the frozen E14 boundary exactly:

```text
unresolved context
→ no motor action
→ no Modulation
→ no plastic update
→ natural quiescence
```

The self-trigger arm successfully caused action number one in every seed. It
deterministically repeated the first opaque downstream opportunity. In the two
seeds where that route happened to be useful, ordinary consequence and PQLC
made it autonomous. In the other six seeds it repeatedly expressed the wrong
route, received no consequence, made no plastic update, and learned no action.

```text
self-trigger
→ first blind route participates
→ same route repeats

useful by coincidence (2/8)
→ consequence
→ PQLC
→ autonomous preference

not useful (6/8)
→ no consequence
→ no update
→ no autonomous action
```

The spontaneous-variation arm also knew nothing about usefulness. It visited
the four opaque opportunities once in replayable seed-dependent physical order.
In every seed the useful route eventually participated. Ordinary consequence
then returned twice through the unchanged E15-C/PQLC machinery, producing
`6|6` Modulatory deliveries and `15|25` plastic updates. The useful action was
autonomous afterward in all eight seeds.

```text
blind variation across eligible routes
→ useful perturbation eventually participates
→ changed experience
→ ordinary consequence
→ PQLC
→ useful perturbation becomes autonomous
```

Action IDs, insertion order, and usefulness were permuted. The learned action
followed consequence in every row.

## Interpretation

Self-triggering is sufficient for initiation but not for general symmetry
breaking. It can continue one opaque possibility; it cannot, by itself, expose
alternatives when that possibility is unhelpful.

Blind spontaneous variation is sufficient for the complete E16 chain. It does
not need curiosity, uncertainty, information gain, history preference, reward
before the first action, or benchmark-specific action knowledge. Once it causes
the first useful perturbation, the already-established consequence-only physics
does the learning.

This is a sufficiency result, not yet an adoption of noise as accepted organism
physics. The experiment used a bounded replayable variation schedule, so it
does not establish the weakest lawful frequency or distribution of spontaneous
events. That is the remaining design question if this affordance is integrated.

## Mechanical evidence

- Matrix rows: 48 (8 seeds × 3 executions × 2 arms)
- Zero controls silent and quiescent: 8/8
- First-action latency when initiated: 4 ticks in every row
- Natural quiescence: 48/48
- Exact replay: 48/48
- Reference/Production exact: 48/48
- Physical work range: 384–8,674

SHA-256:

- matrix: `9ea5cc27e8e9b807be43285f315b970120e14bfc20de76f5b8aaca9b86f42073`
- generated report: `f838fc05ea519b5c76cea5472ccd26ed3ececb6f7031d8507c0adf851d9351e9`
- frozen evaluator: `08b50cacdcc05f2f5721de0267e8449fddaceafe844e6dbc6c5ea9b0077f2912`
- protocol: `00d83e262dc5400d0db0b8b0aa7bd5533a3b500a98c054c261b3cf4fd542c346`

## Scientific boundary

E16 answers only:

> What causes action number one when an unresolved context has several opaque
> possibilities and no learned preference?

It does not alter CORE1-B, rerun ARC, introduce an exploration policy, or claim
that stochasticity is intrinsically valuable. The narrow result is:

> Deterministic self-continuation starts activity but remains on one symmetric
> alternative; bounded blind spontaneous participation exposes alternatives,
> after which ordinary consequence learning acquires the useful perturbation.
