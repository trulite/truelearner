# V20 Iterable Lookup

## Question

Can the frozen v19 lookup consume its own previous output through one learned
feedback route?

V20 does not ask the machine to decide how many times to apply the operation.
The evaluator supplies identical apply pulses and a final read event.

## Supplied Structure

- the frozen v19 lookup operation
- temporary current and result roles
- one apply cell and one read event
- three candidate feedback routes
- opaque identity equality, slot position, and episode-local lifetime from v19

No apply number or iteration counter is visible to the learner. The host does
not pass a result identity back as the next lookup key.

## Learned Structure

Training uses only two-pulse episodes. Terminal supervision contains the
complete final outcome and does not identify an intermediate identity or the
correct feedback route.

```text
training episodes    permanent cells    permanent arrows    validation
10                   10                 7                   100 / 100
100                  10                 7                   100 / 100
1,000                10                 7                   100 / 100
```

The selected route moves the temporary result into the temporary current role.

## Held-Out Depth

Every held-out episode uses fresh opaque identities, shuffled relations, and
distractors.

```text
apply pulses    v20 learner    supplied feedback    two-stage unrolled    lookup spikes
1               1,000/1,000    1,000/1,000          1,000/1,000           23
2               1,000/1,000    1,000/1,000          1,000/1,000           46
3               1,000/1,000    1,000/1,000          0/1,000               69
4               1,000/1,000    1,000/1,000          0/1,000               92
```

Training contains exactly two apply pulses. Depths one, three, and four receive
no additional learning.

## Trace Audit

The representative four-pulse trace uses:

```text
apply cell       6 on every pulse
lookup arrow     1 on every pulse
feedback arrow   4 on every pulse
```

After each successful lookup, its result is the next pulse's current identity.
The host invokes only the no-argument apply event.

## Controls

```text
missing intermediate relation       NOT_FOUND
conflicting intermediate relations  AMBIGUOUS
duplicate identical relation        ANSWER
```

Held-out evaluation encounters 74,000 fresh opaque identities. Permanent
structure remains ten cells and seven arrows, and its canonical fingerprint is
unchanged. Temporary cells, arrows, relation records, traces, and owned
capacity return to zero after every episode.

## Baselines

The two-stage unrolled baseline owns two separate frozen lookup copies. It
passes one and two pulses, then cannot perform a third step.

The supplied-feedback baseline receives the correct result-to-current route
without learning. It passes every tested depth and serves only as an upper
bound.

## Conclusion

V20 supports this narrow claim:

> A feedback route selected from terminal supervision lets the same learned
> lookup operation consume its own output repeatedly on fresh identities.

The operation is iterable. The learner does not yet decide whether another
application is needed or when to stop. Apply timing, working roles, the frozen
v19 lookup, and candidate feedback routes remain supplied.
