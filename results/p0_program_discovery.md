# P0 de-supply: program discovery

## Question

Can generic topology proposal, probation, recently-used traces, terminal
correctness, and pruning reconstruct the reusable v19-v21 program without a
supplied list of route candidates?

## Frozen learning physics

Every experiment uses the same configuration:

- ten permanent role cells,
- generic directed proposals between every coactive pair,
- eight episodes of probation before a repeatedly failing used arrow can be
  pruned,
- successful used arrows gain two strength units,
- unsuccessful used arrows lose one strength unit,
- a unique arrow consolidates at strength six,
- only arrows that actually carried queued activity receive terminal credit,
- an activity limit of 1,600 events.

No route-specific proposal list is present. The initial generic proposal sweep
creates 90 directed arrows, including the four useful routes and 86 irrelevant
routes.

## Isolated feasibility gates

Each gate learns one route while the other execution behavior remains fixed.
The isolated gates establish the shared learning configuration; their learned
arrows are never transferred into P0e.

| Gate | Successful seeds | Average episodes | Average proposals | Final arrows |
| --- | ---: | ---: | ---: | ---: |
| P0a temporary lookup | 8/8 | 7.8 | 91 | 1 |
| P0b result feedback | 8/8 | 11.1 | 92 | 1 |
| P0c self-trigger | 8/8 | 14.0 | 94 | 1 |
| P0d explicit finish | 8/8 | 10.2 | 92 | 1 |

The reversed lookup control discovers the opposite direction. Random terminal
feedback does not reliably consolidate the required route.

## Fresh integrated P0e

Every P0e seed starts with a fresh substrate and no routes inherited from the
isolated gates. Training presents the complete task from the first episode:

```text
relations
query
one start event
terminal correctness only
```

Training chains have depths one through four. There is no lookup, feedback,
continuation, or finish phase in the curriculum.

| Feedback condition | Competent seeds | First actual success | Final arrows | Held-out |
| --- | ---: | ---: | ---: | ---: |
| Real correctness | 8/8 | 8/8 | 4.0 | 512/512 |
| One-episode shuffled | 0/8 | 8/8 | 88.6 | 0/512 |
| Random | 0/8 | 8/8 | 89.8 | 0/512 |
| Activity only | 1/8 | 1/8 | 65.2 | 64/512 |

Real-feedback learners first produce a correct terminal outcome after 256
episodes on average. Before that first success they generate 340 proposals and
47,635 spikes on average. Stable competence follows about 6,671 episodes
later.

The activity-only condition accidentally constructs one correct program in
eight seeds. This is recorded as chance topology, not reliable
outcome-directed discovery. Real correctness produces compact functional
programs in all eight seeds.

## Representative topology trajectory

| Episode | Live candidates | Stable arrows | Lookup | Feedback | Continue | Finish |
| ---: | ---: | ---: | --- | --- | --- | --- |
| 1 | 90 | 0 | no | no | no | no |
| 10 | 88 | 0 | no | no | no | no |
| 100 | 89 | 0 | no | no | no | no |
| 1,000 | 88 | 0 | no | no | no | no |
| 1,153 | 90 | 0 | yes | no | no | no |
| 3,742 | 4 | 4 | yes | yes | yes | yes |
| 10,000 | 4 | 4 | yes | yes | yes | yes |
| 50,000 | 4 | 4 | yes | yes | yes | yes |

This order is observed, not taught. Other seeds may follow different transient
paths.

## Held-out execution

After training, permanent fingerprints are frozen. Evaluation uses fresh
opaque identities and unseen chain depths five, eight, sixteen, and
thirty-two.

Every real-feedback learner:

- receives one external start event,
- traverses autonomously,
- emits an explicit final answer,
- empties its event queue naturally,
- retains exactly four learned program arrows,
- leaves its permanent fingerprint unchanged.

## Interpretation

The supported claim is:

> Generic topology discovery constructed a reusable recurrent program from
> end-to-end task experience without supplied route candidates or staged
> decomposition.

The experiment still supplies role cells, opaque identity equality, relation
slots, temporary lifetime, event meanings, the generic proposal mechanism,
probation, eligibility traces, terminal correctness, and pruning. It does not
show programs emerging from raw sensory data.

