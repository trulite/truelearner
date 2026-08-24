# PX8 LR-C cumulative closure authority v2 negative v1

Status: **IMMUTABLE AUTHORITY-V2 NEGATIVE; PROMOTION STOPPED; NO AUTHORITY CLAIM**.

## Spent execution

The sole authority-v2 command ran once from frozen commit
`c34cdd7ab8039a5bf7819a7dbb3c4db9df04793f`, tagged
`px8-lrc-closure-authority-v2-frozen-v1`, in fresh E2B sandbox
`i163i0qobnyhxbpzf8wl4`, state file
`px8-lrc-authority-v2-definitive.json`:

```text
cargo run --release \
  --manifest-path arms/px8-lrc-closure-authority-v2/Cargo.toml \
  -- --authority-v2
```

It emitted the registered marker exactly once:

```text
PX8_LRC_CLOSURE_AUTHORITY_V2_EVIDENCE_SPENT
```

It then stopped before publication:

```text
thread 'main' panicked at src/main.rs:243:5:
authority-v2 row failed
```

Neither v2 result artifact was created or downloaded. The chained result audit
did not execute. No rerun, partial root, instrumentation rescue, alternate
identity, or fabricated result occurred.

## Preserved diagnostic classification

The accepted negative-v1 diagnostic remains frozen at
`eadc3edad648c19346f3bb7217cebdce77d97579`, tagged
`px8-lrc-closure-negative-v1-diagnostic-result-v1`:

```text
classification: measurement/evaluator/fixture defect
roots:          16/16 serialized
clauses:        224/224 serialized
v1 failures:    clause 12 only, 16/16 roots
maximum work:   14788 / 20000
maximum bytes:  5488 / 8192
replay/quiet:   16/16 / 16/16
```

That classification is not invalidated or reinterpreted by the separate v2
negative.

## V2 diagnostic boundary

The v2 evaluator and protocol are frozen at hashes:

| artifact | SHA-256 |
|---|---|
| active PX8 mechanism | `8623cae7dc1b14e666140a192c49e59ae72df681bc14f0359f0c4465e0d11e8f` |
| v2 evaluator | `e1a830e15c898b113f295d74e22f6dee1d144bd43ee1aa177d4a7c0ef075043c` |
| v2 protocol | `a47866460ecc4504ee713e0b425d049e7816f48e4aa18bceeb0a1705dcbc5328` |

V2 changed evaluator observation only and left the active mechanism and laws
unchanged. Its publication gate asserts all rows before serialization, so the
spent execution does not identify the failing root or clause. Recovering that
information would require a separately preregistered v2 diagnostic workflow;
it cannot be obtained by rerunning or editing authority v2.

Source inspection notes that v2 preregistered same-body byte equality for all
seven fixtures, including the deliberately aged/stale body. Retained LR-C may
physically append new structural proposals when a stale body is reused even
while bounded memory, silence, and quiescence hold. This is a plausible
evaluator-fixture explanation, not executed v2 evidence, and is therefore not
claimed as the exact v2 failure.

## Promotion consequence

Authority v2 did not establish `16/16` rows or `230/230` clauses. Manifest v6,
PX-C taxonomy, comparator, result audit, and authority handoff were not run or
created. Immutable PX7 manifest v5 remains the active serial baseline.

No new mechanism or substrate law was added. PX8 authority and final PX-C
continuous-organism authority remain unclaimed.
