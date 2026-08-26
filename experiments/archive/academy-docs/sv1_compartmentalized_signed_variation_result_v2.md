# SV1 compartmentalized signed variation result v2

Status: development positive by exact cumulative evidence reuse.

## Gate A

Accepted cumulative variation now constructs, without evaluator placement:

```text
P -> C+ ->(+1) X
P -> C- ->(-1) X
```

`C+` and `C-` are ordinary CELL junctions. The proposal is symmetric except
for outgoing coupling sign, bounded to two junctions/four links, and contains
no selected identity or sign preference. Static Gate A therefore passes.

## Runtime gates

The frozen CV0/J0 full matrix at `2ed074a` is exactly the runtime matrix that
SV1 v1 left blocked. It already executed every required family:

- positive-only and negative-only local consequence selection;
- identity, slot, physical-position, sign-order, and anchor permutations;
- neither-useful and both-useful controls;
- deliberate shared-contact reproduction of SV0 attribution aliasing;
- bounded nonrecursive contact variation and generation-safe cleanup;
- exact Reference/Production history, replay, and natural quiescence.

Results reused without rerunning the organism:

```text
cases       240/240
rows        480/480
clauses     5480/5480
```

Evidence hashes:

- matrix: `ff1e4631a518c6f56861b5fa21dac74580fae250b36b5cc633ee545dd8c87abc`;
- report: `a4cf0418a6f7503657109e161af8cb166e574f8620609f84c73087c94fce28c4`.

## Claim

Ordinary local variation can generate signed alternatives in distinct ordinary
contact compartments. Consequence selection—not sign, identity, slot, or
proposal ordering—determines which links persist. Specificity remains exactly
as fine as physical junction topology.

No new execution occurred for SV1 because doing so would duplicate the exact
accepted CV0 worlds and comparator.

