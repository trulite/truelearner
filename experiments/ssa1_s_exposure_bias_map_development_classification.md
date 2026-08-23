# SSA1-S exposure bias map development classification

Status: **Classification E — scientific ambiguity; GATE not run**.

Frozen Organism v1 and all prior SSA classifications remain unchanged. This
result neither repairs SSA1 nor authorizes architecture changes, SSA2, scaling,
or definitive evidence.

## Result

SSA1-S successfully manipulated environmental opportunity and measured its
complete transfer:

```text
scheduled physical opportunity
  -> actual execution
  -> returned physical consequence
  -> M6 evidence
  -> M5 allocation
  -> executable landscape
```

Across every MICRO trajectory, scheduled opportunity transferred exactly to
execution. The tested alternative:incumbent ratios generated these exact
18,000-episode exposure counts:

| B:A | incumbent executions | alternative executions |
|---:|---:|---:|
| 1:8 | 16,000 | 2,000 |
| 1:4 | 14,400 | 3,600 |
| 1:2 | 12,000 | 6,000 |
| 1:1 | 9,000 | 9,000 |
| 2:1 | 6,000 | 12,000 |
| 4:1 | 3,600 | 14,400 |
| 8:1 | 2,000 | 16,000 |

Every execution returned an ordinary consequence. Stale-route, post-closure,
identity/mirror, allocation-layout, schedule-accounting, anti-adaptation,
count-capacity, and exact-replay controls passed.

The intended two-dimensional phase map nevertheless failed its schedule-phase
control. A one-episode rotation of the same deterministic periodic opportunity
word changed the learned outcome at maturity `H=8`, despite identical total
opportunities, executions, and consequence counts.

## Exact H=8 ambiguity

Baseline phase:

| B:A | final landscape |
|---:|---|
| 1:8 | ALTERNATIVE |
| 1:4 | ALTERNATIVE |
| 1:2 | INCUMBENT_LOCK |
| 1:1 | MIXED |
| 2:1 | INCUMBENT_LOCK |
| 4:1 | ALTERNATIVE |
| 8:1 | ALTERNATIVE |

One-episode rotated phase:

| B:A | final landscape |
|---:|---|
| 1:8 | ALTERNATIVE |
| 1:4 | ALTERNATIVE |
| 1:2 | ALTERNATIVE |
| 1:1 | INCUMBENT_LOCK |
| 2:1 | ALTERNATIVE |
| 4:1 | MIXED |
| 8:1 | ALTERNATIVE |

The relation transferred exactly across all four fresh MICRO cells and both
physical-side/route mirrors.

Frozen-state inspection resolves where the divergence appears but does not
reduce it to the requested scalar exposure ratio. For example:

- At H=8 and B:A=`1:2`, baseline ends with M5 role scores `[13,-3]` and
  `INCUMBENT_LOCK`; the rotated word ends `[-828,431]` and `ALTERNATIVE`.
- At H=8 and B:A=`1:1`, baseline ends `[-3250,3256]` and `MIXED`; the rotated
  word ends `[8,-3]` and `INCUMBENT_LOCK`.
- Opportunity, execution, and consequence totals are identical between each
  matched pair. M6 support/margin/eligibility and the temporal sequence of
  applications differ.

Thus the operative map is not:

```text
initial maturity x total environmental opportunity ratio
  -> learned allocation
```

It is at least:

```text
initial maturity
x opportunity ratio
x temporal ordering relative to consequence history
  -> M6 application sequence
  -> learned allocation
```

That additional temporal axis is not an evaluator defect. It is a causal
property of the frozen on-policy learner. But it invalidates the preregistered
claim that the ratio transfer curve is independent of the first handle and
prevents a unique 2-D phase diagram.

## Stable regions already established

The ambiguity is localized rather than universal:

- `H=0` and `H=2`: the changed-world alternative becomes authoritative at
  every tested ratio, including B:A=`1:8`.
- `H=32` and `H=128`: the incumbent remains `[4,1]` at every tested ratio,
  including B:A=`8:1`, despite 16,000 alternative executions and consequences.
- `H=8`: critical order-sensitive band described above.
- Equal-consequence `1:1` controls preserve `[4,4]` only at H=0; any incumbent
  prehistory tested (`H>=2`) retains `[4,1]` while M6 overwhelmingly abstains.

These results strengthen the evidence-monopoly finding: sheer alternative
exposure is not sufficient after maturation. They also show that near the
maturation boundary, the order in which otherwise identical opportunities and
consequences arrive can select different stable learned basins.

## Why GATE was not run

The protocol required schedule-phase invariance as a conjunctive control and
required stopping early for genuine scientific ambiguity. MICRO falsified that
control with a deterministic, mirrored, repeatable result. Running six more
seeds cannot identify a unique ratio-only law, and changing the schedule or
classification after seeing the result would be an unregistered rescue.

Any continuation must therefore be a separately named, separately
preregistered successor that treats temporal ordering as an explicit physical
axis. It may not reinterpret or rerun this MICRO.

## Frozen record

| artifact | commit / tag |
|---|---|
| Protocol | `e4702ef8a2db154aa3bd2bb47303799b7efd5058` / `ssa1-s-exposure-bias-map-protocol-v1` |
| Implementation | `935475afa89ba14e20105ba4bc04c067604e1b80` / `ssa1-s-exposure-bias-map-implementation-v1` |
| PROBE Classification C | `96af6657254332c8af4a336f3354142bfd0ffc1a` / `ssa1-s-exposure-bias-map-probe-v1-classification-c` |
| MICRO Classification E | `e9c028727d37ca8ffdb2097725997af017c835cb` / `ssa1-s-exposure-bias-map-micro-v1-classification-e` |

Artifact SHA-256:

- Protocol: `72cf9a51b2a0d68c99fa7fb4a7d8dd46f3748ac36ab177e934ed028315a5880b`
- Evaluator: `9bbfed24dc0e70b5c5db65c214385124db928ff4cf6ae84229ef4120e1d23b22`
- PROBE report: `b4d761cf09ebd601ee4d43a384f59d3b16c784294807039b143ce0ad5d30e8d0`
- PROBE CSV: `68ff94b11549498dcb68ba6389602ab9ce35353fb941c0d0f610c5cb79066cfa`
- MICRO report: `4bee8bf75bdf437ea8a4efc0f9ea1967108f8a4045f3e906333c139a1477f48b`
- MICRO CSV: `722902179f74298ae17b86f403b5c8988319fd42f4b0802e87f2843aec9ab989`

## Program state

```text
Frozen Organism v1   unchanged
SSA1                 Classification C
SSA1-C1              Classification C
SSA1-C2              Classification A under sufficient paired contrast
SSA1-R               Classification C under natural rich-world contrast
SSA1-S               Classification E: ratio-only map scientifically ambiguous
SSA1-S GATE          not run
SSA2                 blocked
```

## Validation

- Formatting passed.
- The focused SSA1-S library test target compiled in release mode with
  `--no-run`; neither PROBE nor MICRO was re-executed.
- Strict Clippy passed for the library and SSA1-S binary with only documented
  frozen-source/evaluator lint classes allowlisted.
- The definitive surface refused before evaluator entry with exit `2`.
- Every frozen-parent SHA-256 listed in the protocol remained exact.
- The worktree contained only the separately named SSA1-S protocol,
  evaluator/binary registration, frozen PROBE/MICRO artifacts, and this handoff.
