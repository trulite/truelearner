# PX1 recurrent role-stability diagnostic implementation audit

Status: **FROZEN IMPLEMENTATION; DEVELOPMENT EVIDENCE UNSPENT**.

## Frozen implementation

- source: `crates/px0-physical-correspondence/examples/px1_recurrent_stability_diagnostic.rs`
- SHA-256: `ddadb9d4bee80c0e27bda7f67d1e0ab08a2549eb1c0dcf002c3493fbc95f415e`
- protocol commit: `6e7440ab131cc4131b9958e742be1f90a6a2e90d`
- protocol SHA-256: `934dcd65a34d5bccb29915c814e8a7873db745b1136ac388c33d19db497860eb`

The active PX0 physical correspondence law remains byte-identical at
`3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.
The frozen PX1 PROBE v1 implementation, negative, and collapse handoff remain
byte-identical.

## Arm implementation

- margin changes only the source firing threshold and matching anonymous
  external spike count;
- inhibition exposes an endpoint-local weak brake opportunity and an ordinary
  inhibitory brake-to-source arrow;
- distance places role formation behind a fixed physical relay outside the
  immediate reciprocal proposal radius;
- timing delays only the role-return legs beyond the frozen local eligibility
  window.

Every arm uses the same frozen PX0 proposal, return, pressure, propagation, and
deallocation law. Primary and fresh mirrored-transfer worlds use separate
physical namespaces and allocation orders.

## Readout audit

The evaluator subtracts known external source arrivals and the single expected
feed-forward learning-site arrival before counting internal positive return.
Negative inhibitory arrivals therefore cannot satisfy the useful-return
criterion. Source refiring is counted separately from externally initiated
firings. A quiet arm without mature role structure fails.

The productive-recurrence control requires a post-gap external activation to
traverse retained physical structure, create an outward crossing, complete
positive internal return, and naturally quiesce. Exact duplicate executions
must match in trace, crossings, work, and resulting complete fingerprint.

## Bounded execution

The parent process starts the four arm processes independently. An arm that
does not exit within five wall-clock seconds is killed by the evaluator and
serialized as a timeout/non-quiescent failure. A timeout cannot change another
arm's organism state.

Only `--diagnostic` can enter the parent harness. Any other parent invocation,
including `--definitive`, exits `2` before constructing a world. Result files
use create-new writes and are absent at freeze time.

## Pre-evidence validation

- `cargo fmt --all`: pass;
- strict Clippy for the diagnostic example: pass;
- focused `px0-physical-correspondence` test: `1/1` pass;
- frozen source/artifact hash audit: pass;
- forbidden semantic/type/chooser source audit: pass;
- result-path freshness: pass.

No arm world, PROBE retry, MICRO, GATE, or definitive evidence was executed
during implementation validation. PX1 remains non-authoritative.
