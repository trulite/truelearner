# PX5 LR-C cumulative allocation authority implementation audit v1

Status: **FROZEN; TARGETED E2B PREFLIGHT PASSED; DEFINITIVE EVIDENCE UNSPENT**.

## Frozen serial lineage

- PX4 authority parent:
  `2348f4318e4c4ca85d6be06017e8ccd7be8b9c87` /
  `px4-lrc-lifetime-authority-v1`;
- authority protocol:
  `e553f0f73f806ab859a3651e35d8e127ac736f17` /
  `px5-lrc-allocation-authority-protocol-v1`;
- exact successful preflight source commit:
  `21a93e2e38623559e487cf529eb9a1e4a8c4378b`;
- formatting-only E2B sandbox: `irv8t6cv96f5ytwi5cw3q`;
- targeted preflight E2B sandbox: `isrd29atat4dpi6lj18fd`.

The protocol commit's Git parent is the exact PX4 authority commit. The
isolated PX5 development branch and tags remain unchanged and were not
cherry-picked. No unrelated PX4 development source was imported.

## Frozen hashes

| artifact | SHA-256 |
|---|---|
| authority evaluator | `d44c806ac7ecc61ed3b561d210f4d542d9189537f93b33f5f16114ee060b11e3` |
| evaluator Cargo manifest | `4d6272b609c0f151d9227a0a54fabf6179b8c3154f6f1cfb8d655bca1f91600c` |
| static audit | `c47ea65c9d3839417733062068e9ca33a020d1e59b66d5894d90009b33c089c1` |
| active manifest v3 | `32fda2d86f6c836438fdd01fc5433c8731f4ec92e3e052e5d1a64fe751d15388` |
| authority protocol | `497c559f9477252195e870d2b4be8dfd38f09b163438ecce7047e2f63077c443` |
| coverage audit | `4161684ac9eb5b38ad89c5009adf328c4fd60493155752ab3bc92a1ff1aa3876` |
| retained LR-C law | `7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10` |
| retained PX4 API | `a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71` |

## Implementation boundary

PX5 adds no active mechanism source. The authority arm is one evaluator-only
binary with two ordinary direct dependencies: the authoritative PX4 API and
the authoritative LR-C crate. It uses PX4 `arrive` for every anonymous arrival
and LR-C public physical types for the frozen loaded geometry.

The implementation is explicit Rust: ordinary structs, arrays, vectors,
direct loops and public read-only measurements. It contains no unsafe code,
interior mutability, global/thread-local state, proc macro, generated include,
semantic allocator/admission adapter, artificial lifetime/leak workaround or
feedback from measured results into later organism input. Serialization occurs
only after the full matrix verdict and does not participate in organism
behavior.

The 24 rows, eight roots, three loads, order/reflection strata, four reuse
ticks, work ceiling `100000`, byte ceiling `24000`, and every predicate are
fixed in source and protocol. Repeated-reuse memory equality counts existing
parent-law tombstones rather than hiding them.

## Batched E2B validation

No Rust tool ran locally. Fresh formatting sandbox `irv8t6cv96f5ytwi5cw3q`,
state file `px5-lrc-authority-format-v1.json`, ran only `cargo fmt` and returned
the canonical source. It did not compile or construct a world.

After all source, audit and manifest edits were batched and committed, fresh
sandbox `isrd29atat4dpi6lj18fd`, state file
`px5-lrc-authority-targeted-preflight-v1.json`, ran exactly the targeted frozen
gate:

```text
cargo fmt --check                                      PASS
cargo build --release                                  PASS
one no-world matrix-definition test                    1/1 PASS
cargo clippy --release --all-targets -- -D warnings    PASS
static hash/leakage/dependency/coverage audit           PASS
release --authority-preflight                          PASS
```

The static audit reported `active_sources=2`, `evaluator_sources=1`, and
`unclassified=0`. Preflight checked eight unique roots, loads `8/32/128`, 24
cells, frozen input hashes and absence of both final result paths. It did not
call `run_row`, construct a `PlasticSubstrate`, perform exact replay, emit the
evidence marker or create an artifact.

No workspace-wide build, unrelated test suite or repeated compilation was
run. There were no failed compilation/preflight attempts.

## Definitive eligibility

The sole definitive command may now execute exactly once from this unchanged
source plus this audit-only commit in a new E2B sandbox. Any functional failure,
work/memory/quiescence failure, interruption or publication failure is an
immutable negative. No implementation change, rescue or rerun is authorized.

No scientific or architectural fork was encountered. PX6 remains forbidden.
