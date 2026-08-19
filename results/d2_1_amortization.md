# d2.1 Epistemic-Action Amortization

## Question

When does remembering where useful evidence comes from become cheaper than
searching for it again?

D2 established learned action preference but did not beat random exploration
in one four-choice problem. D2.1 keeps the d2 learner frozen and scales only
the environment and number of independent ambiguity problems.

## State Boundary

Every problem creates:

- a fresh d0 topology learner,
- fresh opaque identities,
- fresh candidate arrows and route strengths,
- a fresh temporary evidence workspace.

After the problem resolves, that entire workspace is consumed and dropped.
The next problem cannot access it.

Only one scalable action policy persists. Its state contains:

- opaque action values,
- which actions have been tried,
- an exploration cursor.

It contains no identities, route references, route strengths, or topology
cells.

The harness created and destroyed 12,000 workspaces:

```text
created:                       12,000
destroyed:                     12,000
maximum live after a problem:       0
problem identities in policy:       0
```

## Environment

Total choices are swept across:

```text
4, 8, 16, 32, 64
```

Each action mapping contains:

- one informative action,
- one disruptive action,
- enough inert actions to fill the requested size,
- one no-action choice.

Action meanings remain fixed across one hundred problems in a run but are
randomly permuted between runs.

Every fresh workspace begins with the true route and shortcut equally
plausible, both one successful use below consolidation. One informative action
is therefore sufficient to resolve the problem.

## Comparisons

- Learned: one persistent action policy across problems.
- Random: search without replacement restarts for every problem.
- Oracle: select the informative action immediately.

Each action-space size uses eight independent mappings and one hundred
problems per mapping.

## Results

| Choices | First learned problem | Mature learned | Mature random | Oracle | Break-even problem |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 4 | 2.0 | 1.0 | 2.5 | 1.0 | 1 |
| 8 | 3.8 | 1.0 | 5.1 | 1.0 | 1 |
| 16 | 10.0 | 1.0 | 8.2 | 1.0 | 2 |
| 32 | 15.6 | 1.0 | 17.9 | 1.0 | 1 |
| 64 | 41.4 | 1.0 | 36.0 | 1.0 | 2 |

Mature cost is measured over the final twenty problems.

Learned mature action cost remains exactly one across the complete sweep.
Random cost rises strongly with the number of available actions.

Average complete problem work:

| Choices | Learned spikes | Random spikes | Learned episodes | Random episodes |
| ---: | ---: | ---: | ---: | ---: |
| 4 | 3,347 | 5,586 | 91 | 152 |
| 8 | 3,361 | 7,670 | 91 | 208 |
| 16 | 3,415 | 10,011 | 93 | 272 |
| 32 | 3,458 | 18,573 | 94 | 504 |
| 64 | 3,684 | 35,548 | 100 | 965 |

All three strategies preserve accuracy:

```text
learned: 4,000/4,000
random:  4,000/4,000
oracle:  4,000/4,000
```

## Interpretation

The first problem can be expensive because the learned policy still has to
locate the informative opaque action. Later problems reuse that knowledge and
select it immediately.

Random exploration receives no cross-problem memory, so it pays the search
cost again for every ambiguity.

D2.1 supports the claim:

> Experience locating useful information was amortized across independent
> ambiguity problems, eventually making learned epistemic action cheaper than
> repeated random search.

This benchmark supplies fresh problem boundaries and keeps action meanings
fixed across problems. It does not test action remapping, continual topology
contexts, or predicting the value of an action that has never been tried.
