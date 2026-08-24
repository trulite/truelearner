# P2: Local structural plasticity

## Question

Can reusable sensory representation and recurrent computation grow from
local activity without globally considering every possible connection?

## Supplied physical mechanism

When two nearby cells are active within the same short window, each direction
receives an independent probationary coupling:

```text
A -?-> B
A <-?- B
```

Each cell has eight growth slots in the primary experiment. A coupling that
actually carries activity enters a bounded episode-local eligibility queue.
Terminal correctness updates only entries in that queue. Unsupported
couplings release their slots.

There is no scan over dormant cells or all possible cell pairs.

The three learned sensory cells and eight supplied internal execution cells
form the active workspace for an episode. They receive task-neutral workspace
activation before randomized local placement. This activation is supplied;
P2 does not discover which internal cells should enter the workspace.

## P2a: Local lookup discovery

```text
forward lookup seeds              8 / 8
reverse lookup seeds              8 / 8
random-feedback stable routes     0
average couplings created       194
average couplings ever used       2
surviving lookup coupling          1
```

The local encounter opens both directions with identical opportunity.
Experience selects the surviving direction.

## P2b: Sensory representation

The unchanged P1 sensory learner still acquires three identity-independent
positional structures:

```text
successful seeds                  8 / 8
transferred encodings           256 / 256
permanent role cells                    3
permanent receptor IDs                  0
```

P2 does not remove P1's supplied structural comparison mechanism.

## P2c: Fresh integrated discovery

Each seed starts with no inherited sensory roles or program couplings.

```text
competent seeds                   8 / 8
held-out depth 5-32             512 / 512
average permanent role cells          3.0
average program couplings             4.0
average first success episode        96.6
average competence episode        11558.1

average probationary couplings created   532966
average couplings released               532962
average couplings ever used               19423
peak live probationary couplings            152
average eligibility updates                19761
eligibility queue evictions                     0
```

Shuffled and random terminal feedback each produce zero competent learners
across eight seeds.

The lifetime creation count is high, but the live topology stays bounded by
local growth slots.

## Growth-slot diagnostic

```text
slots per cell             1      2      4      8
successful lookup seeds   0/8    8/8    8/8    8/8
```

One slot does not provide enough simultaneous opportunity for this learner to
bootstrap. Two slots are sufficient for the isolated lookup task. This is a
diagnostic rather than the primary P2 pass condition.

## Dormant substrate scaling

The active task is fixed while total available capacity increases:

```text
total cells              10      100      1,000      10,000
active touches         1,216    1,216      1,216       1,216
local encounters       2,432    2,432      2,432       2,432
dormant touches            0        0          0           0
held-out accuracy       16/16    16/16      16/16       16/16
```

A global all-pairs opportunity mechanism would grow from 90 possible
directions at ten cells to 99,990,000 at ten thousand cells.

## Active irrelevant scaling

Active distractors occupy real local neighborhoods and therefore cost work:

```text
active distractors          0       10       100       1,000
active touches            704    1,344     7,104      64,704
local encounters        1,408    2,688    14,208     129,408
held-out accuracy       16/16    16/16     16/16       16/16
```

The measured distinction is:

- Dormant cells: zero touches and zero discovery cost.
- Active irrelevant cells: activity and local plasticity cost.
- Active relevant cells: the same cost, with useful couplings able to survive.

## Supported claim

> Local structural plasticity grew reusable representation and recurrent
> computation from experienced activity, with discovery cost governed by
> active neighborhoods rather than total available substrate.

## Remaining supplied structure

- Directed local sensor geometry
- Short coactivity window
- Randomized bounded-degree local placement
- Task-neutral activation of the current sensory and internal workspace
- Fixed growth-slot count
- Probation duration
- Direction-specific eligibility traces
- Terminal correctness reaching recent traces
- Strengthening, weakening and pruning
- P1 structural comparison and cell recruitment
- Isolated-query boundary
- Internal execution roles and event meanings
- Opaque identity equality
- Temporary episode lifetime

P2 does not learn its plasticity law. It also does not yet demonstrate local
credit across physically distributed queues or hardware execution.
