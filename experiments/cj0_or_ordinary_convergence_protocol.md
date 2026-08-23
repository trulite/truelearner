# CJ0-OR ordinary convergence diagnostic protocol

Status: **PREREGISTERED; ALL CJ0-OR EVIDENCE UNSPENT; AUTHORITY UNCHANGED**.

## Frozen start and scope

This independent diagnostic begins exactly at authoritative PX2 commit
`2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`, tag
`px2-physical-causal-direction-authoritative`.

| frozen input | commit / SHA-256 |
|---|---|
| authoritative PX0-PX2 ancestry | `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129` |
| authoritative substrate law | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| PX2 authority handoff | `98647ab1563593e18e345cd7e5a71c4991d18b397dfe2dec71a4756106d96509` |
| frozen PX3 no-addition negative handoff | commit `873094497ff6eb74363191dc5edc479c7d66de72`, artifact `a029f250ed88f8f2fc164e0d2c9042675bf0a8c9ae51c89cf83ad1aa42e4fa9b` |
| frozen PX3-R downstream-convergence negative handoff | commit `5feb9b4c4755ed40d58ffc9cb8769d5523ea46f0`, artifact `091fa6dff351f2bee1033c25ea9a569248d88379401a743793fd4d873759d41e` |

The two PX3 negatives are immutable observations from other branches. They are
not parents of this branch, are not imported, and are not reinterpreted. In
particular, this diagnostic does not retry the PX3-R durable coupling
opportunity that failed swap specificity.

Only one additive sidecar example, fresh diagnostic documents, and fresh
result artifacts may be created. No authoritative PX0-PX2 byte may change. No
new CELL, ARROW, SPIKE, threshold, refractory, decay, pressure, plasticity,
queue, or lifetime law may be added to the substrate crate.

## Research question and narrow success meaning

The physical fixture contains two symmetric ordinary source CELLs, one
ordinary convergence CELL, one downstream CELL, two ordinary source-to-
convergence ARROWs, and one ordinary convergence-to-downstream ARROW. The two
source CELLs have identical thresholds and the two incoming ARROWs have
identical coupling, resistance, delay, and phase.

The diagnostic asks whether ordinary convergent propagation already supplies
the following bounded Boolean reachability:

```text
A alone  -> C reaches and drives the downstream CELL
B alone  -> C reaches and drives the downstream CELL
A + B    -> C reaches and drives the downstream CELL at least once
neither  -> no C or downstream firing
```

`A`, `B`, `C`, "alone", and the logical interpretation are evaluator-only
labels. Organism-visible construction and execution receive only numeric
CELL/ARROW/SPIKE parameters and handles.

Success does **not** require a one-pulse normalizer. With simultaneous equal
arrivals, the unchanged one-tick refractory rule may collapse two incoming
SPIKEs to one convergence firing. With positive skew, the same two routes may
produce two convergence firings. Both are a positive Boolean reachability
result if propagation remains bounded by the two source firings, naturally
quiescent, and produces no autonomous refiring. The result may therefore
classify ordinary disjunction as derived while explicitly refusing a claim of
timing-invariant output cardinality.

## Physical fixture

- Each source CELL has threshold `4` and is activated by exactly four fresh
  external SPIKEs.
- The convergence and downstream CELLs have threshold `1` in the candidate
  fixture.
- Both incoming ARROWs have coupling `1`, phase `0`, equal delay, and strong
  resistance `512`.
- The outward ARROW has coupling `1`, delay `1`, and resistance `512`; it
  crosses from region `0` to region `1` so useful downstream propagation is
  independently visible as a crossing and downstream firing.
- All explicit CELL positions are separated by more than the frozen local
  proposal radius. Incidental local structural proposals must therefore be
  exactly zero.
- Every propagation must drain its queue naturally. A second propagation with
  no new input must have an empty trace, zero crossing, zero work, and an
  unchanged complete fingerprint.

## Required positive and control executions per row

Each row uses fresh physical identities except its explicitly paired exact
replay. Every execution is built independently, and every build and every
discarded idle check is included in work/storage/source accounting.

1. neither source active;
2. first source alone;
3. second source alone;
4. both sources active with the row's preregistered skew;
5. exact replay of all four candidate executions;
6. coupling-`0` blocked first route alone and with the other route;
7. coupling-`0` blocked second route alone and with the other route;
8. absent first ARROW alone and with the other route;
9. absent second ARROW alone and with the other route;
10. stale first route (delay `6`, resistance `1`) alone and with the other
    route;
11. stale second route under the same matched control;
12. threshold-`2` saturation control, with each source isolated and both
    simultaneous.

The blocked, absent, and stale route alone must not reach convergence. When
paired with the intact symmetric route, that intact route must still reach the
downstream CELL. Each stale in-flight ARROW must physically deallocate before
delivery. The threshold-`2` control must show `0,0,1` convergence firings for
isolated-first, isolated-second, and simultaneous-both; it is recorded as
conjunction/saturation and may not be counted as evidence for disjunction.

## Exact variation matrix

Stages use fresh namespace bases and row counts:

| stage | namespace base | rows |
|---|---:|---:|
| PROBE v1 | `0xc100_0000_0000` | 8 |
| MICRO v1 | `0xc200_0000_0000` | 24 |
| GATE v1 | `0xc300_0000_0000` | 72 |
| definitive | `0xc400_0000_0000` | 120 |

For stage ordinal `d` in `0..4` and zero-based row `s`, configuration is:

