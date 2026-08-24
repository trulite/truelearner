# PX3-R5 three-factor return attribution implementation audit v1

Status: **FROZEN; E2B PREFLIGHT PASSED; DEVELOPMENT EVIDENCE UNSPENT; PX3 AUTHORITY NEGATIVE**.

- source commit: `55a06b9b5f6764ddf519456947132f576998df19`;
- manifest SHA-256:
  `467c0cac79cded043c0d2f80685700ae939e9c0345f32712133e577ea1d1c9ed`;
- source SHA-256:
  `13ad86ab078e5fb72bdcbd0b5bff87f85f4cca0a493cda33d22f8dac647ac4fe`;
- protocol SHA-256:
  `6d6635962d08c23d4e546af9f1a74d07e30896ac1df05b6c124fb558bf1ba8d7`;
- execution-protocol SHA-256:
  `cfd584c967e67085fa293203ce98f823f5510052f9c7d30a1effe99f02431bd4`.

## Physical surface

The raw global-return input to M is absent. World activity enters an ordinary
threshold-one R source, traverses a fixed physical outlet, and is normalized by
the same direct-plus-local-hub PX1 motif used for P and X. M receives exactly
three ordinary unit trace arrows: P trace, X trace and R trace. Its only output
is the frozen unit, delay-one echo to threshold-two P.

P-to-X begins absent and can arise only through native distance-one proposal
when external-plus-O activity makes P fire. The evaluator cannot insert,
strengthen, select, suppress or delete that candidate. It schedules only
external physical inputs and reads public trace, crossing, work, resistance,
liveness and fingerprint surfaces.

The `xr-p-absent` control drives X through a fixed ordinary physical driver,
not by direct evaluator firing at X. No Event, episode, provenance, return kind,
credit channel or semantic boolean exists in organism-visible state.

## Frozen matrix and validation

Seeds `3601,3609` cross eight exact scenarios for 16 fresh namespaces. The
offset-one adjacent recurrence/no-return control is explicit and independently
classified; it cannot be dropped if it exposes R4 carryover.

Persistent E2B sandbox `i6x9gykt9tvp6xfz5z8ra` passed on exact source commit
`55a06b9`:

- formatting;
- 2/2 release static tests;
- strict release Clippy with warnings denied;
- final/staging artifact absence;
- no-argument and wrong-argument refusal with exit 2 and no evidence marker;
- non-propagating `--preflight`.

Preflight constructed no world, propagated no activity, emitted no evidence
marker and wrote no artifact. No Rust command executed locally.

The next clean commit containing this audit is the exact tagged preflight
target. Only its frozen E2B `--r5` command may spend evidence once.
