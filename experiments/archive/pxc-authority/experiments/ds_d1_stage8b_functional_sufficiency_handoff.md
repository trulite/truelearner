# DS-D1 stage-8b functional-sufficiency handoff

Outcome: **ALL THREE DIAGNOSTIC ARMS RECOVER; ENCODINGS EQUIVALENT AT FROZEN DS1**.

This is evaluator-supplied diagnostic evidence only. It is not claim eligible,
does not establish learnability or current-substrate observability, and does
not advance M0 or create M1.

## GATE result

| Arm | Updates | Strength divergence | Correct mature signatures | Held-out recovery |
|---|---:|---:|---:|---:|
| alternative comparison | 32/32 | 4/4 | 4/4 | 16/16 |
| polarity | 32/32 | 4/4 | 4/4 | 16/16 |
| outcome change | 32/32 | 4/4 | 4/4 | 16/16 |

The result reproduced for seeds 100..104. Every cell had zero held-out
abstentions and zero persistent property bytes.

## Decisive equivalence result

Within each seed, all three arms produced exactly the same:

- boolean update-trace fingerprint;
- E0 episode fingerprint;
- persistent frozen-DS1 learner fingerprint;
- update count and learner work;
- strength separation and mature choices;
- held-out choices and success count.

Therefore the functional screen does not identify three different missing
mechanisms. It identifies three evaluator encodings that collapse to the same
effective directional bit at frozen DS1.

The correct interpretation is:

> A directionally aligned consequence bit is functionally sufficient for
> frozen DS1 role learning. Comparison, explicit polarity, and signed outcome
> change are indistinguishable after evaluator-side conversion to that bit.

This does not authorize choosing polarity because it uses one field, choosing
comparison because roles are relational, or choosing outcome change because it
looks environmental. None has yet been reconstructed from allowed substrate
activity. The next developmental choice must be based on information content,
physical observability, and absence of evaluator semantics before economics.

## Frozen lineage and validation

- exact parent: `b599e601e9a7257c647cf5ca8f4188d77d024f02`;
- protocol: `a916a540f31b2a3b7cf6c863f3611c339ccc6268` /
  `ds-d1-stage8b-functional-sufficiency-protocol`;
- implementation: `9252b37885601bef0cd32a616eaf948bca9258f2` /
  `ds-d1-stage8b-functional-sufficiency-implementation`;
- mechanism SHA-256:
  `bfdec68e291240108f85a70251651931af2e238236653a423b54e381506af10d`;
- runner SHA-256:
  `b63ecbfe2f4c1dd7d2b27f487d74b70cdedc8564c289932ef554203424726862`;
- marked DS1 SHA-256:
  `adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e`.

The exact implementation passed formatting, strict release Clippy, 11 focused
tests, MICRO, GATE, definitive refusal with exit 2, and frozen results-digest
preservation locally and on E2B. Sandbox `i6dgym6d66zw7ug487vhc`, using only
`/Users/satya/.cache/truelearner/ds-d1-functional-sufficiency-e2b.json`,
remains running.

M0 remains authoritative. The interaction stack remains enabling-only. M1 is
absent. No learned prerequisite or cumulative retry was implemented.
