# Discovery d2.2: Silent action remapping

## Question

Can the frozen d2.1 action policy revise a once-correct opaque action mapping
after the environment silently changes?

No forgetting, decay, reopening, or other plasticity mechanism was added.

## Setup

The policy first matures for 10, 50, or 100 independent ambiguity problems.
The informative action is then silently exchanged with either:

- an inert action that the policy has never tried, or
- an inert action that the policy previously tried and weakened.

The mature policy and a fresh zero-history policy then receive the same new
mapping, problem seeds, action costs, and maximum of 500 problems. Adaptation
requires five consecutive correct problems using exactly one paid action, with
the new informative action preferred.

Every problem uses a fresh topology workspace. The workspace is consumed and
destroyed before the next problem.

## Results

All values are averages across two deterministic action permutations.

```text
choices maturity replacement  mature adapts  fresh adapts  mature problems/cost  fresh problems/cost
16       10       untried      2/2            2/2           7.0 / 36.5            6.0 / 19.5
16       50       untried      2/2            2/2          12.0 / 116.5           6.0 / 19.5
16      100       untried      2/2            2/2          19.0 / 216.5           6.0 / 19.5
64       10       untried      2/2            2/2           7.5 / 85.0            7.0 / 68.0
64       50       untried      2/2            2/2           9.0 / 165.0           7.0 / 68.0
64      100       untried      2/2            2/2          10.0 / 265.0           7.0 / 68.0

16/64   10/50/100 rejected     0/12           12/12         no adaptation          5.0 / 5.0
```

The obsolete action begins with value 20, 100, or 200 as maturity increases.
For an untried replacement, the old preference eventually falls and
exploration resumes, but adaptation cost rises with prior evidence.

For a previously rejected replacement, the old preference eventually falls,
but the new informative action is never reconsidered. No mature run adapts
within 500 problems. Every matched fresh policy adapts.

Across the full experiment:

- 24 of 24 mature comparisons are slower or fail relative to scratch.
- 12 of 12 previously rejected remaps fail to adapt.
- 7,547 workspaces are created and all 7,547 are destroyed.
- No workspace remains live between problems.

The complete per-problem action and value history is in
`results/d2_2_remap.csv`.

## Conclusion

The existing policy has limited implicit plasticity:

- Unknown alternatives can eventually be explored.
- Revision becomes more expensive as old positive evidence accumulates.
- Previously rejected alternatives are not reopened.

The precise result is:

> Stable reusable action knowledge has become rigidity. The current learner
> can exhaust an obsolete positive preference, but cannot overturn old
> negative evidence once no-action becomes preferable.

This diagnoses the missing timescale mechanism. It does not introduce or test
a solution.
