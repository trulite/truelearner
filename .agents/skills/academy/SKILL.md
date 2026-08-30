---
name: academy
description: Design, run, inspect, or change TrueLearner Academy curricula, probes, controls, evidence, external-world adapters, review artifacts, and capstone benchmarks. Use for Academy architecture or workflows, capability development and measurement, ARC-AGI integration, harness-facing tests, episode generation, benchmark capstones, and deciding what evidence can support an Academy capability claim.
---

# Academy

```text
world -> physical input -> Harness -> owned observation -> Academy evidence
  |                         |                                |
  +-- evaluator state -----+-----------------------------> claim
                                                           |
                                                           v
                                      optional review or capstone receipt
```

## Ground the work

- Read the active checkout's `docs/academy.md` completely.
- Read `docs/arch.md` boundaries before changing an interface or dependency
  edge.
- Read `docs/LANGUAGE.md` and `docs/algo.md` before proposing organism physics.
- Use `uv` or `uvx` for every Python script, validator, CLI, and dependency
  environment. Run repository scripts as `uv run ...`, add ephemeral libraries
  with `uv run --with`, and run packaged command-line tools with `uvx`. Do not
  use bare `python`, `pip`, or ad-hoc virtual environments.
- Treat archived protocols and results as historical evidence, not current
  authority or a mutable specification.
- State whether the task concerns curriculum, execution, evidence, review,
  external-world adaptation, a capability capstone, or learner physics.

## Preserve the semantic firewall

- Send organism-visible input only through the public `Harness`; read only owned
  harness observations. Academy code and tests may not construct, retain, or
  inspect a body directly.
- Keep expected answers, scores, game state, capability labels, episode classes,
  evaluator decisions, and benchmark identities outside the organism.
- Map external actions only after an outward crossing leaves the harness.
- Keep wall time, rendering, serialization, and benchmark timing out of physical
  time and learner state.
- Stop if Academy supplies an answer, action, route, correctness signal, or
  evaluator knowledge to the learner.

## Build capability evidence

1. Name one external capability claim and its prerequisites.
2. Separate development experiences from fresh probes.
3. Add transfer, negative, retention, interference, and replay controls in
   proportion to the claim.
4. Regenerate identities, positions, timing, contexts, and distractors so fixture
   memory cannot satisfy a probe.
5. Run every interaction through harness send/read/save/restore operations and
   let each physical transition reach natural quiescence.
6. Record admitted input, owned output, crossings, outcome, checkpoint and body
   fingerprints, physical work, spikes, memory, and replay evidence.
7. Derive capability state only from recorded evidence. Preserve failures.

Require exact replay, Production/reference equality, unchanged negative controls,
and bounded cost wherever they apply. Do not accept a score without its causal
trace.

## Keep execution lanes separate

```text
headless run -> frozen evidence -> optional render -> optional Playground
      |
      +---------------------> capability or capstone receipt
```

- Keep the normal Academy loop headless and dependency-light.
- Freeze evidence before rendering posters, videos, or UI catalogs.
- Let review code consume frozen evidence; never give it a live harness.
- Build Playground explicitly and keep it out of default Academy runs.
- Measure already-built release executables. Exclude Cargo, rendering, UI, and
  evidence serialization from a headless runtime sample unless the named
  benchmark explicitly measures that integration effect.

## Make a capstone

Treat a capstone as the final unchanged capability evaluation, not as teaching
and not as a runtime microbenchmark.

- Freeze a clean candidate commit, organism checkpoint or initialization rule,
  adapter, capstone protocol, budgets, and expected evidence schema before the
  scored run.
- Use visible development cases and an unseen holdout. Do not inspect or tune on
  the holdout during discovery.
- Run the official world through an external adapter that communicates only with
  the harness-facing agent protocol.
- Score outside the organism and report task outcome beside physical work,
  crossings, learning updates, memory, quiescence, fingerprints, and replay.
- Reject a sample when its physical result or frozen fingerprint differs; faster
  wrong work is not progress.
- Emit one immutable, machine-readable receipt and preserve the first failure.

Use `$benchmark-climb` when a frozen capstone exposes a missing physical
transition. Let that skill classify and falsify the failure; never modify the
capstone to make the organism pass.

## Route changes

- Hand code modeling, implementation, and verification to `$dev`.
- Keep the representative warm regression strictly under 10 seconds and record
  cold bootstrap separately.

Stop when a capability claim lacks a falsifier, a probe teaches accidentally, a
holdout has leaked, evaluator state crosses the harness boundary, replay differs,
or the first failed physical transition is not understood.
