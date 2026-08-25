# FD1 negative-v1 C3 diagnostic result audit v1

Status: complete; classification inconclusive but bounded.

Diagnostic protocol: `fd1-negative-v1-diagnostic-protocol-v1`
(`70d7379`).

Frozen diagnostic evaluator: `fd1-negative-v1-diagnostic-frozen-v1`
(`e1e3878`).

E2B evidence sandbox: `i1ppgxi6y0htlgd4wjm0j`.

## Result

Across two disjoint roots, creation phases `0..9`, and Reference/Production:

- rows: `40/40`;
- immediate post-consolidation candidate-state equality: `40/40`;
- 39-tick candidate-state equality: `40/40`;
- 40-tick/dead candidate-state equality: `40/40`;
- future PhysicalWork equality: `40/40`;
- exact same-mechanics replay: `40/40`;
- exact Reference/Production observations: `20/20`;
- natural quiescence: `40/40`.

Every serialized candidate state also matched the original C3 numeric
expectations:

```text
after consolidation  live / resistance 4 / decay load 0
39 ticks later       live / resistance 1 / decay load 9
40 ticks later       dead / resistance 0 / stale generation
```

Classification emitted by the frozen diagnostic:
`unreproduced_composite_failure`.

## Accounting correction

The v1 process output named only the failing family. It did not serialize or
print the root or creation phase. Therefore the negative record's statement
that the stop occurred at the first root and phase zero was an unsupported
inference from loop order and is withdrawn here. FD1 v1 remains immutable and
negative; its exact root/phase is unknown.

## Interpretation

The diagnostic does not support a physical-law failure in the normalized C3
candidate state or future PhysicalWork. It also does not identify the original
false predicate because v1 asserted before writing its row and the diagnostic
used the preregistered disjoint identities.

The next scientifically lawful action, if authorized, is a fresh FD1 v2 matrix
with unchanged candidate physics, schedules, families, and predicates, but with
each observation serialized before the assertion and a fresh namespace. This
is measurement hardening, not a mechanism rescue. FD1 v1 may not be rerun or
relabeled.

## Artifacts

- `results/fd1_negative_v1_diagnostic/diagnostic.csv`
  SHA-256 `7119a130f025ec3730b7590a2a073c790722131f98d6bc5be82006fd226730fe`
- `results/fd1_negative_v1_diagnostic/diagnostic.md`
  SHA-256 `a737eb255c0dcae6d8b32a87a22b428f29e005d39d9e800503b2d7a54ecb4201`
- checksum manifest
  SHA-256 `fa0793c7b38631dd17b3549a45f19580da8037da257692303af6ffa59fe05545`

No C0-C2 or C4-C6 family, FD0 replay, ARC, CPC/PQLC, RC0, authority,
oracle, or `arch.md` change was made by this diagnostic.
