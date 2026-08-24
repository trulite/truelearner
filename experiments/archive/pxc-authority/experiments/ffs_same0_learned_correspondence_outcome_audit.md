# FFS-SAME0 definitive outcome audit

Protocol: `ffs-same0-learned-correspondence-v1`

Status: definitive A, B, and C positive; D expensive; E partial.

## Frozen inputs

- FFS0 positive tag: `ffs0-full-fractal-scaling-positive`;
- FFS-SAME0 protocol tag: `ffs-same0-learned-correspondence-protocol`;
- FFS-SAME0 implementation tag:
  `ffs-same0-learned-correspondence-implementation`;
- exact implementation commit:
  `a5bf2dd882684ca5c2561300f274ba0088852c16`;
- identity de-supply ladder tag: `identity-desupply-ladder-protocol`;
- identity de-supply ladder commit:
  `2c4687cff1240e6157a092cb8cf2039693f0035e`.

The implementation source, protocol, seeds, matrices, controls, work
accounting, and result paths were unchanged after the implementation freeze.
The umbrella identity ladder added documentation only.

## Single definitive execution

The exact implementation tag was checked out into an isolated detached Git
worktree and uploaded as a clean immutable snapshot to persistent E2B sandbox
`iv7qfq154p7ffq4xpxw0o`. The only definitive command was:

```text
cargo run --release --bin ffs_same0_learned_correspondence -- --definitive
```

The remote runner compiled the exact frozen commit, executed the complete
matrix, wrote both artifacts using create-new semantics, downloaded them, and
exited zero. The release build completed in 42.33 seconds. No second definitive
command was executed. The persistent E2B sandbox was left running.

## Write-once artifacts

- `results/ffs_same0_learned_correspondence.csv`
  - 1,176 lines;
  - SHA-256:
    `d136492e5ddaa70194f155657e7e86eee97da57e8dedcd9d1f52bcf81395a812`.
- `results/ffs_same0_learned_correspondence.md`
  - 395 lines;
  - SHA-256:
    `2bf80d0ed37e4d57aa3fe9ba2fb9ec2efc3663c2eb7ee0b701796731520c3c1f`.

The CSV contains the fixed schema and `definitive`, `claim_eligible=true`, and
`passed=true` on every evidence row. Its evidence includes:

```text
48 scale cells
16 cross-scale transfer cells
32 adaptive-context cells
136 qualitative control cells
5 independent claim outcomes
4 process-availability outcomes
6 final audits
```

## Independent outcomes

```text
A  correspondence reconstruction       PASS
B  functional binding/execution        PASS
C  recursive fractal recovery          PASS
D  identity economics                   EXPENSIVE
E  process availability                 PARTIAL
```

### A — correspondence reconstruction

Every definitive seed acquired two useful relational motifs from anonymous
temporal and causal evidence without supplied filler equality. Occurrence
relabeling, allocation order, memory order, and evaluator-truth relabeling did
not change the result.

Same-shape/different-continuity evidence was rejected, while
different-shape/same-continuity evidence was accepted. Missing and ambiguous
correspondence delivered no effect. All temporary occurrences retained only
invocation lifetime, and the covert reused-token detector passed in every
seed.

### B — functional recovery

Learned correspondence supported fresh temporary binding, grounded delivery,
changed bindings, interruption boundaries, context invalidation, and
historical context return for every definitive seed. Permanent learned state
remained stable during use; no historical occurrence or evaluator filler
identity entered it.

The 16 cross-scale transfer cells reused the exact persistent asset instance
with zero reacquisition and exact observable traces.

### C — recursive fractal recovery

Every definitive seed reproduced the same level-blind scale law:

| Scale | Depth | Population | Retained / justified / realized depth |
|---|---:|---:|---:|
| S0 | 8 | 16 | 0 |
| S1 | 32 | 64 | 3 |
| S2 | 128 | 256 | 5 |
| S3 | 512 | 1,024 | at least 6, right-censored |
| depth-only | 128 | 64 | 5 |
| population-only | 32 | 1,024 | 3 |

Every claimed promotion preserved the observable trace, reduced physical work
relative to its immediate retained parent, and had finite marginal
break-even. Structural, economically justified, and realized depths were
identical. Over-retention and under-retention were both zero.

The orthogonal probes show that useful recursive depth followed computational
depth rather than identity-population size.

### D — identity economics

The generic learned replacement was useful within the SAME-less organism, but
its mature whole-stack path remained exactly 18 work units per invocation more
expensive than the supplied-SAME reference at every scale:

| Scale | Supplied SAME | SAME-less | Delta |
|---|---:|---:|---:|
| S0 | 36 | 54 | +18 |
| S1 | 52 | 70 | +18 |
| S2 | 106 | 124 | +18 |
| S3 | 317 | 335 | +18 |

Generic correspondence acquisition cost 860 work and retained 26 bytes. This
definitive gate therefore establishes functional de-supply and recursive use,
not mature parity with the supplied prior. The positive recurring mature cost
is the preregistered condition that opens CS0a.

### E — process availability

```text
execution    positive
learning     unavailable
retrieval    unavailable
decision     unavailable
```

No adapter or semantic trace class was added. Cross-process closure remains
separate from the identity branch and does not alter A-D.

## Adaptation and controls

All 32 adaptive-context cells preserved exact behavior:

- a child-local violation exposed the direct parent with fallback distance 1;
- a direct-parent violation exposed the next valid dependency with fallback
  distance 2;
- historical context return reused the old correspondence with zero
  reacquisition.

All 17 preregistered qualitative controls passed for all eight seeds, for
136/136 control cells. Subthreshold and shuffled evidence did not consolidate;
failed evidence pruned; incompatible context invalidated correspondence and
reopened the ordinary path.

The six final audits all passed:

```text
frozen ancestry
duplicate determinism
source audit
identity-leak audits
scaling trend
orthogonal-depth signature
```

## Narrow claim

The definitive evidence supports this claim:

> Filler correspondence was acquired from anonymous temporal and causal
> structure as ordinary substrate structure, and that learned correspondence
> supported fresh binding, grounded execution, and recursively economical
> parent-relative computational organization without supplied filler equality.

The result does not show that learned correspondence is as cheap as supplied
SAME, and it does not establish cross-process closure.

## Frozen stopping decision

The identity de-supply ladder specifies:

```text
FFS-SAME0 A/B/C positive with positive mature generic correspondence cost
    -> open CS0a
```

That condition is satisfied. CS0a is now scientifically permitted, but no
compiled-correspondence outcome is implied by FFS-SAME0 and no downstream
capability was implemented while freezing this result.
