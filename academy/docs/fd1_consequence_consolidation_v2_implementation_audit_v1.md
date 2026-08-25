# FD1 consequence consolidation v2 implementation audit v1

Status: frozen before v2 physical evidence.

Protocol: `fd1-consequence-consolidation-protocol-v2` (`85e4f05`).

## Unchanged physics

The core source is byte-identical to the FD1 v1 frozen candidate:

`e7b9d60ce0330d10692b13fe85967e189d734a00177edef98018f9b4499a09ed`.

No runtime file changed after the v1 freeze. The seven family constructors,
their schedules, the physical observation type, and the scientific `predicate`
function are unchanged.

## V2-only evaluator surface

V2 adds a separate entry point with fresh roots `4_700_000` and `4_800_000`.
It computes the same replay, mechanics, quiescence, and family predicates as
v1, writes both mechanics rows and each gate boolean, writes the report and
checksums, and only then performs its final acceptance assertion.

Evaluator SHA-256:
`3b65baebd11297ee5e4bd3518d5488ac955c4b42485af3f2c2b7532aeef376ab`.

## Remote pre-evidence validation

Reusable E2B worker `idnc9zn44jihlquq89nvl` passed:

- evaluator rustfmt;
- release `cargo check`;
- release Clippy with `-D warnings`.

No v2 physical world, FD0 replay, ARC case, authority matrix, or local Rust
command ran before this freeze.

Any v2 focused or FD0 hash failure remains a negative. No in-run rescue or
rerun is authorized.
