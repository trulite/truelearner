# CJ0 Arm A coincidence-threshold CELL PROBE v3 implementation audit

Status: **IMPLEMENTED FINAL MECHANICAL RETRY; EVIDENCE UNSPENT**.

- v2 invalid commit/tag: `6690d69493d8673b6ce34d6c2a5f9424ff14606d`,
  `cj0-a-coincidence-threshold-probe-v2-invalid`;
- v2 invalid-audit SHA-256:
  `1ad71e8064b412c4e6fff28cfd50f7c758466f045e1a184aff969c66dff3474c`;
- v3 protocol commit/tag: `a114915`,
  `cj0-a-coincidence-threshold-probe-v3-protocol`;
- v3 protocol SHA-256:
  `99ca2690e474ce79e0c761389ff95e50075341218f93dabf02c056b6b680331f`;
- v3 source SHA-256:
  `c9b8747d39f6cfee66fd4ed7e972970b7a67f6ba9033c995d3d0fe8c262e453b`;
- unchanged organism-visible block SHA-256:
  `179737cd7dbe8a45257c4d77afe41505ced671bcb653e9494976b4540103ca87`;
- unchanged authoritative law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

The sole implementation change enqueues both already-registered burst
occurrences before one ordinary propagation call. The same helper stages
genuine bootstrap, changed-world bootstrap, repeated-singleton self-evidence,
three-way ambiguity, four-way activity, and absent-opportunity controls.
Physical timestamps, inputs, topology, claims, and parameters are unchanged.

Formatting, focused compilation, strict Clippy, no-argument/wrong-argument
exit `2`, no-CELL preflight, frozen source/tag/negative audits, forbidden-token
scan, result/staging absence, and diff whitespace pass. No development stage
beyond PROBE has been entered. No definitive or authority surface exists.
