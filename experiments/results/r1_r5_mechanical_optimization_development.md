# R1-R5 Mechanical Optimization Development Result

## Outcome

All five sequential mechanical prefixes preserve the accepted physical history.
This is development evidence, not a new authority run and not a change to the
physical learning law.

```text
R1 TimingWheel  PASS
R2 Adjacency    PASS
R3 Frontier     PASS
R4 SoA          PASS
R5 Batched      PASS
```

The permanent reference configuration remains:

```text
Vec + GlobalScan + FullScan + AoS + Scalar
```

The final candidate configuration is:

```text
TimingWheel + Adjacency + Frontier + SoA + Batched
```

Both execute the same singular CELL/ARROW/SPIKE transition law.

## Differential evidence

The frozen accepted corpus was evaluated as 16 worlds against each of the five
candidate prefixes:

```text
worlds                 16
candidate prefixes      5
differential pairs      80 / 80
behavioral clauses     536 / 536
arena-format tests       4 / 4
core tests              13 / 13
strict Clippy           PASS
format check            PASS
```

Every pair agreed on crossings, drive and modulatory deliveries, plasticity
updates, proposals, deallocations, physical clock and pressure phase,
canonical pending activity, canonical durable body bytes, quiescence, replay,
and physical work categories.

Stable identity was also checked across SoA compaction and live-checkpoint
restart. Resident slots changed while physical references and behavior did not.

## Mechanical result by prefix

- R1 replaces the global pending-spike minimum scan with a bounded near timing
  wheel plus overflow. It reduced recorded ordering comparisons from 1,146,224
  to 439,184. Canonical pending activity remains scheduler-independent.
- R2 adds source adjacency and reduced recorded scans from 32,042,832 to
  23,586,256.
- R3 adds sparse active/eligible frontiers and reduced recorded scans to
  4,304,544. The scientifically delicate pressure law remains a full scan with
  its accepted epoch-zero phase.
- R4 is an actual SoA resident store, not a label over AoS. It is physically
  equivalent and compaction-invariant, but its current accessor reconstruction
  is slower than R3 in this diagnostic corpus.
- R5 performs exact ordered same-tick batching only when live zero-delay
  topology cannot add work to the current tick. Otherwise it deliberately
  falls back to scalar execution. A dedicated no-zero-delay test proves fewer
  queue operations without a physical difference.

## Cost interpretation

The diagnostic cost snapshot is recorded in
`r1_r5_mechanical_optimization_costs.csv`. CPU time is nondeterministic and is
not evidence. The worlds themselves completed quickly; most end-to-end time in
fresh E2B workers was registry download and compilation.

The `allocations` and `bytes_touched` fields are reserved but not yet
instrumented; their zero values must not be interpreted as measured zero cost.

## Boundaries

- SIMD is not implemented or claimed.
- R4/R5 have earned physical equivalence, not final production-performance
  selection.
- The accepted reference configuration remains the default correctness oracle.
- No R6 multi-arena, persistence, transport, framebuffer, forked context, or
  development-journal capability is claimed.
- The accepted learning laws and prior authority evidence were not rerun or
  relabeled.

## E2B provenance

Final consolidated sandbox: `i4g070ock3e9fbqvnlfxi`.

It ran, in one worker and one dependency build chain:

1. `cargo fmt --all -- --check`
2. strict workspace Clippy with warnings denied
3. workspace tests
4. the 16-world, five-prefix differential corpus

No Rust command ran locally.
