# CPC2 adjacent local causal closure implementation audit v1

Status: frozen before CPC2 matrix execution.

## Candidate surface

CPC2 changes no runtime or substrate law. The evaluator constructs exactly the
two preregistered ordinary-topology arms:

- local Modulation at only the last adjacent contact;
- a chain of ordinary Drive relays, each emitting local Modulation and Drive to
  the next adjacent upstream relay.

The evaluator does not mutate participation or support and does not inspect a
stored causal path. It observes existing CPC1 contact states only after the
physical queue has quiesced.

## Matrix surface

The evaluator contains the nine frozen worlds, two arms, two identity roots,
ten pressure phases, Reference and Production mechanics, and exact same-
mechanics reconstruction. The unconditional result contains `360` physical
cases and `720` mechanics rows.

Arm completeness is computed only after every world is serialized. Failure of
both arms is a scientific negative, not a process error.

## Frozen hashes

```text
core lib.rs
027ec827afbf998df07749e428468196f82eb33824401b78aa15a6b48680a6cb

core mechanics.rs
5093e259a324b72a2fd661e1d402030fed356ac19d3b948549d7eea37f8b7295

evaluator Cargo.toml
8c951f19cea7db315ba036c05a666a28d5cd57c6e1ef9da076ca0bc39b861ef8

evaluator main.rs
0728221b1c6aee3b76ce51c8da8bcdc69dcfffac85690ffa6ccdcb2235fb7333

static audit
24fa278b0a8e983e5a6434eac48cceefa1af28a67a396d5f1749d0bd531699f2
```

The core hashes are byte-identical to the frozen CPC1 candidate.

## Targeted E2B validation

Reusable sandbox `itpuv2rfhsh4zc7q2ojqu` ran only evaluator formatting,
targeted release check, and strict evaluator Clippy. No physical world ran.

Two pre-freeze mechanical compile diagnostics were corrected before this audit:
one missing CSV placeholder and one Clippy row-function shape. Neither executed
a world or changed candidate geometry.

## Boundary

No pressure, ARC, authority, oracle, `arch.md`, durable learning, or old
eligibility behavior is changed or tested. No rescue arm is authorized after
the matrix.
