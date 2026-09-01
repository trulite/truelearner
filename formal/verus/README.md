# Executable Rust proofs

TrueLearner uses Verus for narrow invariants over the Rust that actually runs.
Lean remains the observer-side checker for larger compositional claims over
frozen traces.

The first proof slice is in `truelearner-core`. It verifies that:

- only an `Open` return can become `Closed`, `Ambiguous`, or `Expired`;
- every terminal return is absorbing;
- `Ambiguous` and `Expired` transitions retain no closed support; and
- a rejected transition changes neither status nor activity.

It does **not** yet prove that the body supplied unique physical closure
support. That premise is still established by Rust resolution and falsifying
tests. A later Verus slice may move that resolver invariant into the executable
proof boundary.

## Reproduce

The checked toolchain is Verus `0.2026.08.30.b432e82`. The matching `vstd`
crate is pinned in `truelearner-core/Cargo.toml`.

From the repository root, with that Verus release's `cargo-verus` on `PATH`:

```sh
cargo verus verify --manifest-path truelearner/Cargo.toml -p truelearner-core
```

The expected result is `9 verified, 0 errors`. No `assume`, trusted function
body, external specification of learner code, or duplicate transition model is
used in this slice.
