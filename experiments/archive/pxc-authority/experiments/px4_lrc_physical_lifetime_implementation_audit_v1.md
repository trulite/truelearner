# PX4 LR-C physical lifetime implementation audit v1

Status: **IMPLEMENTATION FROZEN; E2B PREFLIGHT PASSED; DEVELOPMENT EVIDENCE UNSPENT**.

## Frozen snapshot

- authority ancestor: `f9057fe78a86db9111b0b69310d03accef3bc970`;
- protocol commit/tag: `6df191b3d6d6e7686d2e0e6e24fc9675e30d2e59` /
  `px4-lrc-lifetime-development-protocol-v1`;
- implementation commit/tag:
  `7c39b45fa9e7edfd393240368684185f46883dd5` /
  `px4-lrc-lifetime-development-implementation-v1`;
- successful fresh E2B preflight sandbox: `ix50ch0df4g0bfyvfb6ce`;
- failed compile-only preflight sandbox before the mechanical shadowing fix:
  `ic74p4yt92m9ho1xyx1kr`; it contained no experimental run and was later
  terminated to release capacity for the final taxonomy replay;
- formatting-only fresh E2B sandbox: `ieehzs5xzu8ivdqo1y7nk`.

The earlier compile-only preflight stopped on a Rust name-shadowing error.
Commit `7c39b45` changes only the local binding `curve` to `base_curve`. No
physical constant, schedule, gate or identity changed.

## Exact hashes

| artifact | SHA-256 |
|---|---|
| retained LR-C source | `7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10` |
| development protocol | `dc1bb5efe1a5cfe2f2be0b6c21d1df675213d3334ca784c0844b1b61bc1577dc` |
| arm manifest | `ad9d7fdf3a14dff580ef3d6b940832ad7b28aa80c68adabb2aab447ede0ae19b` |
| active PX4 source | `a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71` |
| evaluator runner | `1616c9ea339f70ddacfc4c7c4383caa1565043beaa350e6b1ac1be61514882ca` |
| evaluator-only tests | `13e8505fe7cdf281ae4994d5416d74232765f71ab2ae6542ade07390a0a19961` |
| PX-C v1 manifest | `472440f5e989387044fa3d36c5364b2d65f30d01659742a829d007cb67f7ef9a` |
| PX-C v2 manifest | `28924746e951645047225d8d20f5c5f98d93f349f46f7c6d7019e68632ce51b9` |

## Active-mechanism coverage

The complete candidate dependency graph is:

```text
arms/px4-lrc-lifetime/src/lib.rs
    -> crates/lr1-modulatory-physical-return/src/lib.rs
    -> Rust standard library
```

The PX4 library contains only the three-cell and five-cell physical geometry
builders plus ordinary anonymous arrival insertion. It introduces no state
law. Every cell, arrow, spike, proposal, eligibility write, modulation update,
resistance change, pressure update, deallocation, generation check and
propagation step is implemented exclusively by the byte-frozen LR-C source.

`experiments/pxc_active_surface_manifest_v2.csv` therefore manifests both
complete organism-visible sources: the authoritative LR-C substrate and the
new PX4 geometry. There are zero additional active source files and zero
unclassified active files.

The following are evaluator-only and deliberately excluded from the active
surface:

- `arms/px4-lrc-lifetime/src/main.rs` schedules preregistered anonymous
  arrivals, reads public physical observations, applies verdict predicates and
  serializes artifacts. It owns no organism state and implements no state
  transition;
- `arms/px4-lrc-lifetime/tests/physics.rs` supplies two isolated preflight
  assertions and owns no runtime mechanism;
- `arms/px4-lrc-lifetime/Cargo.toml` declares only the direct path dependency
  and is not executable source.

A conservative lexical scan over the complete active PX4 source found zero
headline seams, zero semantic-condition guard matches and zero
evaluator-input guard matches. The runner and tests are not hidden active
adapters: all their mutations enter through public `PlasticSubstrate::enter`,
`propagate` and `advance_time`, whose source is independently manifested.

## E2B preflight

Fresh sandbox `ix50ch0df4g0bfyvfb6ce` received clean commit `7c39b45` and
passed, in order:

```text
cargo fmt --manifest-path arms/px4-lrc-lifetime/Cargo.toml -- --check
cargo build --release --manifest-path arms/px4-lrc-lifetime/Cargo.toml
cargo test --release --manifest-path arms/px4-lrc-lifetime/Cargo.toml
cargo clippy --release --all-targets \
  --manifest-path arms/px4-lrc-lifetime/Cargo.toml -- -D warnings
retained-law SHA-256 check
active-source forbidden/guard vocabulary scan
```

All two focused tests passed. No PROBE, MICRO or GATE command ran in preflight,
and none of their output artifacts existed in the clean snapshot. The sandbox
was left running by the established launcher. Separate fresh sandboxes and
unique state files are required for every registered execution.

## Scientific disposition

No genuine fork was required. The implementation adds no lifetime field or
new physical update. It measures the existing LR-C resistance/pressure path
under recurrence, reuse, qualified modulation, changed participation and
disuse. Development authority remains absent regardless of the forthcoming
functional result.
