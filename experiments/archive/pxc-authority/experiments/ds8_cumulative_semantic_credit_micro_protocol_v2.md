# Cumulative DS8 non-semantic-credit MICRO v2 mechanical retry

Protocol identifier: `ds8-cumulative-semantic-credit-micro-v2`.

Status: **PREREGISTERED DEVELOPMENT RETRY; OUTCOME UNSPENT**.

Exact negative parent:
`78852b56db790472b2bb4ead8a3f46133b957501` /
`ds8-cumulative-semantic-credit-micro-v1-negative`.

Frozen negative result/audit SHA-256:

```text
4c352ebbbeb9bb56c6aeab98164b44085e69a0f0d7d9b128542d2fd0f28313d2
f8d630d54b1cf3ff9211aea5339876728c800b01f186f4a02389bef22f92f3f6
```

V2 changes only two control/audit mechanics.

## Single-record shuffled-value control

If both recognized encounters have value records, retain the v1
`swap_values` control exactly. If only one recognized encounter has a value
record, move that existing record to the other recognized encounter and leave
the original without one. This permutes only learned M5 value association; it
does not create, edit, negate, or reinterpret a value. The resulting admission
must reverse or lose the original preference.

This change is allowed only in the evaluator-side cloned shuffled control. It
may not enter acquisition, the live learner, or held-out admission.

## Fragment-digest preflight

Expose the build-generated fragment digest through a no-cell `--audit`
command. Before MICRO, compare it with the independently extracted positive
PROBE organism path. If the only mismatch is newline extraction convention,
correct the frozen digest under a new implementation commit, rerun the no-cell
audit, and require it to pass. The organism fragment itself must remain
byte-identical.

All seeds, temporal schedules, consequence histories, recurrence/margin
thresholds, M5 mechanism, DS8 consequence learner and linker, physical
accounting, admissions thresholds, perturbations, and cumulative controls
remain unchanged.

Run formatting, focused library compilation, and the no-cell audit in the
dedicated E2B sandbox. Then execute MICRO v2 once in release mode. No rerun or
rescue is allowed. PASS makes GATE eligible; M5 remains authoritative.
