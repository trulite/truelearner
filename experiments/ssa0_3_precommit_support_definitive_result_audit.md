# SSA0.3 definitive pre-closure support result audit

Status: **INDEPENDENT AUDIT PASS; DEFINITIVE POSITIVE FROZEN**.

## Authority chain

| stage | commit | tag |
|---|---|---|
| immutable prior Classification C | `34277893201c1a72765b143de4b3da1912b6e3b6` | `ssa0-spatiotemporal-affordance-micro-v1-negative` |
| frozen developmental Classification A | `eeb14186a000a7eefba17e6f9e288e7335c44043` | `ssa0-3-precommit-support-development-v1-classification-a` |
| definitive protocol | `1ccbc3c588821269eda4fb8552f728a3638808b6` | `ssa0-3-precommit-support-definitive-protocol-v1` |
| exact executable snapshot | `e35b7d22c0bbc80996e344186534e7aedede3191` | `ssa0-3-precommit-support-definitive-implementation-v1` |
| definitive positive result | `cc90d38be2d16bce54daa64986a73f3b3aa36231` | `ssa0-3-precommit-support-definitive-positive` |
| authoritative M6, unchanged | `aa4e22efd8a65b7694956a53cfaa970582695215` | `core-autonomy-checkpoint-established` |

## One-shot execution record

The sole authorized command executed exactly once from the clean tagged
snapshot `e35b7d22c0bbc80996e344186534e7aedede3191`:

```text
cargo run --release --quiet --bin ssa0_3_precommit_support_definitive -- --definitive
```

It ran in fresh dedicated E2B sandbox `irak38zpevfo98joodapc` under state:

```text
/Users/satya/.cache/truelearner/ssa0-3-precommit-support-definitive-authority-e2b.json
```

The command crossed the boundary at ordinal `0`, completed all rows, published
both fixed artifacts, and exited `0`. It was not rerun, rescued, tuned,
resumed, replaced, selectively reused, or reinterpreted. No Rust command has
run after the result. The sandbox remains running with its original executed
snapshot and an 86,400-second timeout.

## Write-once artifact audit

| artifact | lines | SHA-256 |
|---|---:|---|
| `results/ssa0_3_precommit_support_definitive_v1.csv` | 745 | `50c46962c2388359a46b2a12ce74f8bcba4bcbb33c651f2c908fcd35e16ee631` |
| `results/ssa0_3_precommit_support_definitive_v1.md` | 23 | `03a449491c68d33088c12b8710e4bd9ba50a6b2ccc6c9e8534eb459469fa3068` |

The downloaded bytes match read-only hashes recomputed in the original remote
`e35b7d2` execution directory. Both original remote staging paths are absent.
The local staging paths are absent. The result commit contains exactly the two
new artifacts; the executed source and runner retain SHA-256 values
`0ea4ba971be9456b4c737d7e11613f7cfd31b338bcb8a5e256644ecca4c45c1f`
and `8b8b532428795cc3d746c3ea8a765ea494140903743137547985c419cd337c9a`.

## Complete CSV audit

The independent parser consumed all 745 lines. It verified:

- one exact 26-column header and 744 26-column case rows;
- protocol namespace exact on every row;
- ordinals sequential and unique from `0` through `743`;
- all expected and observed first-route values exact;
- all expected and observed closure ticks exact;
- replay, immediate inhibition, added-event order, late visibility, single
  effect, disabled-path absence, and row status true/PASS on every row;
- 744 unique permanent fingerprints and 744 unique complete-start
  fingerprints;
- no duplicate schedule/allocation/mirror/family/condition key;
- target mirrors balanced `372/372`;
- schedules balanced `248` rows each;
- allocations balanced `186` rows each;
- family counts timing `168`, number `96`, impulse `96`, spacing `144`, count
  `96`, and route `144`; and
- the exact descriptive work sum `321,936`.

All 744 rows passed and all 1,488 complete-state physical propagations were
duplicate-exact.

## Causal temporal result

Each condition has 24 transfer/schedule/mirror arms:

| family/condition | target realizations | competitor realizations |
|---|---:|---:|
| baseline | 0/24 | 24/24 |
| well before | 24/24 | 0/24 |
| just before | 24/24 | 0/24 |
| closure tick, before firing order | 24/24 | 0/24 |
| closure tick, after firing order | 0/24 | 24/24 |
| just after | 0/24 | 24/24 |
| well after | 0/24 | 24/24 |

The actual first contender threshold firing was followed immediately by the
same-tick inhibitory impulse `-64` to the other contender in every row. The
other contender never fired afterward. Same-tick support on the pre-firing
side reversed the realized executable route in every arm; physically visible
equivalent support on the post-inhibition side did not. No organism-visible
boundary represented this distinction.

## Count, impulse/activity, and spacing controls

| paired intervention | early target | matched late target |
|---|---:|---:|
| one extra unit supporter | 24/24 | 0/24 |
| two extra unit supporters | 24/24 | 0/24 |
| one impulse-1 supporter | 24/24 | 0/24 |
| one impulse-2 supporter | 24/24 | 0/24 |
| two wide-spaced supporters | 24/24 | 0/24 |
| two near-spaced supporters | 24/24 | 0/24 |
| two coincident supporters | 24/24 | 0/24 |

The same-count comparison reversed only with pre-closure delivery. In the
different-total/equal-preintegration family, target total support was four,
five, six, or seven, while exactly three target deliveries preceded baseline
closure. Every total realized the competitor in `24/24` arms. Thus static
total count had no independent behavioral effect.

These are external deterministic descriptions, not stored probabilities or a
smooth curve. No count became an organism score.

## Transfer and route controls

All three fresh closure schedules, four fresh identity/allocation/layout
families, both mirrors, physical identity/occurrence namespaces, handle/order
reversals, rotations, inert padding counts, and layout shifts passed.

For blocked, stale, and absent target paths, the competitor realized in
`24/24` arms per control. For blocked, stale, and absent competitor paths, the
target independently traversed its live CELL/ARROW/SPIKE route and reached its
distinct nonempty effect in `24/24` arms per control. Disabled contenders and
effects were absent from the fired sequence.

The executed preflight also passed every frozen SHA-256, byte-exact copied
physics, forbidden-runtime, matrix-shape, namespace, and write-once-path gate.
No RNG/noise, chooser, sampling, probability, score, semantic action/effect
label, evaluator-selected firing, or commitment metadata entered the runtime.

## Definitive disposition

The preregistered positive conjunction is satisfied. The narrow law is:

> In deterministic local CELL/ARROW/SPIKE affordance competition, support can
> causally alter the realized trajectory only while it enters before the
> contender's first threshold firing and resulting inhibitory closure;
> equivalent support after closure is behaviorally inert.

This law is admitted to the frozen candidate minimal substrate specification.
Immutable prior Classification C and developmental Classification A remain
unchanged. This result does not claim usefulness, creativity, generativity,
stochastic-policy value, learned exploitation, or a probability law. M6/M7
authority is unchanged. Lane A is isolated. SSA1 and SSA2 remain blocked.
Program-priority decisions remain outside evidentiary scope.
