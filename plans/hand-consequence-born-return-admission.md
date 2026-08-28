```text
modulatory return origin + lineage birth + live return opening
                         |
                         v
              birth > opening ?
                 /             \
              yes               no
              |                  |
       existing admission   typed rejection
       consequence write    return stays unanswered
              |                  |
              v                  v
        continuation       existing fresh-alternative release
```

# Require consequence-born return admission before consequence write

## Outcome

An opt-in recursive learner protocol composes boundary-effect terminality with a
strict consequence-born return-admission gate. A physical origin whose preserved
birth is not later than the live return's opening cannot answer that return, write
consequence, consolidate a reverse path, or renew candidate preference. Genuinely
later local returns retain existing behavior. The unchanged hand is then measured
for upper release and further physical progress without default adoption.

## Authority

- Path: `research/campaigns/hand-upper-boundary-release-localization-v1/convergence.toml`
- Revision: `8e7a19e7e53d1a5e8a33d670b5ea5a61cceb8ef763419dca51f7d56f26289b65`

## Model

For each modulatory return member, the existing lineage maps an origin to its birth
tick and the live return link supplies its opening tick. The partial admission arrow
is defined only when `birth_tick > opened_tick`; otherwise it yields a typed
`RejectedBeforeReturnOpened` observation and leaves the return live and unanswered.
Admitted members compose unchanged through local/direct admission, consequence
write, reverse consolidation, closure, and cohort retirement. Rejected members
produce no learning effect. Protocol selection and artifact I/O remain explicit
boundaries; the eligibility comparison is pure and requires no new persistent state.

## Invariants

- Preserve the earliest physical birth carried by causal lineage; never substitute
  arrival time when lineage exists.
- Use the exact live return generation and its recorded opening tick.
- A rejected pre-opening origin does not enter admitted-origin memory, write
  consequence, consolidate, close a cohort, or retire the return.
- A later direct or local return preserves one effect, existing credit, closure,
  replay, and natural quiescence.
- Boundary-effect terminality remains unchanged and prevents renewed re-entry.
- The learner receives no hand, contact, position, direction, movement, expected
  action, episode, or evaluator state.
- Existing protocols and the default remain behaviorally unchanged.

## Scope

- Add one opt-in `Protocol` variant inheriting the boundary-effect-terminal stack.
- Add one protocol predicate, one typed return rejection, and the pre-admission gate
  in `outcome.rs`; reuse existing causal lineage and return opening state.
- Add focused core tests and a solve experiment/campaign covering temporal
  discrimination, unanswered release, unchanged hand, replay, quiescence, and cost.
- Exclude new learner memory, strength changes, ranking changes, path lifetime,
  hand-world changes, semantic identities, default adoption, and authority promotion.

## Development style

TDD. First add a tiny fixture that requires a pre-opening origin to be rejected and
a later origin to be admitted, plus an alternative-release composition. Then add the
protocol predicate and smallest pre-admission gate before running the hand.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml --test harness_boundary consequence_born_return`
  establishes typed early rejection, later local/direct admission, no early write,
  unanswered-return preservation, replay, and unchanged terminal re-entry safety.
- `cargo test --locked --manifest-path research/experiments/hand-consequence-born-return-admission/Cargo.toml`
  establishes temporal discrimination, no-consequence alternative release, hand
  trajectory, exact replay, natural quiescence, and deterministic evidence.
- `cargo test --locked --manifest-path research/experiments/hand-boundary-effect-reentry/Cargo.toml`
  preserves the parent terminal law and its frozen incomplete hand result.

## Development loop

`cargo test --locked --manifest-path research/experiments/hand-consequence-born-return-admission/Cargo.toml`
is the representative warm regression suite and must remain strictly under 10 seconds.

## Controls and evidence

- Held-out cases: equal birth/opening tick, genuinely later direct return, later
  local return, unrelated origin, duplicate origin, and unchanged terminal protocol.
- Negative controls: old protocols, boundary-effect isolation, exact replay, natural
  quiescence, zero propagation exhaustion, and no semantic hand information.
- Falsifiers: an early origin writes or answers; a later valid return is lost; the
  return is retired on rejection; the incumbent never releases in the tiny fixture;
  hand progress reintroduces feedback exhaustion; or warm runtime reaches 10 seconds.
- Expected artifacts: immutable per-arm results, validated campaign and convergence,
  candidate receipt, and independent verification receipt.

## Risks and rollback

An overly broad gate could reject valid delayed consequence or leave unnecessary
returns live. Detect both with direct/local valid controls, lifetime/quiescence, and
the parent campaign. Rollback removes the opt-in protocol, typed rejection, gate,
experiment, and campaign; all existing protocol behavior remains intact.

## Open decisions

None.
