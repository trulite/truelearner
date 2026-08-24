# PX3-R6 integrated preflight collapse audit v1

Status: **INTEGRATED DEVELOPMENT NOT OPENED; EVIDENCE UNSPENT; PX3 AUTHORITY NEGATIVE**.

Start: frozen R6-A result commit `07300b6`, tag
`px3-r6-return-triggered-trace-readout-positive-v1`.

## Frozen inputs

| input | SHA-256 |
|---|---|
| authoritative PX0 law | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| R6 source | `12d9422cc43d43a88da9d8046a2ab7fbdc8f9447236e97bc73483d0d4ce7eb4f` |
| R6 CSV | `35b68303630f69c326fadad1ccc988e807ae0e1a77703b4e751732e0cdeae4d8` |
| R6 report | `32f338c9eae943159ef54015322973f29983fb899eeddd71f4e0fa35a5c6d796` |
| R6 result audit | `be59e5f23033036cfb8e8735a8c9c2eb93c612458c1e5fd053dae250a8682751` |

## Intended integrated discriminator

The proposed fresh integrated workflow required, among its conjunctive gates:

```text
recurrent A+B without world return
  -> never matures

adjacent A+B episode 1, no world return, A+B episode 2
  -> zero credit from episode 1
  -> zero accidental maturation
```

Read-only inspection of the already-frozen R6 evidence falsifies these gates
before a new implementation or evidence spend.

## Decisive frozen observation

Both `px-no-return-100-adjacent` rows in the R6 CSV record:

| seed | M firings | M echo | candidate resistance | candidate live | local return updates | R6 row |
|---:|---:|---:|---:|---|---:|---|
| 3701 | 0 | 0 | 288 | true | 2081 | pass |
| 3709 | 0 | 0 | 288 | true | 2081 | pass |

Thus R6-A established its narrow registered claim: partial P/X footprints do
not accumulate inside the R6 attribution cell M. It did **not** establish that
world-return absence prevents candidate learning. In the exact no-return
stress row, the candidate becomes strongly reusable despite zero M firing and
zero R6 echo.

The R6 classifier explains why the row was positive. For
`RepeatedNoReturn`, lines 459--464 of the frozen source require only empty
footprint firings, M events, M firings and echo. Candidate resistance,
liveness and `local_return_updates` are serialized but are not classification
predicates for that row.

## Mechanical cause

The authoritative PX0 propagation loop calls:

```text
apply_local_return(spike.target, tick, work)
```

for every accepted delivered spike, before cell-state accumulation and before
the firing test (`lib.rs` lines 273--284). `apply_local_return` then strengthens
every live, eligible outgoing arrow whose source is that target (`lib.rs`
lines 409--422). It does not inspect the spike's physical route, the R6 M cell,
or whether the arrival is a weak attribution echo.

Consequently, while P's candidate is eligible, renewed opportunity/background
activity arriving at P still invokes the generic PX0 return update. R6 can
ensure that M emits no false echo, but it cannot make this pre-existing source
arrival stop being interpreted as return.

This is the same unresolved authority-v1 relation:

```text
candidate traverses -> eligibility live
new upstream episode reaches P -> generic local return update
world return and R6 M echo absent -> candidate nevertheless strengthens
```

## Classification and boundary

This is a **pre-integration collapse**, not a new experimental result:

- no Rust compilation, test or runtime command was executed;
- no E2B evidence was spent;
- no integrated protocol, implementation, result or authority artifact was
  created;
- no authoritative PX0 law or frozen R6 artifact was changed;
- the fresh integrated branch exists only to preserve this audit.

An integrated rerun using byte-identical R6 attribution would be known in
advance to fail the required no-return and adjacent-recurrence gates. It must
not be represented as a test of an open hypothesis.

R6 remains a valid narrow development positive for return-triggered footprint
readout. It is not the missing source-local return discriminator, does not
repair the first PX3 authority counterexample, and does not make PX3 ready for
a fresh authority matrix.
