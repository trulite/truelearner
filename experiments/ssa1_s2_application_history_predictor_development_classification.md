# SSA1-S2 application-history predictor development classification

Status: **Classification B — commitment-state law**.

Frozen Organism v1 and all prior SSA classifications remain unchanged. No
definitive evidence was executed, SSA2 remains blocked, and no architecture
change is authorized.

## Question answered

> Can a small evaluator-side summary of the actual M6 application history
> predict which basin Frozen Organism v1 enters when schedules contain the same
> multiset of physical experiences in different orders?

For the preregistered early-summary library: **no**.

Only physical commitment state predicted the basin reliably.

## Matrix

The GATE used six fresh identity/layout cells. Each cell ran 192 H=8 schedules:

```text
4 opportunity ratios
x 12 coprime temporal strides
x 4 offsets
= 192 equal-multiset schedules
```

Across GATE this produced 1,152 H=8 trajectories, plus H=2/H=32 sentinels and
trace-observation controls. Within a ratio every schedule had identical total
opportunities, executions, and returned consequences. Only order differed.

The basin distribution was identical in every fresh cell:

| B:A | incumbent | mixed | alternative |
|---:|---:|---:|---:|
| 1:2 | 7 | 13 | 28 |
| 1:1 | 13 | 16 | 19 |
| 2:1 | 7 | 14 | 27 |
| 4:1 | 18 | 17 | 13 |

Aggregate GATE basins were `270` incumbent, `360` mixed, and `522`
alternative. No subthreshold outcome occurred.

## Predictor result

| predictor | held-out accuracy | coverage | interpretation |
|---|---:|---:|---|
| P0 opportunity ratio | 39.58% | 75.00% | totals do not determine basin |
| P1 first application | 21.87% | 100.00% | first differential is not decisive |
| P2 first 4 balance | 16.66% | 94.79% | early signed count fails |
| P3 first 8 balance | 20.83% | 83.33% | early prefix fails |
| P4 first 16 balance | 42.70% | 80.20% | longer prefix still fails |
| P5 M5 gap before first opposition | 3.12% | 81.25% | opposing-evidence state fails |
| P6 longest run in first 90 applications | 51.04% | 73.95% | run length is insufficient |
| P7 M5 gap after episode 90 | 28.12% | 100.00% | early state sign fails |
| **P8 structural commitment** | **100.00%** | **100.00%** | threshold/deallocation state is exact |
| P9 composite `(P3,P5,P8)` | 78.12% | 78.12% | sparse exact tuples do not transfer |

The metrics were identical in MICRO and GATE and every physical mirror. P8 was
the same lowest qualifying predictor in all cells.

## Mechanistic interpretation

The result rejects several simple explanations of path dependence:

- the first effective differential does not choose the basin;
- the first 4, 8, or 16 signed M6 applications do not choose it;
- the longest same-direction application run does not choose it;
- the M5 score gap when opposing evidence first arrives does not choose it;
- the opportunity ratio and total evidence multiset do not choose it.

M6 applications are also sparse and late in this regime. Across held-out GATE
trajectories, the latest availability points were:

```text
4th effective application    episode 12,115
8th effective application    episode 15,490
16th effective application   episode 17,997
90th effective application   episode 16,498
```

The only exact law in the preregistered library was:

```text
alternative crosses executable threshold
+ incumbent physically deallocates
  -> ALTERNATIVE

alternative crosses threshold
+ incumbent remains executable
  -> MIXED

alternative never crosses threshold
  -> INCUMBENT_LOCK
```

This P8 statistic can become available only at episode `18,000` in some
trajectories. It is therefore a commitment-state law, not an early forecasting
law.

The narrow conclusion is:

> In the tested transition regime, order-dependent M6/M5 dynamics do not
> compress into the preregistered early directional statistics. The final basin
> becomes perfectly legible only when the accumulated history has been embodied
> as physical support formation and deallocation.

This supports a genuinely developmental interpretation. The same experience
multiset can traverse many application sequences, and no tested small early
summary forecasts the result. Physical commitment is the effective state
variable observed so far.

It does not prove that no other low-dimensional early statistic exists. It
does establish that inventing another statistic now would require a separately
preregistered successor, not post-hoc feature fitting.

## Controls

- M6 application direction was uniquely attributed in every trajectory.
- Trace collection was observationally inert.
- Opportunity schedules were exact and fixed before learner inspection.
- Scheduled opportunities, executions, and consequences matched exactly.
- H=2 and H=32 sentinels preserved their plastic/mature relations.
- Stale routes could not execute; post-closure opportunities were inert.
- Exact complete-state replay, identities, mirrors, handles, allocation/layout,
  count-capacity, and frozen-source controls passed.

## Frozen record

| artifact | commit / tag |
|---|---|
| Protocol | `6e82ad7281b79fd61af8aeeb0636bc48d94476cc` / `ssa1-s2-application-history-predictor-protocol-v1` |
| Implementation | `7b3d83f92121d74501538ce358ec656ce20a8732` / `ssa1-s2-application-history-predictor-implementation-v1` |
| PROBE B | `1e2ce712abac34ebeb1e924c183da699431d28ff` / `ssa1-s2-application-history-predictor-probe-v1-classification-b` |
| MICRO B | `ef94bdd74de3bc6a5c14eaada623d465ffe6d206` / `ssa1-s2-application-history-predictor-micro-v1-classification-b` |
| GATE B | `8e4499256f829392c57aa9fe8aedd9be199ce463` / `ssa1-s2-application-history-predictor-gate-v1-classification-b` |

Artifact SHA-256:

- Protocol: `1ec732c5c20719155b126bc91e5b7bccd300be6e7ccf839d0b37db1e6d664b65`
- Evaluator: `5e9f2055a4ec036f8adbe7c89de7028d2772826b2f5afea4bc97f99ca19d5c57`
- GATE report: `b5bfcfb06eeb23c9e33619a6e920c935df52c12870a6d05e047ce37e1bc095eb`
- GATE trajectory CSV: `164ea561cff5ba910dfb5cc9c2b781de05feb83761afc7582b9ce16cb74ad6cd`
- GATE predictor CSV: `5cf18234b992b20e8f52253094a97ccd0987005a11aca640c18b8b2c241cf89f`

## Program state

```text
Frozen Organism v1   unchanged
SSA1                 C
SSA1-C1              C
SSA1-C2              A under sufficient paired contrast
SSA1-R               C under natural rich-world contrast
SSA1-S               E: ratio-only map invalidated by temporal phase
SSA1-S2              B: commitment-state law
SSA2                 blocked
```

## Validation

- Formatting passed.
- The focused SSA1-S2 library test target compiled in release mode with
  `--no-run`; no PROBE, MICRO, GATE, or prior evidence was re-executed.
- Strict Clippy passed for the library and SSA1-S2 binary with documented
  frozen-source/evaluator lint classes allowlisted. The executed evaluator was
  not modified after GATE for lint style.
- The definitive surface refused before evaluator entry with exit `2`.
- Every frozen-parent SHA-256 in the protocol remained exact.
- Final changes are isolated to the separately named SSA1-S2 protocol,
  evaluator/binary registration, frozen development artifacts, and handoff.
