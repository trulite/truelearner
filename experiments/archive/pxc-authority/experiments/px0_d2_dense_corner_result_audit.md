# PX0-D2 dense-corner result audit

Status: **D2-A ACCOUNTING-ONLY BOUNDARY; DEVELOPMENT EVIDENCE FROZEN; PX0 AUTHORITY ABSENT**.

## Classification

The single complete 256-cell diagnostic pass supports:

> **D2-A — accounting-only boundary.** Physical scheduling can prevent one
> offered stable return from completing without damaging final resistance,
> lifetime, or behavioral specificity.

No mechanism changed. The active PX0 law remained byte-identical.

## Exact result

- cells: `256/256` complete;
- trajectory rows: `8,960` (`35` per cell);
- stable opportunities: `8,960`;
- completed stable returns: `8,896`;
- sparse opportunities: `2,048`;
- completed sparse returns: `1,728`;
- accounting-gap cells: `64`;
- resistance-boundary cells: `0`;
- behavioral-specificity breakdown cells: `0`;
- replay/integrity/quiescence: `256/256`;
- main-path work: `56,879,990`;
- evaluator-clone probe work: `36,455,696`;
- proposals: `157,184`;
- deallocations: `156,576`;
- queue comparisons: `4,946,554`.

Every final cell produced:

```text
B held-out effects       1
A held-out effects       0
stable resistance        41..61
sparse resistance        0
stable no-use lifetime   400..610 ticks
sparse no-use lifetime   10 ticks (measurement floor)
```

Thus no cell exhibited either resistance inversion or behavioral specificity
loss.

## Location of the accounting gap

The gap is determined exactly by context spacing in the tested matrix:

| spacing | cells with gap | completed stable returns |
|---:|---:|---:|
| 13 | 0/64 | 35/35 |
| 15 | 0/64 | 35/35 |
| 17 | 64/64 | 34/35 |
| 19 | 0/64 | 35/35 |

In every spacing-17 cell:

- only context `0` failed to complete its offered stable return;
- contexts `1..34` completed all `34/34` remaining returns;
- B first became executable at context `1` rather than context `0`;
- B remained executable for the remaining `34` observed contexts;
- final stable resistance was `46`;
- final sparse resistance was `0`;
- stable/sparse no-use lifetimes were `450/10` ticks;
- final effects were B=`1`, A=`0`.

The accounting gap was balanced and invariant across all tested neighboring
values:

- stride `24,25,27,28`: `16/64` gap cells each;
- load `32,36,44,48`: `16/64` each;
- incidental phase `0,1,2,3`: `16/64` each;
- normal/reverse allocation: `32/128` each;
- direct/mirrored layout: `32/128` each;
- all three route identities.

This rules out the tested density, stride, phase, allocation, layout, and route
identity as explanations for the nearby gap. It identifies a timing-specific
first-context completion boundary.

## Relation to definitive v2

D2 does not retroactively determine which hidden P6 subclause failed in spent
v2 cells 7 and 15. Those cells remain unrerun and their artifacts remain
unchanged.

D2 establishes only the fresh nearby result: spacing 17 reliably produces the
same one-return accounting shortfall while preserving every physical property
P6 was meant to protect. This supports—but does not rewrite v2 with—the
interpretation that exact return-count equality was stronger than the reusable
correspondence law.

## Integrity and hashes

- protocol commit: `b820179`;
- implementation commit: `6ecfca5`;
- result commit: `c128ddc`;
- active-law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- diagnostic source SHA-256:
  `a50dd8862b304abc107d729f236cb965c2e95cd823228ae9bb41119c65cf46a8`;
- cell CSV SHA-256:
  `2314704d08f318c394e07c3eb6bc856a1b958c6d8d45bb173d044aba0eea02ac`;
- trajectory CSV SHA-256:
  `2a55880a0ca822e0b6d691c5ec0ccd7f83360bf80fbadfe3576042bf9feb4828`;
- report SHA-256:
  `bd4c5f519afbc4688704b73d315c5ccbed7dd807606a37a00a8a2bb48a7e75fd`.

## Program boundary

PX0 remains non-authoritative. Definitive v1/v2 remain immutable failures. D2
does not authorize PX0 v3, PX1, PX-C, the continuous organism, or Harness H1.

Any future authority protocol should serialize opportunity, completion,
resistance, lifetime, and execution as independent claims and should state
whether authority concerns offered-return accounting or reusable physical
correspondence. That protocol requires separate explicit authorization.
