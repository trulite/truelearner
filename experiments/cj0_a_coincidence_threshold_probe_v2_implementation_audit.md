# CJ0 Arm A coincidence-threshold CELL PROBE v2 implementation audit

Status: **IMPLEMENTED MECHANICAL RETRY; EVIDENCE UNSPENT**.

- v1 invalid commit/tag: `62e8774466bfeaa214302fe95b61960c1085d0b7`,
  `cj0-a-coincidence-threshold-probe-v1-invalid`;
- v1 invalid-audit SHA-256:
  `96c0816e6310661243bb1abd04452128d24f0aabe3ca46906297c1d84b2d0f23`;
- v2 protocol commit/tag: `3363cf2`,
  `cj0-a-coincidence-threshold-probe-v2-protocol`;
- v2 protocol SHA-256:
  `0f2918c94f79a1f300240fde0b6a2f1c5adb3c0f334aa27dd01e679b8c56a5c1`;
- corrected source SHA-256:
  `e2af0b54cf92746db97252c46a1e745abafd070e02df8c6448c7ac5a55b013f9`;
- unchanged organism-visible physical block SHA-256:
  `179737cd7dbe8a45257c4d77afe41505ced671bcb653e9494976b4540103ca87`;
- unchanged authoritative law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

The diff from the frozen v1 implementation changes only mode/result names,
frozen-v1/v2 audit constants, and the external post-bootstrap schedule. The
first weak occurrences remain at consecutive ticks. Five later supported
uses replace the mechanically impossible consecutive uses and preserve exact
route marginals. Physical construction, candidate law, threshold, identities,
ordered claims, controls, and evaluator isolation are unchanged.

Pre-evidence formatting, focused compilation, strict Clippy, no-argument and
wrong-argument exit `2`, no-CELL preflight, frozen source/tag/negative endpoint
checks, forbidden-token audit, result/staging absence, and diff whitespace all
pass. No v2 evidence marker or result exists. MICRO, GATE, recursion, OR/timing,
definitive evidence, and authority remain unexecuted.
