# R6 Partition Invariance Handoff

## Frozen result

At zero added latency, resident partitioning does not define the organism.

Keep:

- `ArenaId` for durable identity and content addressing;
- `ResidentArenaId` for disposable execution placement;
- one body-wide serial space;
- the global physical ordering key independent of placement;
- ordinary ARROW semantics across resident boundaries;
- `MechanicalConfig::REFERENCE` as the permanent oracle;
- `MechanicalConfig::PRODUCTION` as the selected production mechanics.

## R7 entry condition

R7 may begin only on a new branch. Its new question is non-residence and
admitted physical availability:

```text
target structure resident
→ ordinary R6 scheduling

target structure not resident
→ explicit load request
→ host completion quantized once
→ physical availability tick admitted
→ waiting physical activity resumes at that tick
```

R7 must preserve the R6 zero-latency resident control. It must not encode
semantic arena names, transparent OS paging, or host wall-clock stalls as
organism time.

## Review targets

- `truelearner/crates/core/src/lib.rs`
- `truelearner/crates/core/src/mechanics.rs`
- `experiments/verification/r6-partition-invariance/src/main.rs`
- `results/r6_partition_invariance_v1/r6_partition_matrix.csv`
- `experiments/results/r6_partition_invariance_development.md`

The untracked root file `academy.md` remains untouched.
