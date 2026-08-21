# RE0 implementation and development audit

Protocol: `reflected-compaction-economics-re0-v1`

Status: implementation and development gates frozen; definitive scientific
matrix not run.

## Frozen references

- parent outcome tag: `rc0b-grounded-motif-substitution-positive`;
- parent outcome commit:
  `f6dcec833966347ce360f3ec126202b4843319d5`;
- RE0 protocol tag: `re0-reflected-compaction-economics-protocol`;
- RE0 protocol commit:
  `4d6587b0f0bfe46ed3ceb919c4c3805c6c473d09`;
- RE0 protocol SHA-256:
  `c8b0633ad5a2459b3f4d61fccbdc7f2dcf6b30990d0157e57181b1803dcfdd71`;
- implementation source commit before this audit:
  `65aa542263ae7e1149e6578c46b749a38078419c`;
- frozen motif source plus additive observer SHA-256:
  `39bbd28d95f4da915106959eeac0fe64793c0b0398889dcea82ae391416f6abf`;
- RE0 binary SHA-256:
  `5950fdaa525a4b093dfb863e411ef16dcf727091e7d7f2b91def46d5c3429090`;
- frozen RC0b CSV SHA-256:
  `285ee87a7a77ea26b154cb728c63b1a53530891ae3ffd370e990dbd38e93f97e`;
- frozen RC0b Markdown SHA-256:
  `fb3fa0bb7b351c4ddc77a8e83fe60f42ad29d57a64fc1edfbbbcc34f24018b72`.

Neither `results/re0_reflected_economics.csv` nor
`results/re0_reflected_economics.md` exists. Development modes do not write
them, and the definitive runner refuses to overwrite either filename.

## Scientific boundary

RE0 adds no learner, execution path, motif, capability, or optimization. It is
accounting over the frozen RC0b learner and frozen definitive RC0b mature
runtime artifact.

The primary acquisition view charges the full blank-start stack:

```text
RP0a acquisition + RC0a consolidation + RC0b motif acquisition
```

Two preregistered secondary views expose RC0a+RC0b and RC0b-only incremental
cost when earlier assets are already owned. Persistent installation work is
zero because the consolidation work that creates each asset is already charged.
Per-use maintenance is zero because mature execution is read-only. Frozen RC0b
per-use runtime already includes temporary binding, validation, installation,
and invocation work, so none is charged twice.

Persistent carrying bytes remain separated by layer: RP0a learned roles and
program, RC0a compiled dispatch, and the RC0b motif. One seed-relative motif is
shared across all tested depths. Its acquisition is charged once and contains
no depth key.

All economic arithmetic uses exact integer millionths. Primary physical
break-even uses zero carrying price. The preregistered carrying prices
`0, 1, 10, 100, 1000` per million byte-horizons are secondary scenarios.
Exclusive-depth and balanced six-depth-cycle accounting both use exact ceiling
division and reconcile their reported horizons against the signed cost formula.

## Additive observer audit

Relative to the frozen positive RC0b tag, the only change to
`motif_substitution.rs` is 89 appended lines after all existing definitions:

```text
89 insertions, 0 deletions
```

The appended `Re0AcquisitionMeasurement` observer calls the unchanged frozen
fixture reconstruction and acquisition functions. It reports their existing
work, persistent byte counts, fingerprints, parity, and lifecycle totals. It
does not evaluate any held-out runtime cell, alter a type or pre-existing body,
or feed information back into learning or execution.

The definitive observer uses the frozen seed indices `0..8`, frozen RP0a
acquisition endpoints, and the frozen RC0b acquisition domain. Its duplicate
run is measurement-only determinism evidence. The development observer uses a
separate development fixture and domain.

## Frozen-runtime artifact audit

The RE0 binary embeds the frozen positive RC0b CSV and checks its SHA-256 before
use. It rejects any artifact that does not have the frozen schema, all 528
result rows, exact seed/depth/arm dimensions, positive RC0b-A and RC0b-B claim
flags, and the preregistered control behavior. It extracts only frozen concrete
and motif per-use work. It never invokes the RC0b definitive executor.

