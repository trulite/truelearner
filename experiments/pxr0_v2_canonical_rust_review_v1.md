# PXR0 v2 canonical Rust review v1

Status: **REVIEW POSITIVE; EXACT KERNEL UNCHANGED; ACCEPTANCE EVIDENCE UNSPENT**.

Review target: `crates/pxr0-physical-runtime/src/lib.rs`, 474 lines, SHA-256
`f6989555f5a43dff91b39a5c7f79038168f39142fdbecca7e5e40938a72785cb`.
The review is against acceptance protocol v1 and development-ready commit
`141adae6e6512a5ca9a7f7ae72907f9c190c4f0d`.

## A. External-arrival proposal gating

**Conclusion: legitimate physical boundary law; not residual orchestration.**

`enter` creates boundary spikes with `arrow: None`. Propagation derives
`external_arrival` only from that physical provenance. If and only if such a
spike makes its target cross threshold, `propose_local_arrows` applies one
uniform position-radius/occupancy rule. The caller supplies no proposal class,
route, mechanism, success value, semantic label, or chosen endpoint.

This is not a retrospective interpretation. The frozen PX0-R protocol states
that generic proposal must be caused by “an ordinary external spike that
physically makes a cell fire” plus current local physical adjacency. The exact
gate and local rule were then retained in authoritative
`crates/lr1-modulatory-physical-return/src/lib.rs`, whose frozen SHA-256 is
`7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10`.
PXR0 copies that law without widening or selecting it.

The boundary/non-boundary distinction is causally meaningful: environmental
arrival exposes local variation opportunities, while an internally traversed
arrow does not recursively create structure at every firing. Broadening this
gate would be a new structural-variation law and is outside v2 acceptance.

## B. Region causal scope

**Conclusion: `region` is causally inert except for outward/inter-region
`Crossing` observation.**

The exact source has four responsibilities for region data: accept it in
`CellSpec`; retain it in private `Cell`; compare `from.region != to.region`
after an already-selected live outgoing traversal; and copy the two values into
the emitted `Crossing`. Region is absent from spike ordering, activation,
threshold/refractory logic, arrow eligibility, Modulation, resistance,
pressure, decay, local proposal target selection, generation checks, and queue
construction. Changing region values alone can change only whether and how an
already-occurring traversal is reported as a crossing. It cannot change the
traversal or retained body.

The values are physical boundary partitions, not visual/language/memory
modules. No region name or semantic interpretation exists in the runtime.

## C. Work and resident-byte accounting

**Conclusion: both are causally inert observer surfaces. They are valid
scientific witnesses in PXR0/PX-C, but do not belong in a later optimized
production hot path unless observation is enabled.**

`Work` is initialized locally, incremented after or alongside already-decided
operations, returned by `advance_time`/`propagate`, and never read by an `if`,
`while`, queue comparison, target choice, pressure calculation, or state
transition. Passing `&mut Work` into helpers permits counter increments only;
no helper branches on a counter. `Work::total` is a read-only sum accessor.

`resident_bytes` executes only while constructing `RunResult` after queue
exhaustion. It reads vector lengths and static type sizes, mutates nothing, and
cannot affect memory allocation, queue order, liveness, or crossings.
`naturally_quiescent` is likewise a terminal observation of an empty queue.

The counters add real instruction overhead, so the forward optimized runtime
should move them behind a causally inert observer/feature boundary and prove
observed/unobserved equivalence. PXR0 v2 intentionally retains the smallest
always-on counters and byte hook because its frozen work/memory bounds and
exact evidence ABI depend on them. Removing them now would change the hash and
evidence surface without repairing a scientific defect.

## D. CELL resistance, generation, and liveness

**Conclusion: `live` is causal; `generation` is read but currently immutable;
stored `resistance` is scaffolding after construction. Retain all three
unchanged in v2.**

- `Cell.resistance` is initialized from `CellSpec.resistance` and is never read
  or mutated afterward. The spec value independently initializes `Cell.live`
  through `spec.resistance > 0`. The retained field itself is dormant
  historical scaffolding.
- `Cell.generation` initializes to one and is never mutated. It is read when an
  arrow records source attachment generation, when an external or propagated
  spike records target generation, when a spike target is validated, and when
  outgoing arrows are filtered. Those checks are structurally causal, but
  because the runtime has no CELL retirement/replacement transition, every
  reachable CELL generation remains one; the checks are presently redundant
  stale-identity scaffolding.
