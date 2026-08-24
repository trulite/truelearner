# S1.2: Learned Guidance Gate

## Question

Can terminal planning cost teach a cheap structural gate when compiled search
guidance is worth using?

The gate chooses one of two supplied modes before search:

- neutral complete search,
- complete search ordered by frozen S1.1 value.

It receives only the selected mode's correctness and actual paid work. It
never receives the unchosen cost, required plan depth, reachability, or an
oracle mode label.

## Input

The gate uses the already compiled:

- temporary-state signature,
- goal structure,
- search budget.

Initial signature recognition and gate retrieval are charged as work.

## Training

Training contains six role-relative structural contexts:

- reachable depths one, two, three, four, and eight,
- unreachable search with budget eight.

All six opaque action orderings are counterbalanced. Early bandit exploration
tries both modes. Close estimates continue exploring; large economic gaps
consolidate earlier. Exploration and its excess work are fully charged.

## Learned Boundary

```text
Context                 Learned mode   Neutral work   Guided work
Reachable depth 1       neutral                2,820         6,180
Reachable depth 2       neutral               11,280        17,460
Reachable depth 3       neutral               36,660        44,760
Reachable depth 4       neutral              112,800       120,120
Reachable depth 8       guided             9,249,600     8,784,000
Unreachable budget 8    neutral           13,865,040    26,260,080
```

The learner is never given these context names. They are interpretation labels
used only in the report.

## Held-Out Economics

Held-out evaluation contains 360 fresh problems and disables exploration.

```text
Learned gate          22,817,640
Always neutral        23,278,200
Always guided         35,232,600
Random gate           30,056,073
Oracle                21,412,520
Shuffled feedback     35,685,600
Exact memorizer       23,283,240
```

The learned gate saves 460,560 counted work versus always-neutral search.

It matches the per-problem oracle on 300 of 360 cases. Opaque action ordering
is deliberately absent from the gate input, so individual cases sharing one
structural key can have different tie-order economics. The gate correctly
chooses the lower expected-cost mode for all six observable contexts.

## Exploration Economics

```text
Training problems                    2,880
Exploratory choices                    221
Exploration regret                2,909,258
Cumulative break-even problem         1,439
Final learned training work      184,328,726
Always-neutral training work     186,225,600
```

The gate remains ahead for the rest of the recorded training stream after
break-even.

## Controls

- All searches remain complete, shortest, and correct.
- The gate plateaus at six permanent entries.
- Frozen S0 and S1.1 fingerprints do not change.
- Shuffled work feedback does not learn a useful gate.
- Exact identity and action memory does not transfer.

## Interpretation

S1.2 demonstrates learned allocation between two supplied reasoning modes.
It does not learn the compact signature, bandit update, exploration schedule,
near-tie policy, search procedure, or how much computation to buy.

All results use deterministic work accounting rather than measured hardware
time.

