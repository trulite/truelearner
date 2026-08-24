# CJ1 path-local saturation candidate PROBE result audit v1

Status: **FROZEN INVALID MECHANICAL RESULT; NO POSITIVE CLASSIFICATION**.

## Frozen execution

- executed implementation commit:
  `c056f356a5aa2f9ee69eddaa2c2f1ffbf3421433`;
- E2B persistent sandbox: `i6x9gykt9tvp6xfz5z8ra`;
- authoritative PX0 SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- candidate PROBE protocol SHA-256:
  `9481bdf46793475ac6ca28d35910ddc51dd7cafbbd94ea9a7ae7ffc2f68b9984`;
- candidate runner SHA-256:
  `6085e98d81b33082349e8e8032e3bcdb83328c18b33afa2ceee1c4135670cded`;
- result CSV SHA-256:
  `38378eac9abf9609f2c73a3a05dafbbafe959fe6623660f6cf04d29850021b76`;
- generated report SHA-256:
  `16739fe0a9c78fc7faf41c1137e5627c5c9d550b94ea5231ea1c0f40f3885060`.

The exact preregistered command ran once in E2B. It produced 26 unique data
rows plus one header, no staging remnants, natural quiescence in every row and
exact duplicate replay in every row. The authoritative PX0 source remained
byte-exact. No Rust process ran on the local host.

## Earliest invalidity

The generated report's `26/26` positive label is not accepted. Its evaluator
checked effects, local firing, held-out effect, quiescence and replay, but did
not conjunctively check that each named repeated-path fixture actually produced
the preregistered source firings and traversals.

The first ordered invalid row is seed `2101`,
`two-unit-firings-one-path`: it records source firings `1` and traversals `1`,
not the required `2` and `2`. The complete mismatch set is:

| seed | row | observed traversals | required traversals | observed source firings |
|---:|---|---:|---:|---:|
| 2101 | two-unit-firings-one-path | 1 | 2 | 1 |
| 2101 | four-firings-one-path | 1 | 4 | 1 |
| 2101 | repeated-a-plus-b | 2 | 3 | 2 |
| 2101 | a-plus-repeated-b | 2 | 3 | 2 |
| 2111 | two-unit-firings-one-path | 1 | 2 | 1 |
| 2111 | four-firings-one-path | 1 | 4 | 1 |
| 2111 | repeated-a-plus-b | 2 | 3 | 2 |
| 2111 | a-plus-repeated-b | 2 | 3 | 2 |

The write-once raw artifacts remain immutable records of what executed. Their
positive wording is superseded by this audit and supplies no candidate PROBE
result.

## Structural cause and correction refusal

This is not repairable by a fresh schedule while keeping the frozen law and the
specified still-live local opportunity:

1. after a CELL fires at tick `t`, PX0 sets `refractory_until = t + 1`;
2. therefore one source CELL cannot fire twice at the same tick, and one
   outgoing physical ARROW cannot traverse twice at that tick;
3. at the next possible firing tick, PX0 decays a prior unit local CELL state by
   one, so the first bounded unit contribution is no longer a live coincidence
   contribution;
4. the repeated external arrivals in this fixture are consequently rejected by
   ordinary source refractoriness before a repeated candidate-path traversal.

Changing refractory behavior, decay, emission multiplicity, or path identity
would be a second scientific rule. Treating evaluator-intended repeated inputs
as if they were actual traversals would violate the physical-participation-only
contract. A mechanical correction cannot create the missing discriminating
event under the frozen substrate.

The candidate trace's repeated-path suppression is therefore not causally
isolated by this PROBE, and the raw positive cannot authorize MICRO. Evidence
stops here under the parent protocol's interpretation/extra-rule refusal.
