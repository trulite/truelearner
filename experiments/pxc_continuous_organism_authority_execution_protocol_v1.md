# PX-C continuous-organism authority execution protocol v1

Status: **PREREGISTERED; AUTHORITY EVIDENCE UNSPENT**.

Authority parent is the positive development-readiness commit
`433785a9dd81ff0fdfd2393e6e123e55ae12e245` / tag
`pxc-continuous-organism-development-ready-v1`. The production runtime,
evaluator, topology, schedules, predicates, work/memory bounds, active spec,
and all static gates are immutable at their frozen hashes.

One fresh E2B worker must first run
`scripts/audit_pxc_authority_firewall_v1.py`. The firewall binds the positive
development CSV/report, runtime/evaluator, one-page spec, exhaustive active
gate, harness gate, zero taxonomy, accepted PXR0 parent, and absence of all
authority outputs. Its JSON must be generated before the evaluator begins.
The firewall must be exact-replayed without changing any input.

If and only if the firewall passes, the same fresh worker may execute exactly
one release invocation:

```text
cargo run --manifest-path arms/pxc-continuous-organism/Cargo.toml --release -- --authority
```

This spends roots `3_200_001..3_200_016`, balanced reverse/reflection
quadrants, and phase-preserving origins `520,650,780,910` once per quadrant.
They are disjoint from development roots. The evaluator must serialize every
row and clause before assertions and must pass all `524/524` clauses without a
rescue run.

After that sole invocation, the already-frozen portable result audit may parse
the generated CSV/report once, verify exact development/authority observation
agreement under the registered transformations, and exact-replay its own
audit artifacts. It must not invoke Rust or regenerate either matrix.

Any firewall, row, global, replay, quiescence, work, memory, boundary,
one-page, dependency, vocabulary, seam, guard, new-kind, or new-surface failure
freezes a negative. No post-evidence change is permitted. A positive result
establishes PX-C continuous-organism authority only; it does not authorize an
`arch.md` edit or any future runtime feature.
