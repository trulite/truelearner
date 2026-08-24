# d0 Topology Discovery

## Question

Can generic local connection proposals and terminal success discover the
renaming-independent routing arrow used by v19, without receiving a fixed list
of candidate routes?

## What Is Supplied

- opaque identity equality,
- permanent cells representing sensory positions and control events,
- episode boundaries and automatic temporary erasure,
- a two-event coactivity window,
- a generic rule that proposes both arrow directions between nearby active
  permanent cells,
- one-arrow competition,
- recently-used connection traces,
- scalar terminal success or failure,
- a fixed consolidation threshold.

The parser does not compare the query with relation identities, create an
answer arrow, or enumerate slot-routing candidates. Terminal feedback contains
no answer identity and does not identify a responsible connection.

## What Happens

Sensory role cells fire while an episode arrives. Permanent cells firing close
together propose directed arrows in both directions. This creates useful and
useless possibilities.

One arrow is tried per training episode. Only that used arrow receives a
temporary trace. Later terminal success strengthens the traced arrow and
terminal failure weakens it. When one arrow repeatedly succeeds, it becomes
the stable route and the remaining provisional arrows are pruned.

Opaque identity cells remain episode-local and never become permanent.

## Results

Topology trajectory for a representative forward learner:

| Episodes | Live provisional arrows | Stable arrows | Held-out accuracy |
| ---: | ---: | ---: | ---: |
| 10 | 18 | 0 | 100/100 |
| 100 | 0 | 1 | 100/100 |
| 1,000 | 0 | 1 | 100/100 |
| 10,000 | 0 | 1 | 100/100 |

The learner proposed 18 arrows, rejected 17, and retained:

```text
Slot1 -> Slot2
```

Eight independent forward runs all reached perfect held-out accuracy using
that route.

The same initial substrate trained on reversed experience retained:

```text
Slot2 -> Slot1
```

Eight independent reverse runs also reached perfect held-out accuracy.

Twenty thousand additional held-out episodes with fresh opaque identities did
not change the permanent fingerprint.

Peak temporary structure for ten relations was 31 cells and 20 arrows.

## Controls

### Random Labels

When the rewarded output was reassigned across presented right-slot
identities, training produced only 85 successes in 10,000 trials and no stable
arrow. The same proposal and reward dynamics therefore do not consolidate a
route when experience contains no consistent routing rule.

### Irrelevant Correlation

Training optionally marked the target relation with an irrelevant cue. Both
the identity-based route and a cue-based shortcut could then predict the
answer.

After removing the cue, 15 of 32 trained learners failed. This is an
intentional negative control:

> d0 discovers whichever locally available route receives reliable success;
> it does not distinguish causal structure from a cheaper correlation.

## Conclusion

d0 supports the narrow claim:

> Generic temporal coactivity generated routing possibilities, and recently
> used connection traces plus scalar terminal feedback selected a compact,
> renaming-independent role route.

This is stronger than v19's supplied four-route selection, but it does not
discover the full temporary-binding machine. Equality, role cells, episode
lifetime, proposal locality, competition, and consolidation remain supplied.
The shortcut result shows that causal discovery remains unsolved.
