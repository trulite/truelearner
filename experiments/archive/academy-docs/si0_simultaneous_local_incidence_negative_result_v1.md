# SI0 simultaneous local incidence negative result v1

Status: immutable stopped negative. SI0 is not development-positive and no
downstream work is authorized.

Frozen candidate: `3a2497f` (`si0-simultaneous-local-incidence-frozen-v1`).

Sole evidence execution: E2B `ia1qo4yxiuy01e4d0t0e8`.

## Result

- Families: `10/10` executed.
- Rows: `116/120` passed the frozen composite gate.
- Reference/Production equality: `120/120`.
- Exact replay: `120/120`.
- Preregistered firing behavior: `120/120`.
- Natural quiescence: `120/120`.
- Pending activity and loads: zero in `120/120`.
- Baseline/permutation equality: `116/120`.
- Maximum PhysicalWork: `37`.

The four failed rows are the Reference and Production copies of exactly two
permutations:

1. `different_junctions / reverse_input`;
2. `zero_delay_fanout_merge / reverse_physical_names`.

Every failed row has `replay_equal=true`, `cross_equal=true`,
`expected_firing=true`, `quiescent=true`, `pending=0`, and `loads=0`. Only the
frozen normalized-trace baseline comparison is false.

## Exact classification

This is an evaluator trace-normalization defect exposed by the candidate's
required two-stage law.

The runtime intentionally records all same-wave `DriveIncidence` events while
updating junction state, then records all causally resulting `Fire` events in a
second pass. The frozen normalizer instead assumes every non-incidence event
belongs to the most recently recorded incidence. With two independent
junctions in one wave, it therefore attaches both fires to whichever incidence
happened to be recorded last.

The serialized first divergence makes the defect explicit:

```text
INCIDENCE:left | FIRE:right | FIRE:left
INCIDENCE:right
```

Reversing admitted order or physical-name order changes which incidence is
last, so this post-execution association changes even though the physical
firing multiset, future state, Reference/Production result, replay, and
quiescence do not.

This classification does not relabel SI0 positive. The preregistered composite
gate failed, and the sole execution is spent. A separately preregistered SI0 v2
may repair only the observation model by representing a wave as:

```text
wave
  incidences by junction
  fires caused by those incidences
```

without relying on sequential adjacency in the trace. The runtime candidate
must otherwise remain frozen if that v2 is authorized.

## Frozen artifacts

- `matrix.csv`: `f3dda9153984d51309794fe520755ecc0b1ec3d2564df1e85555dccdaae75e1e`
- `report.md`: `a81e72edf3adc66e83db98aa781c26cfe18fefd7943dc4b3451d57802d60fcb0`

No RS2, CE1, FD2, ARC, authority, oracle, or `arch.md` work ran.
