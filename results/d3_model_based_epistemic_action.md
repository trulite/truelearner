# Discovery d3: Model-based epistemic action

## D3a: Learn what actions change

### Input

D3a receives:

- an opaque action identity,
- eight role occupants before acting,
- the eight role occupants afterward,
- generic identity equality.

It receives no ambiguity task, action usefulness label, or reward.

For every action and role, generic coactivity proposes two persistent arrows:

- action and role to changed,
- action and role to preserved.

Observed equality strengthens the corresponding arrow.

### Results

Across sixteen independently permuted action mappings:

- held-out structural predictions: 6,144 of 6,144,
- confident models from shuffled action/outcome training: zero,
- proposed persistent arrows: 1,536,
- permanent model entries: 1,536,
- opaque identities observed: 196,608,
- permanent identity-specific cells: zero,
- model size remains fixed as identity experience increases,
- held-out prediction leaves the model fingerprint unchanged.

The learned action effects include both changed and preserved roles. Selective,
disruptive, inert, and irrelevant effects remain distinguishable.

## D3b: Choose an experiment before trying it

D3a models are frozen before D3b begins.

Each test supplies two competing route activity graphs. Generic comparison
produces:

- shared roles and arrows,
- roles and arrows present only in the first route,
- roles and arrows present only in the second route.

It does not emit a named distinguishing role.

Every frozen action model predicts changed and preserved roles. The supplied
epistemic preference favors an action that changes route-specific evidence
while preserving shared and competing evidence.

The complete comparison and every action prediction are recorded before one
action executes.

### First-action results

```text
model-based selection   48 / 48
empty action history    11 / 48
random selection        18 / 48
change-everything        0 / 48
```

All sixteen opaque action permutations pass. The selected action is then
executed exactly once on fresh temporary identities. No learning occurs before
or after selection, and every frozen model fingerprint remains unchanged.

The full pre-action traces are stored in
`results/d3_pre_action_traces.csv`.

## Conclusion

The precise result is:

> Frozen role-relative action-effect models transferred to novel competing
> route structures and selected a distinguishing intervention before its
> usefulness had been observed in that ambiguity.

This is model-based epistemic action within the supplied structural prior.

## Limit

D3 supplies:

- role positions,
- generic same/different comparison,
- route set and connection comparison,
- the preference for changing route-specific evidence while preserving shared
  evidence.

It does not show those priors emerging from raw sensory data, and it does not
learn the epistemic preference itself.
