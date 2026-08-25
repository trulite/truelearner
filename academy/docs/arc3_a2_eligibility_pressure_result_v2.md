# ARC3 A2 eligibility-aware pressure successor result v2

Status: development-positive and ready for a separately frozen successor
authority workflow. The architectural oracle and `arch.md` are unchanged.

## Physical result

Ordinary pressure now excludes only those pressure epochs covered by an
ARROW's already-existing live eligibility window. No new timer or persistent
state was added. If modulation does not arrive, the existing unsupported-use
pressure still applies when eligibility expires.

The rejected `first_effect_due` mechanism is absent.

## Permanent paired regressions

The focused worlds are defined once and run under both:

- `MechanicalConfig::REFERENCE`;
- `MechanicalConfig::PRODUCTION`.

Both passed:

- eligible resistance-1 traversal survives its covered pressure epoch;
- timely modulation strengthens exactly once and consumes eligibility;
- unsupported expiry removes the route;
- dormant ordinary pressure still removes resistance-1 structure;
- late modulation produces no update;
- body, causal work, clock, pressure phase, quiescence, and replay match.

The bounded four-context Academy regression also passed at initial pressure
phases 0 and 9 under both mechanics. Complete observations matched. Its paired
runtime was 0.62 seconds in E2B.

## V1 accounting negative and v2 repair

V1 remains frozen negative because Academy populated `physical_work` from
legacy `Work::total()`, which contains mechanics-sensitive scanning work. V2
added `Work::physical_total()` as the saturating sum of the five causal counters
already used by the permanent differential oracle and changed only the ARC
observer to use it. Execution cost remains separate.

## Official ARC A2 evidence

The official `ls20`, seed 205, A2-only row ran at phases 0 and 9 under both
reference and production mechanics. Each row independently replayed exactly.
After removing only the serialized mechanics label, reference and production
JSON observations were identical.

| Phase | Mechanics | Actions | Changed pixels | Updates | Quiescent | Replay |
|---:|---|---|---|---|---|---|
| 0 | reference | `[1,4,2,3]` | `[52,52,52,52]` | `[0,1,1,1,1]` | true | exact |
| 0 | production | `[1,4,2,3]` | `[52,52,52,52]` | `[0,1,1,1,1]` | true | exact |
| 9 | reference | `[1,4,2,3]` | `[52,52,52,52]` | `[0,1,1,1,1]` | true | exact |
| 9 | production | `[1,4,2,3]` | `[52,52,52,52]` | `[0,1,1,1,1]` | true | exact |

Artifacts and hashes are under
`results/arc3_a2_eligibility_pressure_v2/`.

## Retained runtime conformance

E2B validation on the same frozen candidate:

- core tests: `15/15`;
- strict Clippy: core and Academy ARC pass;
- R1-R5 differential: `80/80`, behavioral clauses `536/536`;
- R6 partition invariance: `36/36`, checkpoint controls `2/2`;
- format checks: pass.

The accepted physical-body and boundary-buffer behavior embedded in the
differential runner remained positive.

## Frozen hashes

```text
core lib.rs       d49a6b98081cee65c3e7a5f64e9cf6356fa06ef456a9cf78a41d0cd1187bcd58
ARC sensorimotor  bd0f5578ef91f0911953c4c9edcf7be4f64517a683335a27c06c2dba5bb628d6
official runner   4a69c7e31a5779a439a3dcc76a63ab2f45486f75227c550cbb4ca997d684a731
```

## E2B provenance

- sandbox: `ijrbkyzlo011lkccwxi9s`;
- targeted paired regressions: pass;
- official paired comparator: pass;
- retained core/Clippy: pass;
- R1-R5 differential: pass;
- R6 partition invariance: pass.

The initial official attempt never created a world because its toolkit working
directory was removed by checkout refresh. A second incomplete attempt used a
debug oracle binary and was stopped before publishing a row. The recorded
official evidence used the release agent from a stable external directory.
