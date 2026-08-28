```text
external incidence
    |
    +-- sample -----------------------> ordinary perception and path firing
    |
    `-- physical transition
             |
             v
     causal lineage carries transition tick
             |
             v
     transition tick > return opening?
          /                         \
        yes                          no / absent
         |                                |
   admit consequence              typed rejection
   and allow closure              return unanswered
         |                                |
         v                                v
   continuation after change      existing alternative release
```

# Distinguish physical transition from unchanged resampling

## Outcome

An opt-in recursive learner protocol admits a return origin only when its live
causal lineage contains a physical transition strictly after the exact return was
opened. Ordinary samples still drive perception and path execution but cannot
renew consequence. The unchanged reflected-hand world marks its already existing
movement-contingent physical return as a transition and marks ordinary active
surface polling as sampling, then measures boundary release and complete control.

## Authority

- Path: `research/campaigns/hand-consequence-born-return-admission-v1/convergence.toml`
- Revision: `sha256:8e7a19e7e53d1a5e8a33d670b5ea5a61cceb8ef763419dca51f7d56f26289b65`

## Model

`PhysicalInput` pairs the existing `Input` with either `Sample` or `Transition`.
Both enter the same drive pipeline. For every anonymous physical origin,
`CausalLineage` preserves its incidence birth and an optional transition tick.
Lineage composition keeps the latest observed birth and transition for that
origin; selection preserves both together.

The new protocol first applies the established birth-after-opening gate. It then
defines consequence admission only when `transition_tick > return.opened_tick`.
An absent or non-later transition produces `RejectedUnchangedSample`, writes no
consequence, performs no reverse consolidation or closure, and leaves the return
unanswered. Accepted transition lineage composes through the existing consequence,
closure, cohort, candidate, and release laws. Input classification is an explicit
world boundary; the learner receives neither the world state nor its interpretation.

## Invariants

- Samples and transitions have identical drive, path-formation, and firing effects;
  only consequence eligibility differs in the opt-in protocol.
- A transition fact is created only at external physical incidence and is carried
  by actual causal propagation; the core never reconstructs it from time or output.
- The reflected hand exposes only an anonymous movement-contingent transition. It
  does not expose position, direction, limit, contact, desired motion, or hand ID.
- The hand's ordinary active-channel polling remains `Sample`; unchanged polling
  cannot answer, write, consolidate, close, or renew a return.
- Existing strict birth-after-opening admission remains required before transition
  admission, and all consequence consumers use the same accepted lineage.
- Existing protocols and default input APIs remain behaviorally unchanged.
- Checkpoint/replay preserves transition lineage exactly and execution remains
  naturally quiescent within the existing propagation bound.

## Scope

- Extend core input, causal lineage, tracing, protocol selection, consequence
  admission, checkpoint coverage, and focused boundary tests.
- Extend the existing hand adapter so its already movement-contingent pending return
  is a transition while recurring active surfaces remain samples.
- Add one experiment and discovery campaign with matched changed/resampled fixtures,
  the aliased-surface counterexample, reflected hand, controls, and convergence.
- Add candidate and verification receipts plus the resulting durable lesson/frontier.
- Exclude strength, ranking, learner memory, path lifetime, capacities, semantic
  sensor identities, hidden evaluator state inside the learner, default adoption,
  commits, and authority promotion.

## Development style

TDD. Add a core fixture in which one sampled and one transitioned occurrence share
the same target, physical origin, and later timing; require only the transition to
answer. Then add lineage transport and the smallest pre-admission gate. Finally mark
the hand adapter's pre-existing movement return and run the unchanged hand.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml --test harness_boundary physical_transition_return`
  proves matched sample rejection, transition admission, no rejected write, delayed
  transition transport, checkpoint replay, and unchanged old-protocol behavior.
- `cargo test --locked --manifest-path research/experiments/hand-physical-change-resampling/Cargo.toml`
  proves existing timing/lineage insufficiency, the aliased stable-surface
  counterexample, transition discrimination, hand predicates, replay, and quiescence.
- `cargo test --locked --manifest-path research/experiments/hand-consequence-born-return-admission/Cargo.toml`
  preserves the parent candidate and frozen counterexample.

## Development loop

`cargo test --locked --manifest-path research/experiments/hand-physical-change-resampling/Cargo.toml`
is the representative warm regression suite and must remain strictly under 10
seconds. Record cold dependency bootstrap separately.

## Controls and evidence

- Held-out cases: same later origin as sample versus transition, transition at or
  before opening, delayed propagated transition, direct and local transition,
  reordered origins, aliased interior movement, and clamped resampling.
- Negative controls: old protocol equality, no sample consequence write, unchanged
  active-surface perception, no semantic state leakage, exact replay, natural
  quiescence, zero propagation exhaustion, and the parent campaign result.
- Falsifiers: sampled input is admitted; transitioned input is lost; classification
  changes ordinary drive; the hand adapter derives learner-visible position or
  direction; aliased surface novelty is claimed sufficient; boundary release fails;
  replay/quiescence regresses; or the warm loop reaches 10 seconds.
- Expected artifacts: validated plan, campaign and arm manifests, deterministic arm
  results, convergence, candidate receipt, and independent verification receipt.

## Risks and rollback

Transition lineage can be accidentally dropped at a propagation or selection
boundary, or incorrectly OR unrelated origins. Matched-origin transport, merged-
lineage, and replay tests detect this. An overly broad gate can reject valid direct
returns; direct/local controls detect it. Rollback removes the opt-in protocol,
physical-input wrapper, transition lineage field, hand classification, and new
experiment/campaign; existing `Input` and protocols remain intact.

## Open decisions

None.
