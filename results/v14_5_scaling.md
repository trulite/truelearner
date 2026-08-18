# v14.5 Scaling Results

**Recorded:** 2026-08-18

**Machine:** Apple M5 Pro, 15 logical CPUs, 24 GiB RAM

**Operating system:** Darwin 25.5.0, arm64

**Rust:** 1.97.1

## Result

```text
data work exponent:             0.996
active-context work exponent:   1.000
topology-search work exponent:  1.000
maximum subcritical error:      1.8%
supercritical runaway rate:    23.3%
associative accuracy:
  0.25x slot load              89.3%
  4.00x slot load              24.6%
```

The v14.5 acceptance criteria pass.

## Deterministic Work Curves

### Training observations

| Observations | Work units | Release wall time |
|---:|---:|---:|
| 256 | 520 | 24.7 us |
| 1,024 | 2,056 | 60.8 us |
| 4,096 | 8,200 | 240.9 us |
| 16,384 | 32,776 | 934.8 us |

The fitted exponent is `0.996`. The small constant offset comes from
eliminating the initial competing hypotheses.

### Active context

| Active sensors | Sensor visits | Release wall time |
|---:|---:|---:|
| 1 | 1,028 | 47.9 us |
| 4 | 4,112 | 63.6 us |
| 16 | 16,448 | 233.3 us |
| 64 | 65,792 | 819.4 us |
| 256 | 263,168 | 4.76 ms |

The fitted active-sensor-visit exponent is exactly `1.000`.

### Topology search

| Sensors | Work units | Release wall time |
|---:|---:|---:|
| 64 | 3,520 | 94.7 us |
| 256 | 14,080 | 355.0 us |
| 1,024 | 56,320 | 1.48 ms |
| 4,096 | 225,280 | 4.98 ms |

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

## Interpretation

These measurements establish linear deterministic work in the tested local
update, active-context, and topology-search regimes. They do not establish a
Transformer-style loss scaling law.

Remaining requirements for v15:

- one unified sequence learner,
- training loss and held-out loss,
- capacity and dataset sweeps,
- matched Transformer compute and memory,
- MQAR, RULER-style retrieval, algorithmic sequence tasks, and language data,
- multiple independent seeds and confidence intervals.

The raw measurements are stored in
[`v14_5_scaling.csv`](v14_5_scaling.csv).