The resulting RE0 CSV has one fixed 25-column schema across acquisition,
exclusive-depth, balanced-cycle, horizon, and audit rows. Definitive mode runs
the eight acquisition observers twice for determinism, computes economics, and
writes the CSV and Markdown once.

## E2B clean-snapshot validation

Persistent sandbox: `iv7qfq154p7ffq4xpxw0o`

The clean implementation commit above was validated in one uninterrupted E2B
command chain:

```text
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -q
cargo run --release --bin grounded_motif_substitution -- --gate
cargo run --release --bin reflected_economics_re0 -- --micro
cargo run --release --bin reflected_economics_re0 -- --gate
```

The remote chain exited `0`.

The single full legacy regression was required because the frozen RC0b source
received the additive observer:

- library: 163 passed, 0 failed, 2 ignored;
- established main binary: 10 passed, 0 failed;
- RE0 binary: 3 passed, 0 failed;
- reviewer API: 4 passed, 0 failed;
- all other binary test targets: no failures.

No further legacy regression is required unless shared or frozen machinery
changes again.

## RC0b compatibility result

The full RC0b development GATE reproduced its frozen signature:

```text
concrete reference                  12/12   trace 12/12   work 10,160
full RC0a                           12/12   trace 12/12   work 11,936
motif substitute                    12/12   trace 12/12   work 10,304
changed surroundings                12/12   trace 12/12
interruption/re-entry               12/12   trace 12/12
context invalidation                12/12   trace 12/12   motif fires 0
forced stale same endpoint          12/12   trace  0/12
RC0a parent invalidation            12/12   trace 12/12   motif fires 0
no bindings                          0/12
workspaces                         569/569 destroyed
```

All fourteen qualitative/accounting gates passed. The shallow
whole-runtime-below-concrete diagnostic remained negative exactly as expected
for this development distribution; it is not an RC0b-A gate and does not alter
the frozen definitive RC0b-B outcome. Frozen RC0b work accounting and behavior
therefore remain compatible with the additive observer.

## RE0 MICRO result

MICRO used synthetic accounting values only and wrote no artifact. All eleven
gates passed:

```text
shallow exclusive depth    no finite break-even
deep exclusive depth       H* = 58 uses
balanced cycle             H* = 61 cycles / 122 invocations
workspaces                 2/2 destroyed; maximum live 1
duplicate measurement      deterministic
```

This validates signed exact-ceiling arithmetic, the no-break-even branch,
balanced-cycle invocation conversion, horizon reconciliation, schema, and
write isolation without consuming scientific data.

## RE0 GATE result

GATE used only development seed `30000` and the development fixture. Its
observed acquisition was:

```text
RP0a acquisition work          0   (prebuilt development fixture)
RC0a acquisition work      1,100
RC0b acquisition work      1,990
RP0a / RC0a / RC0b bytes   656 / 160 / 148
motif fingerprint          11454459317923793212
```

Zero-price physical break-even was:

```text
depth    concrete/use    motif/use    H*
5              468            773    none
8              756            998    none
16           1,700          1,774    none
32           4,356          4,094    12 uses
64          12,740         11,806     4 uses
128         41,796         39,518     2 uses

balanced six-depth cycle
             61,816         58,963     2 cycles / 12 invocations
```

All eleven RE0 gates passed. Both acquisition observations were identical, all
138 workspaces were destroyed, maximum live workspace was one, and the
persistent fixture remained unchanged.

These values are development diagnostics only. They make no RE0 economic
claim.

## Frozen status

```text
RG0a     functional grounding                         positive, frozen
RC0a     compiled recurrent dispatch                  positive, frozen
RC0b-A   genuine lower-work elimination               positive, frozen
RC0b-B   mature runtime below concrete                 positive, frozen
RE0      implementation + development gates           positive, frozen
RE0      definitive economic outcome                  pending
F1       blocked on definitive RE0
```

No RE0 definitive command was executed while preparing this audit. The next
claim-eligible action is the single frozen `--definitive` run after this audit
and implementation are committed and tagged.
