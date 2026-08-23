# CJ1-PA participation/amplitude geometry implementation audit v1

Status: **IMPLEMENTED AND FROZEN; EVIDENCE UNSPENT**.

## Frozen source

- authoritative PX0 SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- authoritative PX1 definitive implementation SHA-256:
  `74716c87d146cb697b37ddf802c12e67a5cb93daf82ec20f8b982e54922bd696`;
- protocol SHA-256:
  `c5827a66f693a2bd6a3558a2fbcece5b38ae1f1c5a66eedcc25bb97d37853abd`;
- `arms/cj1-pa-trace-amplitude/Cargo.toml` SHA-256:
  `13ff34306efa75ce3c400d44050149a6d18db87ad61425441eafb284e1b4bd12`;
- `arms/cj1-pa-trace-amplitude/src/main.rs` SHA-256:
  `80505822b6ba4a53b6b617c6026c13d1c0735a8dac89cae762a292726cca4a5e`.

The arm directly imports the authoritative PX0 crate. It has no build script,
generated substrate, wrapper or alternate law. At runtime it also refuses drift
in the frozen PX1 PT0 source, definitive implementation, definitive CSV, result
audit, authority handoff and this protocol.

## Physical composition audit

The topology is composed only from ordinary PX0 CELLs and ARROWs and preserves
PX1 PT1's discriminating relation:

- one actual outlet firing sends one fixed coupling-one impulse to its local
  threshold-two trace CELL;
- the same outlet firing sends one fixed coupling-one impulse to a shared
  threshold-one return hub;
- one hub firing broadcasts the same unit return to both trace CELLs;
- only a trace CELL that also received its outlet's participation impulse can
  fire;
- each trace firing sends one fixed coupling-one impulse to the shared
  threshold-two conjunction CELL.

Only source->outlet coupling varies by scenario. Every edge after the outlet is
unit coupling. Physical regions are assigned so every discriminating ARROW
traversal is independently present in the native crossing ledger. CELL
positions are separated beyond generic proposal radius.

The six worlds are fresh and each is reconstructed twice. The runner
conjunctively checks actual source firing, raw traversal/amplitude, outlet
firing, unit participation traversal, shared return, trace arrival/firing,
unit conjunction traversal, conjunction firing, replay and quiescence. No
scheduled input substitutes for a firing or traversal.

The executable exposes one command and four create-new result/staging paths. It
contains no amplitude clamp, identity/count operation, contributor storage,
candidate, MICRO/GATE, definitive, authority, PX3 or PX-C surface. The first
E2B preflight refused on formatting before compilation; this audit incorporates
only that exact formatter diff and supersedes the unexecuted `fabf0f0`
implementation snapshot. No evidence ran while this audit was prepared.
