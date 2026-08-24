# CJ1-T refractory/trace window geometry protocol v1

Status: **PREREGISTERED; DEVELOPMENT EVIDENCE UNSPENT**.

## Scope and frozen inputs

- branch: `research/cj1-t-refractory-trace-geometry`;
- start commit: `5ff2f1cef95bd85c69a07c60e802f539d057c3b1`;
- authoritative PX0 law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- seed: `2401`;
- layout: normal;
- source threshold: `1`;
- receiving CELL threshold: `2`;
- ARROW delay/phase/coupling: `0/0/1`;
- load: `0`.

CJ1-T is an unchanged-physics development diagnostic. It imports the exact PX0
crate and adds no wrapper law, field, accessor, mechanism or candidate. PX0--PX2,
CJ1, every earlier result and every authority surface remain read-only. CJ1-T
cannot create definitive evidence, authority, PX3 restart or PX-C.

The question is only:

> Can one physical ARROW traverse twice while its first eligibility trace is
> still live, and how does that compare with two different physical ARROWs
> traversing once each at the same tick?

Scheduled pulses never substitute for observed CELL firings or ARROW crossings.

## Same-path sweep

Each offset `d` in `[0, 1, 2, 3, 4, 5]` starts from a fresh substrate. At tick
`0`, one external unit reaches threshold-one source A, A fires, and the single
A->locus ARROW must traverse once. The substrate then advances ordinarily to
tick `d` and receives a second external unit at A.

For each fresh row independently record:

- first source firing and first A->locus traversal;
- ordinary pressure/expiry work while advancing to `d`;
- whether the second arrival performs an existing local-return update on A's
  outgoing ARROW, which is the unchanged substrate's observable proof that the
  eligibility value was live immediately before that arrival;
- second source firing and actual second A->locus traversal;
- second local arrival, impulse and local firing;
- cumulative local firing;
- eligibility writes, return closes, work, storage, fingerprints and natural
  quiescence.

No result field is inferred from the scheduled second pulse. In particular,
`trace_live_before_second_arrival` is true only when the second execution's
native `local_return_updates` is exactly one.

The fixed geometry checks are:

| offset | location | required observation |
|---:|---|---|
| 0 | inside source refractory interval | no second firing/traversal; first trace observed live and closed by the arrival |
| 1 | first traversable tick | second firing/traversal; first trace observed live and closed before the new traversal |
| 2 | inside eligibility interval | same as offset 1 |
| 3 | inside eligibility interval | same as offset 1 |
| 4 | eligibility deadline | same as offset 1; deadline remains inclusive |
| 5 | first post-expiry tick | ordinary expiry; second firing/traversal; no live trace to close |

At no positive offset may two unit contributions fire the receiving threshold-2
CELL: ordinary unit state decay is independently serialized before the second
arrival.

## Distinct-path control

A fresh control enters A and B together at tick `0`. Two threshold-one sources
must fire, two different physical ARROWs must cross into the same threshold-two
locus, and the locus must fire once. Eligibility writes must cover both paths.

After this execution, two separate exact clones receive a same-tick probe at A
or B. Each probe is source-refractory and therefore creates no new traversal,
but each must independently produce exactly one local-return update. This is
the unchanged substrate's proof that A's and B's separate ARROW-local
eligibility values were simultaneously live after the genuine two-path event.

## Interpretation

The result must distinguish three facts:

1. `R >= T`: no post-refractory same-path traversal occurs while the first
   eligibility is observed live;
2. `R < T, closure-before-retraversal`: such an interval exists, but the second
   arrival closes the old eligibility before the second traversal writes a new
   one;
3. `R < T, retained-through-retraversal`: a second traversal occurs without an
   intervening close while the old eligibility remains live.

The earliest row decides among these only if every required firing, traversal,
native work update and control is present. A fixture/accounting mismatch is
invalid, not a scientific result.

Even a positive geometry result does not erase CJ1's frozen mature coupling-2
amplitude collapse. It answers only the same-path repetition versus genuine
two-path portion of the conjunction question.

## Execution and write-once artifacts

The sole command, run from repository root, is:

`cargo run --manifest-path arms/cj1-t-window-geometry/Cargo.toml --release -- --geometry`

It may create only:

- `results/cj1_t_refractory_trace_geometry_v1.csv`;
- `results/cj1_t_refractory_trace_geometry_v1.md`;
- matching hidden `.staging` files during atomic publication.

Every row is reconstructed twice from fresh matter and must replay exactly.
Before execution: exact hashes, clean tree, focused format/tests/strict Clippy,
absent destination/staging paths and command-surface refusal. After execution:
schema, row count/order/uniqueness, exact hashes, native-accounting invariants,
staging-remnant absence and remote parity are audited. All compile, test and
runtime execution occurs in the established E2B sandbox only.
