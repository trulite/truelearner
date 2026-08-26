# CR0 coupling-necessity v2 implementation audit v1

Status: frozen before v2 physical evidence.

Protocol: `cr0-coupling-necessity-v2-protocol-v1` (`414f953`).

## Exact repair

Relative to frozen v1 evaluator `8618c407...`, v2 changes only:

- roots `5_100_000/5_200_000` to `5_300_000/5_400_000`;
- a `physical_eq` method containing the fields already named by the v1
  protocol and excluding only diagnostic raw `live_hash`;
- the missing `case_pass` CSV header;
- separate replay, mechanics, predicate, and total aggregate booleans;
- conditional claim prose when total acceptance is false.

No family, geometry, durable state, threshold, schedule, input, predicate,
mechanics configuration, replay construction, or scientific decision changed.

V2 evaluator SHA-256:
`6de134112308ac554092653c1946befe6ba39f77813f0402b82c26a3a7ac2062`.

V2 protocol SHA-256:
`af710c6df79267cce469c8d7abbc30983b17bbe3b96d60c02e5d306579b56b5e`.

The physical core and frozen-state anchors remain protected by
`experiments/tools/audit_cr0_coupling_necessity_v1.sh`.

## Pre-evidence validation

Reusable E2B worker `iwlakum29bs73vxsgu8d0` passed targeted rustfmt, release
check, and strict Clippy on the v2 evaluator. It ran no physical world.

The next eligible event is exactly one fresh v2 matrix execution followed by
the static audit. There is no in-gate repair or rerun.
