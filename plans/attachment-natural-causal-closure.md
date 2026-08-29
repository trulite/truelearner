# Attachment-natural causal closure

```text
action path --actual output--> world --returned transition--> attached surface
     |                                                        |
     +---------------- same physical cause -------------------+
                              |
                              v
                  existing outcome closes used path
                              |
                              v
                 repeated local cycles form one learner
```

## Outcome

Let the existing body outcome law close a live action path when a later physical
transition reaches the same surface carrying that actual output's cause, even
when the surface was attached after the body started. The returned transition
is consequence for the open cycle before it can become another action. Repeated
exact closures may form the existing learner boundary, so the completed local
loop becomes one body component without a sensor gate, motor map, fixed new
outcome source, semantic rest label, or new memory kind.

This candidate tests three claims in order. The missing transition is exact
attachment-natural path closure. The predicted transition is existing learner
formation from repeated exact local closures. The unknown capability is holding
the externally measured central region after scalar disturbances from both
sides. A failure stops dependent rungs and establishes no necessity claim.

## Authority

- Path: `arch.md`; `LANGUAGE.md`; `research/constitution.md`;
  `research/programs/learner/program.toml`; lessons `LP-021`, `LP-032`,
  `LP-049`, `LP-058`, `LP-067` through `LP-070`;
  `research/campaigns/runtime-attached-homeostatic-surface-v1/convergence.toml`;
  `plans/composable-multiscale-physical-competence.md`.
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`; `arch.md` SHA-256
  `02d837a8dc205aae7b088147226c94aa08783898a653550334718bbdf0cc003f`;
  `LANGUAGE.md` SHA-256
  `2b1954b161358c4a259198b0b9e4c66a93e47350d749d7c3baf3ddcef7bb8a41`;
  parent convergence SHA-256
  `1710b6d3680e2e3a1da5ec8a99075865c4f6ec148706769f4cb21328deed8c73`.

## Model

The objects are the body's existing open physical components. Their observable
boundary facts are a fired surface, a live participating action path, an actual
output, and a later physical transition with causal lineage. Attachment remains
ordinary boundary gluing. It allocates no causal gate and names no resident
output.

The candidate adds one opt-in transformation after a junction fires and before
new path formation or output firing. It intersects transition-bearing causal
origins with the physical identities of outputs reached by live paths from that
surface. A unique matching path whose two links actually participated is a
completed physical cycle. No match is identity. More than one match is
ambiguous and is identity. A unique match applies the existing outcome at the
path junction, records the existing causal-closure evidence over the exact live
lineage, and consumes this firing as consequence so it cannot also request an
action in the same cycle.

The categorical law is naturality of closure under attachment: closing a local
cycle and then composing with an independent attachment must equal composing
first and then closing that same cycle. Identity is an unchanged sample, an
unrelated origin, a stale transition, or an ambiguous match. Parallel
composition keeps independent closures independent. Existing learner
construction is the only coarse-graining operation: after its ordinary evidence
threshold, the local closed loop is owned as one component at the next scale.

Implement this as one new protocol successor and one small body operation using
ordinary Rust functions and an explicit diagnostic decision enum. Add no
generic category traits, recursive component wrapper, attachment registry,
sensor type, controller, or body field. Exact checkpoint support follows the
existing serialized protocol, links, consequences, closures, and learners.

## Invariants

- Default and accepted protocols are unchanged; the new law is opt-in discovery
  physics only.
- Attachment remains atomic and quiet and cannot name or inspect resident
  outputs, outcomes, learners, or paths.
- Only a `PhysicalIncidence::Transition` carried through actual causal lineage
  may close a cycle; an unchanged sample never closes, strengthens, or consumes.
- The matched origin must be the physical identity of the output reached by the
  exact live participating path from the fired surface.
- Zero matches and multiple matches are observational identity. They neither
  write consequence nor suppress ordinary input behavior.
- A closed return writes only through the existing `apply_outcome` and
  `observe_causal_closure` laws. It creates no fixed output relation and no new
  durable memory type.
- The consequence is processed before formation and firing, and a successful
  closure is consumed exactly once.
- Physical origin, transition tick, link generation, participation, learner
  ownership, replay, Production/reference behavior, natural quiescence, and
  active-frontier cost remain truthful.
- Independent attachment order changes private identities only; it cannot alter
  whether the same local cycle closes or which learner boundary forms.
- Central-region regulation remains an external measurement. No central value,
  desired direction, correct motor, evaluator result, or semantic setpoint
  enters the organism.

## Scope

- `truelearner/crates/core/src/core.rs`, `physics.rs`, `outcome.rs`, and
  `trace.rs`: one opt-in protocol, the pre-formation closure hook, exact matching,
  existing outcome/learner composition, and causal diagnostics.
- `truelearner/crates/core/tests/runtime_attachment.rs`: identity, uniqueness,
  causal matching, closure-before-action, attachment naturality, order, replay,
  and learner-boundary tests.
- `truelearner/crates/embodiment/tests/runtime_attachment.rs`: the retained
  scalar parent and conditional regulation rung.
- `research/campaigns/runtime-attached-natural-closure-v1/`: preregistered arms,
  retained logs, and convergence.
- `research/programs/learner/program.toml`: frontier result only after
  convergence.
- `factory/receipts/`: candidate and independent verification receipts.
- Excludes changes to attachment topology, sensor builders, body persistence
  fields, output-source wiring, accepted defaults, workstation or Academy
  harnesses, collective competence, morphology, keyboard use, and authority
  promotion.

## Development style

TDD. First add a tiny core fixture in which a runtime-attached surface forms an
ordinary path, an actual output cause returns as a transition, and the unchanged
parent emits another action. Add identity and ambiguity controls. Implement the
single opt-in hook until the exact return records consequence and emits no
second action. Then test repeated closure and independent attachment naturality.
Only if those pass, run the unchanged scalar regulation ladder.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --test runtime_attachment physical_cycle_closure_`
  proves exact causal pullback, consequence-before-action, sample/unrelated/
  ambiguous identity, no fixed outcome map, and checkpoint replay.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --test runtime_attachment physical_cycle_composition_`
  proves attachment naturality, independent order, repeated closure, and one
  existing learner boundary.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment regulation_attachment_natural_closure -- --ignored --exact`
  conditionally tests residence after both disturbances only after the two core
  gates pass; a negative result remains an ignored frozen research probe.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core -p truelearner-embodiment`
  preserves existing core, body, driver, categorical, and attachment behavior.
- `cargo fmt --all --manifest-path truelearner/Cargo.toml -- --check`,
  `cargo check --locked --manifest-path truelearner/Cargo.toml -p truelearner-core -p truelearner-embodiment`, and
  `cargo clippy --locked --manifest-path truelearner/Cargo.toml -p truelearner-core -p truelearner-embodiment --all-targets --all-features -- -D warnings`
  enforce the changed-crate gates.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --test runtime_attachment && cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment`.
