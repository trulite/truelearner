# PX3 participation-trace organization PROBE protocol v1

Status: **PREREGISTERED; PROBE EVIDENCE UNSPENT**.

This file narrows Stage 1 of the cumulative PX3 development protocol to one
write-once executable. It authorizes implementation, audit and E2B preflight;
it does not authorize the evidence command before the implementation freeze is
committed and audited.

## Frozen execution surface

- package: `arms/px3-participation-trace-organization`;
- preflight command:
  `cargo run --manifest-path arms/px3-participation-trace-organization/Cargo.toml --release -- --preflight`;
- sole evidence command:
  `cargo run --manifest-path arms/px3-participation-trace-organization/Cargo.toml --release -- --probe`;
- CSV destination:
  `results/px3_participation_trace_organization_probe_v1.csv`;
- Markdown destination:
  `results/px3_participation_trace_organization_probe_v1.md`;
- corresponding hidden `.staging` paths use the same basenames;
- no MICRO, GATE, definitive or authority command or artifact path may exist.

`--preflight` performs frozen-source hashes, matrix/schema assertions, forbidden
surface checks and destination/staging absence checks. It constructs no world,
calls no substrate propagation, and writes no result. `--probe` repeats every
preflight assertion before emitting the evidence marker and running the matrix.
Any other argument exits nonzero.

## Frozen matrix

Seeds are `2601` and `2609`. Each seed runs these twelve unique scenarios in
this exact order:

1. `a-alone`;
2. `b-alone`;
3. `a4-alone`;
4. `a-repeated`;
5. `a-then-b-late`;
6. `ab-one-overlap`;
7. `ab-recurrent-1-1`;
8. `ab-recurrent-2-1`;
9. `ab-recurrent-4-4`;
10. `ab-recurrent-heldout-matrix`;
11. `ab-blocked-return`;
12. `proposal-only`.

Every scenario is executed twice from a fresh world and must reproduce exactly.
This yields exactly 24 CSV rows, unique by `(seed, scenario)`.

The six anonymous unordered opportunity incidences exist before training in
lexicographic participant order: `AB, AC, AD, BC, BD, CD`. Every opportunity is
primed identically at tick `0`; its sole distance-two candidate emits once,
receives no context or return, and expires from resistance `2` to `1` by tick
`5`.

Real exposures begin at tick `5`. A recurrent second exposure begins at tick
`14`. Source inputs are physical entries; repeated A supplies two scheduled
entries at the same tick and must serialize one actual source firing/traversal.
The late A/B control schedules A at tick `5` and B at tick `7`, so their unit
trace firings occur at ticks `6` and `8` and cannot fire a threshold-two
opportunity. Shared context is supplied identically for every training exposure
at the trace/opportunity tick and reaches every consequence at its candidate
arrival tick.

The held-out pressure gap ends at tick `50`. Held-out trained AB, crossed AD,
gapped A/B and singleton A uses run on separate exact clones of the learned
world and receive no context. One-overlap, blocked-return and proposal-only
controls also face the same gap. No evaluator uses a result expectation to add,
route, strengthen, delete or select structure.

## Frozen physical topology

Each primitive path is the authoritative PX1 normalization motif:

```text
source --raw coupling--> outlet --unit--> trace(threshold 2)
                               `--unit--> shared PX1 hub --unit--> every trace
```

Each trace has one fixed coupling-one ARROW to every incident opportunity. Each
opportunity is an ordinary threshold-two CELL. Its only cell within PX0's local
variation radius is its own consequence CELL at distance two; all other CELLs
are farther away. The initial uniform external opportunity pulse therefore
causes generic PX0 local proposal to construct exactly one anonymous candidate
per opportunity. Consequences are ordinary threshold-two CELLs receiving a
candidate unit plus one shared context unit. Every firing consequence reaches
one shared PX3 return hub, and that hub broadcasts the same unit return to all
six opportunities. The blocked-return world omits only consequence-to-return
ARROWs. There are no pair IDs or semantic fields in substrate state.

All discriminating edges cross physical regions and are serialized from native
`Execution.trace` and `Execution.crossings`. Fixed topology ARROWs have
resistance `100`; only generic candidates use their native distance-derived
resistance.

## Required row schema and conjunctive outcome

Each row records seed/scenario/raw couplings; scheduled entries; source,
raw-ARROW, outlet, unit-participation and trace firing counts per primitive;
trace firing ticks; per-opportunity arrivals/firings; proposal count; candidate
liveness/resistance/coupling after prime, training and pressure; consequence
and return activity; held-out trained/crossed/gapped/singleton consequences;
native work ledgers; persistent bytes; complete/permanent fingerprints;
quiescence; exact replay; and a conjunctive pass bit.

The pass bit requires all of the following:

- A alone, B alone, A(4) alone, repeated A and late A/B fire no opportunity;
- raw coupling `1`, `2` or `4` still yields exactly one outlet and one trace
  firing per actually participating path;
- simultaneous A+B fires only opportunity AB;
- one A+B exposure acts transiently but leaves no reusable route at tick `50`;
- recurrent A+B at all frozen amplitudes leaves only candidate AB live and
  reusable;
- trained AB held-out produces one consequence while crossed AD, gapped A/B
  and singleton A produce none;
- blocked return and proposal-only worlds leave no reusable route;
- all unused symmetric candidates deallocate under ordinary pressure;
- every run is naturally quiescent and exact duplicate replay holds.

The evidence run is successful only if all 24 rows pass, ordering and uniqueness
are exact, no staging remnant remains, and both artifacts are published by
create-new staging, sync and atomic rename. A scientific failure stops this
lane. A mechanical correction requires a fresh protocol with the PX0 law and
all frozen scientific dimensions byte-exact.
