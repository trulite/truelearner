# PXR0 single-file physical-runtime development v2 validation audit

Status: **TARGETED VALIDATION PASS; DEVELOPMENT MATRIX UNSPENT; NO AUTHORITY**.

Frozen candidate `dc8a40fe9f559db4814165fe1bc1f2779853299a` / tag
`pxr0-single-file-physical-runtime-v2-harness-frozen-v1` was validated in
fresh E2B sandbox `i04c5jc607ptb1okviloy` using unique state file
`/Users/satya/.cache/truelearner/pxr0-v2-targeted-20260824-a.json`.

The worker ran formatting checks and release Clippy with `-D warnings` for the
unchanged runtime and v2 evaluator. It compiled but did not execute either
project binary. The complete development matrix remains unspent.

## Positive gates

- immutable runtime SHA-256:
  `f6989555f5a43dff91b39a5c7f79038168f39142fdbecca7e5e40938a72785cb`;
- manifest-v6 extraction replay: `101` original entries, `28` retained or
  canonically compacted, `73` moved, exact prior CSV replay;
- one active source file, `474` lines, `13/13` types, `15/15`
  functions/methods, `28/28` total entries, zero omissions/extras;
- active taxonomy: primary `0`, semantic guard `0`, evaluator guard `0`, every
  named layer count `0`;
- runtime dependencies `0`, banned vocabulary hits `0`, hidden/cfg/module
  surfaces `0`, and all retained authority hashes exact;
- all three evaluator construction paths create an empty substrate, call
  `advance_time(origin)`, and only then add the first cell;
- invariance origins and moduli exact at `0,130,260,390` / zero;
- phase-control origins and moduli exact at
  `3,6,9,133,136,139,263,266,269,393,396,399` / repeating `3,6,9`;
- row/control publication precedes aggregate assertions and phase-zero effect
  comparison is serialized;
- frozen PDF contains one page and every exhaustive inventory name; the fresh
  150-DPI render was visually inspected with no clipping, overlap, missing
  glyph, or illegible inventory entry.

## Frozen artifact hashes

| artifact | SHA-256 |
|---|---|
| static gate JSON | `423759a2f53cf28b5a0296ef8a7ac7b58edd93578f9254fbfe28fc4f0c63f7de` |
| static inventory CSV | `c385c8046eea9eac920cca8902961ba9dccff86ee3d372195aef7a2d5de587b9` |
| fresh page PNG | `37952648e7e844a7c90009c249f88302c8aec94a0dda5b1c22209d3c1a5cb5c2` |
| harness geometry JSON | `ea56af0f8accd0aeb5e185821c66fa1ff689c1239994f1006048163e7003c9cc` |
| taxonomy summary CSV | `55a318766e289645a0da947f3cdfeeac82d3c3aa39744a2a68ff910746c911db` |
| taxonomy inventory CSV | `69a462ef864cfea79596d3b4547175a0e6cd14e768f4836da344025bc28f870f` |
| taxonomy guard CSV | `b67add85e46265999a606cb81e866f3d87d56a3e55052e0f5f59036647970cb3` |

The base E2B image lacked `uv`, `pypdf`, and Poppler. The same targeted worker
installed `pypdf` and `poppler-utils` and then completed only the previously
unrun static/page steps. This was dependency completion, not a project or
matrix rerun.

The next and only authorized execution is one complete release invocation of
the frozen v2 evaluator in a new E2B sandbox from this validation commit. Any
failed phase-preserving row must be frozen negative without rescue or tuning.
