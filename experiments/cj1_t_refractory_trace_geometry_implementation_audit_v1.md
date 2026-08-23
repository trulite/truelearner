# CJ1-T refractory/trace geometry implementation audit v1

Status: **IMPLEMENTED AND FROZEN; EVIDENCE UNSPENT**.

## Frozen source

- authoritative PX0 SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- protocol SHA-256:
  `4c904459c7684261d1a5c63b1ff16eb3a6dc47dbf8596ec23386f254834c8762`;
- `arms/cj1-t-window-geometry/Cargo.toml` SHA-256:
  `45e8c268482d8a95d1cbafdcb55f69d7c0bc1b31eda4b8918a35d71485729d8e`;
- `arms/cj1-t-window-geometry/src/main.rs` SHA-256:
  `fe0998bbc6847d99f38906dda5c4bffd49199b7e22717cb4d0682e7e3672cbfa`.

The arm imports `px0-physical-correspondence` directly. It has no build script,
generated substrate, wrapper, candidate or alternate law.

## Fixture audit

The same-path rows use one threshold-one source, one coupling-one ARROW and one
threshold-two receiving CELL. Each row independently propagates the first input
to quiescence, advances through the native API to its fixed offset, then enters
and propagates the second input. The runner counts actual source trace firings
and cross-region ARROW crossings; scheduled inputs are not counted as either.

Native `local_return_updates` is used as the only liveness observation. PX0
increments that ledger field exactly when the arrival finds a live
`eligible_until` on an outgoing ARROW and performs its ordinary close. No
private field accessor or evaluator-side reconstruction is introduced.

The distinct control has two independently allocated sources and two physical
ARROWs into one locus. After genuine simultaneous traversal, separate exact
clones probe A and B. A native local-return update in each clone independently
establishes that both path-local eligibility values were live in the shared
post-event state.

All seven rows are reconstructed twice and exact-equality checked. The runner
contains one command and four create-new destination/staging paths. It contains
no mechanism change, candidate, MICRO, GATE, definitive, authority, PX3 or PX-C
surface. No evidence or Rust command ran while this audit was prepared.
