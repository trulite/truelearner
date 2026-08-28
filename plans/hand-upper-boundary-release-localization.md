```text
quiet hand reaches +4
        |
        v
existing candidate + provenance + origin-selection events
        |
        v
last moving step  <->  first clamped contact step
        |
        v
first failed stage: path -> drive -> factorization -> emission -> effect
```

# Localize the first upper-boundary release failure

## Outcome

The research adapter preserves the full existing causal-origin selection event, and
a bounded diagnostic campaign compares the last successful hand movement with the
first clamped upper-contact moment. The evidence identifies whether each physical
motor has a live path, enough drive, an executable origin group, selection, an
emitted output, and a physical effect. It changes no learner state or behavior and
does not propose a solve until the first failed transition is known.

## Authority

- Path: `research/campaigns/hand-boundary-effect-reentry-v1/convergence.toml`
- Revision: `259f1ef07aba8ae48c3f14f259297d8440b595e4f35abb4586fbd8c2b26d5ea2`

## Model

An existing `CausalOriginCandidateResolved` trace event maps directly to immutable
research evidence containing target, origin-group count, executable-group count,
selected physical origin, selected ownership, and selected path-input count. One
hand step maps to per-physical-motor stage evidence by composing provenance,
candidate evaluation, origin selection, output emission, and resulting movement.
The comparison is pure; artifact writing is the only new effect. Missing events are
represented by empty collections or `Option`, not inferred from expected behavior.

## Invariants

- Every reported value comes from an already emitted core event or observed output;
  the adapter does not reconstruct causal history.
- Physical IDs identify observed endpoints only; no motor direction, desired action,
  boundary answer, or evaluator choice enters the learner.
- Both motor candidates are retained, including rejected and non-executable groups.
- The terminal candidate remains opt-in, and its trajectory, exact replay, natural
  quiescence, work bound, and core state remain unchanged.
- A clamped output is distinguished from physical movement; effort is not success.
- No mechanism is adopted or benchmark law changed in this diagnostic round.

## Scope

- Extend `developmental-hand-construction-admission` evidence with detailed causal-
  origin selection observations already present in the core trace.
- Add `hand-upper-boundary-release-localization` experiment, tests, manifests,
  frozen evidence, convergence, and durable lesson/program update if established.
- Exclude new core events, learner physics, hand-world changes, contact semantics,
  candidate ranking changes, reflection machinery, default adoption, and solve arms.

## Development style

TDD. First require the adapter to preserve exact selection fields and the experiment
to locate the last moving and first clamped steps. Then add the smallest mapping and
pure stage classifier.

## Focused tests

- `cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml causal_origin_selection`
  establishes lossless export of the existing selection event.
- `cargo test --locked --manifest-path research/experiments/hand-upper-boundary-release-localization/Cargo.toml`
  establishes per-motor stage localization, the matched pre-boundary control,
  deterministic evidence, exact replay, and natural quiescence.
- `cargo test --locked --manifest-path research/experiments/hand-boundary-effect-reentry/Cargo.toml`
  preserves the selected terminal law and its known incomplete hand result.

## Development loop

`cargo test --locked --manifest-path research/experiments/hand-upper-boundary-release-localization/Cargo.toml`
is the representative warm regression suite and must remain strictly under 10 seconds.

## Controls and evidence

- Held-out cases: the last moving step, both stable motor endpoints, rejected origin
  groups, and a step after first clamp.
- Negative controls: unchanged emitted-output trajectory, exact replay, natural
  quiescence, zero propagation exhaustion, and the frozen boundary-effect campaign.
- Falsifiers: the trace cannot distinguish both motor paths; selection fields are
  lost or reconstructed; diagnostics change behavior; the stage ordering is
  ambiguous; or the conclusion depends on knowing which direction is correct.
- Expected artifacts: three immutable arm results, campaign protocol and manifests,
  convergence, candidate receipt, and independent verification receipt.

## Risks and rollback

Extra serialized evidence can enlarge research artifacts. Keep it per-step and only
under existing physical tracing. Rollback removes the adapter field and diagnostic
experiment; the core trace and all learner behavior remain unchanged.

## Open decisions

None.
