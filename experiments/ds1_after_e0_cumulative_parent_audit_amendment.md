# DS1-on-DS-E0 parent-audit amendment

Status: audit/evidence correction only. The original preregistration commit/tag
and collapse handoff commit/tag remain preserved. M0, DS-E0, E0-B, the
serializer, frozen DS1 learner, protocol, and physical/consequence mechanisms
are unchanged.

## Correction

The original handoff constructed `SeedCompositionAudit` with a literal absent
action-surface value. That recorded the intended conclusion but did not derive
it from the exact frozen substrate. This amendment replaces the literal with
an `ActionSurfaceInventory` computed from `include_str!` views of exact frozen
`src/ds_e0_anonymous_event_formation.rs`, `src/ffs_same0.rs`, and
`src/ffs_same0/cs0a.rs`.

The inventory extracts function signatures and balanced-brace struct surfaces,
counts definitions and method calls, and tests structural compatibility. It
does not invoke, modify, or widen a frozen interface.

## Exact type and data-flow evidence

| Surface | Exact derived evidence | Classification |
|---|---|---|
| frozen DS1 choice | one `fn choose`; zero `.choose(` call sites in the complete DS-E0 execution source | defined but unreachable |
| frozen DS1 consequence | one `fn apply_consequence`; zero `.apply_consequence(` call sites | defined but unreachable |
| frozen read-only query | one `fn frozen_choice`; one `.frozen_choice(` call on the just-serialized probe | reachable read-only consumption |
| E0 proposal | one `self.work.proposals += 1` site inside `acquire_episode` | event-formation proposal only |
| E0 propagation | `RawActivity { propagation: Vec<Propagation> }` plus `Candidate { shape: RelationShape }` | observed event evidence; not an action value |
| E0 callback | `acquire_episode<F>(..., mut consequence: F)` with `F: FnMut(&[Occurrence; 3]) -> bool` | membership credit during E0 formation; precedes DS1 choice |
| composition lifetime | owned `SeedReport` and `GateReport`; neither struct surface contains a reference | post-run summaries; no live action handle/callback |
| action pairs at boundary | zero two-element pair fields in either public report surface | no action alternatives exposed |
| M0 execution | `execute_resolution`, `execute`, and `execute_compiled_or_generic` | three correspondence/arrow execution surfaces |
| DS1-compatible M0 execution | zero signatures accepting `Neighborhood`, `choice: usize`, or `[usize; 2]` | type-incompatible with DS1 choice |
| cross-mechanism mapping | zero reachable DS1-choice-to-M0/DS-E0 execution edges | no selected-action path |
| natural consequence | zero reachable post-action consequence edges | no compatible route-contingent return path |

M0's `execute_resolution` consumes private `Resolution` plus `ArrowStore`,
roots, work, and `Environment`; the compiled wrapper consumes private
correspondence store/rules and an evaluator episode. These are actual M0
execution APIs, but none receives a DS1 choice or exports two anonymous actions
to the composition boundary. E0's boolean callback consumes a proposed current
membership before `Neighborhood` serialization and therefore cannot be a
consequence of an uncalled DS1 action.

The derived availability predicate requires all of: a reachable DS1 `choose`,
an exposed pair value, a structurally compatible M0 execution signature, and a
choice-to-execution edge. Each seed copies that derived predicate; construction
contains no literal `false`.

## Guard tests

Two new focused tests:

1. freeze all exact inventory counts and require the absence proof;
2. reject a stage-4-absent report if any exported action pair,
   DS1-compatible M0 execution surface, choice-to-execution path, or natural
   post-action consequence path becomes nonzero.

Together with the existing three tests, five focused library tests pass
locally. Release MICRO/GATE report the identical inventory for every seed and
retain the exact stage-4 collapse. `--definitive` still rejects before the
harness, and the results digest remains unchanged.

## Validation and freeze

Local targeted validation: PASS. Clean committed-snapshot E2B validation:
PENDING before the amendment tag. The dedicated persistent sandbox will be
reused, never killed, and left running.

Scientific first-collapse: **unchanged at stage 4**. This amendment strengthens
only the evidence supporting absence; it adds no action, mapping, consequence,
learner acquisition, or result artifact.
