# PX3 participation-trace organization PROBE implementation audit v1

Status: **IMPLEMENTATION FROZEN; E2B PREFLIGHT PASSED; PROBE EVIDENCE UNSPENT**.

## Frozen implementation

- branch: `research/px3-participation-trace-organization`;
- implementation source commit: `f91bfd6a25faae987f132bb7dd086fe72e3b32b9`;
- package manifest SHA-256:
  `14aa2b3ed617f2f7f3daacf848063ac4acf74525bfccc28315d6524b9ac0b847`;
- executable source SHA-256:
  `9cb100f967df668cf54577c605d970f6f4a7fac8d61e230c0a696fc5c4887d1d`;
- cumulative development protocol SHA-256:
  `61ce9535ccab0133db660f8fc4d1e408bc61c2f9d8948877f34a431ec063c9c7`;
- unexecuted PROBE protocol v1 SHA-256:
  `ca95a80eef60b13f5e1b975533723de46e591bc96621965beaa1b903501f1de7`;
- corrective PROBE protocol v2 SHA-256:
  `a7ce66c9ffc1fe20dc85334d8c7622855cddfd1971f5ce7785d50bf2acd410cb`.

Protocol v2 corrects only the native generic-candidate schedule: PX0 assigns a
distance-two proposal delay `2`, coupling `1` and resistance `1`. It moves the
first real overlap early enough to refresh eligibility before expiry. No world
was constructed and no evidence was observed before this correction was
preregistered.

The executable contains exactly two accepted flags:

- `--preflight`: source/matrix/forbidden-surface/destination audits only; it
  constructs no `PlasticSubstrate`, calls no `propagate`, and writes nothing;
- `--probe`: the sole frozen write-once 24-row evidence command.

There is no MICRO, GATE, definitive or authority command, module or artifact
path. The six unordered opportunities are created uniformly before training;
all candidate routes arise through unchanged PX0 generic local proposal. Raw
couplings are confined to source-to-outlet ARROWs, while every incident
opportunity receives unit input only from an authoritative PX1 trace firing.

PX0 exposes native candidate liveness and resistance but no coupling getter.
The result schema therefore pairs native candidate crossing impulses with the
law-exact post-return coupling value: a selected eligible resistance-one
candidate receiving PX0's local return becomes resistance four and coupling
two; ordinary pressure does not change coupling. This is serialization only and
does not affect routing, selection or substrate state.

## E2B preflight

The established persistent E2B sandbox `i6x9gykt9tvp6xfz5z8ra` ran the clean
Git snapshot for the implementation commit. No Rust compilation, test or
runtime occurred on the host.

The following chain passed:

```text
cargo fmt --manifest-path arms/px3-participation-trace-organization/Cargo.toml -- --check
cargo check --manifest-path arms/px3-participation-trace-organization/Cargo.toml --release
cargo test --manifest-path arms/px3-participation-trace-organization/Cargo.toml --release
cargo clippy --manifest-path arms/px3-participation-trace-organization/Cargo.toml --release -- -D warnings
cargo run --manifest-path arms/px3-participation-trace-organization/Cargo.toml --release -- --preflight
```

Observed preflight facts:

- static tests: `3 passed; 0 failed`;
- strict Clippy: clean;
- marker: `PX3_PARTICIPATION_TRACE_ORGANIZATION_PROBE_PREFLIGHT_OK`;
- all authoritative and protocol hashes: exact;
- CSV, Markdown and both staging destinations: absent;
- MICRO and GATE surfaces: absent;
- `PX3_PARTICIPATION_TRACE_ORGANIZATION_PROBE_EVIDENCE`: not emitted;
- `--probe`: not invoked;
- PROBE rows observed: `0`.

This audit freezes implementation only. It makes no PX3 result or authority
claim. The next scientific action, if explicitly authorized, is one execution
of the frozen `--probe` command in E2B followed by artifact download and result
audit.
