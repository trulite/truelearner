# RS2 learned inhibitory topology implementation audit v5

Status: frozen before RS2 v5 evidence.

Protocol: `f7dcacf` (`rs2-learned-inhibitory-topology-protocol-v5`).

## Exact delta

The substrate is byte-identical to AH0 development-ready parent `6ab8a15`.
RS2 v5 changes only the existing RS2 evaluator:

- enables the already-frozen AH0/SI0 causal-wave baseline;
- maps runtime CELL/ARROW handles and physical identities to logical world
  names before comparison;
- compares physical transition multisets per tick/phase while retaining SI0
  causal-wave labels inside Drive incidences;
- retains raw ordered trace, raw durable-body, and raw live-checkpoint hashes as
  diagnostics only;
- compares normalized durable bodies, clock, pending native continuation, and
  identical-future-input continuation;
- explicitly compares the identity/insertion permutation family to its
  unpermuted learned-negative baseline after logical renaming.

Training geometry, generated signed topology, consequence timing,
consolidation predicates, probe recurrence, nine families, roots, phases, and
observation ceiling are unchanged from RS2 v4.

No runtime, learning, proposal, inhibition, recurrence, checkpoint, or
serialization law changed.

## Targeted validation

Reusable E2B Rust worker `ifk44bxtlfjlci644r63m`, committed evaluator
`52500baa9395fa30738bdc8d08ca15578537c017`:

- targeted rustfmt check: PASS;
- targeted release `cargo check`: PASS;
- targeted strict release Clippy: PASS.

The RS2 v5 matrix had not executed when this audit was frozen.
