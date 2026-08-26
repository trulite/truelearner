# PC0 pressure / closure compatibility implementation audit v1

Status: evidence-eligible implementation frozen before the focused matrix.

Parent: `e3aff995c670406b5242f8ea3e7e42d726dcd941`.

## Active-law delta

The entire organism-law change is one removed mutation:

```diff
 let attenuation = min(participation, Q)
-participation -= attenuation
 pressure_load += Q - attenuation
```

Ordinary time relaxation remains unchanged. Traversal remains the only renewal
source. Modulation, PQLC, resistance, generation, scheduling, ordering, and all
mechanical strategies remain unchanged.

No state field, timer, deadline, pressure exception, closure predicate, route
identity, or ARC fact was added. Pressure uses the full arithmetic magnitude;
there is no positive-participation branch.

Hashes:

```text
core lib.rs  6b9651cec46de87d2b2e5f39e11bf3ce51862510176c87afdd8d6ad179e8c4bc
evaluator    1eb837ca0bc5f30817ad853390f0939ce60964b0c7cb591148bbac13141a2083
```

## Pre-evidence validation

Reusable E2B sandbox `iutm0f927ofx88230lkoo` established:

- targeted rustfmt: pass;
- targeted check: pass;
- targeted strict Clippy: pass;
- permanent core/R1-R6 tests: `14/14` pass;
- diagnostic unchanged CPC1: `620/620` complete;
- diagnostic unchanged PQLC0: `200/200`, positive;
- diagnostic unchanged PQLC1: `780/780`, positive.

The diagnostic retained runs occurred before the focused PC0 evaluator freeze
and are not the final PC0 evidence. No Rust or project command ran locally.
