```text
same initial hand
      |
      +-- parent execution ---- shared prefix ---- old arrow ---- downstream path
      |
      `-- candidate execution - shared prefix ---- fixed arrow -- downstream path
                                      |               |
                                 direct cost      trajectory cost
```

# Localize bounded-continuation work by prefix and physical activity

## Outcome

Add behavior-inert hand evidence that records live topology beside existing
per-step work, choices, and construction-consumption events. Freeze one
diagnostic successor that compares the unchanged parent and bounded-first-use
candidate in three regions: the shared physical prefix, the first repaired
choice, and the divergent downstream trajectory. The result must identify the
first comparison and scan divergence without changing learner physics or
reinterpreting the predecessor's failed cost gate.

## Authority

- Path: `research/campaigns/hand-bounded-first-use-construction-continuation-v1/convergence.toml`
- Revision: `sha256:a2bf55a62aa9e6d06165bc118d6f1983b6d70c4c74e4b7c62a2c054fee489989`

## Model

The parent and candidate are two arrows from the same initial hand state. A pure
evidence projection maps each external step to position before and after,
comparisons, scans, learner count, junction count, live-link count, output
choices, and exact held-state consumptions. A total comparison fold classifies
aligned steps as `SharedPrefix`, `FirstRepairedChoice`, or `DivergentTrajectory`.

The shared prefix ends before the first step whose selected physical output
differs. The first repaired step is retained separately because its input state
is still shared while its output arrow changes. Later steps are not paired as
equivalent computations; their raw and activity-normalized work is descriptive
evidence about different physical trajectories. I/O is confined to the
experiment binary after the pure fold has produced its artifact.

## Invariants

- Parent and candidate protocols, learner state, choice ranking, costs, and hand
  adapter physics are unchanged.
- Existing per-step comparison and scan totals remain exact; their sums equal
  each run's published total.
- Junction and live-link counts are projected from the observation already read
  at the end of each step and do not trigger an additional learner run.
- Choice and consumption evidence comes from existing trace events; no new core
  event or diagnostic-only organism state is added.
- The immutable predecessor artifacts and negative adjudication are hash checked
  and never mutated, loosened, or rerun.
- Position and phase labels are evaluator-only diagnostic data and never enter
  learner execution.
- Exact replay, natural quiescence, zero propagation exhaustion, and accepted
  default behavior remain unchanged.

## Scope

- Expose existing per-step junction and live-link counts in
  `developmental-hand-construction-admission` evidence.
- Add `hand-prefix-and-activity-work-localization`, a pure comparison evaluator
  and one-shot artifact writer for the unchanged parent and candidate runs.
- Add and validate a frozen diagnostic campaign, protocol, arms, evidence,
  adjudication, and convergence result.
- Add a factory candidate receipt and independent verification receipt.
- Exclude learner/core behavior changes, cost-counter changes, adapter force
  changes, lower-side behavior solves, default adoption, cost-gate revision,
  and authority promotion.

## Development style

TDD. Add pure fold tests for phase boundaries, first-delta localization, total
preservation, and zero-denominator normalization before wiring the existing hand
runner. Compile and validate every frozen manifest before the single evidence
execution.

## Focused tests

- `cargo test --locked --manifest-path research/experiments/hand-prefix-and-activity-work-localization/Cargo.toml`
- `cargo test --locked --manifest-path research/experiments/developmental-hand-construction-admission/Cargo.toml reflected_hand_step_evidence_retains_topology`
- `cargo check --locked --manifest-path research/experiments/hand-prefix-and-activity-work-localization/Cargo.toml`
- `cargo clippy --locked --manifest-path research/experiments/hand-prefix-and-activity-work-localization/Cargo.toml -- -D warnings`
- `uv run research/validators/validate_campaign.py --file research/campaigns/hand-prefix-and-activity-work-localization-v1/campaign.toml`

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`.
It must remain strictly under 10 seconds; cold bootstrap is recorded separately.

## Controls and evidence

Held-out cases are not applicable because this is a behavior-inert localization
of two already frozen executions, not a capability or transfer claim. Negative
controls are the immutable predecessor's three artifact digests, evidence digest,
negative adjudication, and unchanged accepted default. Live parent and candidate totals, trajectories,
first-wall choice, consumptions, replay, quiescence, and exhaustion must reproduce
the frozen observations exactly. The diagnostic is falsified if retention changes
either execution, if per-step sums do not equal totals, if cost diverges before
the protocols can differ, or if the first absolute work difference cannot be
assigned to a retained step and its observed topology.

## Risks and rollback

The main risk is falsely treating post-choice steps as matched computations.
Explicit phase labels and separate region totals prevent that interpretation.
Another risk is measurement perturbation; exact frozen summaries and sum checks
detect it. Rollback removes the two projected topology fields and the successor
experiment/campaign without touching either protocol or predecessor evidence.

## Open decisions

None.
