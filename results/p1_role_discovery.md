# P1: Identity-independent sensory roles

## Question

Can repeated local sensor structure create reusable positional
representations, and can generic topology discovery build the recurrent P0
program over those learned structures?

## What comes in

- Anonymous receptor IDs
- Opaque identity activity
- Directed local sensor connections
- Temporary episode lifetime
- Terminal correctness during training

The learner is not given first-field, second-field or query role labels.

## P1a: Positional representation

Training and evaluation use entirely different receptor IDs. Evaluation also
changes node and relation serialization order while preserving the same local
geometry.

```text
successful seeds                 8 / 8
transferred encodings          256 / 256
learned permanent role cells             3
permanent receptor-specific cells        0
permanent fingerprint changed        false
```

In the impossible control, both relation fields have identical local
geometry. They activate the same learned structure; the learner does not
invent a first/second distinction.

This demonstrates identity-independent positional representation, not
semantic role understanding.

## P1b: Lookup over learned roles

The lookup learner receives only:

- Learned role-cell activity
- Opaque identities
- Spikes

It receives no receptor ID, field index, parser offset or serialization
position.

```text
forward lookup seeds             8 / 8
reverse lookup seeds             8 / 8
encoding-transfer answers      512 / 512
generic proposals / used / kept    6 / 3 / 1
```

The same frozen P1a representation supports opposite learned task use. Random
terminal feedback does not produce the expected stable lookup.

## P1c: Fresh integrated discovery

Every seed starts with:

- No inherited P1a role cells
- No inherited P1b lookup
- No P0 program arrows
- No staged curriculum

The full traversal task is present from the first episode.

```text
competent seeds                   8 / 8
held-out answers                512 / 512
average first success episode      351.0
average competence episode        7523.8
permanent role cells                   3
surviving program arrows               4

possible global proposals            110
actual proposals                      110
proposals that carried activity       36
surviving program topology              4
```

Shuffled and random terminal feedback each produce zero competent learners
across eight seeds.

Held-out testing uses fresh identities, unseen depths five, eight, sixteen and
thirty-two, new receptor IDs and transferred serialization. Every successful
run emits an explicit answer, settles naturally and leaves the combined
permanent fingerprint unchanged.

## Interpretation

The supported claim is:

> Repeated positional structure acquired reusable functional roles, and
> generic topology discovery built a recurrent program over those learned
> roles.

## Remaining supplied structure

- Directed local sensor geometry
- Generic structural comparison and recurrence counting
- Separation of an isolated query occurrence from relation components
- Internal current, result and control-event roles
- Opaque identity equality
- Temporary episode lifetime
- Global all-pairs proposal
- Terminal correctness
- Strengthening, consolidation and pruning

P1 does not demonstrate sparse local topology growth or discovery of internal
execution roles.
