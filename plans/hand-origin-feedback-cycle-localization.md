```text
scheduled drive firing
        |
        v
observe edge + carried origin + path owner + origin owner + causal wave
        |
        v
bounded hand trace -> provenance disagreements + repeated directed edges -> first cycle
```

# Localize origin/path ownership disagreement and the hand feedback cycle

## Outcome

Physical tracing records the actual source-to-target edge, carried origin, owner of
that origin, owner of the traversed path, strength, and causal wave for every valid
drive firing. A bounded diagnostic experiment uses those observations to identify
the first ownership disagreement and the smallest repeated directed cycle before
the step-eight propagation exhaustion, without changing candidate selection or hand
behavior.

## Authority

- Path: `research/campaigns/hand-causal-origin-ownership-factorization-v1/convergence.toml`
- Revision: `b31ad8a0a2a52caf98eea474d8d87ae61d6346620d9c11914ee34ae983e2eb6b`

## Model

A valid scheduled drive firing maps to one immutable provenance observation:
`(source?, target, source_physical?, target_physical, source_region?, target_region,
link?, completes_path, carried_origin, origin_owner?, path_owner?, strength, causal_wave)`.
External input maps to absent source, link, and path owner. A finite bounded trace
maps to a sequence ordered by its observed ordinal. Filtering unequal owner pairs
locates provenance disagreement. Grouping directed edge signatures across causal
waves identifies recurrence; a directed cycle is a non-empty closed walk whose
edges recur before exhaustion. The trace transformation is observational. The
only effects remain trace emission and experiment artifact I/O.

## Invariants

- Provenance is read from the live firing, live traversed link, and current learner
  membership before candidate selection mutates the incidence.
- Path ownership and carried-origin ownership remain separate optional values.
- External input and unowned physical origins remain explicitly unowned.
- Trace enablement does not alter outputs, work, canonical state, replay, or quiescence.
- Existing trace event meanings and all protocol behavior remain unchanged.
- The diagnostic uses a fixed propagation bound only to observe the already-frozen
  non-quiescent candidate; bounded termination is never classified as quiescence.
- Cycle localization uses physical IDs and graph continuity only, never motor,
  direction, hand, score, or expected-action semantics.
- The frozen causal-origin candidate and its failed controls are not repaired.

## Scope

- Add one diagnostic `PhysicalEvent` and one link-owner query inside the core.
- Emit provenance while valid firings are assembled into drive incidences.
- Export the observations through the existing reflected-hand evidence adapter.
- Add a diagnostic experiment and frozen campaign for ownership disagreement,
  step-eight cycle localization, matched behavior, replay, quiescence, and cost.
- Exclude selection changes, origin propagation changes, refractory/lifetime changes,
  new learner state, hand-world changes, default adoption, and any solve arm.

## Development style

TDD. First require one traced fixture to report the exact path owner and origin
owner while a matched untraced fixture preserves behavior and canonical state.
Then add the event, adapter mapping, and bounded cycle analysis.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml --test harness_boundary drive_provenance`
  establishes exact edge/origin/path ownership, external absence, observational
  purity, and replay.
- `cargo test --locked --manifest-path research/experiments/hand-origin-feedback-cycle-localization/Cargo.toml`
  establishes ownership disagreement, a reproducible pre-exhaustion physical cycle,
  unchanged frozen behavior, and compact deterministic evidence.
- `cargo test --locked --manifest-path research/experiments/hand-causal-origin-ownership-factorization/Cargo.toml`
  preserves the frozen candidate failure and controls.

## Development loop

`cargo test --locked --manifest-path research/experiments/hand-origin-feedback-cycle-localization/Cargo.toml`
is the representative warm regression suite and must remain strictly under 10 seconds.

## Controls and evidence

- Held-out cases: external input, a link with no learner owner, owned path with an
  unowned carried origin, identical owner pair, and input-order replay.
- Negative controls: physical tracing disabled, boundary reference behavior,
  unchanged same-origin failure, exact replay, natural-quiescence classification,
  and fixed 256-moment bound.
- Falsifiers: provenance changes behavior or state; owner fields are reconstructed
  after topology changes; no repeated directed cycle precedes exhaustion; a reported
  cycle is not graph-contiguous; or evidence depends on semantic hand identities.
- Expected artifacts: per-arm immutable evidence, convergence, candidate receipt,
  and independent verification receipt.

## Risks and rollback

Per-firing observations can enlarge trace memory. Keep the event opt-in under
physical tracing, keep campaign evidence compact, measure warm cost, and retain the
existing bound. Rollback removes the event, adapter field, and diagnostic experiment;
the failed opt-in protocol and every older behavior remain unchanged.

## Open decisions

None.
