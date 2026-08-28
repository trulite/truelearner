# Localize the first failure after upper release

## Outcome

Preserve one complete, unchanged hand trajectory under the organism-to-root candidate
and its strict parent, then identify the earliest post-release physical transition
that stops cumulative travel. This is a diagnostic only; it changes no learner law.

## Parent evidence

`research/campaigns/hand-organism-root-fresh-opportunity-v1/convergence.toml`
shows one typed transfer, both motors, upper escape, exact replay, and quiescence,
but no lower contact or complete one-joint control.

## Diagnostic order

For every step, preserve external transition incidence, complete/consequential paths,
all motor candidate quantities, owner reads and writes, return decisions, fresh
opportunity decisions, motor output, and actual world change. After the first upper
escape, compare the last continuing step with the first non-continuing step in this
order:

1. external physical incidence;
2. complete live path;
3. motor candidate formation and ownership;
4. opportunity, drive, threshold, and executability;
5. owner-local recent consequence read;
6. motor output and actual world transition;
7. return opening, admission, writing, and release.

## Invariants

- No learner, path, ranking, lifetime, or world behavior changes.
- The organism receives no position, direction, boundary, hand, or desired-action
  label; those remain evaluator-only capstone measurements.
- Candidate and strict-parent runs retain exact replay, natural quiescence, bounded
  propagation, and the unchanged sixteen-step world.
- The first missing transition, not the final benchmark score, chooses the next arm.

## Verification

- `cargo test --locked --manifest-path research/experiments/hand-post-release-continuation-localization/Cargo.toml`
- Validate the campaign and convergence records with the repository validators.
- Keep the representative warm experiment suite strictly under ten seconds.
