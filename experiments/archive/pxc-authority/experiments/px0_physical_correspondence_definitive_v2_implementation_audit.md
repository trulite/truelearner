# PX0 definitive v2 implementation audit

Status: **IMPLEMENTATION FROZEN; DEFINITIVE V2 EVIDENCE UNSPENT; PX0 AUTHORITY ABSENT**.

## Frozen source

- protocol commit: `8c41a68`
- protocol tag: `px0-physical-correspondence-definitive-v2-protocol`
- protocol SHA-256:
  `899819b4605f811211916da447c211604dfd7757b9f7b94aa87084c2ed0e534d`
- authority executable:
  `crates/px0-physical-correspondence/examples/definitive.rs`
- authority executable SHA-256:
  `48b66f62b6127a8ec760a2f4678d0b6f9f5e677fcad956bd457529d231fd52f4`
- active-law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`
- retained-physics SHA-256:
  `6aa28a76e1362ac8dfb1d33fb68807da40e7604dfdc8cca9efa1e314e3ce4263`

The implementation changes only the authority/evaluator example. Neither the
active PX0 law nor retained physics changed.

## Matrix implementation

The executable constructs exactly sixteen fresh blank cells at namespace bases
`0x12000000 + i * 0x100000`. Each cell executes all thirteen preregistered
claims continuously or in explicitly independent namespace-disjoint control
fixtures.

The primary fixture performs:

```text
acquire A
-> held-out execution
-> bounded-absence reuse of identical arrows
-> full pressure/deallocation
-> stale-path attempt and fresh reproposal
-> 8 × 4 interleaved dense contexts
-> 3-context no-incidental tail
-> held-out contemporary B and sparse A
-> sparse-A final deallocation
```

Four distinct support devices have physical delays `1,2,3,4`. Dense return is
created only by ordinary local proposal from a physically active driver two
position units from a gate. The wrapper never inserts or mutates a learned
candidate arrow.

Independent fixtures implement return-free probation, recurrent dense return,
stability swap, absence, equal ambiguity, and exact duplicate replay.

The earlier v1 evaluator body remains source-visible only behind
`#[cfg(any())]`; it is statically absent from compilation and has no call path.
The active `run_cell` is the v2 implementation described here. Historical v1
result/source identity remains protected by its immutable tags and artifact
hashes.

## Pre-evidence checks

The frozen tree passed:

- `cargo fmt --all -- --check`;
- strict focused Clippy with `-D warnings`;
- focused crate test `1/1`;
- release compilation;
- no-cell `--preflight-v2`;
- refusal without `--definitive-v2` with exit `2`;
- active-law and retained-physics hashes;
- v1 negative, PX0-P1, and PX0-S lineage hashes;
- zero normal dependencies and active-source semantic isolation;
- absence of both v2 final and staging result paths.

Preflight ran no cell, emitted no evidence marker, and created no result.

## Sole authority surface

Only this command may spend evidence:

```text
cargo run --release -p px0-physical-correspondence --example definitive -- --definitive-v2
```

It repeats preflight, emits one `PX0_V2_DEFINITIVE_EVIDENCE_SPENT` marker,
executes literal cells `0..15`, and publishes complete CSV and Markdown
artifacts atomically. It cannot overwrite an existing staging or final path.

No definitive command, cell, marker, or output has yet occurred.
