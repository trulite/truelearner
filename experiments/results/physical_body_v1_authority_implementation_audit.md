# Physical Body V1 authority implementation audit

Outcome: **EVIDENCE ELIGIBLE — AUTHORITY NOT YET EXECUTED**.

## Frozen lineage

- Protocol commit: `0953fc618fa2dcf8464bd14f80f4d7ea6c97e36f`.
- Protocol parent: `97d74d5341bbba778ceea23ea6e4c0f4e16fbd11`.
- PX-C authority ancestor: `ec87c438aa8c52389fd2734667363ef43acaef93`.
- Protocol tag: `physical-body-v1-authority-protocol-v1`.

The protocol commit is a direct child of the exact reviewed production tip.
There is no production delta from `97d74d5` through this audit.

## Frozen production hashes

| Artifact | SHA-256 |
|---|---|
| `truelearner/crates/core/src/lib.rs` | `e6767845f27ddb9bb57bfb1fcab6dd1663178449faddc4a630b628e3d1148a8d` |
| `truelearner/crates/arena-format/src/lib.rs` | `8c35c3c07fe95b2cc76cbe9ceb47d83f250c5e0c7c40481e7371583afa48a812` |
| `truelearner/Cargo.lock` | `592e90c54d28b6cb6cfdb970db3120ffe4c97c50adb3720627ff7f6c34f4900d` |

## Frozen verifier hashes

| Artifact | SHA-256 |
|---|---|
| verifier `Cargo.toml` | `a4fb3a3cb2db75796439e401877381562ebbd7dfd9076523ad5b3e4b171475bc` |
| verifier `Cargo.lock` | `4872af06aaf99b4cc54a9c6f363701a0cf588321b0b0291c4da4fa42b6b53242` |
| verifier `src/main.rs` | `2216d171756c44219b0949c265d037a9564c7f284833c1423851293f0f04613c` |
| verifier `src/body.rs` | `4a3cfc69eced9097f73bfb4efe973cded40a2d18298e74eed1c733bd87cf706a` |
| static audit | `9b23a5b0431b993e65077fab003180ccac0da5eb20cda6f755190394a438701e` |
| E2B launcher | `d93e6e14ec9e5da2b4c9771cb585018ed2ba9dfd3933ca133cee619a5b7abaf2` |

## Authority surface

The verifier contains exactly:

- the retained cumulative PX-C geometry;
- sixteen fresh roots `4_100_001..4_100_016`;
- four phase-preserving origins `1_040, 1_170, 1_300, 1_430`;
- both allocation orders and both reflected layouts;
- the unchanged 32 row clauses;
- the unchanged twelve cumulative globals, with relocated immutable lineage
  paths;
- sixteen independently serialized Physical Body V1 clauses;
- a non-executing `--preflight` mode;
- one authority-only evidence marker and fresh result paths.

The authority path writes CSV first and Markdown second through create-new
staging files and atomic rename. Assertions occur only after both artifacts
contain all rows and all clause vectors. A negative is therefore diagnosable
without rerunning the matrix.

## E2B validation

Formatting-only sandbox `itlxq0vec6heq7sebpjip` formatted only the new
verifier sources. Metadata-only sandbox `ior6mt0cy2c2ramb1hdmq` generated the
verifier lockfile. Neither compiled or executed an organism world.

Targeted sandbox `ilh5udfs2drfk7h690p2w`, commit `04179f3`, established:

- production workspace formatting: PASS;
- verifier formatting: PASS;
- production strict Clippy `-D warnings`: PASS;
- verifier strict Clippy `-D warnings`: PASS;
- non-executing authority registry preflight: PASS.

That sandbox exposed an audit portability defect: `rg` was absent, so its
static-search result was rejected even though the shell command returned zero.
No authority world was constructed. The audit was changed only from `rg` to
portable `grep`.

Corrected static-only sandbox `iptflmw8cwslm9h3iu4g6`, commit `2bb0237`, then
established:

- all frozen production hashes: PASS;
- exactly three classified production Rust files: PASS;
- no executable `unsafe`, mmap, or memmap surface: PASS;
- exactly two production packages: PASS;
- no production dependency under `experiments/`: PASS;
- `PHYSICAL_BODY_V1_AUTHORITY_STATIC_GATE_PASS`: PASS.

No test suite, release build, development matrix, or authority matrix was
executed during these checks. Every created sandbox was terminated by the
launcher.

## Frozen definitive command

The next organism command is the sole evidence event:

```text
cargo run --release --locked --manifest-path \
  experiments/verification/physical-body-v1-authority/Cargo.toml -- \
  --authority
```

Expected positive marker:

```text
PHYSICAL_BODY_V1_AUTHORITY_ESTABLISHED rows=16/16 clauses=540/540
```

Authority evidence remains unspent at this audit.
