---
name: benchmark-climb
description: Advance a benchmark frontier by locating missing physical transitions, testing competing benchmark-blind laws, and promoting only mechanisms that survive falsification, transfer, controls, cost limits, and clean integration. Use when diagnosing a benchmark failure, climbing a benchmark family, evaluating candidate learner physics, or deciding whether a benchmark-driven result is safe to adopt.
---

# Benchmark Climb

```text
frozen organism + unchanged benchmark
                  |
                  v
       strongest benchmark-blind
          established candidate
                  |
                  v
        climb upward capability
                 ladder
                  |
          +-------+-------+
          |               |
        fails            works
          |               |
 descend to first     downward ablation
 physical break       ladder; rerun upward
          |               |
          +-------+-------+
                  |
                  v
       transfer + holdout + adoption
```

## Rule

Use a benchmark to expose missing physics, never to specify the physics.

A benchmark may show where the organism fails. It may not tell the organism
what it is allowed to know.

## Freeze

- Require a clean, named pre-benchmark commit and record its exact digest.
- If no such commit exists, stop and request authorization before creating one.
- Run the unchanged benchmark from that state.
- Record behavior, not score alone: where activity stops, which paths form,
  what participates, what fires, what strengthens, and what later reappears.
- Keep a hidden holdout that is not inspected during discovery.

## Synthesize and climb

- Compose every applicable compatible established lesson into the strongest
  benchmark-blind candidate.
- Preserve incompatible alternatives and falsified proposals as constraints; do
  not withhold a known prerequisite merely to rediscover it one rung at a time.
- Freeze the complete candidate, then test it upward from primitive physical
  continuity through composed capability, the unchanged benchmark, transfer,
  controls, replay, natural quiescence, and cost bounds.
- Stop dependent upward rungs after the first failed prerequisite.

## Descend a failed candidate

Classify the first failed physical transition:

1. perception;
2. route genesis;
3. participation;
4. consequence;
5. credit;
6. consolidation;
7. autonomous reuse.

Do not debug a later stage until the earlier stages are observed working.
Track routes generated, routes participating, motor firings, credit returns,
PQLC updates, autonomous probes, work, spikes, and quiescence separately from
benchmark score.

Once the first physical divergence is localized:

- State two to four benchmark-blind physical hypotheses before coding.
- Give each hypothesis a prediction, killing falsifier, negative control, and
  expected cost.
- Prefer laws that could help unrelated worlds.
- Test the smallest adversarial constructed world before the benchmark.
- Stop and freeze an arm as soon as its prediction fails.
- Preserve failures, cross-pollinate compatible survivors, and choose the next
  discriminating test.

A candidate must earn itself twice: the constructed world must expose its
mechanism, and the original benchmark must improve. Either result alone is
insufficient.

## Climb the downward ablation ladder

- Freeze the successful complete candidate as the positive reference.
- Remove one mechanism at a time, rerun the complete upward ladder, and restore
  the reference before the next removal.
- Then test only evidence-motivated coalitions and interactions.
- Treat a removal as surviving only when capability, transfer, controls, replay,
  natural quiescence, and cost bounds remain intact.
- Do not infer necessity or run removal ablations from a candidate that never
  passed the complete upward ladder.

## Transfer

Require every surviving law to:

- improve the benchmark family that exposed it;
- improve at least one unrelated world;
- preserve old boundaries and unchanged negative controls;
- pass the regression suite;
- pass an unseen holdout;
- retain exact replay, Production/reference equality, and natural quiescence;
- avoid an unexplained increase in work, spikes, memory, or runtime.

Treat a single-task gain, opaque score jump, sideways capability trade, or
large cost explosion as a failed gate until explained.

## Keep the organism blind

Never admit benchmark object IDs, grid positions, answer counts, task-family
hints, episode IDs, evaluator knowledge, special resets, path memory, or
correct-answer state. Do not patch negative controls or benchmark contracts to
make a candidate pass.

For software benchmarks, preserve the generic workstation: benchmark output
enters through its monitor, and only ordinary workstation `DeviceEvent`s leave
for the external application. Never introduce a benchmark-native body or map
internal motor activity directly to benchmark actions. If the claim builds on a
completed body course, load its produced checkpoint; a fresh body is a negative
control, not the candidate.

Use benchmark families as milestones. State the physical capability gained
across unseen variants; do not celebrate an individual task number.

## Adopt

- Promote a local law, never an experimental harness.
- Require minimal, readable, benchmark-blind adoption code.
- Hand clean adoption to `$dev`.
- Keep representative warm wave time strictly under 25 ns unless the user
  expressly approves otherwise.
- Keep the representative warm regression loop strictly under 10 seconds.
- Re-run the complete upward capability ladder from the frozen clean organism
  after each declared adoption group.
- Do not let experiment order or residual state become a dependency.

Record one frontier entry per level with only:

- first failing transition;
- hypothesis;
- candidate law;
- falsifier;
- result;
- adopt or reject.

Stop climbing when the next failure is not physically understood. Prefer a
lower score with a causal account over a higher score produced by opaque
patches.
