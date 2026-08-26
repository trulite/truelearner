# CORE1-E26 — Consolidation-Born Re-entry Protocol v1

## Status

Preregistered after frozen E25 result `23cf456` and before E26 implementation
or ARC runtime observation. E26 tests one candidate physical solve. It is not a
localization audit or a tournament.

## Frozen boundary

E25 G+W established at `8/8`:

```text
context -> route participation -> motor action              yes
later consequence -> E22 return -> Modulatory -> PQLC       yes
later context -> autonomous useful action                    no
```

Every root produced teaching actions `[1,4,2,3]`, Modulatory
`[0,1,1,1,1]`, two PQLC updates on each used two-arrow route, and autonomous
probes `[none,none,none,none]`.

E26 freezes E24 atomic route closure, E25 G+W, E22 atomic
participation-born credit return, and existing PQLC unchanged.

## Hypothesis

> PQLC consolidation changes route strength, but does not make the
> strengthened route physically re-enterable from its source on a later
> encounter.

Learning and later physical invocation are separate transitions.

## Sole candidate

Under one default-off CORE1 flag, successful PQLC consolidation at a contact
may create one durable local Drive re-entry edge.

Creation requires, in the same local consolidation event:

- a live participating incoming Drive stem `source -> contact`;
- source and contact at the same local position, preserving the existing
  subdivision relation;
- a live participating outgoing Drive route `contact -> target`;
- actual Modulatory return at that contact and at least one ordinary PQLC
  update there.

Then atomically:

```text
PQLC consolidates complete used contact route
-> create durable source -> contact re-entry edge
```

The edge uses ordinary SourceFires Drive transmission, unit positive material,
the existing one-tick stem delay, and physically maximal resistance. At most
one live E26 edge may exist for a source/contact endpoint pair.

The edge contains no motor endpoint, action, context label, episode, result,
reward, sign choice, path, expected consequence, counter, timeout, or future
input. Its endpoints are supplied by the participating local stem. The
participating outgoing route is only the physical evidence that a complete
route was consolidated; it is not copied into the new edge.

Later source activation must still fire the learned contact and traverse the
already-existing strengthened contact→motor route. E26 may not create a direct
source→motor shortcut, emit an action during consolidation, replay a spike,
alter a threshold, change coupling on any existing arrow, protect an outgoing
route, modify PQLC, or change G, W, E22, route formation, consequence timing,
or evaluator inputs.

## Frozen fixture

- roots `93_000_000..=93_000_007`;
- five deterministic E14 frames;
- action map `[1,2,3,4]`;
- teaching curriculum `[1,4,2,3]` and unchanged support timing;
- E24 atomic route closure enabled;
- E25 local signed gating and motor integration window enabled;
- E22 atomic credit return remains candidate-default;
- Reference, exact Reference replay, and Production equality.

Frozen E25 is cited and not rerun as the negative control.

## Gate 1 — creation is consequence-born

For each root, run the first teaching action and require:

- outward action `1`;
- one E22 return path;
- zero E26 re-entry edges before consequence.

Run the second frozen teaching observation, which admits consequence for the
first action, and require:

- positive Modulatory delivery and PQLC updates;
- exactly one E26 re-entry edge now exists for the first consolidated route;
- the next teaching action is `4`;
- passive USED-PENDING remains zero;
- exact replay, mechanics equality, bounded work, and natural quiescence.

If no edge is born, or an edge appears before consequence/PQLC, stop without
the full regimen.

## Gate 2 — full teaching chain

Only if Gate 1 passes `8/8`, run the complete frozen E25 G+W regimen. Require:

- teaching actions `[1,4,2,3]`;
- later Modulatory delivery and nonzero PQLC updates;
- one new re-entry edge per successfully consolidated taught route;
- four live E26 re-entry edges after the closing consequence;
- temporary E22 returns clear and passive USED-PENDING remains zero;
- natural quiescence, bounded work, exact replay, and mechanics equality.

The E25 update count of two is recorded but is not changed by E26. Gate 2
requires nonzero existing PQLC, not a new update magnitude.

## Gate 3 — autonomous expression

Clone and ordinarily recover the learned organism. Probe the four taught
frames once each with no babble and no support.

E26 succeeds only if every root autonomously expresses `[1,4,2,3]` while the
re-entry edges remain local source→contact topology and all runs quiesce.

The decisive comparison is frozen:

```text
E25 G+W:       PQLC > 0; autonomous probes 0/8
E25 G+W + E26: PQLC > 0; autonomous probes must be 8/8
```

## Controls and rejection screen

- candidate defaults off and changes no frozen evaluator;
- first action without consequence creates zero re-entry edges;
- Modulatory incidence at a contact without both participating route halves
  creates no edge;
- a participating stem without a participating outgoing creates no edge;
- a participating outgoing without a participating stem creates no edge;
- repeated consequence cannot duplicate the same source/contact edge;
- the edge targets the contact, never a motor or output;
- the edge does not itself create E22 return topology;
- no timer, stored spike, scheduled replay, action identity, or evaluator
  route is permitted;
- reject exactness mismatch, non-quiescence, unbounded work, changed teaching
  actions, PQLC modification, or autonomy caused by a direct motor shortcut.

## Evidence discipline

Provide a non-executing check mode, focused core controls, strict release
Clippy for the E26 target, formatting/diff audits, and a static implementation
audit. Commit implementation before emitting the one-shot evidence marker.
Run the staged eight-root evaluator once. Preserve per-root evidence and stop
at the first failed gate without repair or rerun.
