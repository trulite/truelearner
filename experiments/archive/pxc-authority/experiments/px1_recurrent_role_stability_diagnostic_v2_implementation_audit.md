# PX1 recurrent role-stability diagnostic v2 implementation audit

Status: **FROZEN IMPLEMENTATION; V2 DEVELOPMENT EVIDENCE UNSPENT**.

## Exact implementation

- active source: `crates/px0-physical-correspondence/examples/px1_recurrent_stability_diagnostic.rs`;
- SHA-256: `21480777059d7446c248e388819004807732e3c1a6949480a71c06cf3dbb1587`;
- v2 protocol commit: `0d795ee908d9ec8e2d11c023750076b996bcc092`;
- v2 protocol SHA-256:
  `c8523cf39861f8779233739151431b408caefef903846090bfe90dbc46261881`.

Relative to frozen v1 implementation
`ddadb9d4bee80c0e27bda7f67d1e0ab08a2549eb1c0dcf002c3493fbc95f415e`,
the only executable changes are:

1. fresh v2 result paths and evidence marker;
2. frozen v1 failure plus v2 protocol hash checks;
3. `parse_bool` accepts `0/1` in addition to `false/true`;
4. a codec-only exact round-trip preflight;
5. the generated report title says v2.

The margin, inhibition, distance, and timing arm code is byte-identical to v1.
No physical parameter, namespace, schedule, world construction, metric, pass
rule, timeout, or interpretation changed.

## Pre-evidence checks

- exact codec round trip: pass;
- formatting: pass;
- strict example Clippy: pass;
- focused PX0 test: `1/1` pass;
- v1-to-v2 executable diff audit: pass;
- frozen input hashes: pass;
- fresh v2 result paths: pass;
- definitive/refusal surface: unchanged and exits before world construction.

No v2 arm world or development evidence was executed during this audit.
