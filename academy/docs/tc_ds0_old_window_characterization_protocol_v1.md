# TC-DS0 old-window characterization protocol v1

Status: frozen before evaluator implementation or execution.

Parent methodology: `temporal-credit-desupply-protocol-v1` at commit
`6c7b0e1be2b33f936d9291df4e637a71571b3e15`.

## Scope

TC-DS0 characterizes the retained rectangular eligibility law. It does not
select a replacement, change a constant, run ARC, or advance authority.

The active core must remain byte-identical to the ARC A2 development-positive
candidate:

```text
truelearner/crates/core/src/lib.rs
sha256 d49a6b98081cee65c3e7a5f64e9cf6356fa06ef456a9cf78a41d0cd1187bcd58

LOCAL_WINDOW = 4
```

No causal organism file may change. The evaluator is experiment-only.

## Matrix

Every row is executed independently under both
`MechanicalConfig::REFERENCE` and `MechanicalConfig::PRODUCTION`.

```text
initial pressure phase  0..9
event delay             0, 1, 2, 3, 4, 5, 8, 12 ticks
candidate resistance    1, 20
scenario                prompt_modulation
                        unrelated_modulation
                        nearby_drive
                        same_path_repeat
                        two_paths_one_modulation
                        wrong_path_only
```

The Cartesian product contains 960 physical cases and 1,920 mechanics rows.
No row is authoritative evidence.

### Scenario geometry

- `prompt_modulation`: the candidate traverses; an ordinary Modulatory route
  physically arrives at the candidate source after the selected delay.
- `unrelated_modulation`: the candidate traverses; the same-amplitude
  Modulatory arrival is emitted by a physically separate route with no
  downstream relation to the candidate.
- `nearby_drive`: the candidate traverses; equal-timed nearby Drive activity
  occurs without Modulation.
- `same_path_repeat`: the candidate traverses again after the selected delay,
  renewing whatever the retained law renews; no Modulation occurs.
- `two_paths_one_modulation`: two outgoing paths from one source traverse;
  only one return-shaped Modulatory arrival reaches their common source.
- `wrong_path_only`: the observed candidate never traverses; equal activity
  traverses a separate path and a Modulatory arrival reaches the observed
  candidate's source.

The names describe fixture geometry only. They do not add provenance labels to
the organism. All arrivals remain ordinary Drive or Modulatory transmissions.

## Observation schedule

Let traversal time be `T`, equal to the selected initial pressure phase.

1. Construct the body at tick zero.
2. Advance quiescently to `T`.
3. Admit the initial physical activity at `T`.
4. Advance one tick at a time.
5. Admit the selected delayed event at `T + delay`.
6. Continue to `max(T + 15, T + delay + 6)`.

The evaluator records the candidate state after every admitted event and every
one-tick advance. This is observation only; it may not alter the body.

## Serialized evidence

Each mechanics row records:

- case identity, mechanics, phase, delay, scenario, and initial resistance;
- traversal ticks and emitted rectangular `until` values from the physical
  trace;
- delayed-event tick;
- per-tick candidate `live`, resistance, coupling, and reconstructed
  eligibility-live state;
- plastic update count;
- proposal and deallocation counts;
- final durable-body hash;
- final clock and pressure phase;
- natural quiescence;
- exact live-checkpoint replay.

For the competing-path scenarios, the same fields are recorded separately for
the observed and competing paths.

Eligibility-live reconstruction may use only emitted `Eligible` transitions,
candidate-specific resistance updates, deallocation, and physical time. It may
not read or mutate private runtime fields.

## Gates

TC-DS0 is accepted as characterization only if:

1. the active core hash is exactly the frozen hash above;
2. all 1,920 mechanics rows are present;
3. every reference/production pair matches on all physical observations,
   excluding mechanics label and `ExecutionCost`;
4. every row is naturally quiescent after each admitted batch;
5. every final live checkpoint round-trips exactly;
6. fresh rerun artifacts are byte-identical;
7. no ARC world, Academy curriculum, parameter selection, or candidate law is
   executed;
8. `arch.md`, oracle status, `LOCAL_WINDOW`, and authority remain unchanged.

Expected rectangular behavior is descriptive, not a pass predicate. In
particular, attribution aliases must be serialized rather than suppressed.

## Decision

Any missing row, mechanical divergence, replay failure, runtime-source change,
or evaluator ambiguity freezes TC-DS0 negative. A positive result permits only
a separately preregistered TC-DS1 candidate workflow.
