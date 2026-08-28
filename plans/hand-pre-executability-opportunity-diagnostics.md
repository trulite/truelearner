```text
output incidence
      |
      v
measure complete positive and negative path strength
      |
      +--> measure current opportunity and ownership
      |
      `--> if executable, measure live unanswered returns
                    |
                    v
          one observational trace event
```

# Expose pre-executability opportunity diagnostics

## Outcome

Core output-candidate diagnostics report raw positive and negative complete-path
strength plus the count of live unanswered returns used by an executable candidate.
This localizes cancellation, ownership, and return availability in one run without
changing selection, firing, learning, or lifetime behavior.

## Authority

- Path: `research/campaigns/hand-physical-change-resampling-v1/convergence.toml`
- Revision: `sha256:afbb99e4d39228c61f5e6ee76bdd3e8cc06772dcc39ff41952762d915571d06c`

## Model

An output incidence is transformed into one `OutputCandidateEvaluated` observation.
The pure signed-strength fold measures complete positive and negative path inputs
before `admitted_path_drive`. Existing opportunity and ownership then determine
admitted drive and executability. Only an executable candidate resolves its
admitted-sign return set, exactly as before; the resulting count is included in the
same event and candidate value. Tracing remains an effect at the run boundary.

## Invariants

- Raw strengths are observations of live complete-path firings; they do not affect
  drive admission.
- Return discovery occurs once per executable candidate and feeds both the trace and
  the existing candidate-selection path.
- Non-executable candidates report zero admitted unanswered returns; the diagnostic
  does not speculate which sign would have won.
- Trace enabled versus disabled preserves outputs, work, state, replay, and natural
  quiescence.
- No motor identity, position, direction, hand state, or evaluator fact enters core.
- Existing protocol behavior and public input semantics remain unchanged.

## Scope

- Extend `PhysicalEvent::OutputCandidateEvaluated` and its construction in core.
- Extend core boundary tests and the developmental-hand evidence adapter.
- Add a diagnostic experiment/campaign that classifies the frozen three
  explanations from one hand run.
- Exclude any opportunity, ranking, strength, route, ownership, protocol, or hand
  behavior change; exclude adoption and authority promotion.

## Development style

TDD. First require raw opposing strengths and incumbent unanswered-return count in a
tiny core fixture while retaining trace/no-trace equality. Then implement the pure
measurement and reuse the existing return-resolution result.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml --test harness_boundary pre_executability_diagnostics`
  proves signed-strength and return-count observations plus observational equality.
- `cargo test --locked --manifest-path research/experiments/hand-pre-executability-opportunity/Cargo.toml`
  proves the three frozen diagnostic predicates, exact replay, and quiescence.
- `cargo test --locked --manifest-path research/experiments/hand-physical-change-resampling/Cargo.toml`
  preserves the frozen parent campaign.

## Development loop

`cargo test --locked --manifest-path research/experiments/hand-pre-executability-opportunity/Cargo.toml`
is the representative warm regression suite and must remain strictly under 10
seconds. Record cold dependency bootstrap separately.

## Controls and evidence

Held-out cases include unequal signed drive, no path, no unanswered return, answered
return, and trace disabled. Negative controls are unchanged outputs/state/work,
parent hand behavior, exact replay, natural quiescence, zero propagation exhaustion,
and evaluator isolation. The diagnostic is falsified if it cannot distinguish raw
zero drive from balanced nonzero drive or cannot associate a live unanswered return
with its executable output. Expected artifacts are a validated plan, campaign,
deterministic arm results, convergence, candidate receipt, and verification receipt.

## Risks and rollback

Resolving returns twice could alter work or scheduling; compute once and reuse it.
Large traces could increase diagnostic cost; add only scalar fields to the existing
event. Rollback removes those fields, adapter projections, experiment, and campaign.

## Open decisions

None.
