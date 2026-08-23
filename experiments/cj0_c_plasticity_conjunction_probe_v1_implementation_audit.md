# CJ0-C plasticity-only conjunction PROBE v1 implementation audit

Status: **IMPLEMENTED AND FROZEN FOR ONE-SHOT DEVELOPMENT EXECUTION**.

## Lineage and protocol

- authoritative start: `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`;
- frozen v1 protocol SHA-256:
  `a9821e060394e77f5e15c90710e8d1809133d800091cf0cd530792af1be54427`;
- frozen v2 cadence amendment SHA-256:
  `adc58816e9f78634879c16140de2bb508c5f11261a13d871063e1aa0d68d019f`;
- authoritative PX0--PX2 law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

No PX0--PX2 path was edited. No PX3 or PX3-R source, protocol, result, or
mechanism was imported or reinterpreted.

## Isolated implementation

- manifest SHA-256:
  `7cf337f9a68135cda81e7d84f81ba9bf12105be94469ba0d028f34e7701527e9`;
- lockfile SHA-256:
  `d80245feb045d9e483592958909f617aaaa31cf17eeaf243321ffdecc2594caf`;
- mechanical authority-include build script SHA-256:
  `b249097dee53f68e0bb73a7da690b06eb0d306e555d92f3b653a19d514528f1f`;
- library include boundary SHA-256:
  `8b760b67839476460c2537491cd56e911641b377f6623290210b4a8b49863aeb`;
- organism-visible candidate addition SHA-256:
  `078e6d9fc653b8c8f07243b581c99a5e6a9fbb4371d5c92e197b6bcecda8fd40`;
- construction/evaluator/atomic runner SHA-256:
  `004bd46c169881a8f545cb79d8de2a2da398430639915eef43378de032312aa6`.

The build script reads the authoritative source directly, verifies its exact
frozen header, strips only crate-root inner documentation/attribute lines that
cannot appear inside an included module, and mechanically includes every law
byte after that header. The candidate addition is a distinct included source
with explicit organism-visible boundary markers.

## Static mechanism audit

- persistent fields added to substrate state: `0`;
- new execution primitive: `0` (ordinary CELL threshold, ARROW scan, and
  SPIKE delivery execute retained structure);
- candidate proposal uses current CELL refractory timing plus live incoming
  ARROW eligibility from actual traversal;
- candidate support uses two distinct currently eligible incoming ARROW
  sources and existing resistance;
- candidate structure uses only ordinary coupling-`2`, resistance-`1`,
  delay-`1` ARROW records into ordinary threshold-`4` CELL matter;
- external co-firing without incoming traversed eligibility cannot propose;
- a learned output fires through the same ordinary incoming ARROW path and is
  therefore eligible as a contributor without a level flag;
- no reset, cutoff, expected-result callback, changed-organization signal, or
  historical reactivation path exists.

The exact forbidden-token scan over the organism-visible addition returned
zero hits for Event, Episode, History, Boundary, Pair, Group, member,
semantic, relation, evaluator, scenario, expected, trained, crossed, and
serializer as whole words, case-insensitive.

## Pre-execution validation

- focused `cargo fmt --check`: pass after mechanical formatting;
- focused `cargo check`: pass;
- focused strict `cargo clippy --all-targets -- -D warnings`: pass;
- no-CELL/ARROW preflight is embedded;
- exact protocol/authority hash preflight is embedded;
- no-argument and wrong-argument refusal are embedded;
- result/staging overwrite refusal and atomic same-directory rename are
  embedded;
- compilation did not execute the organism or create result artifacts.

The one permitted command after this implementation is frozen is:

```text
cargo run --quiet --manifest-path arms/cj-c-plasticity-conjunction/Cargo.toml \
  --bin probe -- --protocol adc58816e9f78634879c16140de2bb508c5f11261a13d871063e1aa0d68d019f
```

This remains a non-claim-eligible PROBE. A positive result can only enable a
separately preregistered MICRO.
