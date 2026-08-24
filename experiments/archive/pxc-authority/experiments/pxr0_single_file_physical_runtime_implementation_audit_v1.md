# PXR0 single-file physical-runtime implementation audit v1

Status: **FROZEN CANDIDATE; DEVELOPMENT VALIDATION UNSPENT; NO AUTHORITY CLAIM**.

Parent authority is exact commit
`9ba11aeb4f88d6707cfba1afdb5f6dce3e380b9f`. Protocol commit/tag is
`96e61f4a9fd289d54e05f51038ba598f34880a8a` /
`pxr0-single-file-physical-runtime-protocol-v1`.

## Candidate surface

The production closure is the single 474-line file
`crates/pxr0-physical-runtime/src/lib.rs`. It contains 13 types and 15
functions/methods. Its Cargo manifest has no dependency. All world geometry,
choreography, inspection, result publication, page rendering, and conformance
logic is tooling outside the active manifest.

| frozen input | SHA-256 |
|---|---|
| PXR0 runtime source | `f6989555f5a43dff91b39a5c7f79038168f39142fdbecca7e5e40938a72785cb` |
| PXR0 Cargo manifest | `0755bf5663f943cf1343dabf70862291c042d0aefe5bd75b8ecf65549e0038c2` |
| one-file active manifest | `fc68a856cfd4524c55aef098927c7ad1bc1da628f879dce3270d2c741199998d` |
| exhaustive spec source | `ccd1509df27ba09f4420dfec18585fe10ae787426bd76bba89ea14e9d1832462` |
| readiness evaluator | `d892aaa8bdc3da82dbf6d9ced8c8cd86a3a65fdafc349ba99b9f0635e4654449` |
| evaluator Cargo manifest | `e8597c10378228dd750fc563d8f0badbf8382eba0432b98872788e0a6be0b2b1` |
| static/page gate | `af9dceaf0dc8e043a4e766256ebfcfb2e555445cc6c7be0d1e53997f8a7f1932` |
| extraction-map audit | `ce65449d8fd53323220c334f3c0a09b06ad95a8642ad868c845f6f0d5d1af090` |
| PDF renderer | `b16f554d983fe9dfa91219480b090f449fa78448245d59b6065424477b62f582` |

## Law-preservation audit

The six LR-C constants and all state mutations retain their authoritative
values and order. Queue selection remains arrival tick, phase, physical origin,
target physical identity, then serial. Generation/live validation precedes
delivery. Drive retains activation, threshold, refractory, eligibility and
emission behavior. Qualified Modulation retains resistance/coupling changes.
Elapsed time retains ordinary and expired-use pressure, deallocation and cell
decay. Local proposals retain radius, physical-ID order, delay, resistance,
generation and Drive mode.

Only diagnostic representation changes: the detailed ledger becomes a total
plus five conformance counters; trace and fingerprint materialization is
removed; resident bytes is returned with each run. These do not feed execution.
The fresh behavioral matrix is required to reject any outcome drift.

## Frozen validation spend

Exactly one targeted E2B validation may now format-check and Clippy the two
packages, generate the 101-to-28 movement map, run taxonomy with zero ceilings,
mark and render the one-page PDF, render it to PNG for visual inspection,
reconcile the PDF/source/dependency gates, and execute the 16-row internally
replayed readiness matrix. The candidate source may not change after this
point. Positive results establish development readiness only.
