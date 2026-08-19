# P2.1: Selective structural plasticity

## Question

Can experience learn which kinds of local coactivity are worth spending growth
slots on, reducing P2's probationary topology churn without supplying the
desired program?

## Boundary

P2.1 does not expose exact sensory-role cells or internal event roles to the
plasticity-value learner. Before a coupling exists, it sees only whether each
endpoint is:

- sensory workspace activity,
- internal workspace activity, or
- active irrelevant activity.

This produces six possible unordered pair signatures. The gate decides only
whether the pair becomes temporarily plastic. An admitted pair still opens
both directions with identical probation. Ordinary P2 activity, eligibility,
terminal correctness and pruning determine direction and survival.

## P2.1a: Learn plasticity value

Four completed P2 discovery runs provide post-hoc experience. A pair category
is useful when a coupling from that category survives in the completed
program. The frozen six-entry model is then evaluated on eight separate
always-plastic runs:

```text
permanent value entries                 6
categories predicted useful             2
held-out useful categories          16 / 16
held-out rejected categories        32 / 32
shuffled predictor useful recall      0 / 16
forward lookup transfer               8 / 8
reverse lookup transfer               8 / 8
```

The model therefore learns broad regions where structural plasticity has
historically paid. It does not identify a specific desired coupling.

## P2.1b: Gate local plasticity

The predictor is frozen before eight fresh integrated learners start. A small
generic exploration chance remains available for low-valued encounters.

```text
                              always P2    learned gate
competent seeds                    8 / 8           8 / 8
held-out depth 5-32              512 / 512       512 / 512
average competence episode        15103.6          2434.5
directional couplings created       696314           11817
directional couplings released      696310           11813
gate admissions / evaluations       399297      7872 / 83504
deterministic discovery work        1582909          239025
```

The learned gate reduces coupling creation by 98.3 percent and the recorded
deterministic discovery work by 84.9 percent.

Work counts one unit for each active-cell touch, local encounter, directional
coupling creation and eligibility update. Gated conditions additionally count
one unit for every gate evaluation. These are implementation-independent
operation counts, not hardware timing.

## Controls

```text
condition                  competent seeds    couplings created
random, matched rate             6 / 8              150280
shuffled value model             0 / 8              533999
exact-pair oracle                8 / 8                  18
```

The random gate uses the learned gate's per-opportunity admission rate, but
must continue searching longer when it misses useful encounters. The oracle
is an evaluation-only upper bound that receives the exact useful pairs.

## Supported claim

> Experience learned a coarse, reusable estimate of where local structural
> plasticity was likely to pay, reducing probationary coupling churn while
> preserving discovery of the recurrent program.

## Remaining supplied structure

- Sensory, internal-workspace and irrelevant activity classes
- The six-entry pair-signature representation
- The gate lookup and low-rate exploration rule
- Local geometry and task-neutral workspace activation
- Coactivity timing, growth slots and probation
- Direction-specific eligibility and terminal correctness
- Strengthening, weakening and pruning
- Internal execution roles and event meanings
- Identity equality and temporary episode lifetime

P2.1 does not learn the plasticity law. It also does not yet transfer a
plasticity-value model across substantially different program families. The
gate learns a broad admission bias inside the existing P2 computational
algebra.
