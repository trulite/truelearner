# CE0 consequence-supported efficacy handoff v1

Status: stopped immutable negative; no promotion.

## Scientific state

```text
CR0
coupling has an independently necessary physical job             DONE

CE0
accumulated local support -> unopposed positive efficacy          NEGATIVE

use without consequence -> no efficacy change                    PASS
qualified local consequence -> graded efficacy acquisition        PASS
threshold and fan-out behavior                                   PASS
reciprocal stability                                              FAIL
```

The failure is not ambiguity or packaging. Both mechanics reproduce it exactly:
coupling 2 in both directions creates 32 unintended reciprocal re-firings before
local forgetting breaks the cycle.

## Frozen lineage

- protocol: `0f5524b`, tag
  `ce0-consequence-supported-efficacy-protocol-v1`;
- first frozen candidate: `631f5fe`, tag
  `ce0-consequence-supported-efficacy-frozen-v1`;
- mechanical pre-evidence enum correction: `34bce7f`, tag
  `ce0-consequence-supported-efficacy-frozen-v1-corrected`;
- evidence-eligible preflight record: `023a023`, tag
  `ce0-consequence-supported-efficacy-evidence-eligible-v1`;
- immutable evidence: `60d41ed`, tag
  `ce0-consequence-supported-efficacy-negative-v1`.

## Operational provenance

- reusable implementation/preflight worker: `iq2ph1xuh7t0p1ckb4r9p`;
- sole fresh evidence worker: `i3anz5jq8nxgudrg0dfy1`;
- CE0 formatting: PASS;
- CE0 strict Clippy: PASS;
- CE0 static/source-boundary audit: PASS;
- core tests: 13/14, with the sole inherited FD0 checkpoint-contract failure
  reproduced identically with CE0 disabled;
- matrix replay and Reference/Production equivalence: exact.

## Boundary

Do not return to FD2 or ARC from this branch. Do not restore historical
`coupling += 1`, add a ceiling, or tune thresholds. A fresh successor must ask
which general physical constraint allows efficacy to mature without converting
ordinary recurrent topology into an oscillator.

Authority, oracle, ARC, FD2, and `arch.md` remain unchanged.

