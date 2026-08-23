# PX3-R Arm C downstream-convergence PROBE v1 result audit

Status: **DEVELOPMENT PROBE POSITIVE; EVIDENCE SPENT; PX3 ABSENT**.

## Sole execution

The preregistered PROBE command executed exactly once from frozen mechanism
commit `42dd2da` / tag
`px3-r-c-downstream-convergence-development-implementation-v1`:

```text
cargo run --release -p px0-physical-correspondence \
  --example px3_r_c_downstream_convergence -- --probe
```

It printed exactly one `PX3_R_C_PROBE_DEVELOPMENT_EVIDENCE_SPENT` marker and
then `PX3_R_C_PROBE_PASS`, exiting `0`. There was no rerun, rescue,
regeneration, tuning, or source/protocol change after the marker.

## Atomic artifacts

- CSV: `results/px3_r_c_downstream_convergence_probe_v1.csv`, SHA-256
  `b34cc9a49bb89b7ad0196c3d92d6ada7ee4171b7c86624d651b3f1caac644f8e`,
  `679` bytes;
- report: `results/px3_r_c_downstream_convergence_probe_v1.md`, SHA-256
  `8e3c8e1da1d7200dfb6012e4b6bf8d75fe8e0eec8e48147a26f9b55ba3e4d951`,
  `1,041` bytes;
- staging paths: absent.

## Conjunctive observation

- individual correspondence resistance: `20|20|20|20`;
- individual learned-direction resistance: `15|15|15|15`, all live;
- retained opportunity resistance by route x continuation:
  `8,0,0,0 | 8,0,0,0 | 0,15,0,0 | 0,15,0,0`;
- measured opportunity impulse:
  `2,0,0,0 | 2,0,0,0 | 0,2,0,0 | 0,2,0,0`;
- held-out A+B and C+D downstream crossings: `1|1`;
- held-out A+D and C+B downstream crossings: `0|0`;
- A, B, C, and D individual downstream crossings: `0|0|0|0`;
- trained common live endpoints: `1|1`;
- crossed common live endpoints: `0|0`;
- correlation without route participation: `0` crossings;
- participation without returned activity: `0|0` crossings;
- absent opportunity: `0|0` crossings;
- duplicate complete replay: exact;
- natural quiescence and zero autonomous source refiring: pass.

The different `8` versus `15` opportunity resistances reflect ordinary
arrival/pressure phase across the two continuation occurrences; they do not
alter the discriminator. The separately serialized individual route strengths
are exact, and each trained common endpoint supplies coupling-`2` impulses.

## Accounting and disposition

- ledgered work: `165,730` operations;
- opportunity additions: `216` full-field missing-edge replacements;
- final ARROW count: `308` including physically deallocated prior weak
  opportunities;
- persistent substrate storage: `22,832` bytes;
- result storage: `1,720` bytes;
- first collapse: none.

This positive development PROBE authorizes only the already-preregistered
MICRO. It is not definitive evidence, PX3 authority, or permission for PX4.
