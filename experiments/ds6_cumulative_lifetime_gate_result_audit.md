# DS6 cumulative learned-lifetime GATE result audit

Outcome: **DEVELOPMENT POSITIVE WITH RECORDED MATRIX OVERRUN**.

This document supersedes the interpretation in
`ds6_cumulative_lifetime_gate_invalid_execution_audit.md`. That audit was
written while the asynchronous E2B command was still running and before its
delayed atomic artifact/download became visible. It remains preserved as an
operational record; its `NO GATE RESULT` conclusion is withdrawn.

There was one GATE learner execution, not a rerun.

## Frozen provenance

- protocol commit/tag:
  `4670f561d240c0b1e86633c38a83ccf32cdded2e` /
  `ds6-cumulative-lifetime-gate-protocol-v2`;
- implementation commit/tag:
  `0eea89ba786ba45b88aa98a625d5a1662cdcf6e9` /
  `ds6-cumulative-lifetime-gate-implementation`;
- E2B sandbox: `iyrkw7af5qpmwwfmq3bwm`;
- mechanism/harness SHA-256:
  `3d5659fb26ae804dee6122408f9d703ea1f226349772883075a42686ac3fd110`;
- runner SHA-256:
  `50ce486ead4b0bd4452ecd87a1e4d26907075b85a478db7c13975af0410e783d`;
- result SHA-256:
  `f805e5fed5e109d9e6c829bad9ca0b69f0c01eafe0f831ca844683226713d968`.

The atomic result reports `PASS` and exact duplicate replay.

## Matrix overrun

The protocol specified six fresh seed anchors:

```text
111000 112000 113000 114000 115000 116000
```

The implementation mistakenly encoded Rust's inclusive integer range
`111_000..=116_000`, executing all 5,001 integers rather than the six anchors.
No mechanism, gate, or result was changed after execution, and no cell was
rerun.

All 5,001 executed cells passed every conjunctive gate. In particular, each of
the six preregistered anchors is present and passes:

| Seed | Recurrence | Pressure | Lifetimes | Crossed | Interleaving | Loads | Reuse | Contradiction | M3 | Controls | Result |
|---:|:---:|:---:|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| 111000 | yes | yes | 1,3,6,13,27 | yes | yes | yes | yes | yes | yes | yes | PASS |
| 112000 | yes | yes | 1,3,6,13,27 | yes | yes | yes | yes | yes | yes | yes | PASS |
| 113000 | yes | yes | 1,3,6,13,27 | yes | yes | yes | yes | yes | yes | yes | PASS |
| 114000 | yes | yes | 1,3,6,13,27 | yes | yes | yes | yes | yes | yes | yes | PASS |
| 115000 | yes | yes | 1,3,6,13,27 | yes | yes | yes | yes | yes | yes | yes | PASS |
| 116000 | yes | yes | 1,3,6,13,27 | yes | yes | yes | yes | yes | yes | yes | PASS |

Because every unintended cell also passed, restricting the scientific audit
to the preregistered anchors does not select favorable observations. The raw
5,001-cell artifact remains immutable. All 5,001 namespaces are retired from
future definitive evidence.

## Frozen development result

Across every preregistered anchor:

- recurrence and pressure orderings passed;
- learned deallocation pressure was exactly `1,3,6,13,27` for recurrence
  counts `1,2,4,8,16`;
- crossed high-use/long-disuse and low-use/short-disuse cells straddled the
  survival boundary as preregistered;
- interleavings and loads `8,32,128` preserved matched trajectories;
- gap reuse added exactly `+2`, while removed state reacquired at `1`;
- changed-regime competition monotonically spent old protection and built new
  protection without a contradiction-specific delete;
- fresh identities/layouts, cumulative M3 reconstruction, economy, lifecycle,
  controls, and exact duplicate replay passed.

The development claim is:

> The unchanged cumulative mechanism implements a use-dependent physical
> lifetime law: recurrence builds resistance to erasure, ordinary non-use and
> competing activity spend it, and zero strength physically deallocates the
> structure without a supplied lifetime class or retention oracle.

DS6 is cumulative-development ready. This does not create M4 or authorize a
definitive execution. M3 remains authoritative until a separately
preregistered definitive matrix passes.

