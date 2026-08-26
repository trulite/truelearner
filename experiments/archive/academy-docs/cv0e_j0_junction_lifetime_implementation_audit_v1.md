# CV0-E/J0 junction-lifetime implementation audit v1

Status: frozen before Gate E evidence.

Parent protocol: `a62ddc7` (`cv0e-j0-junction-lifetime-protocol-v1`).

## Cumulative implementation

- The existing CV0 local proposal operation is unchanged: one eligible local
  relation opportunity creates two ordinary junction CELLs and four ordinary
  Drive ARROWs, with outgoing couplings `+1` and `-1`.
- A new compile feature, `cv0j0 = ["j0"]`, exposes that already-implemented
  proposal operation on the J0 lineage.
- The historical `cv0 = ["cc0"]` feature remains unchanged for exact lineage
  replay.
- The CV0-J0 evaluator enables only `cv0j0`; `cc0` is not in its feature graph.
- J0 consequence processing can therefore update participating incident Drive
  ARROW resistance, but the CC0 CELL-resistance update and its work/event fields
  are absent from this executable.
- Junction retention, orphan deallocation, generation invalidation, and slot
  reuse are inherited unchanged from J0.

## Frozen evaluator stages

The evaluator accepts exactly two stages:

- `gate-e`: the preregistered positive `+1` selection/re-execution gate only;
- `full`: cumulative Gates A--I after Gate E has separately frozen positive.

Every case executes Reference twice and Production twice. The frozen equality
contract includes ordered physical trace, physical work, final body hash,
clock, replay, and natural quiescence. It does not compare implementation cost.

## Targeted validation

E2B reusable Rust worker: `ifk44bxtlfjlci644r63m`.

- package rustfmt check: PASS;
- release `cargo check`: PASS;
- strict release Clippy (`-D warnings`): PASS.

The only compile correction removed a stale CC0 evaluator read of CELL
participation. The J0 evaluator now checks participation on the four generated
links. No runtime law changed.

No Gate E or full CV0 evidence had executed when this audit was frozen.

