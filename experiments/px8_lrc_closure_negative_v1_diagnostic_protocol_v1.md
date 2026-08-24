# PX8 LR-C serial closure negative-v1 diagnostic protocol v1

Status: **PREREGISTERED; DIAGNOSTIC EVIDENCE UNSPENT; NOT AUTHORITY**.

## Frozen parent and purpose

This protocol must be the first child of immutable negative commit
`eca6245475bd680f1876822efc1230aea400a968`, tagged
`px8-lrc-closure-authority-negative-v1`. It accepts PX8 authority v1 as spent
and negative. It may identify the exact failed serial root and predicate only;
it may not repair, reinterpret, rerun, or claim v1 authority.

Frozen inputs are:

| artifact | SHA-256 |
|---|---|
| active PX8 mechanism | `8623cae7dc1b14e666140a192c49e59ae72df681bc14f0359f0c4465e0d11e8f` |
| retained LR-C law | `7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10` |
| retained PX4 API | `a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71` |
| retained PX7 source | `d248a8af479872d8148115a405ae7332f7d24ca229378d3fde898ffd3d19e63e` |
| frozen authority-v1 evaluator | `ccbf3547ae0534ccbbb0c00e8d058f47f9471afb4a30733cc124e981a0f606d0` |
| negative diagnostic | `3c8df23536157cc91d315c96862d408027aabdb28c65bab133696405132b3116` |

No active PX8 file or retained PX0--PX7+LR-C file may change. Only a new
evaluator-only diagnostic package, its static audit, and diagnostic artifacts
may be added.

## Fresh diagnostic identities

The diagnostic roots are exactly `862001..862016`. Every primary namespace is
`root << 32`; compact topology namespaces are primary plus
`10000,20000,...,60000`; cumulative PX7 namespaces are
`(root + 1_000_000) << 32`. They are disjoint from authority-v1 roots
`861001..861016`, isolated PX8 identities, and all retained serial matrices.

The sixteen layouts cross every construction/reflection pair exactly four
times and every twist in `0,137,274,411` exactly four times. Physical schedules
remain exactly those frozen in v1:

```text
formation        learn_twice
complete         reuse all four at 61
incomplete       omit side four at 70
blocked          outward resistance 0, reuse at 61
stale            learn_once_then_age, reuse at 111
compact          direct/open/fork/ring at 0; aged at 10
PX7 cumulative   maturation at 0 and 10; held-out boundary at 20
```

No measured diagnostic value may influence a later schedule or physical
input.

## Complete diagnostic serialization

The new package `arms/px8-lrc-closure-diagnostic` depends directly and only on
active PX8 and retained PX7. One execution reconstructs every root twice and
must publish all `16 * 14 = 224` clause records even when any clause fails.

Every clause record must contain:

- root, namespace, reverse, reflect, twist, and the full fixed schedule label;
- clause index/name, exact expected predicate, exact actual observation, and
  pass/fail;
- all formation/completed/negative/compact crossing, physical-return, and
  qualified-update counts;
- formation and per-batch work, maximum work, persistent bytes, memory
  stability, pause equality, resume equality, and natural quiescence/queue
  exhaustion;
- cumulative PX7 training modulation/update counts, mature coupling and
  resistance, held-out crossing, work, bytes, and quiescence;
- exact duplicate-state replay result;
- the first failed clause for that root; and
- the first field divergent between independently reconstructed states, or
  `none` when replay is exact.

A companion report must summarize every failing root/clause and preserve exact
expected/actual text. It must state diagnostic completeness independently of
whether physical predicates pass.

## Diagnostic firewall

The evaluator must not contain or emit
`PX8_LRC_CLOSURE_AUTHORITY_V1_EVIDENCE_SPENT`, accept `--authority-v1`, or
write either v1 authority result path. It may accept only `--diagnostic-v1` and
emit one distinct `PX8_LRC_CLOSURE_NEGATIVE_V1_DIAGNOSTIC_SPENT` marker.

It publishes create-new artifacts only after all rows and clause records have
been evaluated:

```text
results/px8_lrc_closure_negative_v1_diagnostic.csv
results/px8_lrc_closure_negative_v1_diagnostic.md
```

Diagnostic success means complete, internally consistent serialization. It is
not `16/16`, `230/230`, PX8 promotion, or authority.

## Execution economy

Instrumentation, Cargo, hashes, and the static firewall audit must be batched.
No Rust, project program, or project audit may run locally.

If compilation assurance is needed, one fresh E2B sandbox may run only:

```text
package rustfmt check
package cargo check
no-world static hash/dependency/identity/firewall audit
```

It may not construct a body or create a diagnostic artifact. One second fresh
E2B sandbox then executes exactly once:

```text
cargo run --release \
  --manifest-path arms/px8-lrc-closure-diagnostic/Cargo.toml \
  -- --diagnostic-v1
```

No workspace-wide build, full test suite, Clippy, authority execution, or
diagnostic rerun is registered.

## Classification and successor rule

The diagnostic must freeze exactly one evidence-based classification:

1. **measurement/evaluator/fixture defect**: active physical observations
   satisfy the intended claim, but a registered measurement, predicate,
   identity cross-product, or fixture is wrong;
2. **physical/mechanism counterexample**: the active mechanism violates a
   registered physical claim for at least one fresh layout; or
3. **new-law fork**: satisfying the claim requires changing retained substrate
   behavior or adding organism law.

For classification 1, freeze the defect and then preregister a completely
disjoint authority-v2 protocol whose only change is the evidenced measurement
repair. Authority v2 must remain unspent until a later implementation/audit
freeze.

For classification 2 or 3, stop before modifying the active mechanism. Record
the scientific fork and do not prepare authority v2.

This workflow may not create manifest v6, run PX-C taxonomy/comparison, claim
PX8 promotion, or claim final PX-C continuous-organism authority.
