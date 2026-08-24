# PX4 LR-C physical lifetime authority implementation audit v1

Status: **FROZEN; E2B PREFLIGHT PASSED; DEFINITIVE EVIDENCE UNSPENT**.

## Frozen lineage and surfaces

- PX3+LR-C authority parent:
  `f9057fe78a86db9111b0b69310d03accef3bc970`;
- PX4 development-ready parent:
  `20bc9ce384b74b6e5cca04f4bed2599932a34e92`;
- protocol commit/tag:
  `12e6e451e98b9b120732ada8ab3bb079fa27ad4c` /
  `px4-lrc-lifetime-authority-protocol-v1`;
- exact successful preflight source commit:
  `7e305fa5ed5cbd6ca439bfc9aa976b0d148c63b5`;
- successful fresh E2B sandbox: `i62kdw2rt1g37qdqyj5en`;
- unique state file:
  `px4-lrc-authority-preflight-20260824-v4.json`.

| frozen artifact | SHA-256 |
|---|---|
| authority protocol | `fa04de4ec43c10f3878b86d920c2a67243b84201e8759950075c069548153ba8` |
| evaluator source | `e696c8e1e50ac9504c180094daf90182d0854755a2b6289826f8de19397bfc5d` |
| fresh evaluator wrapper | `a181fa810cef8edfe557daaf8dae9948ebd37dd429bb084d8ffedb6d84615b4c` |
| static audit script | `3f5dc56374062a07f1513bddebb4ea013d3f2a741f89ea40b8a49fedfd531bf3` |
| active PX4 mechanism | `a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71` |
| retained LR-C law | `7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10` |
| active-surface manifest | `28924746e951645047225d8d20f5c5f98d93f349f46f7c6d7019e68632ce51b9` |

The active PX4 mechanism and retained LR-C law are byte-identical to
development readiness. The authority changes are evaluator-only: a new binary
name and protocol namespace, fresh identities, two translated absolute-time
schedules, fixed non-adaptive 24-step pressure observations, explicit LR-C
conformance serialization, and row/clause accounting. No substrate field,
transition, mode, threshold, coupling, resistance rule, pressure rule,
eligibility rule or proposal rule changed.

## E2B preflight

Fresh sandbox `i62kdw2rt1g37qdqyj5en` ran the clean
`7e305fa5ed5cbd6ca439bfc9aa976b0d148c63b5` archive and passed:

```text
cargo fmt --manifest-path arms/px4-lrc-lifetime/Cargo.toml -- --check
cargo build --release --manifest-path arms/px4-lrc-lifetime/Cargo.toml \
  --bin px4_lrc_lifetime_authority_v1
cargo test --release --manifest-path arms/px4-lrc-lifetime/Cargo.toml
cargo clippy --release --manifest-path arms/px4-lrc-lifetime/Cargo.toml \
  --all-targets -- -D warnings
PX4_AUDITED_COMMIT=7e305fa5ed5cbd6ca439bfc9aa976b0d148c63b5 \
  ./scripts/audit_px4_lrc_authority_v1.sh
cargo run --release --manifest-path arms/px4-lrc-lifetime/Cargo.toml \
  --bin px4_lrc_lifetime_authority_v1 -- --authority-preflight
```

Results were format/build/Clippy pass, `2/2` release tests, static lineage/law/
active-source/leakage/dependency/coverage pass, foundation zero, `16` unique
identities, `16` unique strata, schedule origins `200|400`, zero worlds and
authority artifacts absent. The sandbox remains running.

Three earlier fresh preflight sandboxes stopped before world construction and
are retained as technical preflight negatives:

| sandbox | state suffix | frozen stop |
|---|---|---|
| `inn79xz46vtdxp5lxcniu` | `v1` | E2B rustfmt reported mechanical evaluator formatting |
| `i8ok7ly7gepwnmc18f11y` | `v2` | build/tests/Clippy passed; archive had no `.git` for the audit |
| `imzlhehafh6aukf14j5zt` | `v3` | build/tests/Clippy passed; E2B image lacked `rg` |

None invoked `--authority-v1`, created a substrate world, emitted an evidence
marker or wrote an authority artifact. Corrections were limited to the exact
E2B formatting diff and audit portability; the protocol, mechanism, schedules,
identities and verdict predicates did not change.

## Leakage and coverage result

The active mechanism contains zero lifetime/history/episode/reset/cleanup/
delete tokens and exposes only ordinary `Field`, `Fork` and anonymous arrival
construction over the authoritative LR-C substrate. The evaluator declares no
forbidden semantic object or member access and invokes no resistance,
eligibility, generation, cleanup or deletion setter. The pressure scan is
fixed before observation and feeds no observed lifetime quantity back into an
organism input.

All candidate source files are classified:

- `src/lib.rs`: sole active PX4 mechanism, manifested;
- `src/main.rs`: evaluator-only scheduler/observer/serializer;
- `src/bin/px4_lrc_lifetime_authority_v1.rs`: evaluator-name wrapper only;
- `tests/physics.rs`: preflight assertions only.

Unclassified active sources are zero and authoritative-foundation seams remain
zero. No scientific fork or new law was required.

The sole definitive command may now execute once from this unchanged source
snapshot plus this audit-only commit. Any definitive failure is immutable.
