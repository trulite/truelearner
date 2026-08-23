# PX3-R Arm C downstream-convergence MICRO v1 negative audit

Status: **FROZEN DEVELOPMENT NEGATIVE; FIRST COLLAPSE SPENT; PX3 ABSENT**.

## Sole execution

The preregistered MICRO command executed exactly once from mechanism commit
`42dd2daf89019ba12042fc37ba325e81545fee70`, tag
`px3-r-c-downstream-convergence-development-implementation-v1`, after the
positive PROBE was frozen at commit
`3ede0e57575070215eb83facbb8b2939fb34e1d8`:

```text
cargo run --release -p px0-physical-correspondence \
  --example px3_r_c_downstream_convergence -- --micro
```

It printed exactly one `PX3_R_C_MICRO_DEVELOPMENT_EVIDENCE_SPENT` marker and
then `PX3_R_C_MICRO_FIRST_COLLAPSE`, exiting `1`. There was no rerun, rescue,
regeneration, tuning, source/protocol change, or GATE execution after the
marker.

## Atomic artifacts

- CSV: `results/px3_r_c_downstream_convergence_micro_v1.csv`, SHA-256
  `5422b491e7a74bf4b7d3135c563932eec00c2a3a82df95b4e1a687a8bedc874a`,
  `767` bytes;
- report: `results/px3_r_c_downstream_convergence_micro_v1.md`, SHA-256
  `f05b05e9f552e29930a53043b3786a64e132e455112aa76ddb7690b1b9672952`,
  `1,154` bytes;
- staging paths: absent.

## Exact first collapse

The initial matched-marginal discriminator repeated exactly: trained use
`1|1`, crossed use `0|0`, trained common endpoints `1|1`, crossed common
endpoints `0|0`, and individual use `0|0|0|0`.

The first ordered MICRO requirement then failed at swap weakening:

- old A+B/C+D overlap score before swap: `23`;
- old overlap score after swap: `35` — it strengthened instead of weakening;
- new A+D/C+B overlap score after swap: `24`;
- held-out new A+D/C+B crossings: `2|2`, not exactly `1|1`;
- held-out old A+B/C+D crossings: `2|2`, not `0|0`.

The final opportunity resistance matrix was:

```text
A  20 11  0  0
B  20 13  0  0
C  13 27  0  0
D  11 15  0  0
```

Every route therefore retained coupling-`2` input to both active ordinary
continuations. Individual route strengths remained exactly equal:
correspondence `32|32|32|32`, direction `27|27|27|27`.

## Physical cause

During swapped contemporary experience, one old coupling-`2` opportunity and
one freshly replenished coupling-`1` opportunity from the two active routes
sum to the threshold-`3` old continuation even without its contemporary
downstream-driver SPIKE. That old continuation fires physically and returns
activity to its approaches. The new crossed edge is then strengthened, so old
and new endpoints recruit one another instead of ordinary pressure weakening
the old organization.

This is not an evaluator leak or an individually stronger route. It follows
from the preregistered symmetric field, the retained old coupling, threshold
summation, and frozen local-return law. Avoiding it would require an
unpreregistered physical selector/gate, a different topology, or a new law.
Those are rescues and are outside this frozen arm.

## Later observations, not the first collapse

- stale `220`-tick pressure control: `0` crossings;
- ambiguous activity: symmetric, no arbitrary single selection;
- correlation without participation: `0` crossings;
- participation without return: `0|0` crossings;
- absent/blocked opportunity: `0|0` crossings;
- duplicate complete replay: exact;
- stable-multiple control: false, but blocked behind the earlier swap collapse;
- natural quiescence and zero autonomous route-source refiring: pass.

## Accounting and disposition

- ledgered work: `446,632` operations;
- opportunity additions: `364`;
- final ARROW count: `456` including deallocated weak opportunities;
- persistent substrate storage: `32,304` bytes;
- result storage: `1,921` bytes.

MICRO is frozen negative. GATE and definitive evidence are forbidden and were
not executed. The positive PROBE remains preserved but is insufficient for a
positive candidate because the mandatory swap/relearning discriminator fails.
