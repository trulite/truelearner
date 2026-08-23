# PX5 no-new-mechanism physical plasticity-allocation MICRO protocol

Status: **PREREGISTERED; MICRO EVIDENCE UNSPENT; DEVELOPMENT LANE ONLY**.

## Frozen basis

This MICRO begins at frozen positive PROBE commit
`f5e1b80677ef89a28801eadb58a613497f139fd8` while retaining authoritative PX2
parent `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129` exactly.

| frozen input | SHA-256 |
|---|---|
| unchanged PX0--PX2 law | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| positive PROBE implementation | `099670d97cd7c3679612237eac3d1cd580b6ec3bb841f8720c27d9cab1cf315f` |
| positive PROBE CSV | `866673057bef55e45a284b6d6d557f7dd4fccc7b0fab3f73f5028656f8047ded` |
| positive PROBE report | `2913dd85020f6250fa9d4dd1cec39b5115d3fcb0d843fba54a6054bd18506c0a` |
| positive PROBE audit | `bbb1833e70d049526be8dcbb7155f72eda775c07e51f6dc3b78f5d8df9efc994` |

The PROBE is immutable and will not be rerun. This lane still has zero
authority to modify PX0--PX2, advance PX3--PX8, create an authoritative
ancestor, or execute a definitive matrix.

## No-new-mechanism question

The MICRO asks whether the same retained physical law scales from one useful
neighborhood to several simultaneous useful neighborhoods under distractor
load, without adding an allocator or gating representation.

## Exact matrix and schedule

There are exactly eight blank cells at namespaces
`0x5_6000_0000 + cell * 0x0100_0000`. Four cells use `8` distractor
neighborhoods and four use `24`. The cells cross:

- normal and mirrored absolute positions;
- normal and reversed CELL allocation;
- normal and reversed external SPIKE insertion/phase order.

Every cell contains exactly four useful two-CELL neighborhoods. All useful and
distractor sources receive the same generic local variation opportunity at
tick `0`; only physical useful participation receives ordinary return at tick
`2`. Useful sources recur at ticks `10, 20, 30, 40, 50` with return two ticks
later. No organism-visible class identifies either set.

At tick `60`, ordinary pressure must leave all four returned routes live and
all distractor routes dead. Held-out activity tests all four physical routes
and returns at tick `62`.

The first useful source is then physically withheld while the other three
continue recurring every ten ticks through tick `140`. At tick `150` the
withheld edge must be dead while the three recurrent edges remain live. Fresh
activity at the withheld source must receive generic local variation, execute
exactly one crossing, and receive ordinary return at tick `152`.

A separate blank matched return-free world fires one local source at ticks
`0, 6, 12, 18, 24, 30` and advances to tick `36`. It must retain no live route.

Finally, a read-only evaluator-side allocation vector is reversed across all
useful and distractor observations. Because it is not organism state and no
selected mutation API exists, it must change neither complete fingerprint nor
physical liveness. It cannot substitute for returned activity.

Every cell is repeated from an independently constructed byte-identical blank
state. No PROBE identity or layout is reused.

## Ten conjunctive claims per cell

1. `P0`: exact frozen hashes/constants and fresh namespace;
2. `P1`: initial generic structural proposals equal `4 + load`;
3. `P2`: the primary training phase performs exactly `24` useful local return
   updates and zero distractor return updates;
4. `P3`: all four useful routes are live and execute `4/4` at tick `60`;
5. `P4`: all `load` distractor neighborhoods have zero live variation;
6. `P5`: returned routes have strictly more retained resistance and future
   structural work than every distractor;
7. `P6`: matched return-free recurrence creates six generic proposals, zero
   return updates, and zero live routes;
8. `P7`: selective withholding removes only the stale useful edge while the
   other three remain live, then generic opportunity reacquires exactly one
   executable replacement with the old ARROW dead;
9. `P8`: shuffled evaluator allocation is causally inert and cannot revive any
   distractor or replace physical return;
10. `P9`: natural quiescence, duplicate exactness, work/storage accounting,
    fresh identity/layout transfer, zero dependencies, and zero old-M linkage.

The outcome is conjunctive: `8/8` cells and `80/80` claims. Any other outcome
is an immutable negative.

## Boundary and execution

Organism execution is exclusively the byte-identical retained
`PlasticSubstrate` CELL/ARROW/SPIKE state and local laws. Encounter classes,
`LEARN_HERE`, proposal-site labels, supplied gating/allocation policies,
semantic enums, typed intermediates, serializers, adapters, hidden task
boundaries, evaluator-selected mutations, old M5 schemas, and renamed
equivalents are forbidden.

After implementation commit/tag and pre-evidence validation, execute exactly
once:

```text
cargo run --release -p px0-physical-correspondence \
  --example px5_physical_plasticity_allocation_micro -- --micro
```

Atomic outputs:

```text
results/px5_physical_plasticity_allocation_micro_v1.csv
results/px5_physical_plasticity_allocation_micro_v1.md
```

Validation requires formatting, focused build/tests, strict Clippy, unchanged
frozen hashes, zero dependencies, source/forbidden-path audit, no-cell
preflight, refusal without `--micro`, and artifact absence. A positive MICRO
makes only a separately preregistered PX5 GATE eligible.
