# CV0 bounded local contact genesis Gate D negative v1

Status: immutable static negative; contact genesis not implemented.

## Result

CV0's proposed variation form requires two fresh ordinary contact CELLs per
local opportunity:

```text
             C+ --(+1)--> X
            /
P ----------
            \
             C- --(-1)--> X
```

The frozen Gate D asks whether unsupported `C+` and `C-` already have an
ordinary physical path to disappear and become reusable after their weak
ARROWs decay. The accepted substrate does not provide that path.

Current CELL behavior is:

```text
construction
    resistance copied from CellSpec
    live = resistance > 0

ordinary time
    activation relaxes toward zero
    refractory state advances

but
    CELL resistance never changes
    CELL live never becomes false
    CELL generation never advances
    CELL identity/slot is never reused
```

By contrast, ordinary ARROW local forgetting reduces resistance, marks an
ARROW non-live at zero, advances generation, clears transient state, and makes
its resident slot reusable.

Therefore implementing the proposed contact form on the current substrate
would produce permanent orphan CELLs after unsupported candidate ARROWs die.
Repeated opportunities would consume CELL capacity monotonically. This fails
the bounded-creation/reclamation requirement before sign selection can be
tested.

Under the preregistered stop rule, no contact-genesis implementation,
evaluator, runtime world, CELL cleanup, Rust compilation, or organism run was
created.

## Frozen static evidence

The evidence audit ran in fresh E2B worker `iv52x0ubcrh6htz58uztw` from
corrected audit commit `492f5e9`:

```text
gate_d=negative
cell_construction_sets_live_from_resistance=true
cell_decay_relaxes_activation_only=true
cell_lifetime_evolution=false
cell_deallocation=false
cell_slot_reuse=false
arrow_deallocation=true
contact_genesis_implemented=false
runtime_selection_gates_constructed=false
CV0_BOUNDED_LOCAL_CONTACT_GENESIS_GATE_D_STATIC_NEGATIVE_V1
```

Frozen hashes:

```text
core lib.rs  b6b7f2a47818d84ac2fd69aab466f5f917e6d3ba7cfc8f8c5db4ce91b97fbae5
core Cargo   4cb6d665d738cdea61f928975fa34ddf89d62aa9150420748d94d574ed731aeb
audit script d179a8ca7b93df4832f7b605c5523222c466cbdf0e83c6282c61669fa5c5639a
audit output b463fe14b5cb9093f67d12fe8adaaec9c838cef1f80b540117058507ff38d3e5
```

Two pre-evidence technical attempts are excluded from the result. Fresh worker
`ikpm1lh8wivss9bwp1rt4` produced an empty artifact because an outer `tee`
masked a source-sentinel failure. The empty file was deleted. Fresh worker
`i4uenanhzr1acz2064uh5` localized that failure to a pattern that accidentally
matched `active_cells.remove`; the sentinel was corrected and committed before
the successful audit. Neither attempt changed or executed organism physics.

## Unrun gates

CV0 did not construct or evaluate:

- signed contact symmetry;
- unsupported contact/ARROW runtime decay;
- bounded repeated contact creation;
- positive-only or negative-only consequence selection;
- identity, slot, position, or usefulness permutation;
- both-useful or neither-useful selection;
- the shared-contact SV0 control; or
- Reference/Production history, replay, and quiescence.

## Classification

CV0 does not falsify contact-compartment variation or signed consequence
selection. It identifies a prior substrate deficiency:

> The organism can forget and reuse ordinary ARROW structure, but it cannot
> forget or reuse ordinary CELL structure.

The next independent question is CELL lifetime. Any successor must discover a
general ordinary physical persistence/deallocation law for CELLs, not add
orphan detection or contact-specific garbage collection. CV0 selects and
authorizes no such law.
