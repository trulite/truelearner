# DS7 cumulative learned plasticity-allocation GATE v3 result audit

Status: **POSITIVE DEVELOPMENT GATE; DEFINITIVE PREREGISTRATION ELIGIBLE**.

Frozen lineage:

- authoritative M4:
  `8db47281a7c9c97cbb52ced6fc3dcff0e7efa9b2`;
- positive MICRO readiness:
  `9e7d197e2915fa0c550160d2cdd3dbb04884f168`;
- immutable GATE v2 negative:
  `f80db49` /
  `ds7-cumulative-plasticity-allocation-gate-v2-negative`;
- exact v2 collapse handoff and v3 protocol:
  `9aa6eda` /
  `ds7-cumulative-plasticity-allocation-gate-protocol-v3`;
- v3 implementation:
  `8044a55aa0be11ec092ff5c3f0e14a96bab3b92d`;
- M4-linked allocator source SHA-256:
  `e755a70deada891e5c4db3b55809ca84ea8ad31a8bd3affe564bf08a95f8dff7`;
- GATE source SHA-256:
  `abaedd16717543270c5ed0ef2c8a16e3a4c0fed0215764443948c36d4adfa297`;
- runner SHA-256:
  `606f1a3900f0f251da090ac9cfada39e35e5bfd0db301f0f945d4a3408cad97c`;
- v3 protocol SHA-256:
  `324d328ed1ec1f20edfa3e5372a5fcefcca37d973e7b033f50cd2a0d26cfc9f5`.

## Execution

The complete fixed 18-cell GATE v3 ran once in persistent E2B development
sandbox `iyrkw7af5qpmwwfmq3bwm`, from the clean immutable v3 implementation
commit. The sandbox was left running.

```text
cargo fmt --check                                      PASS
focused release unit test                              1 / 1
release GATE                                           PASS, 18 / 18
duplicate exact                                        PASS
```

No broad repository suite or definitive matrix ran.

## Relationship to the v2 negative

The v2 matrix is preserved exactly. It failed because its arbitrary 400-event
withholding cap supplied 100 pressure ticks to resistance-105 route edges,
leaving resistance 5. The v3 retry changed only:

```text
withholding window       400 -> 424 events
branch instrumentation   joint -> split resistance/removal
```

The observed v3 entry resistance was exactly 105 in both branches and final
resistance exactly 0. This matches the preregistered mechanical localization.

## Scientific reading

The positive GATE supports the cumulative development claim that learned
encounter history can allocate structural plasticity over an M4-governed
substrate:

```text
local coactivity forms initially broad variation
delayed active-path experience values encounter history
learned value suppresses later unproductive structural work
live retained arrows execute a multi-edge route
M4 pressure can remove a long-used but later withheld edge
learned allocation can reacquire the missing productive edge
shuffled allocation cannot
```

The still-supplied semantic outcome is strictly delayed and remains the named
DS8 target. This development result does not create M5.

