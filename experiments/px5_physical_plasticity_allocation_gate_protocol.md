# PX5 no-new-mechanism physical plasticity-allocation GATE protocol

Status: **PREREGISTERED; GATE EVIDENCE UNSPENT; DEVELOPMENT ONLY; AUTHORITY ABSENT**.

## Frozen basis

The GATE starts from positive MICRO commit
`1b2c4aaaf0f6fbc858e7289b527e884cc9623d45` and authoritative PX2 parent
`2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`.

| frozen input | SHA-256 |
|---|---|
| unchanged PX0--PX2 law | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| positive MICRO implementation | `f0ee5f1ce3ea5e4ee84cc82da3164c03d72327437b9616ba8e01ac259621d4c6` |
| positive MICRO CSV | `16bc4595a90cd61b015874358eb21dbf2dee469c76b2c8a813f0aa036476837f` |
| positive MICRO report | `888e5177110af7b9f891666e232e4751d9d0574151220098afc1577c7af08ca5` |
| positive MICRO audit | `b0fa8aef57c8933dc529d3f7f28fb84143983e3729abd85da0902f7422839f25` |

PROBE and MICRO sources/evidence remain immutable and will not be rerun.

## Exact fresh matrix

The development GATE has exactly twelve blank cells at namespaces
`0x5_7000_0000 + cell * 0x0100_0000`. Each cell is independently reconstructed
and repeated exactly once for duplicate equality. No earlier identity or layout
is reused.

The matrix uses distractor loads `32, 64, 128` (four cells each), useful route
counts `4` and `8`, local distances `1` and `2`, normal/mirrored absolute
positions, normal/reversed CELL allocation, and normal/reversed SPIKE insertion
order. Pair anchors are twelve position units apart, so every generic local
opportunity sees only its own physical neighbor.

## Primary physical schedule

All environmental opportunities are explicit:

1. At tick `0`, ordinary external activity fires every useful and distractor
   source. The unchanged local law alone creates generic nearby variation.
2. At tick `2`, ordinary external activity returns only to every useful source.
3. Half the useful sources (hot physical recurrence) fire/return at `10/12`,
   `30/32`, and `50/52`. All useful sources (hot and warm) fire/return at
   `20/22` and `40/42`.
4. Up to eight fixed return-free distractor sources recur at ticks `15, 30, 45`.
   They receive no return and no selected update.
5. At tick `60`, all useful sources execute held-out, then receive ordinary
   return at tick `62`.
6. Useful source zero is withheld. Every other useful source fires/returns at
   `70/72` and `80/82`. At tick `90`, ordinary pressure must have removed only
   the withheld useful edge. Fresh activity at `90`, followed by return at
   `92`, must generically reacquire it.

The schedule contains no hidden task boundary, reset, proposal choice, or
allocation call. The useful/distractor and hot/warm descriptions are evaluator
names for fixed physical activity schedules only.

## Hardened independent controls

Each cell also constructs fresh blank controls:

- **return-free recurrence:** one source fires at `0, 6, 12, 18, 24, 30` and
  pressure advances to `36`; it must create six generic variations, perform
  zero return updates, and retain zero live structure;
- **late return:** one source fires at `0`, receives external activity only at
  `6` (outside the retained eligibility window), and advances to `12`; it must
  perform zero return updates and retain no route;
- **outside radius:** a source and neighbor are separated by `3`, beyond the
  frozen local radius; source activity must create no proposal or crossing;
- **evaluator-only shuffle:** after primary execution, a read-only allocation
  vector is reversed; complete fingerprint and live topology must be exact;
- **physical-return permutation:** in a separate matched blank world all sites
  fire at tick `0`, but the same count of tick-`2` return SPIKEs reaches the
  first distractor sites instead of the original useful sites. At tick `20`,
  the original useful routes must be dead and the physically returned
  distractor routes live. Thus an evaluator shuffle cannot preserve the
  original functional allocation; only the actual location of return matters.

## Twelve conjunctive claims per cell

1. `P0`: frozen hashes, constants, and fresh namespace;
2. `P1`: exact initial proposals equal `useful + load`;
3. `P2`: exact primary useful return work equals
   `3 * useful + 3 * (useful / 2)` and no distractor receives a return update;
4. `P3`: every hot and warm useful route remains live and held-out execution is
   exactly `useful/useful`;
5. `P4`: all one-shot and repeatedly encountered return-free distractors have
   zero live structure at tick `60`;
6. `P5`: minimum useful resistance is positive and strictly exceeds maximum
   distractor resistance;
7. `P6`: matched return-free recurrence remains structurally empty;
8. `P7`: late return and outside-radius controls remain structurally empty and
   emit no false crossing;
9. `P8`: selective withholding physically removes the stale useful ARROW while
   all other useful routes remain live, and generic opportunity reacquires one
   exact replacement crossing with old generation blocked;
10. `P9`: evaluator-only shuffled allocation is fingerprint-exact and causally
    inert;
11. `P10`: matched physical-return permutation fails to preserve every original
    useful route and instead follows the actual returned physical sites;
12. `P11`: natural quiescence, duplicate exactness, exact work/storage
    accounting, topology/layout/identity transfer, zero dependencies, and zero
    old-M linkage.

The GATE passes only at `12/12` cells and `144/144` claims. Any failure is an
immutable negative. No rescue, rerun, or tuning is permitted.

## Organism boundary

Only the byte-identical retained `PlasticSubstrate` and actual CELL, ARROW,
SPIKE, timing, local eligibility, participation, return, pressure, resistance,
generation, liveness, topology, and crossing state execute. Encounter classes,
`LEARN_HERE`, proposal-site labels, a supplied gating/allocation policy,
semantic enums, typed intermediate representations, serializers, adapters,
hidden task boundaries, evaluator-selected local mutation, old M schemas, and
renamed equivalents are forbidden.

## Validation, one-shot execution, and consequence

Pre-evidence validation requires formatting, focused build/tests, strict
Clippy, unchanged frozen hashes, immutable prior evidence, zero dependencies,
source/forbidden-path audit, no-cell preflight, refusal without `--gate`, and
artifact absence.

Execute exactly once after implementation commit/tag:

```text
cargo run --release -p px0-physical-correspondence \
  --example px5_physical_plasticity_allocation_gate -- --gate
```

Atomic outputs:

```text
results/px5_physical_plasticity_allocation_gate_v1.csv
results/px5_physical_plasticity_allocation_gate_v1.md
```

A positive result establishes non-authoritative PX5 development readiness and
lane classification A only. It does not authorize or simulate an authority
workflow, execute a definitive matrix, advance PX3--PX8, modify PX0--PX2, or
create an authoritative ancestor. The final handoff must include the mandatory
unchanged-port contract and keep authority absent.
