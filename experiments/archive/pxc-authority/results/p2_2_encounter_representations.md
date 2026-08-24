# P2.2: Learned encounter representations

## Question

Can generic pre-coupling experience form reusable encounter representations
that learn where local structural plasticity should be allocated, without six
supplied encounter classes?

## What comes in?

Before a coupling exists, each endpoint exposes only generic local facts:

- whether it persists across episodes;
- whether its recent activity arrived from the environment;
- whether its recent activity arrived through the internal event queue;
- whether it is temporary activity.

The immutable snapshot contains no role ID, receptor ID, desired arrow,
terminal outcome or later survival status.

## What happens?

Recurring snapshots recruit encounter-representation cells. A separate value
record learns whether plasticity associated with each representation has ever
participated in successful computation.

During fresh end-to-end learning:

1. Plasticity is initially broad because no representation has useful
   evidence.
2. Probationary couplings produce delayed useful or rejected outcomes.
3. Those outcomes update the representations active when the coupling opened.
4. Once useful representations emerge, the gate admits them directly and
   admits rejected representations only through low-rate exploration.
5. Ordinary P2 activity, directional traces and terminal correctness still
   determine the exact surviving program.

## P2.2a: Representation and value

```text
learned encounter representations       6
representations assigned useful value   2
representations with mixed outcomes     2
context-sensitive representations       1
useful context recall                11 / 11
rejected context recall              70 / 78
shuffled useful recall                 0 / 11
```

The two mixed representations each contain both useful and rejected exact
pairs. One representation also produces different value predictions in
different surrounding temporary contexts. This prevents interpreting the
representation itself as a hidden `GOOD` or `BAD` label.

Contextual negative evidence is not allowed to permanently suppress a
representation with established useful history. It influences the learned
value, while admission uses a conservative positive backoff plus exploration.
This prevents sparse early context evidence from starving later discovery.

## P2.2b: Frozen gate

```text
                                  always P2    learned representation gate
competent seeds                        8 / 8                         8 / 8
held-out depth 5-32                  512 / 512                     512 / 512
couplings created                      696314                          7509
counted discovery work                1582909                        291093
```

Coupling creation falls by 98.9 percent and counted work by 81.6 percent.
Counted work includes encounter-representation comparisons and value
consultation.

Controls:

```text
random admission gate                 6 / 8 competent
shuffled representation value         0 / 8 competent
exact-pair oracle                      8 / 8 competent
```

## P2.2c: Fresh lifetime learning

Every seed starts without encounter representations, plasticity values,
sensory roles or program couplings.

```text
competent seeds                        8 / 8
held-out depth 5-32                  512 / 512
average learned representations          6.0
average positively valued                 1.9
average first useful evidence episode   420.9
average competence episode             2200.5

couplings created                       28059
counted discovery work                 523458
representation comparison work         276640
delayed value updates                    16894
```

Relative to always-plastic P2, coupling creation falls by 96.0 percent and
counted work by 66.9 percent.

The lifetime trajectory changes:

```text
early coupling creation per episode    46.24
later coupling creation per episode     4.83
```

The generic exploration floor admitted an average of 13 otherwise rejected
encounters. No additional useful representation happened to be discovered
through those late exploratory admissions in this task.

Held-out evaluation freezes the representation learner, value records,
plasticity gate, sensory roles and recurrent program. All fingerprints remain
unchanged.

## Runtime accounting

Cached compilation is not the bottleneck:

```text
cached test compilation                 below 1 second
focused optimized P2.2 experiment      about 47 seconds
complete optimized historical report   about 250 seconds
```

Most runtime comes from executing full 50,000-episode negative controls that
never reach competence. Development therefore uses exact focused tests and the
standalone P2.2 executable; the complete historical regression is run only
after the experiment is frozen.

## Supported claim

> Generic local experience formed reusable pre-coupling encounter
> representations that learned to control structural plasticity without
> supplied encounter classes.

## Remaining supplied structure

- The generic endpoint facts listed above
- Exact matching for recurring pre-coupling snapshots
- The separation between encounter representation and plasticity value
- Exploration and confidence behavior
- Local sensor geometry and task-neutral workspace activation
- Growth slots, probation and the local structural-plasticity law
- Directional eligibility traces and terminal correctness
- P1 sensory cell recruitment
- Internal execution roles and event meanings
- Opaque identity equality and temporary episode lifetime

P2.2 does not learn its plasticity law and has not demonstrated transfer of the
encounter representations across substantially different program families.
