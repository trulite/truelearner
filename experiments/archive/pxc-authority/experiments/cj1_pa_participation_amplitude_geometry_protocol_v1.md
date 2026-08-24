# CJ1-PA participation/amplitude geometry protocol v1

Status: **PREREGISTERED; DEVELOPMENT EVIDENCE UNSPENT**.

## Frozen lineage and scope

- branch: `research/cj1-pa-participation-amplitude`;
- start commit: `a7ef15d69f35d4314fa71761bf868f614dab2713`;
- authoritative PX0 law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- frozen PX1 PT0 source SHA-256:
  `f0b754ed6f7b0603668319a0735da91b4c168f909d4024fd5ce5e2aea4197410`;
- authoritative PX1 definitive implementation SHA-256:
  `74716c87d146cb697b37ddf802c12e67a5cb93daf82ec20f8b982e54922bd696`;
- PX1 definitive CSV, result audit and authority handoff SHA-256:
  `6613ff0a96bb3a60fbe7afeb92cd64edced3c6df5dcc04fe47518db158dd88f6`,
  `fa4a516fcb6977a45e547ca1bb3b7db3b427c05b381fb60d2700e92fa2ae7c70`,
  and `ab4142a24f6ca1095c1c1364f391253752808382ac6ee70ef9d49eac722df28c`;
- CJ1-T positive result commit:
  `a7ef15d69f35d4314fa71761bf868f614dab2713`;
- seed: `2501`.

CJ1-PA is an unchanged-law development diagnostic. It imports exact PX0 and
composes the already-authoritative PX1 physical topology from ordinary
CELL/ARROW/SPIKE objects. It adds no field, wrapper law, amplitude clamp,
contributor identity, unique-count operation, list, set, candidate mechanism or
semantic event object.

PX0--PX2, CJ1/CJ1-T and all earlier protocols/results remain read-only. This
diagnostic cannot create definitive evidence, authority, PX3 restart or PX-C.

## Question

> Does the existing PX1 outlet/participation-trace topology convert one actual
> path traversal into exactly one ordinary unit trace firing independent of raw
> incoming ARROW coupling, so that a threshold-two downstream CELL responds to
> two participating physical paths but not one strong path?

## Frozen physical composition

Each side A/B contains:

```text
threshold-1 source
  -- raw scenario coupling --> threshold-1 outlet
  -- outlet firing, coupling 1 --> side-local threshold-2 trace CELL
  -- outlet firing, coupling 1 --> shared threshold-1 return hub

shared return hub
  -- identical coupling 1 --> A trace CELL
  -- identical coupling 1 --> B trace CELL

side-local trace CELL firing
  -- ordinary coupling 1 --> shared threshold-2 conjunction CELL
```

This is the authoritative PX1 PT1 relation reduced to the discriminating
surface: actual outlet execution supplies one subthreshold local trace impulse;
one ordinary shared return reaches every trace CELL; only a participating trace
CELL fires. Raw coupling exists only on source->outlet. Every downstream ARROW
has coupling `1`. All delays and phases are `0` except outlet->trace and
outlet->hub delay `1`, matching a shared later return opportunity.

Cells are physically separated far enough that generic local proposal adds no
path. A and B use separately allocated source, outlet and trace CELLs and
physical ARROWs. Evaluator scenario identity never enters the substrate.

## Six fixed worlds

Each row starts from fresh matter:

| row | raw participating paths | required trace firings | required conjunction firing |
|---|---|---:|---:|
| `a1` | A coupling 1 | A=1, B=0 | 0 |
| `a2` | A coupling 2 | A=1, B=0 | 0 |
| `a4` | A coupling 4 | A=1, B=0 | 0 |
| `a1-b1` | A coupling 1 + B coupling 1 | A=1, B=1 | 1 |
| `a2-b1` | A coupling 2 + B coupling 1 | A=1, B=1 | 1 |
| `a4-b4` | A coupling 4 + B coupling 4 | A=1, B=1 | 1 |

Every row independently records scheduled entries, actual source firings, raw
source->outlet crossings and carried impulse, outlet firings, unit
outlet->trace crossings, hub firing/return crossings, local trace arrivals and
firings, unit trace->conjunction crossings, conjunction arrivals/firing,
quiescence, work, storage and complete/permanent fingerprints.

Each world is reconstructed and executed twice. Exact replay is conjunctive.

## Decision rule

The existing-physics amplitude-normalization hypothesis passes only if all six
rows satisfy all required physical counts. In particular:

- changing one path's raw coupling from `1` to `2` or `4` changes carried raw
  impulse but never outlet firing count, unit participation crossing count,
  trace firing count or conjunction firing;
- every two-path row has two actual raw crossings through two different ARROWs,
  two outlet firings, two trace firings and exactly one conjunction firing;
- a shared return alone at the nonparticipating trace never fires it;
- all runs naturally quiesce and exactly replay.

If a strong single path creates two trace firings or fires the conjunction, the
hypothesis is negative. If a fixture fails to instantiate its required actual
crossings, the result is invalid. No correction may change the physics or
accept raw impulse in place of trace firing.

A positive result resolves only CJ1's mature-amplitude aliasing at the
authoritative PX1 participation layer. It does not itself authorize a new law
or a later research stage.

## Execution and write-once artifacts

The sole command, run from repository root, is:

`cargo run --manifest-path arms/cj1-pa-trace-amplitude/Cargo.toml --release -- --geometry`

It may create only:

- `results/cj1_pa_participation_amplitude_geometry_v1.csv`;
- `results/cj1_pa_participation_amplitude_geometry_v1.md`;
- matching hidden `.staging` files during atomic publication.

Before execution: exact hashes, clean tree, focused formatting/tests/strict
Clippy, refusal of other commands and absent destination/staging paths. After
execution: schema, row order/uniqueness, exact hashes, independent native-count
audit, staging-remnant absence and remote parity. All compilation, testing and
runtime execution occurs in E2B only.
