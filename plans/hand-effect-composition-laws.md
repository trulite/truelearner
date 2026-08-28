```text
                    frozen root-fresh learner
                              |
                 +------------+------------+
                 |                         |
        quiescent-phase world       unresolved-effect learner
          composition arm             coherence arm
                 |                         |
                 +------------+------------+
                              |
                    explicit composition arm
```

# Compare hand effect composition laws

## Outcome

Test two isolated explanations for opposing outputs that cancel inside one external
hand step. The first composes each naturally quiescent Harness run with the world
before the next run. The second keeps the frozen batched force adapter and lets one
recent unresolved local output hold competition until its return arrives or ages
out. A third arm tests their composition. This is discovery evidence, not an Academy
capability claim or adoption.

## Authority

- Path: `research/campaigns/hand-physical-transition-continuation-v1/convergence.toml`
- Revision: `sha256:3480c7cefabfc919669804eeffbe72a51b095295fdf867d9c88edc51dbde5ec2`

## Model

The hand state and each naturally quiescent Harness state are objects. A Harness run
and a world force update are arrows. The sequential arm composes each complete
Harness arrow with one fixed-force world arrow before forming the next input. It
never exposes position or direction to the learner. The coherent arm adds an opt-in
protocol: among local executable candidates, exactly one candidate with an
unanswered return opened within the existing recent window holds the output choice;
otherwise ordinary root-fresh replacement runs unchanged. The recent return is the
only transaction state, and its existing answer, supersession, or decay closes it.

External I/O stays in the hand adapter. Return-age classification and competition
stay in the core. `EffectComposition` and the new protocol variant make the two
boundaries explicit without generic category machinery.

## Invariants

- `EffectComposition::Batched` remains byte-for-behavior equal to the frozen hand
  adapter and remains the only official Academy force law.
- Sequential composition applies the same opposing-force rule within each complete
  Harness run; it only changes when the world is updated and resampled.
- No evaluator position, direction, boundary, hand-step, desired action, or score
  enters Harness input or learner state.
- Coherence requires one unique, owner-compatible, recent unanswered return; sample,
  stale, answered, ambiguous, nonlocal, and multiple-incumbent cases fall back to the
  existing law.
- No new durable memory, path, owner, strength, or semantic output identity is added.
- Exact replay, natural quiescence, bounded propagation, old protocols, root-fresh
  behavior, and the fixed batched reference remain intact.

## Scope

- Extend the developmental hand adapter with an experimental typed effect-composition
  mode and full phase evidence.
- Add one cumulative experimental core protocol and causal diagnostics for coherent
  unresolved-effect admission.
- Add focused core/adapter tests and one four-arm campaign: sequential, coherent,
  composition, and frozen-parent control.
- Exclude official Academy adapter replacement, default protocol adoption, semantic
  action selection, external-step input, persistent direction memory, commits, and
  authority promotion.

## Development style

TDD. First freeze batched identity and tiny unresolved-return controls, then implement
each boundary independently. Run the full hand only after both cheap gates pass.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml --test harness_boundary coherent_unresolved_effect`
  proves recent unique hold, stale/sample release, protocol scope, reflection, and
  old-protocol equality.
- `cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml effect_composition`
  proves batched identity and quiescent-phase ordering without a learner leak.
- `cargo test --locked --manifest-path research/experiments/hand-effect-composition-laws/Cargo.toml`
  checks frozen arm predicates over the shared evidence runner.

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path research/experiments/hand-effect-composition-laws/Cargo.toml`.
It must remain strictly under 10 seconds; cold dependency bootstrap is recorded
separately.

## Controls and evidence

Held-out cases are reflection, perturbation recovery, transition delivery with no
output, two recent unresolved candidates, and an unresolved output that causes no
world change. Negative controls are the frozen batched root-fresh parent, samples,
answered and stale returns, unrelated owners, old protocols, replay, quiescence,
propagation, and exact batched trajectory equality. Falsifiers are any learner leak,
changed batched reference, permanent boundary lock, multiple opposing exported
effects under coherence, lost quiescence, or failure to improve actual travel.

Evidence records each Harness phase, force group, position transition, return
lifetime decision, both limits and escapes, perturbation, work, and exact replay.
Sequential success is explicitly counterfactual adapter evidence and cannot by
itself support an Academy capability claim.

## Risks and rollback

Sequential phase composition may accidentally change the official force law; the
typed default plus exact batched equality detects this. Coherence may hold an output
forever after a clamped action; the existing recent window and stale-release fixture
kill that failure. Rollback removes the new enum mode and protocol while retaining
all frozen artifacts and diagnostics.

## Open decisions

None.
