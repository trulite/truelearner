# Post-M6 DS4 arrival-initiation definitive implementation audit

Status: **AUTHORITY IMPLEMENTATION FROZEN; DEFINITIVE EVIDENCE UNSPENT**.

## Frozen protocol and ancestry

- authoritative M6:
  `aa4e22efd8a65b7694956a53cfaa970582695215` /
  `core-autonomy-checkpoint-established`;
- development readiness:
  `7c39cb4306c61b8e67119c91c3f9e3802dab9a39` /
  `post-m6-ds4-arrival-initiation-development-readiness`;
- exact mechanism snapshot:
  `e11a6a7927d86bbe0fff5f55942a8b418de39de1` /
  `post-m6-ds4-arrival-initiation-gate-implementation-v4`;
- protocol v1:
  `6ba5deee16c37362f99833d1c5afb53b4dee2a2f` /
  `post-m6-ds4-arrival-initiation-definitive-protocol-v1`;
- controlling pre-evidence protocol v2:
  `5fb002397b5f80ba1b6fa586b432861f37893d21` /
  `post-m6-ds4-arrival-initiation-definitive-protocol-v2`.

Protocol v1 executed no test, audit, learner, control, cell, evidence marker, or
artifact. Its initial no-cell sequence stopped at the formatting check. After
formatting, the next no-cell sequence stopped at strict Clippy because v1 had
named `manual_range_contains` instead of the documented byte-frozen M5/M6
`derivable_impls` allowance. Protocol v2 corrected only that allowance name
before evidence. Both tags are preserved.

## Frozen implementation hashes

The final no-cell-validated authority source snapshot is
`b84f96a5ae99d8d7b32ab082844a028f80f7a9bd`.

| implementation input | SHA-256 |
|---|---|
| authority wrapper | `4ec3e293461e826a087bcfa58d70538881eaef8c6539b63e7174010b25c72e50` |
| atomic write-once runner | `d68b7b465d3cc11c62b9cd75dc1af815e153a29bab3fa478ae7521f13998f051` |
| build composition/hash plumbing | `fd45d1156a3eb96390e998fed1a7925fc171fc372e48c6e41825189d3b439679` |
| library module registration | `4bc9215d4370d371af87ad1c67da5ca45800a67e4948294cf03fd61c7d844591` |
| controlling definitive protocol | `ac8e32b23cf55bd84f6b7d050d3694b348540dc8f75993a33834f3e9062f6b72` |
| byte-frozen mechanism | `67e170f12d7b7649a0a291ddfc16cd80e4b5c15564b65cd09c884f3e52b9ac5b` |
| byte-frozen development runner | `0a7bc106fe5135c04c4e62aed8de77d50ba7b0756ea51548d391ce33c00796e2` |

The mechanism and development runner compare byte-for-byte with their paths at
the exact mechanism tag. The pre-existing `results/` digest remains
`558168c61dac6f714329e2e0b0872073cea35f01d3b1a8b9cadd4570374a90ea`.
Neither definitive final nor staging artifact exists.

## Mechanism and information-flow boundary

The authority wrapper composition-copies the exact frozen mechanism. It does
not edit or reformat the M3 event learner, P4 occurrence-role learner, M5/M6
consequence learner, linker, recurrence condition, thresholds, constants,
development artifacts, or organism information flow.

The only marked cell call is:

```text
literal seed + fixed held-out count 64
  -> frozen authority observation
```

The wrapper calls each complete blank-start physical cell exactly twice, as
the preregistered duplicate control, and independently reconjoins the returned
measurements. Its focused missing/stale/invalid control uses only the frozen
arrival, occurrence, recurrence, and credit APIs. It adds no organism state or
learning input.

The source audit proves that frozen physical execution precedes the recurrent
activity check, the recurrent check precedes the single M6 learner application,
the M6 differential precedes feedback to the already-selected occurrence
trace, and evaluator answer comparison occurs after the marked linker. No
expected answer/trace, semantic correctness, reward/loss, target identity or
channel, request/start flag, stable occurrence identity, or Lane-B SSA state
appears in either linker or the authority call.

All M0--M6 definitive CSV/Markdown pairs, the original DS4 negative, PROBE-v1
negative, positive retry, positive MICRO, positive GATE/readiness, frozen M3,
P4, M5, and M6 linker hashes passed the no-cell source audit.

## Explicit matrix and refusal boundary

The wrapper contains exactly the sixteen literal protocol-v2 bases from
`700_123_457` through `850_123_457` at a `10_000_000` stride. Each owns a
`6_300_000` half-open derived region. The regions are mutually disjoint and
begin above the audited development-derived ceiling `152_700_000`. The fixed
held-out count is `64`; there is no seed, held-out, output, cell, partial,
resume, replay, append, overwrite, or alternate-mode argument.

The runner exposes only `--audit` and `--definitive`. Both require both final
and both fixed staging paths to be absent. Publication fully writes and syncs
create-new staging files, hard-links them without replacement to fixed final
paths, syncs the directory, and removes only staging links. The focused runner
test proved that a second publication returns `AlreadyExists` while preserving
the original bytes.

## Fresh E2B no-cell validation

Dedicated authority state:

```text
/Users/satya/.cache/truelearner/post-m6-ds4-arrival-initiation-definitive-authority-e2b.json
```

Fresh sandbox: `i27300rd4hx0nbmifilae`, template
`truelearner-rust-1-97-worker`. It is distinct from development sandbox
`icmxrqcsf8br7shgus934` and remains running.

From clean snapshot `b84f96a5ae99d8d7b32ab082844a028f80f7a9bd`, the seven
protocol-v2 no-cell commands passed:

```text
cargo fmt --all -- --check                                      PASS
focused definitive-bin compilation                             PASS
strict release Clippy with three frozen-code allowances        PASS
focused library no-cell preflight/refusal tests                2/2 PASS
focused runner atomic publication/refusal test                 1/1 PASS
release --audit                                                 PASS
release no-argument refusal                                    PASS (exit 2)
```

The release audit reported every source, lineage, original-negative, M0--M6,
source-order, information-flow, Lane-B, literal-matrix, namespace,
fixed-held-out, final-path, and staging-path field true.

No positive development control, mechanism snapshot, definitive learner,
definitive seed/cell, report function, result artifact, `--definitive`
command, or evidence-spend marker ran during validation. The next and only
claim-eligible action is the single protocol-v2 release `--definitive` command
from the clean commit/tag containing this audit. The sandbox must remain
running after the final result audit.
