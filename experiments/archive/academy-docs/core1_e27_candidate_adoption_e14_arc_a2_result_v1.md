# CORE1-E27 — Candidate Adoption on Unchanged E14 ARC A2 Result v1

## Status

**UNCHANGED E14 CONTRACT: NEGATIVE.** The complete E24/E25/E22/PQLC/E26/E27
chain repaired the original behavioral frontier and produced teaching actions
`1|4|2|3`, but it failed E14's frozen exact update predicate. The result is not
an adoption pass.

Reference, exact Reference replay, and Production produced identical complete
observations. Natural quiescence held throughout. No repair or rerun occurred.

## Exact observation

All three mechanics executions produced:

```text
actions                 1|4|2|3|none
plasticity updates      0|2|2|2|2
Modulatory deliveries   0|1|1|1|1
E26 re-entry topology   0|1|2|3|4
E27 executable edges    0|2|4|6|8
E22 returns             1|1|1|1|0
USED-PENDING            0|0|0|0|0
physical ticks          4|8|12|16|19
natural quiescence      true
```

The unchanged E14 acceptance requires:

```text
actions                 1|4|2|3
plasticity updates      0|1|1|1|1
natural quiescence      true
```

Actions and quiescence pass. Updates fail exactly because consequence
consolidates both participating halves of the physical route—the
source-to-contact stem and contact-to-motor outgoing edge—yielding two PQLC
updates rather than one.

## What closed

The original E14 negative was:

```text
actions                 none|none|none|none|none
updates                 0|0|0|0|0
Modulatory              0|0|0|0|0
```

The candidate integration now demonstrates on the original full 1,024-context
body and frames:

```text
context
-> complete route formation
-> motor participation
-> outward useful action
-> later consequence return
-> PQLC on both used route halves
-> re-entry topology
-> executable consolidation
```

Thus the **behavioral frontier is repaired**. The candidate reaches every
physical transition the E15-E27 ladder was intended to supply.

## What did not close

The unchanged E14 contract counts one plastic update per consequence. The
learned route discovered by the ladder is explicitly two-arrow topology, so
its local consequence produces two updates. That mismatch was already visible
in E25 and remains exact here.

The preregistered acceptance predicate is not relaxed after evidence.
Accordingly:

- E27 remains earned candidate physics from its positive v2 experiment;
- the complete candidate chain is sufficient for the original E14 action and
  credit behavior;
- candidate-chain adoption is **not** earned under the unchanged E14 contract;
- no E28 mechanism or post-hoc update-count accommodation is introduced.

## Exactness and harness

- original E14 root: `93000000`;
- original contexts: `779|34|980|430|702`;
- full 1,024-context body; no compact fixture;
- original frame, curriculum, consequence, closing, and acceptance semantics;
- Reference replay exact: `true`;
- Reference/Production exact: `true`;
- wall time: `33.16 s` with three full executions concurrent;
- evidence marker emitted once; no rerun.

## Evidence

- `experiments/results/core1_e27_candidate_adoption_e14_arc_a2_v1/matrix.csv`
- `experiments/results/core1_e27_candidate_adoption_e14_arc_a2_v1/report.md`

SHA-256:

- matrix:
  `f80f9d6e4a453a23f6fdf3e0e34bf0ab3dfc3afe8e9ea7961497888fbdd35030`;
- generated report:
  `b7312c1b1f7354ebbe301de8df9490ff3af6f3bdc9fc86e0bc28d137540c82fa`.
