# RS2 learned inhibitory topology implementation audit v2

Status: frozen before RS2 v2 evidence.

Protocol: `2cb8028` (`rs2-learned-inhibitory-topology-protocol-v2`).

## Surface

RS2 adds one evaluator-only package. `truelearner-core` is unchanged from the
CV0/J0 positive candidate.

The evaluator enables accepted `cv0j0` variation plus the causally inert RS0
observation ceiling. It constructs only boundary anchors and the ordinary
recurrent probe topology. Signed candidate junctions and links are generated
exclusively by the organism's accepted variation operation.

Training returns ordinary Modulation to a generated contact. The probe adds no
Modulation and observes whether the frozen selected relation changes recurrent
execution. Periodic controls are paused at 64 scheduled deliveries rather than
being terminated by forgetting.

The nine preregistered families cover learned negative stabilization, no
Modulation, candidate identity/sign order, location translation, irrelevant
negative alternatives, useful positive selection, disconnected inhibition,
untraversed inhibition, and fresh recurrence packing.

## Targeted validation

Reusable E2B worker `ifk44bxtlfjlci644r63m`:

- package rustfmt check: PASS;
- release `cargo check`: PASS;
- strict release Clippy (`-D warnings`): PASS.

No RS2 v2 physical matrix had executed when this audit was frozen.

