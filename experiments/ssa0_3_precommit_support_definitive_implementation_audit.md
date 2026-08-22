# SSA0.3 definitive pre-closure support implementation audit

Status: **EXACT EXECUTABLE AUTHORITY SNAPSHOT READY; EVIDENCE UNSPENT**.

## Frozen chain

- immutable SSA0 Classification C:
  `34277893201c1a72765b143de4b3da1912b6e3b6` /
  `ssa0-spatiotemporal-affordance-micro-v1-negative`;
- frozen SSA0.3 developmental Classification A:
  `eeb14186a000a7eefba17e6f9e288e7335c44043` /
  `ssa0-3-precommit-support-development-v1-classification-a`;
- definitive protocol commit/tag:
  `1ccbc3c588821269eda4fb8552f728a3638808b6` /
  `ssa0-3-precommit-support-definitive-protocol-v1`;
- clean authority implementation candidate:
  `65ed12ebf3a9877e59acc756bc1aebbf96c7520b`;
- authoritative M6:
  `aa4e22efd8a65b7694956a53cfaa970582695215` /
  `core-autonomy-checkpoint-established`.

The candidate is a linear descendant of the preregistration. It adds only:

```text
src/ssa0_3_precommit_support_definitive.rs
src/bin/ssa0_3_precommit_support_definitive.rs
```

No frozen development, Classification C, M6, Lane A, shared library, manifest,
lockfile, or prior result byte changed.

## Exact hashes and copied physics

| artifact | SHA-256 |
|---|---|
| definitive protocol | `2185e4d10dca9919184c12df14f95a7100ea963ba0abdc7b0162cd834558d220` |
| definitive authority source | `0ea4ba971be9456b4c737d7e11613f7cfd31b338bcb8a5e256644ecca4c45c1f` |
| definitive write-once runner | `8b8b532428795cc3d746c3ea8a765ea494140903743137547985c419cd337c9a` |
| frozen SSA0.3 source | `4a4e727f4f8ca6ee03faaae76de1a1091472de20ed9d91388e7a36056326edd7` |
| frozen SSA0.3 runner | `3711f123be3a4efc0494cc01f85fd8bc176ffc765eb1a2183b1e14c76baea435` |

The marker-bounded propagation region is byte-identical among immutable SSA0,
frozen SSA0.3 development, and the definitive authority source. Its exact
substring SHA-256, including the newline on each side of the marker-bounded
body, is
`575c63751528aaeb8b111236c3a7ab94e716393689e1840d6af96751c7a096ce`.
The preregistration's `a744...` value is the independently recorded line-scan
digest; that shell scan also encounters the later marker strings in the source
auditor. Direct substring extraction resolves the convention while the
governing byte-equality predicate is exact under both comparisons. No physics
or predicate was changed.

The runtime preflight recomputes all fourteen frozen SHA-256 values, compares
all three physics substrings directly, scans the authority runtime for every
forbidden primitive, verifies the exact three schedules/four allocations/two
mirrors/31 conditions, proves ordinals `0..743`, and checks the disjoint
physical and occurrence namespaces. It never builds a fixture or calls the
propagation loop.

## Matrix and information-flow audit

The static matrix is exactly `3 * 4 * 2 * 31 = 744` rows and 1,488 exact replay
propagations. Every row has a unique physical world and occurrence identity.
All commitment ticks, intervention ticks/phases, route identities,
allocations, inert layouts, and protocol namespaces are absent from prior SSA
evidence.

The frozen physics sees only ordinary integer CELL/ARROW/SPIKE state. It does
not receive the external expected route/tick, family, condition, added-side
classification, physical-role mirror, or pass predicate. Closure is derived
after propagation from the first contender firing in the actual trace. The
next delivered physical event must be the same-tick inhibitory impulse `-64`
to the other contender. Late-event visibility is derived only from trace order
after that actual firing.

No RNG/noise, chooser, sampling, stored probability, count/precommit score,
softmax, temperature, noisy argmax, semantic action/effect label,
evaluator-selected firing, commitment metadata, or commitment CELL exists.

## Atomic write-once and refusal audit

The runner exposes only `--audit` and `--definitive`. No argument, unknown
arguments, or occupied fixed final/staging paths refuse before row zero with
exit `2`. There is no override, filter, partial mode, resume, append,
replacement, seed, schedule, output, development, or replay option.

Both artifacts are first fully written and synced at fixed create-new staging
paths. Hard links then publish the final paths without replacement, the parent
directory is synced, and the staging links are removed. A bounded temporary
test proved first publication succeeds, a second publication returns
`AlreadyExists`, and original bytes remain exact. No definitive path was used
by the test.

## Focused zero-cell validation

Local checks against candidate `65ed12e`:

```text
cargo fmt --all -- --check                                      PASS
cargo check --release --bin ssa0_3_precommit_support_definitive PASS
cargo clippy ... -D warnings                                    STOPPED
  only on the same ten frozen/generated warnings documented by development
cargo clippy ... -D warnings with the three established frozen-code allows
                                                                 PASS
cargo test --release --bin ssa0_3_precommit_support_definitive  PASS (3/3)
--audit                                                         PASS
no argument                                                     REFUSED (2)
```

The three tests are restricted to a known SHA-256 vector plus frozen/source
preflight, static matrix shape, and bounded temporary create-new refusal. They
execute zero definitive rows. The Clippy allowances are only
`derivable_impls`, `manual_is_multiple_of`, and `manual_div_ceil`, matching the
ten pre-existing frozen/generated warnings in the developmental GATE audit.
No broad historical suite ran because shared/frozen code did not change.

## Fresh E2B authority validation

Dedicated state:

```text
/Users/satya/.cache/truelearner/ssa0-3-precommit-support-definitive-authority-e2b.json
```

The state path was absent before first connection. It created fresh sandbox
`irak38zpevfo98joodapc` from template `truelearner-rust-1-97-worker` with
`reused=false`, distinct from every development and Lane A sandbox. The
sandbox remains running with an 86,400-second timeout.

The clean Git archive of exact candidate `65ed12e` passed remote formatting,
focused release compilation, focused Clippy with only the three frozen-code
allows, all three zero-cell tests, `--audit`, and no-argument exit-2 refusal.
Remote preflight reported:

```text
frozen_hashes_exact       true
physics_byte_exact        true
forbidden_runtime_absent  true
matrix_shape_exact        true
namespaces_fresh          true
outputs_absent            true
staging_absent            true
passed                    true
```

Remote source/runner/protocol hashes match the table above. The fixed CSV,
Markdown, and both staging paths are absent locally and remotely. No
definitive row has executed. Evidence remains unspent.

## Frozen execution boundary

After this audit is committed and tagged as the exact executable authority
snapshot, the sole authorized evidence command remains:

```text
cargo run --release --quiet --bin ssa0_3_precommit_support_definitive -- --definitive
```

The boundary is entry into ordinal `0` after the repeated zero-cell preflight.
The command may be issued exactly once in the named sandbox. It may never be
rerun, rescued, tuned, resumed, or reinterpreted. The sandbox must remain
running afterward.
