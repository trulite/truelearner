# CJ0-F matched discriminator PROBE result audit v1

Status: **INTERPRETABLE DEVELOPMENT PROBE; BOTH FAIL; SHARED BOUNDARY**.

## Frozen execution

The corrected frozen comparator at commit `fdf1602` executed `probe` once.
It constructed 120 B worlds and 120 E worlds independently from 120 identical
physical row serializations. No final or staging artifact existed beforehand;
all four outputs were create-new, synced, and atomically renamed. No staging
path remains.

| artifact | rows / bytes | SHA-256 |
|---|---:|---|
| CJ-B CSV | 120 / 35,224 | `c6208e432e6489d77664fb7e7d5b7b5f2bc031f69646eb8e491256aeb3dd83f0` |
| CJ-E CSV | 120 / 35,327 | `986314ccae118f6fcc94e86394b89a03973c2f282a556419d4c28ac2fe0fb482` |
| paired CSV | 120 / 17,540 | `f19bfb8bfab445933da3a68898d26ac322e910e9be5f069876a9b992c2227882` |
| report | 950 bytes | `f4838977de54563d5c16c28c5645ae7e477e9359afbc2f7121eda5d572a0141c` |

Both candidate CSVs have exactly 54 columns and no malformed row. The paired
CSV has 120 unique joined identifiers and 120 equal physical-spec
fingerprints.

## Paired result by discriminator family

| family | rows | CJ-B pass | CJ-E pass | prediction differences | B false conjunction | E false conjunction |
|---|---:|---:|---:|---:|---:|---:|
| same-source bursts | 34 | 22 | 28 | 7 | 6 | 5 |
| amplitude vs multiplicity | 8 | 4 | 6 | 2 | 4 | 2 |
| dense return topology | 12 | 12 | 12 | 0 | 0 | 0 |
| timing transfer | 42 | 24 | 39 | 15 | 0 | 0 |
| shared controls | 24 | 21 | 24 | 3 | 3 | 0 |
| **total** | **120** | **83** | **109** | **27** | **13** | **7** |

The decisive unambiguous physical observations are:

- with threshold 2/coupling 1, one impulse-2 arrival produces one CJ-B effect
  but zero CJ-E effect; CJ-B therefore conflates amplitude with conjunction;
- two matched weak arrivals from one physical source/path produce one effect
  in both candidates, so neither frozen law establishes genuine contributor
  multiplicity in this lawful same-source world;
- distinct same-tick physical priming plus trigger reaches the effect in both;
- dense traversed-return/crossed-return/no-return attribution passes for both
  at sparse and dense allocation;
- at mature coupling 2, CJ-B can emit from trigger coupling without live
  contributor state, including singleton/crossed controls; CJ-E rejects those
  rows but retains a strong first arrival as transient state, which can combine
  with later matter in some before/edge timing rows.

These are predictions of the unchanged laws under equal fixtures. Physical
return is not marked incorrect and no candidate-specific rescue was supplied.

## Work and storage

| measure | CJ-B | CJ-E |
|---|---:|---:|
| total native work | 3,930 | 3,218 |
| summed per-row persistent bytes | 36,896 | 36,896 |
| per-row persistent range | 160..832 | 160..832 |
| summed temporary-byte lower bound | 9,520 | 9,520 |
| maximum per-row temporary-byte lower bound | 160 | 160 |
| naturally quiescent rows | 120/120 | 120/120 |
| runaway rows | 0 | 0 |

Economics does not select: both candidates fail a genuine scientific
requirement before work is considered. The equal persistent values reflect
the frozen implementations' equal private CELL/ARROW record sizes in this
matrix, not shared candidate storage.

## Classification and continuation

PROBE classification is **both fail; shared boundary frozen**. The test is not
ambiguous: evaluator-only provenance establishes that the failing arrivals
share one physical source/path, while neither law receives that label. The
result is therefore not eligible for repair or reinterpretation.

The preregistered MICRO and GATE expansions may still run to establish whether
the boundary is robust to fresh seeds, thresholds, loads, mirrors and
allocation. This does not reopen either frozen law and cannot promote either
candidate unless scientific sufficiency changes under the same fixed decision
rule. PX3 remains absent and no definitive/authority evidence exists.
