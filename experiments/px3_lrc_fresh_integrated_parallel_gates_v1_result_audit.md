# PX3 on LR-C fresh integrated parallel gates v1 result audit

Status: **FROZEN DEVELOPMENT RESULT; JOINT VERDICT UNINTERPRETABLE**.

Frozen implementation commit: `1676a2364e4d2878d47e3122709bea278f33115c`
(`px3-lrc-fresh-integrated-development-v1`).

## Immutable inputs and outputs

- protocol SHA-256:
  `b118d84d81856a2d193dc4ec89b67f7df449855eed3b5a432eea329282031a5f`;
- lifecycle source SHA-256:
  `d6537fefd385c9d556d88740d1ed5fcdca6e9e73fd2c53d02da3eecd36da557f`;
- recursion source SHA-256:
  `a7872cadd04e4f8aa835f325e54633746f6402840f0238f6496ca5941428d7e5`;
- lifecycle CSV / report SHA-256:
  `1f7911072a4dfb0efc888d2a8f5fdefd9878a7deff48124c6c1c54c12d293580` /
  `7011e5008802e339d4f2870c08e809b4092379084b7c3138fb87ef551ff88443`;
- recursion CSV / report SHA-256:
  `c9d45f87ec1237e8a9febadea769a1e9fd8fcd1397f96b6c178c8622ef9b33fb` /
  `cfb9ee353cb62ee67ce28f29c134f7d19f1b50ad6f6398139eef8eda2de609cd`.

Both matrices executed once, concurrently, in distinct fresh E2B sandboxes.
The generated reports remain unchanged.

## Gate L

The generated lifecycle report is `4/4` positive. In every row:

- two immediately adjacent unsupported AB episodes produced exactly two
  candidate traversals, zero plasticity updates and resistance zero after the
  gap;
- recurrent completed AB/CD loops matured;
- obsolete AB/CD candidates deallocated under pressure;
- fresh AD/BC identities formed and matured;
- held-out behavior followed only the reversed world;
- replay was exact and all activity quiesced.

However, registered control 1 was absent from the implementation: strong A,
repeated A and gapped A then B were not independently instantiated. This is a
protocol-coverage failure, not a physical counterexample. Gate L v1 therefore
cannot carry the whole preregistered claim despite its generated positive.

## Gate R

The generated recursion report is `0/4` negative. Every substantive physical
quantity is nevertheless identical across all four rows and matches the
registered expectations:

- one-exposure candidates died;
- recurrent AB, XC and YD candidates reached resistance `13|9|5`;
- final context-free reuse emitted exactly one X, Y and Z trace;
- single-sided and gapped controls did not form the next stage;
- replay was exact and all activity quiesced.

The sole failing predicate is copied accounting from the retired attribution
geometry. `context_free()` demands `source_trace == active`. In LR-C v1 that
field serializes the threshold-two world-return relay. Context-free reuse
deliberately supplies no world return, so all four rows lawfully serialize
`source_trace == 0|0|0`; output participation traces still serialize the
expected active stages. The evaluator therefore rejected the absence of
learning traffic during execution-only reuse.

This is an evaluator error, but the generated negative is not promoted or
rewritten.

## Boundary

PX3 authority remains negative. A successor development protocol may correct
only these two defects:

1. instantiate the three omitted lifecycle controls; and
2. make context-free recursion expect zero return-relay and Modulatory traffic.

It must use fresh seeds and may not change LR-C physics or the shared PX3
physical chain.
