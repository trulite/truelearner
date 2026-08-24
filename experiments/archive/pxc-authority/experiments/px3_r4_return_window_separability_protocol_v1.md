# PX3-R4 return-window separability protocol v1

Status: **PREREGISTERED; DEVELOPMENT EVIDENCE UNSPENT; PX3 AUTHORITY REMAINS NEGATIVE**.

Start: frozen PX3 definitive-negative commit
`b8e4410fb8b5a38c8fb38969e2a92a35a6defe3c`, tag
`px3-physical-event-organization-definitive-negative-v1`.

| frozen input | SHA-256 |
|---|---|
| authoritative PX0 substrate law | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| failed PX3 definitive source | `288ce23199f66b65e022afac4314629ac133edaea93072486326357f8c58b328` |
| failed PX3 definitive CSV | `3fa85616ae97faef0db2200941a41a6ea7d51f9e254267f84fbed8684a0e0d06` |
| failed PX3 definitive report | `3a8f96587b751f3e19303582861d8320c0a78ae1aa25da24b8959efb809c3317` |
| definitive result audit | `37a2bd47474020510d076e131c65acfa60f0ac34ee1b6f583aa42c3d6f6fd3d5` |
| definitive negative handoff | `f6cd3e4436071758697f8cb7805189aca747914ec6a4999f5143f142f02ca336` |
| positive R3 CSV | `62b34a64396728c28b617bab75cf1141ee2b2db53897ee655809b6180cb2a67b` |
| R3 result audit | `6f565cf8397afb55e28293360f1ade5aa51b89ba5fa8c19ce0eacaa23086e299` |

R4 is a readout diagnostic, not an authority retry. It may not alter the PX0
law, rescue the failed definitive cells, or make a PX3 authority claim.

## Question

> Are legitimate completed-path return and renewed upstream joint input
> temporally separable at the eligible candidate source under the unchanged
> physical law?

Define both latencies relative to the first firing of the candidate source P:

```text
T_return = latest lawful R3 attribution echo arrival at P
T_recur  = earliest renewed upstream A+B arrival at P
```

The diagnostic records actual physical arrivals and firings. Scheduled inputs
are not accepted as traversal or arrival measurements.

## Frozen one-stage geometry

R4 extracts stage one byte-for-byte in mechanism from the failed definitive
arm:

```text
A/B outlet -> direct + local hub -> unit PX1 trace
A trace + B trace -> O(threshold 2)
O unit + uniform P background unit -> P(threshold 2)
external threshold crossing at P -> native distance-one P->X proposal
P->X candidate + uniform delayed context -> X(threshold 2)
P and X -> identical unit PX1 participation traces
P trace + X trace + global return -> M(threshold 3)
M -> P delay 1, coupling 1; P threshold 2
```

P first fires at tick 1, the candidate traverses at tick 1 and is eligible
through tick 5 under the unchanged four-tick PX0 local window. A correctly
aligned global return at tick 3 makes M fire at tick 3 and its lawful unit echo
reach P at tick 4. Nothing is fed back strongly enough to refire P in the
single-return sweep.

The candidate begins absent. Native proposal, coupling, resistance, pressure,
eligibility, return update order, refractory dynamics and trace decay are
unchanged. No Event, episode marker, direction bit, provenance identifier,
return channel or execution suppression may be added.

## Exact matrix

Two fresh deterministic namespaces run in normal and mirrored cell/arrow
insertion order. Each row is executed from complete fresh state twice for
exact replay.

### S0 — lawful-return timing sweep

One A+B episode starts at tick 0 with background/context at tick 1. Global
return is scheduled at each tick `0..=6`, one tick per row.

The row records P firing, candidate traversal/impulse, P/X trace firing ticks,
global-to-M arrival, M firing, M-to-P echo crossing/arrival, P refiring,
candidate resistance, local-return updates, quiescence and replay.

Expected physical alignment:

```text
global return tick 3 -> M tick 3 -> lawful echo reaches P tick 4
other return ticks   -> no M firing and no M->P echo
```

The tick-6 row also exposes ordinary expiry/deallocation after the eligibility
window. Exact state is serialized rather than inferred from resistance.

### S1 — renewed-input timing sweep

No global return exists. The first A+B episode starts at tick 0. A second real
A+B episode starts at offset `1..=6`, with its own uniform background/context
at offset `+1`.

The row records both primitive traversals, both normalized traces, every O->P
arrival, every P firing, candidate history/liveness/resistance, candidate
crossing impulse, attribution/echo absence, local-return updates, quiescence
and replay.

Expected boundary under the unchanged law:

```text
offsets 1..=4 -> renewed O->P reaches P at ticks 2..=5 while old eligibility is live
offsets 5..=6 -> renewed O->P reaches P only after old eligibility expires
```

The first renewed arrival is therefore expected one tick after the first P
traversal. R4 does not label the resulting update correct; it measures whether
it occupies the same source-local window as lawful return.

### S2 — same-tick collision

The first episode uses its lawful global return at tick 3. A second A+B episode
starts at tick 3, with uniform background/context at tick 4. Thus the first
episode's M->P echo and the second episode's O->P arrival both physically reach
P at tick 4.

The row serializes the ordered P arrivals, their origins and impulses, P
firings, candidate crossings, resistance updates and quiescence. This is a
measurement of the existing queue/order law, not a requested resolution.

The total matrix is exactly `2 * (7 + 6 + 1) = 28` rows.

## Validity clauses

Every row records five independent validity bits:

1. `V0` frozen hashes, fresh namespace, insertion stratum and scenario identity
   are exact;
2. `V1` actual primitive/O/P/candidate/X participation matches the scheduled
   physical sweep and no attribution exists in recurrence-only rows;
3. `V2` source-local arrival ticks and origins are serialized directly from
   trace/crossing records;
4. `V3` no substrate law or physical mechanism differs from the frozen
   one-stage definitive geometry;
5. `V4` propagation naturally quiesces and complete-state replay is exact.

All 28 rows and 140 validity bits must pass for the timing classification to
be interpretable. A validity failure is `R4-C UNINTERPRETABLE`, not evidence
for either temporal conclusion.

## Frozen classification

- **R4-A TEMPORALLY SEPARABLE:** the latest actual lawful echo arrival at P is
  strictly earlier than the earliest actual renewed upstream arrival at P.
  A source-local eligibility cutoff exists between them.
- **R4-B TEMPORAL OVERLAP:** the measured lawful-return and renewed-input
  arrival windows intersect, including an exact same-tick collision. No
  expiry-only source-local window can admit every measured lawful echo while
  excluding every measured renewed input.
- **R4-C UNINTERPRETABLE:** any validity clause, quiescence, replay, frozen hash
  or exact-matrix condition fails.

R4-B establishes only that timing alone is insufficient for this geometry. It
does not specify a replacement law or authorize a PX3 modification.

## Evidence discipline

Implementation and preflight run in the established E2B development sandbox.
No Rust compilation, test or runtime may execute locally. Preflight must
construct no world, propagate no activity, emit no evidence marker and create
no result artifact.

After implementation is committed and tagged, a separate execution protocol
will freeze the sole command and write-once artifact paths. The evidence
command may execute exactly once. No rerun, rescue, regeneration or tuning is
allowed after its evidence marker.
