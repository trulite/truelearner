# FD0 phase-free local forgetting implementation audit v1

Status: evidence-eligible implementation freeze.

Protocol: `fd0-phase-free-local-forgetting-protocol-v1`.

## Active candidate

The predecessor global pressure-epoch loop is no longer called. `elapse_to`
invokes one local elapsed-time decay function. For every live ARROW, that
function:

1. relaxes participation independently using the unchanged CPC1 law;
2. adds elapsed ticks to ARROW-local `decay_load`;
3. converts each ten load units into one resistance loss;
4. deallocates at zero and increments generation;
5. counts only the physical ticks for which the structure remained live.

The decay arithmetic contains no read of participation, plastic support,
transmission mode/trigger, capacity, world identity, ARC state, or Modulation
timing. Traversal changes participation but neither resistance nor decay load.

The predecessor transient checkpoint slot formerly named `pressure_load` is
renamed `decay_load`; its byte representation is unchanged. The old public
`local_pressure_load` method remains only as an evaluator compatibility alias
to `local_decay_load` and has no causal use.

## Dormant compatibility state

`pressure_tick`, `pressure_phase`, and `pressure_epoch` remain solely in
construction/checkpoint compatibility paths. The candidate decay function does
not read them. FD0 therefore tests removal of their causal role without yet
claiming the source/API retirement reserved after FD1.

## Static boundaries

- no ARROW creation/expiry timestamp;
- no global modulo/epoch branch in local decay;
- no participation/eligibility shield or attenuation;
- no decay-load reset on traversal;
- no resource-capacity or scarcity branch;
- no change to Modulation, proposal, QLP, scheduling, or mechanics selection;
- no ARC or evaluator-derived input in active substrate code.

## Frozen surface

- core: `truelearner/crates/core/src/lib.rs`
  - SHA-256 `268fdce9c326f9e18c434d5eb36f11f436791ed295e4cb42da9d0e29ae22174d`
- resident mechanics: `truelearner/crates/core/src/mechanics.rs`
  - SHA-256 `297775ee625d55e116adb92c9f6906c8a5da40e8533bce2fa71cf7bf4b002947`
- evaluator: `experiments/arms/fd0-phase-free-local-forgetting/src/main.rs`
  - SHA-256 `f98e151cb507c2e153f73289969dd8561f5681a6cc133d101254ca58e62ae5ce`

Reusable E2B worker `i1uo1iw01zrr9o3b3cki5` passed targeted rustfmt,
release check, and strict Clippy for the core and evaluator. It did not execute
an FD0 physical world.
