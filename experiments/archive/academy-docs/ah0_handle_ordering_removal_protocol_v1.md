# AH0 handle-ordering removal protocol v1

Status: frozen before AH0 implementation.

Parent: SI0 v2 development-positive `3f889bc`.

## Question

Can every causal dependency on numeric `CellId` / `ArrowId` ordering be
removed while preserving SI0 and the retained physical behavior?

AH0 adds no physical law. It is an architectural hardening and source-shape
refactor.

## Opaque-handle boundary

Causal code may use handles only for:

- equality and membership;
- hashing;
- generation-safe lookup;
- slot resolution;
- reference construction.

Causal code may not use handle `<`/`>`, min/max, ordered sets/maps, sorting,
or tie-breaking. Scheduler selection may use physical time, phase, SI0 causal
wave, and mechanically assigned serial where the accepted non-SI0 lineage
still requires sequential observation. Under SI0, serial cannot affect a
same-wave local incidence because the complete wave is drained before physics.

Ordering numeric handles remains allowed only in explicitly named canonical or
mechanical-layout operations:

- arena/checkpoint serialization and hashing;
- durable decode/validation;
- debug/test normalization;
- deliberate resident compaction/packing.

## Authorized causal refactor

- Remove origin/target identity from active scheduler keys.
- Replace ordered active/required handle sets with equality/hash membership.
- Remove ARROW-handle sorting from firing, modulation, qualified-local
  transmission, adjacency reconstruction, and proposal execution.
- Where sequential iteration is still needed, order only by ordinary physical
  fields such as tick, phase, delay, geometry, mode, trigger, coupling,
  resistance, participation, and generation. Exact physical ties must not be
  distinguished by handles.
- Keep SI0 Drive incidence and causal-wave law unchanged.

## One-file runtime closure

After hardening, the complete active runtime implementation must again reside
in `truelearner/crates/core/src/lib.rs`. The current `mechanics.rs` content is
inlined as a private module and the separate file is deleted. `arena-format`
remains separate because it owns durable representation, not active physics.

This source-shape change must not alter behavior.

## Gates

1. Static audit of the active causal runtime; every surviving numeric-handle
   ordering site must be classified storage/debug/layout-only.
2. SI0 v2 unchanged: `120/120`.
3. R1-R5 mechanical differential and R6 partition invariance.
4. Retained CPC0, CPC1, PQLC0, PQLC1, FD0, FD1, J0, CV0/J0, and SV1
   scheduling/topology lineages using their already frozen worlds.
5. Representative arbitrary handle renamings map back to identical physical
   histories.
6. Reference/Production equality, replay, quiescence, and frozen physical work
   wherever each retained evaluator already asserts them.
7. Exactly one active core Rust source file and no runtime dependencies added.

Development validation may compile and enumerate tests but may not execute the
frozen cumulative evidence matrix. The final AH0 evidence run is one fresh E2B
execution. Scientific advancement is prefix-based and stops at the first
failed retained gate.

AH0 does not rerun RS2, advance CE1/FD2/ARC, change authority/oracle status, or
edit `arch.md`.
