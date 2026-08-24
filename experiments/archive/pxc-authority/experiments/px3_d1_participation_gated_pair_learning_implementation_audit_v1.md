# PX3-D1 participation-gated pair learning implementation audit v1

Status: **IMPLEMENTATION FROZEN; E2B PREFLIGHT PASSED; D1/D1-R EVIDENCE UNSPENT**.

## Frozen implementation

- implementation source commit:
  `d1a39994eb92a894cca9bcfaf896a4141cc46633`;
- package manifest SHA-256:
  `23c6c735a3ae154815f591e4f311a412a6c9087f78d2176e4920fb452220e9ff`;
- executable source SHA-256:
  `744f3052c38c102a930ca1c58175b0d89230786c3b0f4d97d4214ac707919808`;
- scientific protocol SHA-256:
  `8da3c6ca5b5b548233662eacd20d6263f25c35411a48e392f4bd30403c08785f`;
- execution protocol SHA-256:
  `2258cd30e8fcf04fb7ac9942f1beffeeb4110243623eeb88cb26cda02c5a78bf`.

The implementation adds six symmetric ordinary opportunity->consequence
ARROWs at resistance `1`, coupling `1`, delay `2`. They are added without
firing their source CELLs and therefore begin with no eligibility. There is no
opportunity priming and native generic proposal count is required to remain
zero.

The executable contains exactly `--preflight` and the sole write-once `--d1`
evidence command. Its 26 rows partition into 24 independently scored D1 core
rows and two independently scored D1-R provenance rows. It never infers
coupling from resistance; only native candidate crossing impulses are recorded.

No D2, MICRO, GATE, candidate-formation/reproposal, definitive or authority
surface exists.

## E2B preflight

Persistent sandbox: `i6x9gykt9tvp6xfz5z8ra`.

The clean implementation snapshot passed:

```text
cargo fmt --manifest-path arms/px3-d1-participation-gated-pair-learning/Cargo.toml -- --check
cargo check --manifest-path arms/px3-d1-participation-gated-pair-learning/Cargo.toml --release
cargo test --manifest-path arms/px3-d1-participation-gated-pair-learning/Cargo.toml --release
cargo clippy --manifest-path arms/px3-d1-participation-gated-pair-learning/Cargo.toml --release -- -D warnings
cargo run --manifest-path arms/px3-d1-participation-gated-pair-learning/Cargo.toml --release -- --preflight
```

Observed:

- static tests: `3 passed; 0 failed`;
- strict Clippy: clean;
- preflight marker:
  `PX3_D1_PARTICIPATION_GATED_PAIR_LEARNING_PREFLIGHT_OK`;
- frozen hashes: exact;
- result and staging paths: absent;
- evidence marker: not emitted;
- substrate worlds propagated by tests/preflight: `0`;
- D1/D1-R rows observed: `0`.

The next authorized scientific action is one execution of the exact frozen
`--d1` command in E2B. Any physical or predicate failure must publish and freeze
without implementation correction or rerun.
