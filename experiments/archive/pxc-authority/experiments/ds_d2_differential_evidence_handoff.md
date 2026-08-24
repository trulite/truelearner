# DS-D2 differential-evidence formation handoff

Status: **DEVELOPMENT IMPLEMENTATION READY; ENABLING ONLY; NOT CLAIM ELIGIBLE**

DS-D2 was run only in MICRO and GATE modes. No definitive experiment was run,
no result artifact was created, M0 remains authoritative, and M1 does not
exist.

## Frozen lineage

- exact parent: `353285fda96061bdcab640e53d77e710be966f06`
- protocol: `c66ad6e3d4ab8e078208ec903ee30ad6f57857e0`
- implementation: `2f1de91f9dfcf77b869fe197d0da5dae6e08a656`
- authoritative M0: `1d74c0ed0b515446161a63a6d43ecbe27514dc85`
- DS-D2 source SHA-256:
  `ac257b53e28b0bdbcfd4cbcb7ca855086d1de5812a07029f4b2405fda2a6da8f`
- runner SHA-256:
  `b5215586e3efc700009c0602618ee3962ca50fd5333b1cb5dd9929cd471d6121`
- frozen A1 source SHA-256:
  `b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1`
- frozen DS1 learner SHA-256:
  `adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e`

## Development outcome

For GATE seeds 100 through 104, the exact frozen E0+A1 lineage produced two
real executable route roots. Each root was physically executed in a temporary
counterfactual branch to expose its role-relative trace. The selected route was
then physically executed again to produce returned anonymous activity.

In every seed, exactly one route trace was structurally compatible with the
returned trace. DS-D2 formed one temporary local ARROW from the compatible
alternative CELL to the evidence CELL and verified it by firing one SPIKE
through that ARROW.

| Measure | Result |
|---|---:|
| GATE seeds | 5/5 pass |
| actual executable roots | 2/seed |
| unique structural relation | 5/5 |
| temporary directional ARROW formed | 5/5 |
| directional ARROW physically traversed | 5/5 |
| DS1 calls | 0 |
| DS1 updates | 0 |
| persistent bytes | 0 |
| DS-D2 primary work | 15/seed |
| temporary peak | 184 bytes/seed |

The two route effects have equal total activation magnitude. Supplying the
other route's structural trace reverses the relation, so magnitude alone does
not determine direction.

## Required interpretation boundary

This is an observability/attribution result only:

> Anonymous returned activity can form a temporary substrate-native
> directional relation to exactly one of two live anonymous alternatives by
> differential structural compatibility, without a supplied polarity bit.

It does **not** establish that this direction is useful credit, that DS1 will
update, that strengths will diverge usefully, or that boundary roles will be
reconstructed. In the development fixture, the returned activity is emitted by
the selected route, so structural compatibility naturally attributes the
aftermath to that route. A separately preregistered unchanged-DS1 composition
retry must determine whether that attribution merely reinforces the selected
route or supplies useful role discrimination.

## Validation

Local and exact-commit E2B validation passed:

- formatting;
- strict release Clippy;
- 15 focused release tests;
- release MICRO;
- release GATE;
- `--definitive` rejected before the harness with status 2;
- results digest remained
  `491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4`.

E2B sandbox `it2xqz1lm73su08j7snhj` uses only
`/Users/satya/.cache/truelearner/ds-d2-differential-evidence-e2b.json` and was
left running.

## Next authority boundary

DS-D2 is an enabling ancestor only. The only eligible next scientific action
is a separately preregistered, byte-identical frozen-DS1 cumulative retry.
Only a pass through DS1 update, useful strength divergence, and held-out
boundary-role reconstruction may create M1.
