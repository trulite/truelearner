# PX1 physical boundary roles definitive implementation audit

Status: **IMPLEMENTATION FROZEN; DEFINITIVE EVIDENCE UNSPENT; PX1 AUTHORITY ABSENT**.

## Frozen implementation

- source:
  `crates/px0-physical-correspondence/examples/px1_pt1_attributed_margin_stability.rs`;
- definitive source SHA-256:
  `74716c87d146cb697b37ddf802c12e67a5cb93daf82ec20f8b982e54922bd696`;
- definitive protocol SHA-256:
  `166cabd14f3c1d53830fc673530cb6d7c0f32125468c4120ace04025e7586bef`;
- frozen positive GATE source SHA-256:
  `b0d549077ca49b4bebd3a692ad3906f800bc94aae348301a2206f03f16b90b07`.

The active physical mechanism is byte-identical to the frozen GATE source:

| physical block | current SHA-256 | frozen GATE SHA-256 |
|---|---|---|
| `run_world` | `07000164f62cabc7a49fee6b0f3d82f29985f1e053e736e73ad6c9b0c8828d7b` | `07000164f62cabc7a49fee6b0f3d82f29985f1e053e736e73ad6c9b0c8828d7b` |
| `build_world` | `0fe85d1d88061788bb26e1e2bd5659aa8eeccfe5db84543434ababb48d980787` | `0fe85d1d88061788bb26e1e2bd5659aa8eeccfe5db84543434ababb48d980787` |
| `measure_execution` | `025dc27789015c7454bdbd76013dcf00958947d8d2728e9d99a9583f9e11a31e` | `025dc27789015c7454bdbd76013dcf00958947d8d2728e9d99a9583f9e11a31e` |

Authority changes are limited to sixteen fresh seed namespaces, the fixed
six-world cross, independent P0–P12 serialization, preflight/refusal guards,
the single evidence marker, and staged write-once artifacts.

## Pre-evidence validation

- formatting: pass;
- focused compilation: pass;
- strict focused Clippy: pass;
- focused substrate test: `1/1` pass;
- no-argument/wrong-argument refusal: exit `2` before source audit/cells;
- no-cell `--preflight`: pass with exactly one preflight marker and no evidence marker;
- definitive final/staging paths: absent;
- sorted pre-existing result digest:
  `1e6d369b75aa639351a84a367bcee1f6d3f792f92d6259c002019c2ea8a51480`;
- authoritative PX0 and all development hashes: exact;
- forbidden typed representation scan: clean;
- fresh namespace base `0xd100_0000_0000`: absent from development evidence.

No definitive cell, seed, result, or evidence marker was executed during
validation. The sole `--definitive` command remains unspent.