- `Cell.live` initializes from positive resistance and is read when accepting
  spike targets and when selecting local proposal targets. A zero-resistance
  CELL therefore cannot execute or receive a fresh proposal. `live` is
  causally active construction state even though no later CELL liveness
  transition exists.

Removal is not an acceptance cleanup: it changes struct size, the registered
resident-byte results, source inventory text, and canonical hash. Adding CELL
deallocation would be a new law. The accepted v2 classification is therefore
explicit retention, with any compaction deferred to a separately
preregistered successor and exact equivalence study.

## E. Pressure phase

**Conclusion: pressure phase is intrinsic retained substrate state, not wall
clock housekeeping.**

`pressure_tick` begins at zero. `elapse_to(tick)` computes only complete
`ORDINARY_PRESSURE_PERIOD = 10` intervals since the retained pressure epoch,
applies that many ordinary pressure steps to live arrows, and advances
`pressure_tick` by exactly those integral periods. Empty-substrate
`advance_time(origin)` moves this epoch before topology exists.

Translations congruent modulo ten preserve the relative pressure phase when
construction and arrivals translate together. Noncongruent construction
origins leave `pressure_tick` at the preceding ten-tick epoch and may lawfully
produce different physical histories. Pause/resume retains both `tick` and
`pressure_tick`; neither is consulted from an external clock. This is the
frozen interpretation demonstrated by v2’s 16 invariant rows and 12 separate
phase controls.

## Exhaustive active inventory

### Types/state/results: 13

1. `CellId` — opaque vector identity for one retained CELL.
2. `ArrowId` — opaque vector identity for one retained ARROW.
3. `CellSpec` — generic physical CELL bootstrap fields.
4. `TransmissionMode` — physical Drive versus Modulatory transmission.
5. `ArrowSpec` — generic directed ARROW bootstrap fields.
6. `SpikeInput` — physical boundary arrival fields.
7. `Cell` — retained identity/geometry/activation/refractory/liveness state.
8. `Arrow` — retained traversal, generation, resistance, eligibility and mode.
9. `Spike` — queued arrival, deterministic ordering and provenance state.
10. `Crossing` — observer record for an ordinary inter-region traversal.
11. `Work` — causally inert bounded-work counters.
12. `RunResult` — crossings plus observer work/quiescence/resident bytes.
13. `PlasticSubstrate` — retained CELL/ARROW body, queue, time and pressure epoch.

### Functions/methods: 15

1. `Work::total` — read-only accumulated work count.
2. `PlasticSubstrate::new` — empty retained physical substrate.
3. `PlasticSubstrate::add_cell` — validate and append generic CELL state.
4. `PlasticSubstrate::add_arrow` — validate and append generic ARROW state.
5. `PlasticSubstrate::enter` — append one physical boundary arrival.
6. `PlasticSubstrate::advance_time` — apply elapsed-time physics with empty queue.
7. `PlasticSubstrate::propagate` — deterministic execution to natural quiescence.
8. `PlasticSubstrate::apply_modulatory_return` — update only live eligible arrows.
9. `PlasticSubstrate::elapse_to` — ordinary pressure, expiry pressure and decay.
10. `PlasticSubstrate::propose_local_arrows` — uniform local boundary-driven variation.
11. `PlasticSubstrate::decay_cell` — move activation toward zero with elapsed time.
12. `PlasticSubstrate::require_cell` — enforce substrate-local CELL identity.
13. `PlasticSubstrate::spike_order` — total deterministic queued-arrival order.
14. `PlasticSubstrate::resident_bytes` — read-only retained body size witness.
15. `pressure_arrow` — lower ARROW resistance and retire at zero by generation.

## Hidden-logic audit and verdict

The active file contains no Episode, History, Query, Role, Event, Cause,
Credit, Start, Finish, Answer, correctness, composite/level, named world,
fixture, seeded history, reset, cleanup, typed handoff, explicit mechanism
selector, evaluator input, semantic adapter, module, include, macro, generated
body, cfg branch, test, dependency, unsafe code, or reverse tooling call.
Generic bootstrap is physical initialization; assertions enforce local time,
identity, threshold, and queue invariants. Derives add no transition logic.

No genuine runtime defect or new-law ambiguity is present. The dormant CELL
fields and observer overhead are known compaction opportunities, not causes of
incorrect physical behavior. The exact unchanged kernel is suitable for PXR0
v2 acceptance and as the parent of a separately preregistered PX-C workflow.