Its measured warm duration must remain strictly under 10 seconds. Record cold
bootstrap and research probe time separately.

## Controls and evidence

- Positive software fixture: one unique live participating path, followed by an
  actual later transition carrying that output's physical cause.
- Negative control and frozen parent: unified sensor surface reaches the central boundary
  and immediately acts again.
- Identity controls: unchanged sample with the same origin, transition from an
  unrelated live origin, transition before participation, stale/deallocated
  path generation, no match, and two matching participating paths.
- Composition controls: no attachment, silent attachment, unrelated attachment
  before and after the active component, reversed registration order, exact
  checkpoint replay, and fixed active work with dormant tissue.
- Held-out cases: an unrelated modality label, a second independently attached
  component, reversed attachment order, and restore before the returned
  transition must preserve the same local closure decision.
- Missing-transition falsifiers: no consequence write, wrong path changes,
  returned transition emits another output, or a named outcome/motor relation is
  required.
- Predicted-transition falsifiers: two exact closures do not construct the
  existing learner, construct more than one learner, absorb unrelated tissue,
  or differ under attachment order.
- Unknown-capability falsifiers: either side does not enter and hold four
  central observations, any cause is lost, the run fails to quiesce, or the
  result requires a central label or chosen direction.
- Expected artifacts: lossless core closure trace, component-formation trace,
  both scalar trajectories or the first retained failure, campaign manifests,
  convergence, and valid factory receipts. These are development evidence only.

## Risks and rollback

The main risk is broad false credit from matching only an output identity. Exact
surface path, both-link participation, transition timing, live generation, and
unique-match controls kill that design. Consuming a true return may stop useful
continuation; the scalar rung will expose this immediately and its trace becomes
the next frontier rather than prompting a sensor-specific patch. Cost risk is
bounded by paths local to the fired surface and measured independently from
dormant resident storage. Rollback is deletion of the opt-in protocol and hook;
all accepted/default protocols and frozen parents remain byte-for-byte
unchanged.

## Open decisions

None.
