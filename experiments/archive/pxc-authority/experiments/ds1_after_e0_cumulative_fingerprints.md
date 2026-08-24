# Unchanged DS1-on-DS-E0 composition fingerprints

Status: development-collapse evidence only. No definitive command or result
artifact was produced.

## Lineage

- authoritative M0: `1d74c0ed0b515446161a63a6d43ecbe27514dc85`;
- enabling parent: `d154fde5632c0ba9d76fc2d1d1a700276045adc8`;
- enabling tag: `ds-e0-cumulative-anonymous-event-formation-readiness`;
- composition preregistration commit:
  `002ba648c849c713903fae3394232a3bdcf7c076`;
- preregistration tag:
  `ds1-after-e0-cumulative-composition-attempt-protocol`.

M0 is authoritative. The enabling parent is M0 plus DS-E0 only. M1 does not
exist.

## Exact frozen boundaries

| Boundary | SHA-256 | Continuity |
|---|---|---|
| inclusive marked frozen DS1 learner | `adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e` | exact required value |
| DS-E0 mechanism/serializer source | `fc5d426cc8a5116dbd2749b914e6c30db88529d3070a844a20fc76ac88782615` | byte-identical to `d154fde` |
| M0 parent mechanism | `50cf169bb293177a35270adde656f28f98e68c83a4d39d2876399261b7ee697c` | byte-identical to `d154fde` |
| M0 compiled correspondence | `430cd2206c8baa7106c4de7f203d4d0c48b544290e6266596ebcdb91d02655c9` | byte-identical to `d154fde` |
| E0 serializer/provenance audit | `edfa40c5cedd9359b0677d36a26844a502a2c5b05e7f4c0b137c8922ff4c11a7` | byte-identical to `d154fde` |
| E0 dependency manifest | `c3b9a5daf1e27a98283432e0f992c1cb4d801505c1790af99a22ba408ca06181` | byte-identical to `d154fde` |

The independent pre-execution marked-hash command was:

```text
sed -n '/DS1_LEARNER_BEGIN/,/DS1_LEARNER_END/p' \
  src/ds_e0_anonymous_event_formation.rs | shasum -a 256
```

The frozen-file check compared each working file's SHA-256 with a SHA-256 of
`git show d154fde:<path>` and passed before MICRO/GATE. `git diff d154fde --`
over every frozen path is empty.

## New development-only sources

| Source | SHA-256 before handoff metadata update |
|---|---|
| composition harness with parent-audit inventory | `a4deadedfde7b9896d64d0cacd41560441ea85cf3bda119a5d09aa3aaddcd7a0` |
| composition runner with inventory report | `3dce1c2a85e1576b23e1eff8e1f9d453bd3ca6352205f989a56c93f2fedacaad` |
| behavior-neutral library export | `6903e7d0d73a72c5e8b1673c84e767ea0aae71456b4ccc4d8ebcea2bf7f7b2ef` |
| preregistration | `1b76adf6b34b15d29cbdd5e31730cf781d47641da624994dc642c7cf42cb74c3` |

The only call from the composition harness into the frozen mechanism is
`ds_e0::run(mode)`. The resulting existing DS-E0 probe calls the frozen
`frozen_choice(&neighborhood)` read-only. The new wiring never calls `choose`
or `apply_consequence` because stage 4 is unavailable.

The parent-audit amendment derives that unavailability from the exact included
M0, compiled-M0, and DS-E0 sources. It extracts the frozen function signatures
and public report struct surfaces, then counts definitions, method call sites,
candidate/propagation surfaces, exposed pair values, compatible execution
signatures, choice-to-execution edges, and post-action consequence edges. No
M0 or DS-E0 source was modified to enable this inventory.
