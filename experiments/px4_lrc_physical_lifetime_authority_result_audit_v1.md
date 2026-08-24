# PX4 LR-C physical lifetime authority result audit v1

Status: **DEFINITIVE POSITIVE; PX4 AUTHORITY EARNED; PX5--PX8 UNCHANGED**.

## Frozen authority chain

| role | commit | tag |
|---|---|---|
| PX3+LR-C parent authority | `f9057fe78a86db9111b0b69310d03accef3bc970` | `px3-lrc-physical-event-authority-v2` |
| PX4 development readiness | `20bc9ce384b74b6e5cca04f4bed2599932a34e92` | `px4-lrc-lifetime-development-ready-v1` |
| authority protocol | `12e6e451e98b9b120732ada8ab3bb079fa27ad4c` | `px4-lrc-lifetime-authority-protocol-v1` |
| definitive implementation/audit | `c39a97b1cb28b8a748047ac9aff903d9fe22dc35` | `px4-lrc-lifetime-authority-frozen-v1` |
| immutable functional evidence | `48394cdbcc82c803f3e9e20418c4ea1f247cbf6b` | `px4-lrc-lifetime-definitive-positive-v1` |
| frozen authority taxonomy | `5cfbe7940c48c3a6bdff367e921e58ddca9c2530` | pending this audit |
| frozen authority comparator | `fb7a4fb9f402993140694f8d48ece4d1340a96ee` | pending this audit |

The exact mechanism source and LR-C law hashes remained
`a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71`
and
`7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10`.
No mechanism changed after protocol freeze and no genuine scientific fork was
required.

## One-shot definitive matrix

The registered command executed exactly once from clean frozen commit
`c39a97b1cb28b8a748047ac9aff903d9fe22dc35` in fresh E2B sandbox
`inxw30l2xt15psd7b0xnj`, state
`px4-lrc-authority-definitive-20260824-v1.json`:

```text
cargo run --release --manifest-path arms/px4-lrc-lifetime/Cargo.toml \
  --bin px4_lrc_lifetime_authority_v1 -- --authority-v1
```

It emitted one completion line and atomically published the sole CSV/Markdown
pair. There was no rerun or cross-sandbox regeneration.

| functional artifact | SHA-256 |
|---|---|
| `results/px4_lrc_lifetime_authority_v1.csv` | `050a2b489e41d13e8d8a3d55dd7d69c6e06894b85b2c172f7dc24614af09aeaa` |
| `results/px4_lrc_lifetime_authority_v1.md` | `445c465ba61cc12c0ece84a8ebb9a83bea1e67c1a4d640964cc7d93c0dbe4390` |

Exact result:

```text
rows                                      16/16
row clauses                             656/656
global layout/schedule clause               1/1
total clauses                           657/657
exact complete-state replay               true
natural quiescence                        true
identity/layout/schedule invariance        true
PX0--PX3+LR-C conformance                  true
resistance sequence                   4,7,12,22
pressure-to-deallocation sequence     4,7,12,22
```

All sixteen fresh roots `461001..461016`, both allocation orders, both layout
reflections, absolute origins `200|400` and both replicates passed. Every row
covered one exposure; recurrence-earned persistence; fixed pressure and
physical deallocation; mature reuse with no proposal and impulse `2`; ordinary
qualified `+3` modulation; disuse; changed experience; reproposal and distinct
generation reacquisition; stale-generation blocking; return-alone, late-return
and Drive-return controls; directionality; replay; and quiescence.

## Independent result, leakage and coverage audit

Fresh E2B sandbox `ii87wds6zcy2j53i2uamr`, state
`px4-lrc-authority-result-audit-20260824-v3.json`, ran the frozen static and
matrix audit scripts from commit
`7c0e27f6de2a9a538ed89666fc04bb23a547203c`. It independently reproduced:

```text
artifact hashes          PASS
rows/clauses             16/16, 657/657
controls                 PASS
replay                   PASS
quiescence               PASS
cumulative conformance   PASS
semantic leakage         ZERO
unclassified sources     ZERO
foundation seams         ZERO
```

The active PX4 mechanism contains no lifetime object or field, `History`,
episode/reset, cleanup/delete semantic, evaluator-derived lifetime input,
typed lifetime handoff or explicit lifetime invocation. The authority runner
is evaluator-only and writes no organism state. All 24 pressure observations
per recurrence count are fixed before observation and cannot feed a measured
lifetime back into the substrate.

