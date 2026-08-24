# PX-C PX4 development-readiness seam handoff v1

Status: **PX4 DEVELOPMENT READY; AUTHORITY NOT RUN AND NOT CLAIMED**.

## Lineage

- serial authority ancestor:
  `f9057fe78a86db9111b0b69310d03accef3bc970`;
- development implementation commit/tag:
  `7c39b45fa9e7edfd393240368684185f46883dd5` /
  `px4-lrc-lifetime-development-implementation-v1`;
- functional GATE result commit/tag:
  `8714ce5e53717572551b8bc53edf3cb9d649d861` /
  `px4-lrc-lifetime-gate-positive-v1`;
- before active-surface manifest SHA-256:
  `472440f5e989387044fa3d36c5364b2d65f30d01659742a829d007cb67f7ef9a`;
- after active-surface manifest SHA-256:
  `28924746e951645047225d8d20f5c5f98d93f349f46f7c6d7019e68632ce51b9`;
- functional GATE E2B sandbox: `i9y7kazos146gzwuphwk6`;
- exact artifact-replay E2B sandbox: `i05mu61g04ubqr5aqn6n5`;
- final taxonomy E2B sandbox: `i6mlrh7u2iv4abtdkz01e`;
- final readiness comparator E2B sandbox: `ijq864x9iqlqlfenpakht`.

All retained evidence sandboxes were fresh, used unique state files through
the established launcher, and were left running. Failed compile-only sandbox
`ic74p4yt92m9ho1xyx1kr` and superseded taxonomy sandbox
`ivhh9mi0wathsydi2wbiq` were terminated to release capacity; all their
downloaded evidence had already been frozen, and the taxonomy was replayed in
the retained final sandbox above.

## Functional result

- protocol:
  `experiments/px4_lrc_physical_lifetime_development_protocol_v1.md`;
- result:
  `results/px4_lrc_lifetime_gate_v1.csv` and
  `results/px4_lrc_lifetime_gate_v1.md`;
- result audit:
  `experiments/px4_lrc_physical_lifetime_gate_result_audit_v1.md`;
- verdict: `PASS`;
- rows: `8/8`;
- exact in-row replay: `true`;
- exact cross-sandbox artifact replay: `true`;
- natural quiescence: `true`;
- first collapse: `none`.

The exact resistance and pressure-to-deallocation sequences for qualified
recurrence counts `1,2,4,8` were both `4,7,12,22`. One exposure, recurrent
supported persistence, disuse, pressure deallocation, reuse advantage,
changed participation, reacquisition, stale-generation blocking, fresh
identities/layouts, and PX0--PX3 conformance all passed with their registered
controls.

## Manifest coverage

- predecessor entry replaced:
  `PX4,src/ds6_cumulative_lifetime_probe.rs,predecessor-target`;
- new active mechanism source:
  `arms/px4-lrc-lifetime/src/lib.rs`;
- unchanged authoritative dependency:
  `crates/lr1-modulatory-physical-return/src/lib.rs`;
- evaluator-only sources excluded with reasons:
  - `arms/px4-lrc-lifetime/src/main.rs` schedules anonymous arrivals,
    observes public physical quantities and serializes verdicts; it owns no
    organism state or state transition;
  - `arms/px4-lrc-lifetime/tests/physics.rs` contains preflight assertions
    only;
  - `arms/px4-lrc-lifetime/Cargo.toml` declares the sole direct dependency and
    is not executable source;
- source/dependency audit proving complete candidate coverage:
  `experiments/px4_lrc_physical_lifetime_implementation_audit_v1.md`;
- unclassified active source files: `0`.

The v2 manifest changes no PX0--PX3+LR-C or PX5--PX8 row. The active PX4
library depends directly and only on the manifested authoritative substrate.
All physical mutation in the excluded evaluator enters public substrate
methods whose implementation is independently manifested.

## Mandatory PX-C delta

