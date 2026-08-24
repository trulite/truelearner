# PX3-R6 implementation audit v1

Status: **FROZEN; E2B PREFLIGHT PASSED; EVIDENCE UNSPENT**.

- source commit: `97c37458427e18e401e3fcc7818a2b308229f56e`;
- manifest SHA-256: `27b7be2176022a2c6d18f3a278e47e7052c539ec48ad78ae7e166b6d49e409d9`;
- source SHA-256: `12d9422cc43d43a88da9d8046a2ab7fbdc8f9447236e97bc73483d0d4ce7eb4f`;
- protocol SHA-256: `25a30a95c629d3e67665ee70f7f42024f6afff5996e4cc82b83e1511460ea6ce`;
- execution protocol SHA-256: `73168bcb69e0a8e846289653f08a0678f224a5fce69b3cc4ccfea8a731624bf3`.

R6 uses only authoritative PX0. Separate FP/FX threshold-two cells hold
ordinary unit footprints. Only the normalized R trace supplies the second unit
that can make them fire; only FP/FX firings reach M. The CJ-B gated-ARROW law
is neither copied nor depended upon. No evaluator read, state mutation,
provenance field, event boundary or semantic return surface exists.

E2B sandbox `i6x9gykt9tvp6xfz5z8ra` passed formatting, 2/2 release tests,
strict Clippy and non-propagating preflight. Artifacts were absent. Preflight
constructed no world and emitted no evidence marker. No Rust ran locally.

The next tagged commit containing this audit is the sole evidence target.
