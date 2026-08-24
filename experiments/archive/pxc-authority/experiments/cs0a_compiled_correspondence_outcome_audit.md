# CS0a definitive outcome and grounding-trigger audit

Protocol: `identity-desupply-ladder-v1/cs0a`

Outcome: compiled correspondence positive; mature parity negative; CS0b
skipped because the frozen grounding trigger is absent.

## Frozen inputs

- FFS-SAME0 positive tag: `ffs-same0-learned-correspondence-positive`;
- CS0a implementation tag: `cs0a-compiled-correspondence-implementation`;
- exact implementation tag commit:
  `2ec9508a5ac7358ecc46ca73bdaf4c7174c02782`;
- exact CS0a kernel/harness SHA-256:
  `d1513b7dbbc5c8fb7d9453f27b6d3172d5a769337228ee2e967e7eb3aeb48c9b`;
- exact CS0a runner SHA-256:
  `6929b945e937cca8a0eed44bfd7d0efb33c1b169dde1e783c049bc5f0add490a`;
- implementation audit SHA-256:
  `ed81895cc80eb15561edec2cc1b2576024003f41d76ce1c1d2d248fa09a9219b`.

No implementation, threshold, matrix, accounting field, control, or output
path changed after the implementation freeze.

## Single definitive execution

The exact implementation tag was uploaded as a clean immutable snapshot to
persistent E2B sandbox `iv7qfq154p7ffq4xpxw0o`. The only definitive command
was:

```text
cargo run --release --bin cs0a_compiled_correspondence -- --definitive
```

The release build completed in 40.21 seconds. The runner executed the complete
matrix, wrote both paths using create-new semantics, downloaded them, and
exited zero. No second definitive command was executed. The sandbox remains
running.

## Write-once artifacts

- `results/cs0a_compiled_correspondence.csv`
  - 358 lines;
  - SHA-256:
    `98b983fb9cc88149c2c9de83e74e11d94e1aa3488a6f63f756e835fbd341bf36`.
- `results/cs0a_compiled_correspondence.md`
  - 378 lines;
  - SHA-256:
    `7ab1c68bcbb28db850010adcfcb99bc4a90438eacd76829beac9f9d458ce118b`.

The evaluator-derived trigger artifact is:

- `results/cs0a_grounding_tax_attribution.csv`
  - 6 lines including its header;
  - SHA-256:
    `8230b24109e924b264d2f3b3335acde06b91c81a4363d39a13a38afd1af9a5ec`.

The fixed CSV contains:

```text
8 acquisition rows
112 arm rows
232 control rows
1 summary row
4 final audit rows
```

Every row is `definitive`, `claim_eligible=true`, and `passed=true`. Every arm
row has exact behavior for all sixteen held-out episodes.

## Primary result

All eight seeds produced the same physical result:

```text
generic learned correspondence       18 work/use
compiled learned correspondence       6 work/use
reduction                            12 work/use
                                     66.67%
```

The whole mature depth-32 invocation is:

```text
generic learned                     162
compiled learned                    150
supplied-SAME reference             144
```

CS0a is therefore a definitive compilation positive and a mature supplied-SAME
parity negative. It removes two-thirds of the generic learned correspondence
tax but leaves a six-work execution premium.

Each seed acquired two compiled routes after threshold evidence. Compilation
cost 988 work and retained 80 bytes after the frozen 860-work generic
correspondence acquisition. Subthreshold and shuffled evidence retained zero
compiled routes.

## Functional and leak controls

Across all eight seeds:

```text
compiled ordinary uses                         128/128 exact
fresh occurrence uses                          128/128 exact
permuted allocation uses                       128/128 exact
permuted memory-order uses                     128/128 exact
changed binding uses                           128/128 exact
changed context -> generic reopening           128/128 exact
stale dependency -> invalidate + reopen        128/128 exact
historical compatibility -> compiled reuse     128/128 exact
missing correspondence                         128/128 exact
ambiguous correspondence                       128/128 exact
```

The compiled path never reopened generic inference during an ordinary,
fresh-identity, permuted, changed-binding, or historical-return use. Every
stale parent dependency invalidated and reopened the unchanged generic path.

All 232 control rows pass: 29 controls for each of eight seeds. These include
all inherited FFS-SAME0 occurrence-lifetime, covert-token, truth-relabeling,
same-shape/different-continuity, different-shape/same-continuity, missing,
ambiguous, and persistent-state audits.

Persistent compiled state remained fingerprint-identical across all uses and
contains no occurrence or filler identity type. Fresh temporary source/target
routes were erased after each invocation.

The four final audits pass:

```text
frozen ancestry
positive parent fixture
duplicate determinism
persistent source audit
```

## Narrow claim

The definitive evidence supports:

> Repeated successful use of learned filler correspondence consolidated into
> ordinary role-relative local substrate structure that preserved fresh
> binding behavior, reduced mature correspondence execution work, invalidated
> on stale dependencies, and reopened generic anonymous inference without
> reintroducing supplied filler equality.

This result does not establish parity with supplied SAME.

## Frozen six-unit attribution

The compiled path's six units are physically attributed as:

```text
compiled local activation              1
context/support validation             1
parent dependency validation           1
ambiguity handling                     1
temporary route installation           1
temporary binding write                1
-----------------------------------------
total                                  6
```

The definitive CSV directly records, for every compiled seed, sixteen local
activations, sixteen support validations, sixteen dependency comparisons,
sixteen temporary installations, and sixteen binding writes across sixteen
held-out uses. The remaining one unit per use is the frozen ambiguity check in
the kernel work counter.

No new execution was needed to construct the preregistered `U` attribution.
The frozen implementation has no cross-use mutable episode state, so its
measured per-use primitive counters are exactly additive. The evaluator-only
derived artifact `results/cs0a_grounding_tax_attribution.csv` records:

| U | Residual overhead | Binding | Installation | Grounding total | Other |
|---:|---:|---:|---:|---:|---:|
| 1 | 6 | 1 | 1 | 2 | 4 |
| 2 | 12 | 2 | 2 | 4 | 8 |
| 4 | 24 | 4 | 4 | 8 | 16 |
| 8 | 48 | 8 | 8 | 16 | 32 |
| 16 | 96 | 16 | 16 | 32 | 64 |

The exact preregistered slopes are:

```text
residual repeated-overhead slope = (96 - 6) / 15 = 6
grounding slope                  = (32 - 2) / 15 = 2
grounding share                 = 2 / 6 = 1/3
```

## CS0b trigger decision

The frozen trigger requires both:

1. `grounding slope > 0`;
2. grounding accounts for at least half of the positive residual slope.

The first condition passes, but the second fails because `1/3 < 1/2`.
Therefore the required outcome is:

```text
CS0b: SKIPPED — trigger absent
```

This is not a negative CS0b experiment. CS0b was never opened or implemented.
The residual cost is dominated by ordinary compiled activation and validation,
not repeated temporary grounding.

Under the frozen ladder, a positive CS0a with an absent CS0b trigger opens
FFS-SAME1 directly. FFS-SAME1 must use the unchanged compiled correspondence
path, including its six-unit mature tax. No optimization may be inserted before
that reintegration experiment.

## Updated identity ladder

```text
FFS-SAME0   supplied SAME architecturally necessary?    NO, definitive
CS0a        mature learned correspondence compilable?   YES, definitive
CS0b        repeated grounding amortization?            SKIPPED, trigger absent
FFS-SAME1   full recursive reintegration                 NEXT
IP0         final identity-prior economics               blocked on SAME1
```