```text
layout          = (s + 3d) mod 4
CELL permutation= (5s + 7d) mod 24
identity rotate = (s + d) mod 4
mirror          = (s + d) mod 2
ARROW allocation= (floor(s / 2) + d) mod 2
SPIKE insertion = (floor(s / 3) + d) mod 2
route delay     = (s + 2d) mod 6       # 0..5
source skew     = (floor(s / 6) + d) mod 5 # 0..4
phase pattern   = (s + d) mod 3
```

The four layouts use symmetric source distances `8, 16, 32, 64`; mirroring
exchanges their physical positions around the convergence CELL. The complete
24-permutation table is fixed lexicographic order over the four CELL roles.
Identity rotation changes physical ordering independently of handle
allocation. Reverse ARROW allocation and reverse SPIKE insertion do not alter
the symmetric route specifications.

Every case within a row receives a fresh `0x1000` subnamespace. A second run
with the same subnamespace is permitted only for the exact replay comparison.

## Fourteen conjunctive clauses

Each row serializes these bits independently:

1. `P0`: ancestry, frozen hashes, fresh namespace, and symmetric physical
   parameters are exact;
2. `P1`: first source alone fires once and produces exactly one convergence,
   crossing, and downstream firing;
3. `P2`: the symmetric second source alone does the same;
4. `P3`: both sources fire once and produce one or two bounded convergence and
   downstream firings, never zero and never more than two;
5. `P4`: neither source produces no convergence, crossing, or downstream
   firing;
6. `P5`: both coupling-`0` blocked-route controls suppress only the blocked
   route and preserve the intact route;
7. `P6`: both stale-route controls deallocate and suppress only the stale
   route while preserving the intact route;
8. `P7`: both absent-ARROW controls suppress only the absent route and preserve
   the intact route;
9. `P8`: the threshold-`2` control is exactly `0,0,1` and is excluded from the
   disjunction claim;
10. `P9`: observed simultaneous-versus-skewed output cardinality exactly
    matches the frozen refractory rule and is recorded, not hidden;
11. `P10`: every queue drains naturally and every no-input follow-up is inert;
12. `P11`: source firings equal externally activated sources, convergence and
    downstream firing are bounded by live source routes, and there is no
    autonomous or runaway refiring;
13. `P12`: complete execution, permanent state, work, trace, crossing, and
    fingerprints replay exactly;
14. `P13`: work, persistent storage, external SPIKEs, source firings,
    crossings, deallocations, and incidental proposals include every executed
    candidate, control, replay, and idle check with no discarded-clone leak.

Every stage is conjunctive. A failed row or clause is frozen as that stage's
negative and later stages do not run. PROBE and MICRO are warranted because
they separately expose the threshold-saturation and refractory-cardinality
artifacts before GATE. GATE must pass before definitive evidence is spent.

## Pre-evidence validation and focused tests

The committed/tagged implementation must pass:

- focused formatting, compilation, strict Clippy, and example unit tests;
- no-argument and wrong-argument refusal before physical execution;
- a no-CELL `--preflight` hash/ancestry/artifact audit;
- authoritative-source diff and SHA-256 audits;
- dependency, changed-path, forbidden-token, staging, and namespace audits;
- verification that the organism-visible block contains no evaluator labels;
- absence of every fresh final and staging artifact.

Focused unit tests use only the disjoint development namespace
`0xcf00_0000_0000`. They cover isolated propagation, simultaneous refractory
suppression, positive skew, blocked/absent/stale controls, threshold
saturation, natural quiescence, and exact replay. They never use an evidence
namespace and never publish an artifact.

The preregistration digest of sorted pre-existing result hashes is
`4f6bd31a84e94e8df85d736d25cb615f04586c33b4ccb48fbc140399c888fbc1`.

## Write-once execution and atomic artifacts

After implementation freeze, each command may execute exactly once, in order:

```text
cargo run --release -p px0-physical-correspondence --example cj0_or_ordinary_convergence -- --probe
cargo run --release -p px0-physical-correspondence --example cj0_or_ordinary_convergence -- --micro
cargo run --release -p px0-physical-correspondence --example cj0_or_ordinary_convergence -- --gate
cargo run --release -p px0-physical-correspondence --example cj0_or_ordinary_convergence -- --definitive
```

Each command emits exactly one stage-specific evidence-spent marker. It writes
CSV and Markdown to fresh `results/.cj0_or_ordinary_convergence_*.staging`
paths with create-new semantics, syncs both, and only then renames them to
their final paths. Existing staging or final paths cause refusal. No evidence
artifact may be rerun, regenerated, rescued, or overwritten.

Each stage result and result audit is frozen separately. Any accounting defect
must be frozen separately before a correction is preregistered; an accounting
negative may not be silently replaced by a scientific positive.

## Classification and stop rules

- Definitive `P0-P13` success in all `120` fresh rows classifies ordinary
  convergent propagation as **CJ0-OR derived**, limited to bounded Boolean
  reachability and explicitly excluding timing-invariant pulse cardinality.
- Any genuine physics failure freezes the first failing boundary and stops.
- A threshold-saturation or refractory-cardinality observation is not a
  genuine failure unless it violates its preregistered control clause.
- No outcome authorizes a new OR law, a semantic selector, a shared logical
  record, a PX3 reinterpretation, PX3/PX4 work, or an authority advance.

The final handoff must preserve PX2 as the sole authority, report all commits,
tags, hashes, work/storage/source accounting, and leave the branch clean and
remote-exact.
