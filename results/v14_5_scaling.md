# v14.5 and v14.6 Scaling Results

**Recorded:** 2026-08-18

**Machine:** Apple M5 Pro, 15 logical CPUs, 24 GiB RAM

**Operating system:** Darwin 25.5.0, arm64

**Rust:** 1.97.1

## Result

```text
data work exponent:             0.996
active-context work exponent:   1.000
topology-search work exponent:  1.000
stabilization work exponent:     1.000
maximum subcritical error:      1.8%
supercritical runaway rate:    23.3%
associative accuracy:
  0.25x slot load              89.3%
  4.00x slot load              24.6%
```

The v14.5 acceptance criteria pass.

## Deterministic Work Curves

### Training observations

| Observations | Work units |
|---:|---:|
| 256 | 520 |
| 1,024 | 2,056 |
| 4,096 | 8,200 |
| 16,384 | 32,776 |

The fitted exponent is `0.996`. The small constant offset comes from
eliminating the initial competing hypotheses.

### Active context

| Active sensors | Sensor visits |
|---:|---:|
| 1 | 1,028 |
| 4 | 4,112 |
| 16 | 16,448 |
| 64 | 65,792 |
| 256 | 263,168 |

The fitted active-sensor-visit exponent is exactly `1.000`.

### Topology search

| Sensors | Work units |
|---:|---:|
| 64 | 3,520 |
| 256 | 14,080 |
| 1,024 | 56,320 |
| 4,096 | 225,280 |

The fitted experiment-selection exponent is exactly `1.000`. This also
identifies a scaling liability: exhaustive hypothesis-directed experiment
selection scans the complete topology.

## Event Cascades

The harness uses a queue-based branching process with four possible child
events per processed spike.

| Branching ratio | Measured mean spikes | Theoretical mean | Relative error | Runaway |
|---:|---:|---:|---:|---:|
| 0.25 | 1.3332 | 1.3333 | 0.01% | 0% |
| 0.50 | 2.0217 | 2.0000 | 1.08% | 0% |
| 0.75 | 4.0701 | 4.0000 | 1.75% | 0% |
| 0.90 | 10.1186 | 10.0000 | 1.19% | 0% |
| 1.10 | capped | unbounded | n/a | 23.3% |

Below one, the observed cascade size follows `1 / (1 - r)` within 1.8%.
Above one, 23.3% of trials reached the 10,000-spike safety cap.

## Bounded Associative Capacity

The probe stores associations in 4,096 single-entry hashed slots.

| Associations | Slot load | Recall accuracy |
|---:|---:|---:|
| 1,024 | 0.25x | 89.3% |
| 2,048 | 0.50x | 78.4% |
| 4,096 | 1.00x | 63.7% |
| 8,192 | 2.00x | 43.5% |
| 16,384 | 4.00x | 24.6% |

Accuracy degrades monotonically as collisions overwrite prior associations.
The result exposes the memory-capacity knee; it does not demonstrate that the
current organism uses this exact storage scheme.

## Learned Stabilization

Each route begins with a useful recurring path and a useless loop. Training
creates a short concept route and weakens activity that no longer contributes
to the answer.

| Independent routes | Training spikes | Final spikes per route | Final runaway rate |
|---:|---:|---:|---:|
| 1 | 1,295 | 2 | 0% |
| 2 | 2,590 | 2 | 0% |
| 4 | 5,180 | 2 | 0% |
| 8 | 10,360 | 2 | 0% |
| 16 | 20,720 | 2 | 0% |

Training activity grows exactly in proportion to the number of independent
routes in this test. Every learned route compresses to one cue spike followed
by one output spike.

This does not yet measure interacting concepts that share cells or
connections. Shared structure may create interference and different scaling.

## Interpretation

These measurements establish linear deterministic work in the tested local
update, active-context, topology-search, and independent stabilization
regimes. They do not establish a Transformer-style loss scaling law.

Remaining requirements for v15:

- one unified sequence learner,
- training loss and held-out loss,
- capacity and dataset sweeps,
- matched Transformer compute and memory,
- MQAR, RULER-style retrieval, algorithmic sequence tasks, and language data,
- multiple independent seeds and confidence intervals.

The raw measurements are stored in
[`v14_5_scaling.csv`](v14_5_scaling.csv).
