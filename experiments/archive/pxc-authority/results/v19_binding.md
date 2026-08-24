# v19 Temporary Binding

**Recorded:** 2026-08-18

## Question

Can permanent learned structure route between arbitrary identities using only
episode-local state, then erase those identities without changing permanent
knowledge?

## Supplied Substrate

- relation slot positions
- opaque identity equality and hashing
- permanent cells and arrows
- episode-local cells and arrows
- automatic temporary erasure
- answer, not-found, and ambiguous output cardinalities
- four possible routes between the two slot roles

The parser does not create an answer arrow or compare the query identity with
relation statements.

## Training

Each episode contains ten relations with twenty fresh opaque identities. The
query is one slot-1 identity, and terminal supervision provides only the
complete correct outcome after the learner has answered.

Routes that produced the correct terminal identity become stronger. Routes
that produced a different complete outcome become weaker.

## Checkpoints

```text
training episodes    permanent cells    permanent arrows    validation
10                   6                  4                   100 / 100
100                  6                  4                   100 / 100
1,000                6                  4                   100 / 100
10,000               6                  4                   100 / 100
```

Permanent structure plateaus before the first checkpoint.

## Held-Out Evaluation

```text
episodes                              20,000
correct answers                       20,000
fresh opaque identities encountered  400,000
permanent identity-specific cells     0
average spikes per query              23
```

No terminal feedback or permanent learning occurs during evaluation.

A persistent identity memorizer retains 100,000 training entries and answers
zero held-out episodes because every identity is new.

## Temporary Lifetime

For ten relations:

```text
peak temporary cells     31
peak temporary arrows    20
cells after episode      0
arrows after episode     0
owned temporary capacity 0
```

Temporary relation records are also released.

## Controls

```text
querying slot 2                    NOT_FOUND
identity absent from relations    NOT_FOUND
one identity with two outputs     AMBIGUOUS
same relation repeated twice      ANSWER
```

## Permanent-State Audit

The canonical fingerprint includes every logical permanent cell, arrow,
strength, threshold, training counter, and persistent cache.

The fingerprint is identical:

- before and after all twenty thousand held-out episodes,
- immediately before and after the ten-thousandth held-out episode.

The fingerprint deliberately excludes temporary episode state but not any
permanent confidence counter.

## Conclusion

V19 demonstrates a narrow form of learned indirection:

> Permanent role-routing knowledge can operate on arbitrary episode-local
> identities without retaining those identities permanently.

This is not yet general variable binding. Slot structure, equality, temporary
lifetime, candidate role routes, and output cardinality are supplied.
Composition remains untested.