## Fresh PX-C taxonomy and strict parent comparator

Fresh E2B sandbox `i119in5wzroaoj6phd8lv`, state
`px4-lrc-authority-taxonomy-20260824-v1.json`, audited evidence/audit commit
`7c0e27f6de2a9a538ed89666fc04bb23a547203c` with exact active-manifest hash
`28924746e951645047225d8d20f5c5f98d93f349f46f7c6d7019e68632ce51b9`.
It produced:

| taxonomy artifact | SHA-256 |
|---|---|
| inventory | `742532d904622ecf4c5641b55f078fbd9f732b7f5898d5120b0f730c3a0ccec0` |
| guard inventory | `5a23d1f89476d87f2a630d89145cd87a8a6169d9a1a72d6b309cf859400d8675` |
| summary | `59c48919e91d698033a7931c15690fa148691a8afaf89cd22a4b7794ce4e2ee0` |
| report | `5c30e57313de7ef304fcf5a9bd48206c271c195a394ffdb222d5fa447d7b536a` |

Fresh E2B sandbox `iujplkc0wbqakrbnlrm55`, state
`px4-lrc-authority-comparator-20260824-v1.json`, compared those raw artifacts
only with the authoritative PX3 parent baseline and exact manifest hashes
`472440f5e989387044fa3d36c5364b2d65f30d01659742a829d007cb67f7ef9a`
and `28924746e951645047225d8d20f5c5f98d93f349f46f7c6d7019e68632ce51b9`.

| metric | parent | PX4 authority | delta | passed |
|---|---:|---:|---:|:---:|
| primary seams | 368 | 297 | -71 | true |
| semantic guard | 218 | 162 | -56 | true |
| evaluator guard | 752 | 559 | -193 | true |
| new seam kinds | 0 | 0 | 0 | true |
| new guarded surfaces | 0 | 0 | 0 | true |
| PX0--PX3+LR-C foundation | 0 | 0 | 0 | true |

| comparator artifact | SHA-256 |
|---|---|
| readiness CSV | `3fb524dd7afd98a5fe78c02e43c612987304d5e1a13123af6e675ade56864afe` |
| readiness report | `a5bd68ac4f66409abf7b28e18f5d600d4df2e39a9f6b28c1370f28ad5b29bffd` |
| kind delta | `2a0e306e05d25e3bfe7894d8cc6d939d159513fe6ddf2ea0bdc94cee3a8c4a95` |
| layer delta | `08274b875966cf76ff5e874f1711e67d3bc5af1bb0a1f083aec65bcc0b30b1d5` |
| new kinds | `7e5ecf41e673f27bfc5957420ba466da02c700c15e982bfeed4727058ce3c0de` |
| new surfaces | `d5033ae75b748d89a215895d25406b7ab5155f622e42dcd59ec72db19a3f7ca9` |

The raw new-kind and new-surface files contain headers only. PX5--PX8 layer
counts remain exactly `14,37,136,110`; they were neither changed nor used to
support the PX4 claim.

## Retained technical preflight/audit negatives

All stopped before definitive world construction or affected only a post-result
verifier; none changed or reran authority evidence:

| sandbox | immutable technical stop |
|---|---|
| `inn79xz46vtdxp5lxcniu` | rustfmt check exposed mechanical evaluator formatting |
| `i8ok7ly7gepwnmc18f11y` | archive snapshot had no `.git` for the first static audit |
| `imzlhehafh6aukf14j5zt` | E2B image lacked `rg` before fallback was frozen |
| `i60o8wr5w3kxh9zbztxbs` | inline post-result `awk` was shell-interpolated before parse |
| `iu30k971t84e7g1b79avv` | POSIX `awk` rejected multiline boolean syntax |

Successful no-world preflight sandbox `i62kdw2rt1g37qdqyj5en` and all evidence/
audit sandboxes were left running under unique state files.

## Scientific result

Within the preregistered smallest existing-physics geometries, learned physical
lifetime collapses completely onto existing ARROW resistance, ordinary
pressure, recurrence/reuse, eligibility and qualified LR-C Modulation. One
exposure is short-lived; qualified recurrence earns proportionally greater
physical persistence and immediate reuse; unsupported structure disappears;
changed supported experience retains the new path; reacquisition is a new
generation; stale work cannot revive the old generation. No lifetime-specific
law or semantic mechanism is present.

All preregistered authority conditions passed. A separate PX4 authority handoff
may now be frozen. This audit does not execute, import or authorize PX5--PX8.
