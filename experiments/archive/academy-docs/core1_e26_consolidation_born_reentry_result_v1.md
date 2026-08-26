# CORE1-E26 — Consolidation-Born Re-entry Result v1

## Status

**FALSIFIED AFTER CREATION.** The candidate passed its preregistered creation
gate at `8/8`: successful PQLC consolidation atomically created exactly one
local source-to-contact re-entry edge after the second teaching consequence.
It then failed the full autonomous solve at `0/8`. All later probes remained
silent despite four learned re-entry edges being physically present.

Reference, exact Reference replay, and Production agreed throughout. No repair
or rerun occurred.

## Creation gate

Every root and mechanics execution produced the same row:

```text
teaching actions       1|4
Modulatory             0|1
PQLC updates           0|2
re-entry topology      0|1
E22 returns            1|1
passive USED-PENDING   0|0
natural quiescence     true
```

This is positive evidence for the narrow construction claim:

> A successful complete-route consolidation can leave a minimal durable
> physical edge by which the participating source can later reach the
> consolidated contact.

The edge was absent after the first action, appeared only with later
consequence/PQLC, and was exact across all frozen roots and mechanics.

## Full-chain result

Every root produced the same full row:

```text
teaching actions       1|4|2|3
Modulatory             0|1|1|1|1
PQLC updates           0|2|2|2|2
re-entry topology      0|1|2|3|4
E22 returns            1|1|1|1|0
passive USED-PENDING   0|0|0|0|0
autonomous probes      none|none|none|none
natural quiescence     true throughout
```

The preregistered decisive prediction therefore split cleanly:

```text
PQLC updates > 0       yes
future re-entry edges  yes
autonomous useful act  no
```

E26 changed the learned physical topology exactly as proposed, but that
topology was not sufficient to turn a later frozen E14 context encounter into
an action.

## Interpretation

E26 rejects **consolidation-born source-to-contact re-entry alone** as the
missing complete solve. It does not reopen consequence timing, E22 return, or
PQLC: those transitions again completed exactly and uniformly.

The result advances the boundary from E25 without closing it:

```text
teaching causal chain                         works
successful PQLC creates local re-entry edge   works
four such edges persist                       works
later context expresses learned action        fails
```

The experiment does not distinguish whether later source activation fails to
traverse the new edge, whether reached contact state fails to propagate along
the consolidated outgoing half, or whether motor expression still depends on
teaching-only opportunity. Any successor must state and solve one such
downstream physical hypothesis; E26 itself earns no additional mechanism.

## Exactness and controls

- creation gate: `8/8`;
- full autonomous solve: `0/8`;
- exact Reference replay and Reference/Production mechanics: `true`;
- all eight roots: `93000000..93000007`;
- focused candidate controls, E25 G+W controls, checkpoint pending-return
  control, and Academy blocked-return control passed before evidence;
- E14, E16, E22, E24, and E25 evaluators remained byte-identical to the frozen
  E25 result;
- evidence marker emitted once; no rerun or post-evidence repair.

## Evidence

- `experiments/results/core1_e26_consolidation_born_reentry_v1/creation_gate.csv`
- `experiments/results/core1_e26_consolidation_born_reentry_v1/full.csv`
- `experiments/results/core1_e26_consolidation_born_reentry_v1/report.md`

SHA-256:

- creation gate:
  `65a4e8adf30903a35c1e7cf3f46beffb8b8ed4ad48e2ae20ea11acf64718f22e`;
- full matrix:
  `d4c5ad76a82d9a620b2bf0a81a3a014d6e973ceb8a3c68a61604546f9d1e59ac`;
- generated report:
  `1fb635abc662bb72e4f4964eee7102cb011e8aef82b18feb01190de85cb22273`.
