# DS-D0 stage-8b single-property discrimination handoff

Outcome: **DIAGNOSTIC REACHABILITY MATRIX COMPLETE**.

This result is development-only and not claim eligible. It does not install a
property, alter C0, advance the cumulative prefix, or create M1.

## Matrix result

The exact same frozen C0 parent episode and byte-identical marked DS1 learner
were used across five independent arms. GATE executed 25 cells in parallel and
restored deterministic seed/arm order.

| Single property | Update input reachable | Frozen DS1 update per seed |
|---|---:|---:|
| ownership only | no | 0 |
| temporal contrast only | no | 0 |
| alternative comparison only | yes | 1 |
| explicit polarity only | yes | 1 |
| signed outcome change only | yes | 1 |

Every result reproduced across seeds 100..104. Alternative-comparison,
polarity, and outcome-change signs varied across the frozen choices/seeds; an
update occurred for both `true` and `false` values. The two alternative
magnitudes were fixed before reading the chosen index.

The narrow result is:

> Frozen DS1's stage-8b interface requires an evaluatively directional boolean.
> Temporal ownership and raw earlier/later contrast do not populate that input.
> A supplied alternative ordering, supplied polarity, or supplied signed state
> change can each populate it and physically execute one frozen update.

This is an interface/reachability result. It does not show that any sufficient
property is necessary, learnable from allowed physics, stable across episodes,
or capable of producing correct boundary-role reconstruction. It does not
authorize pairwise arms because three single-property arms already reach the
edge. Those three are candidates for separately preregistered functional
diagnostics and later learned-enabling experiments.

## Frozen controls

- exact stage-8b collapse parent and C0/E0/DS1 fingerprints;
- same C0 choice and diagnostic choice in every cell;
- five disjoint property variants and zero combination variants;
- one diagnostic accessor outside the marked learner slice;
- zero persistent property bytes;
- ownership-only reproduces zero updates;
- temporal contrast cannot be reinterpreted as evaluative direction;
- no correctness, expected role, expected choice, or economics channel;
- no deeper learning gate or cumulative port was run.

## Lineage and validation

- exact parent: `7ea5680046b57fcbd81e31996e49be3ec3e9fc36`;
- protocol: `a029f267d28edeab328821e2e1904e78660424a7` /
  `ds-d0-stage8b-single-property-discrimination-protocol`;
- implementation: `2b937ebf0a0b68a8245d84f99491b553f3074f0a` /
  `ds-d0-stage8b-single-property-discrimination-implementation`;
- mechanism SHA-256:
  `00d30cd3d71d002e4e37fc3a47f94cb6d9bacb8ec97be54a04318b92195b4902`;
- runner SHA-256:
  `378cb24c9306dd608a4d478d67560909cb3d4445c7d64bc2d0fc5bafac910629`;
- marked DS1 SHA-256:
  `adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e`.

The exact implementation passed formatting, strict release Clippy, 34 focused
tests, MICRO, GATE, definitive refusal with exit 2, and results-digest
preservation locally and on E2B. E2B sandbox `imml7l9h23zxu3ti5xwis` uses
`/Users/satya/.cache/truelearner/ds-d0-stage8b-discrimination-e2b.json` and
remains running.

M0 remains authoritative. E0+A0+A1+R0+C0 remain enabling-only. M1 is absent.
