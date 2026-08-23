# PX7 anonymous physical arrival initiation MICRO protocol

Status: **PREREGISTERED; MICRO EVIDENCE UNSPENT; PX7 AUTHORITY ABSENT**.

## Frozen positive parent

MICRO starts from the immutable positive PROBE result commit
`1ff0eb11229bc20ab43bc649f1f377a9417f98ac`, tag
`px7-physical-arrival-initiation-probe-v1-positive`, whose parent remains
authoritative PX2 commit `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`.

| frozen artifact | SHA-256 |
|---|---|
| PX0 law | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| retained substrate law | `6aa28a76e1362ac8dfb1d33fb68807da40e7604dfdc8cca9efa1e314e3ce4263` |
| PROBE protocol | `08d3719c263a5dbd8fd4077fff28356b1d5d37dfc9d372f5edecefacd23b4beb` |
| PROBE implementation | `b24bc71d49466abe86a5fbb64a66a080cdc95c51391a3d5bf2dee64364d907ca` |
| frozen PROBE execution block | `41d75cbd90687eaee43b8f6aa5e27d157781eb6a6b71bbe7b5a1aa248e23f57a` |
| PROBE CSV | `a68e867d77b6b8f6459cf1210166969f78507456dcdaf14887a451ce7273da0a` |
| PROBE report | `18f32ae350fafbba7585e0fa99b00aee12e281b1c9b9a91089c5281e25db4ed8` |
| PROBE result audit | `c3efcbe6d14929ae470c3a909e596fea55ff70aba1539a494bde9bac684bd81e` |

The PROBE is never rerun. Its marked block must remain byte-extract identical.
MICRO adds a separate parameterized fixture/executor but uses the same frozen
law and the same physical edge: arrival, proposal traversal, local returned
activity, retained coupling, downstream firing, and queue quiescence.

## Question

> Does the no-new-mechanism positive survive fresh physical identities,
> mirrored geometry, reversed allocation and same-tick insertion, background
> load, late return, learned-locus specificity, and a natural post-training
> gap?

No MICRO condition may select an initiating event based on an expected answer.
Each condition supplies a fixed anonymous physical schedule. The evaluator
observes only after each propagation.

## Exact fresh MICRO matrix

Eight fresh rows use namespaces beginning at `0x7b20_0000_0000`, disjoint from
the PROBE and all PX0--PX2 evidence. Every row is rebuilt and executed twice
for exact duplicate comparison.

| row | physical condition | expected held-out behavior |
|---|---|---|
| M0 | learned return, ordinary layout | execution and boundary fire once |
| M1 | learned return, mirrored positions | execution and boundary fire once |
| M2 | learned return, reversed cell allocation | execution and boundary fire once |
| M3 | learned return, reversed same-tick background insertion, load 4 | execution and boundary fire once |
| M4 | learned return, mirrored + reversed allocation/insertion, load 12 | execution and boundary fire once |
| M5 | otherwise learned world, held-out arrival at a fresh nearby locus | no execution or boundary firing; a subsequent learned-locus arrival fires both |
| M6 | all training returns arrive at offset 6, outside the frozen local window | no held-out execution or boundary firing |
| M7 | learned return followed by held-out arrival at tick 70 | execution and boundary fire once |

Background arrivals target physically distant cells with subthreshold impulse
at the same tick as the focal arrival. They are inserted in normal or reversed
order but queue order remains a physical property of spikes and identities.
They carry no condition or expected-result metadata.

M5 contains a second ordinary nearby arrival cell. The learned locus is not
named in organism state. The evaluator physically supplies one held-out impulse
to the fresh locus, lets the queue drain, then later supplies the same impulse
to the previously recurrent locus. Success requires quietness first and
execution second.

## Frozen pass clauses

Every row serializes acquisition firings/returns, candidate liveness,
coupling/resistance, held-out and follow-up firing/crossing counts, source
firings, quiescence, exact duplicate equality, work, storage, and permanent and
complete fingerprints.

MICRO passes only if all eight rows satisfy their specified held-out behavior,
M5 preserves learned-locus execution after its novel-locus control, M6 remains
quiet after late return, M7 remains live after its declared gap, no supplied
subthreshold background arrival fires, there is no autonomous source refiring,
all propagations become naturally quiescent, duplicates are exact, frozen
hashes match, and the PROBE artifacts remain unchanged.

## Isolation and stop rule

Organism-visible execution remains exclusively actual frozen CELL/ARROW/SPIKE
state. `REQUEST`, `START`, respond-now, task/event boundaries, evaluator path
selection, initiation flags or roles, semantic enums, adapters, serializers,
typed intermediate representations, old M schemas, and later-lane mechanisms
are forbidden.

Any failed row is frozen unchanged. If failure has a mechanically unique
fixture error, it may be diagnosed only in a fresh separately named protocol;
MICRO itself is never rescued. If progress needs new representation, new law,
or any PX0--PX2 edit, stop.

## Atomic one-shot execution

After focused format/build/test/Clippy, frozen-hash and source isolation audits,
future-stage refusal, absent-artifact checks, and `--preflight`, the only MICRO
evidence command is:

```text
cargo run --release -p px0-physical-correspondence \
  --example px7_physical_arrival_initiation -- --micro
```

It executes once, emits one MICRO evidence marker, and atomically creates only:

- `results/px7_physical_arrival_initiation_micro_v1.csv`;
- `results/px7_physical_arrival_initiation_micro_v1.md`.

Final or staging path existence causes refusal. MICRO is development-only and
cannot create PX7 authority or authorize a definitive matrix.
