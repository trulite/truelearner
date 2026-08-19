# S0/S1: Learned Search Value and Lazy Supplied Search

## Question

Can completed searches teach a reusable estimate of which predicted partial
states are promising, and can that estimate reduce later search work without
changing the supplied complete search algorithm?

## S0: Learned Value

The learner receives:

- a canonical equality pattern for the predicted temporary state,
- the two competing route structures,
- the remaining search budget,
- one coarse outcome: high, medium, or low promise.

The private training harness may determine the minimum remaining depth, but
never exposes a successful action sequence.

The same state can receive different values for different goals. Completely
renamed identities produce the same value key. Opaque action identities never
enter the value representation.

## S1: Lazy Supplied Search

For each possible plan length, a supplied complete search maintains a lazy
frontier of partial sequences. Frozen S0 values order that frontier. The
search exhausts every unsuccessful length and stops at the first solution in
the first successful length.

The learned heuristic changes ordering only. It cannot prune candidates,
skip a length, or weaken the shortest-plan guarantee.

## Experience Curve

```text
Training       Heuristic     Held-out partial     Model         Total
examples       entries       states expanded      applications  work
0                   0             80,102              80,112     2,699,656
32                 14             75,454              75,528     2,545,072
128                32             66,398              66,504     2,241,136
512                58             40,158              40,272     1,357,960
2,048              81             40,158              40,272     1,357,960
8,192              90             40,158              40,272     1,357,960
32,768             92             40,158              40,272     1,357,960
65,536             92             40,158              40,272     1,357,960
```

Search performance plateaus after 512 examples. Permanent heuristic size
eventually plateaus at 92 entries.

## Ordering Results

Average partial states expanded per problem:

```text
Required    Exhaustive   Learned   Random    Oracle   Shuffled   Path memory
depth
1                2.8        2.8       3.0       2.8        3.0          2.8
2               11.8        7.8      11.0       7.8       16.0         11.8
3               38.8       21.8      43.6      21.8       56.0         38.8
4              119.8       62.8     125.2      62.8      177.0        119.8
8            9,839.8    4,924.8   8,548.8   4,924.8    8,767.0      9,839.8
```

The mature heuristic matches oracle ordering at every tested depth. The
exact-path memorizer provides no advantage under fresh opaque action
permutations.

All plans remain shortest, all real executions match prediction, and all
unreachable cases are reported.

## Full Cost Accounting

For reachable problems:

```text
                            Exhaustive    Learned ordering
Partial states expanded        80,102          40,158
Model applications             80,112          40,272
Total counted work          1,255,048       1,357,960
```

For unreachable problems:

```text
Partial states expanded       118,048         118,048
Total counted work          1,848,672       3,973,536
```

The heuristic halves model expansion on reachable problems, but evaluating it
at every partial state costs more than the saved model work. It cannot reduce
complete search for unreachable problems and therefore adds pure overhead
there.

## Interpretation

S0 succeeds:

> Search experience produced a compact, identity-independent,
> goal-conditioned estimate of partial-state promise.

S1 succeeds as an ordering capability:

> Frozen learned value transferred to fresh problems, matched oracle ordering,
> and reduced model expansion while preserving complete shortest-plan search.

The economic hypothesis does not pass:

> After charging for heuristic evaluation, total planning work is higher than
> neutral exhaustive search.

This is not yet an efficient learned search system. The next bottleneck is
making value evaluation cheaper or using value without paying for it at every
partial state.

Raw measurements are in [`s0_s1_search_value.csv`](s0_s1_search_value.csv).