| metric | before | after | delta | accepted |
|---|---:|---:|---:|:---:|
| primary seams | 368 | 297 | -71 | true |
| semantic guard | 218 | 162 | -56 | true |
| evaluator guard | 752 | 559 | -193 | true |
| new seam kinds | 0 | 0 | 0 | true |
| new semantic surfaces | 0 | 0 | 0 | true |

### Primary-kind delta

| kind | before | after | delta |
|---|---:|---:|---:|
| typed representation | 87 | 85 | -2 |
| explicit mechanism invocation | 72 | 66 | -6 |
| episode/reset boundary | 1 | 1 | 0 |
| seed/history synthesis | 61 | 9 | -52 |
| semantic condition | 38 | 38 | 0 |
| manual temporary cleanup | 1 | 1 | 0 |
| typed handoff | 98 | 90 | -8 |
| evaluator-derived input | 10 | 7 | -3 |

### Layer delta

| layer | before | after | delta |
|---|---:|---:|---:|
| PX0--PX3+LR-C | 0 | 0 | 0 |
| PX4 | 71 | 0 | -71 |
| PX5 | 14 | 14 | 0 |
| PX6 | 37 | 37 | 0 |
| PX7 | 136 | 136 | 0 |
| PX8 | 110 | 110 | 0 |

## Generated readiness artifacts

| artifact | SHA-256 |
|---|---|
| readiness delta CSV | `3fb524dd7afd98a5fe78c02e43c612987304d5e1a13123af6e675ade56864afe` |
| readiness delta report | `a5bd68ac4f66409abf7b28e18f5d600d4df2e39a9f6b28c1370f28ad5b29bffd` |
| primary-kind delta CSV | `2a0e306e05d25e3bfe7894d8cc6d939d159513fe6ddf2ea0bdc94cee3a8c4a95` |
| layer delta CSV | `08274b875966cf76ff5e874f1711e67d3bc5af1bb0a1f083aec65bcc0b30b1d5` |
| new-seam-kinds CSV | `7e5ecf41e673f27bfc5957420ba466da02c700c15e982bfeed4727058ce3c0de` |
| new-guarded-surfaces CSV | `d5033ae75b748d89a215895d25406b7ab5155f622e42dcd59ec72db19a3f7ca9` |
| after taxonomy inventory | `742532d904622ecf4c5641b55f078fbd9f732b7f5898d5120b0f730c3a0ccec0` |
| after guard inventory | `5a23d1f89476d87f2a630d89145cd87a8a6169d9a1a72d6b309cf859400d8675` |
| after taxonomy summary | `59c48919e91d698033a7931c15690fa148691a8afaf89cd22a4b7794ce4e2ee0` |
| after taxonomy report | `0949eb76b65ca6f45d46fdb3d8178267c133f9f32cdfee222f651d3fea7d68f6` |

The immutable before inventory, guard and summary hashes remained
`b19bf54d7d3133cca0caf98ecca89d483499cae8a6fe53ac0faac464df186441`,
`471905f91806a0fa9b4bb9419653e8a98b0e0cb1784638b1ff5e7f6414b5f1d8`,
and `ccfb10e50e491067fbd7e52157161f6a096e69f5e5e6b832245ce876c730c607`,
respectively.
The raw comparator outputs contain only headers for new seam kinds and new
guarded surfaces.

## Readiness verdict

`PX4 DEVELOPMENT READY` is positive because:

1. PROBE, MICRO and GATE passed without a mechanism change;
2. primary seams strictly decreased;
3. both relocation guards decreased;
4. new seam kinds and new guarded surfaces are zero;
5. complete active-surface coverage is proven;
6. all functional runs naturally quiesced; and
7. exact in-process and independent E2B artifact replay passed.

This handoff freezes development readiness only. It does not run a definitive
PX4 experiment, advance the serial authority branch, reinterpret PX3, or
authorize PX5. Any authority workflow must be separately preregistered and
must start from the exact serial ancestor required at that future time.
