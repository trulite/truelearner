# CORE1-E25 — Motor Integration Solve Tournament Result v1

## Status

**NO FULL-CHAIN SOLVE.** G and W each failed the motor-participation gate at
`0/8`. G+W repaired motor participation at `8/8` and carried later consequence
through E22 into PQLC, but it failed the preregistered full-chain acceptance at
`0/8`: the two-arrow route produced two PQLC updates per consequence rather
than the frozen E14 count of one, and every autonomous probe remained silent.

No arm is earned candidate physics from E25. No repair or rerun occurred.

## Gate 1 — motor participation

All roots, exact Reference replays, and Production mechanics agreed.

| Arm | Motor gate | E22 returns | Decisive local state |
|---|---:|---:|---|
| G | 0/8 | 0 | babbler activation decayed; four tied gates admitted none |
| W | 0/8 | 8 | babbler activation persisted; `+1/-1` contact incidence still summed to zero |
| G+W | 8/8 | 1 | one active opportunity admitted `+1`; three inactive tied gates admitted none |

The intended G+W motor trace was exact at every root:

```text
tick 1  direct +1/-1 incidence       net 0   activation 0
tick 1  babbler incidence            +1      activation 1
tick 2  gated contact incidence      +1      activation 2
        motor fires                  action 1
```

This is positive evidence for the narrow composition claim:

> Local signed admission plus within-wave motor persistence is sufficient to
> turn the frozen E24 complete routes into an outward action.

It is not sufficient evidence for a learned autonomous policy.

## Gates 2 and 3 — credit and autonomy

Only G+W advanced. Every root produced the same full row:

```text
teaching actions       1|4|2|3
Modulatory             0|1|1|1|1
PQLC updates           0|2|2|2|2
temporary returns      1|1|1|1|0
passive USED-PENDING   0|0|0|0|0
autonomous probes      none|none|none|none
natural quiescence     true throughout
```

Thus the physical causal chain now reaches farther than E24:

```text
atomic route formation                  yes
motor-local G+W participation           yes
outward teaching action                 yes
participation-born E22 return            yes
later Modulatory consequence             yes
PQLC updates on the used two-arrow route yes (2)
autonomous re-expression                 no
```

The update-count mismatch is real under the frozen E14 predicate: modulation
at the admitted contact updates both participating halves of the physical
route, the source→contact stem and contact→motor outgoing. But it is not the
decisive capability failure. Even if the count predicate were relaxed from one
to nonzero, autonomous behavior is `0/8`.

## Interpretation

The preregistered tournament interpretation is
`ALL_FAIL_COMBINED_HYPOTHESIS_WRONG`: neither component nor their composition
solves the complete E14 frontier.

The result also cleanly separates the next boundary:

```text
context -> participation -> action -> consequence -> credit     works
credited physical route -> later autonomous action              fails
```

E25 therefore rejects further motor-integration work as the immediate next
move. Any successor must state a new physical hypothesis about why an actually
credited two-arrow route does not later re-express. E25 itself earns no repair,
parameter change, altered acceptance predicate, or post-hoc adoption.

## Exactness and controls

- G Gate 1: `0/8`, exact replay and mechanics throughout;
- W Gate 1: `0/8`, exact replay and mechanics throughout;
- G+W Gate 1: `8/8`, exact replay and mechanics throughout;
- G+W full chain: `0/8`, exact replay and mechanics throughout;
- G stronger-negative, W stagger/cancel/clear, E18 in-flight, checkpoint, and
  Academy blocked-return focused controls passed before evidence;
- frozen E14, E16, E22, and E24 evaluators remained byte-identical;
- evidence marker emitted once; no rerun or post-evidence repair.

## Evidence

- `experiments/results/core1_e25_motor_integration_solve_tournament_v1/preflight.csv`
- `experiments/results/core1_e25_motor_integration_solve_tournament_v1/full.csv`
- `experiments/results/core1_e25_motor_integration_solve_tournament_v1/report.md`

SHA-256:

- preflight:
  `385ab6958ccbdde1c39567c5b16bc5fa2ec3493f77bc9617b261723dfce3ab0a`;
- full matrix:
  `e2758ba09bdcc09c98403737a7f508b7599691a9ef14fbc2f7140b0c09b1eace`;
- generated report:
  `cf4ff27583dadb119bd0f6f985bfcf75c14e8a862b97d581540efaba7c54b7f3`.
