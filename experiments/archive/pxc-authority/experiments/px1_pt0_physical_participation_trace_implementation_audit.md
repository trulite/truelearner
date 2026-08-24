# PX1-PT0 physical participation-trace implementation audit

Status: **FROZEN IMPLEMENTATION; DEVELOPMENT EVIDENCE UNSPENT**.

- protocol commit: `639169dc7a30b902f703745cfd14bf4ce8d12e00`;
- protocol SHA-256:
  `9c0e26e9b81cf41c91a5f6044969569ffe43e110e5db835dafd26d18f0eec17a`;
- implementation:
  `crates/px0-physical-correspondence/examples/px1_pt0_physical_participation_trace.rs`;
- implementation SHA-256:
  `f0b754ed6f7b0603668319a0735da91b4c168f909d4024fd5ce5e2aea4197410`;
- active PX0 law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

## Physical construction

Three anonymous branch cells each have one identical weak outgoing
continuation. Branch and effect thresholds are physical scalar thresholds.
When a branch fires, the frozen substrate marks its emitted arrow eligible for
the existing four-tick local window.

Executed effects reach one ordinary return hub. The hub broadcasts an
identical impulse to all three branch cells. The return contains no branch
identity or provenance field. A branch receiving one return impulse remains
subthreshold; the impulse can change an outgoing arrow only if that arrow's own
recent physical eligibility is live.

The third branch is an always-nearby nonparticipating opportunity. Joint worlds
permit two branches to participate and mature together. Late and no-return
worlds alter only physical delay/path availability.

## Validation

- formatting: pass;
- strict Clippy: pass;
- focused PX0 test: `1/1` pass;
- frozen parent hashes: pass;
- semantic/type/chooser source audit: pass;
- PROBE and MICRO result paths absent: pass;
- definitive/refusal path exits `2` before world construction: pass.

No PT0 physical world was executed during implementation validation. PX1
remains non-authoritative.

