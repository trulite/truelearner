# PX0 definitive v2 result audit

Status: **DEFINITIVE FAIL; IMMUTABLE; PX0 AUTHORITY ABSENT**.

## Sole evidence spend

The exact preregistered command executed once from clean implementation commit
`5c14a8e5d23c60d11463254790c9550209f7399d`, tagged
`px0-physical-correspondence-definitive-v2-authority-implementation`:

```text
cargo run --release -p px0-physical-correspondence --example definitive -- --definitive-v2
```

It emitted exactly one observed marker:

```text
PX0_V2_DEFINITIVE_EVIDENCE_SPENT
```

and exited `1`. No rerun, rescue, regeneration, or post-evidence source change
occurred.

## Conjunctive outcome

- cells passed: `14/16`;
- claims passed: `206/208`;
- failed cells: `7` and `15`;
- sole failed claim in each: `P6`;
- every other claim `P0..P5` and `P7..P12`: `16/16`;
- proposals: `15,268`;
- physical deallocations: `15,082`;
- ledgered work: `5,563,497`.

The two failures share the same physical corner:

```text
spacing                 17
reverse allocation      true
mirrored layout         true
route stride            26
dense distractor load   40
incidental phase        3
```

Their route identities differ (`1→2` and `0→1`), so the failure does not track
a fixed route handle.

## Exact scientific boundary

`P6` was the conjunction:

```text
stable-return count equals all 35 contexts
AND stable resistance exceeds sparse resistance
AND contemporary B held-out effect equals 1
```

The frozen result records that contemporary B produced exactly one held-out
effect in both failed cells. It does not serialize the two remaining P6
subclauses separately. Therefore this audit does not infer which of those
subclauses failed and does not recreate the cells to find out.

The negative is correspondingly narrow but authoritative: the full
preregistered contemporary reversed-world conjunction did not hold under the
two densest, slowest mirrored/reverse cells.

All sixteen cells nevertheless passed independently frozen claims for:

- initial acquisition;
- survival-window reuse;
- true forgetting and stale-path blocking;
- fresh post-forgetting reproposal;
- return-free physical probation;
- sparse dense-topology non-reusability;
- recurrent dense-path learnability;
- stability swapping;
- absence and ambiguity;
- replay, quiescence, provenance, work, and storage accounting.

Those observations do not override the conjunctive authority failure.

## Integrity

- protocol commit: `8c41a68`;
- implementation commit: `5c14a8e`;
- negative result commit: `098f02c`;
- implementation SHA-256:
  `48b66f62b6127a8ec760a2f4678d0b6f9f5e677fcad956bd457529d231fd52f4`;
- CSV SHA-256:
  `486970d22b7282c8a89b2f06b811644841b6b905563c9e12e676b3da33150119`;
- report SHA-256:
  `460d183d9cfb8e0f244209801b4cb4f140dc474089120473fd1de5bf738aba33`.

All preflight fields in the frozen report are true, including active-law,
retained-physics, v1-negative, PX0-P1, PX0-S, protocol/tag, dependency/source,
and pre-execution output-absence checks. Staging paths are absent after atomic
publication.

## Program consequence

PX0 remains non-authoritative. PX1, PX-C, the continuous organism, and Harness
H1 remain blocked. The definitive v2 matrix is permanently spent and may not
be rerun.
