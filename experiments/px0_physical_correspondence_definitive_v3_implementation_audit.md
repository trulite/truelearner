# PX0 definitive v3 implementation audit

Status: **IMPLEMENTED AND VALIDATED; DEFINITIVE EVIDENCE UNSPENT**.

## Frozen protocol

- protocol commit: `dfe30d4a3273a3947ce8d3e23075d0ec04ee11cf`
- protocol tag: `px0-physical-correspondence-definitive-v3-protocol`
- protocol SHA-256: `084275c5f3fd9694942098b668618841e5c035ff9599dca48c478c6f8de20974`
- pre-existing results-tree digest: `c84d00e8dbcea86add2537d3be2c00f241c5c5db5e91176d7b04800fd2a8ceb7`

## Frozen implementation surface

- authority wrapper:
  `crates/px0-physical-correspondence/examples/definitive.rs`
- wrapper SHA-256:
  `cba69204ea9c0fcf72a2ec6777f11d0110bacc61ef099c58597748510a5f68b0`
- active PX0 law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`
- retained physics SHA-256:
  `6aa28a76e1362ac8dfb1d33fb68807da40e7604dfdc8cca9efa1e314e3ce4263`

Only the authority wrapper changed. The active correspondence law and retained
CELL/ARROW/SPIKE physics remain byte-identical.

## Measurement audit

The wrapper serializes each authority clause independently as `P0..P23` and
also emits the underlying observables. In particular, the former P6 bundle is
split into:

- stable opportunities and completed returns;
- sparse opportunities and completed returns;
- first executable B context and executable-context count;
- stable and sparse final resistance;
- stable and sparse cloned no-use deallocation delay;
- stable and sparse held-out effects;
- sparse eventual deallocation.

The offered stable schedule remains exactly 35 contexts. Completed stable
returns are not required to equal opportunities. They must be nonzero and
strictly exceed completed sparse returns; maturation, durability, lifetime,
and behavior are tested by separate claims.

## Fresh matrix audit

Exactly 24 cells use namespace `0x16000000 + i * 0x100000`. Static inspection
confirmed coverage of all three route rotations, both allocation directions,
both layouts, spacings `10..=19`, all listed stride and distractor-load values,
and all four incidental phases. Subfixtures remain within each cell's namespace
stride. Repository search found no prior use of the v3 namespace base or v3
evidence marker outside the preregistered protocol and wrapper.

## Pre-evidence validation

The following completed without executing a cell or emitting an evidence-spend
marker:

- `cargo fmt --all -- --check`;
- `cargo check -p px0-physical-correspondence --example definitive`;
- `cargo test -p px0-physical-correspondence` (`1/1` focused test passed);
- `cargo clippy -p px0-physical-correspondence --example definitive -- -D warnings`;
- `cargo run --quiet -p px0-physical-correspondence --example definitive -- --preflight-v3`;
- invocation without the v3 authority flag refused with exit `2`;
- source-token, dependency, namespace, frozen-hash, result-digest, formatting,
  staging-absence, and output-absence audits.

The no-cell preflight returned `true`. Final and staging v3 result paths remain
absent. Definitive execution remains authorized only through the sole command
frozen in the protocol.
