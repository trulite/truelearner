# PX4 physical learned-lifetime PROBE v2 result audit

Status: **FROZEN POSITIVE DEVELOPMENT RESULT; MICRO ELIGIBLE**.

## Frozen result

- implementation commit/tag:
  `df7e2a0cc039c1b64bb3c83c5adcceaa87b30638` /
  `px4-physical-lifetime-probe-v2-implementation`;
- CSV SHA-256:
  `085afebff478505ef47cf26edaf092cd5f6a503dd599007807228227aaec0ed9`;
- Markdown SHA-256:
  `2dc4b702c41ad0cf398a97e7c6f93b448d47ccb7432dabae8601e368c99ea7ae`;
- result: `6/6` PASS;
- frozen substrate-law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

The Markdown title says `v1` because the unchanged formatter labels all stage
reports `v1`; the write-once path, namespace, protocol, implementation, and
this audit unambiguously identify PROBE v2. The artifact is preserved
unchanged.

## Physical result

- twelve returned uses left resistance `23`; matched later pressure left `12`
  and held-out outward execution remained `1|0`;
- three returned uses left resistance `9`; the same later pressure reached
  zero, advanced generation, and held-out execution became `0|0`;
- the disuse trajectory was exactly `17,13,5,0` and nonincreasing;
- contemporary reverse activity replaced the unused old path:
  `17|0 -> 0|35`, effects `1|0 -> 0|1`;
- correlation without traversal and traversal without return both ended
  `0|0`, non-live, with no effects;
- source hashes, normalized duplicate outcomes, natural quiescence, work,
  pressure, deallocation, allocation-slot, live-allocation, byte, and
  fingerprint accounting passed in all rows.

This establishes PROBE-level support that existing PX0--PX2 physics is
sufficient. It does not establish development readiness or authority.
