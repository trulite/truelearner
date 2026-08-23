# CJ0 ARM CJ-B locally gated ARROW GATE v2 protocol

Status: **PREREGISTERED; GATE V2 UNSPENT; TERMINAL DEVELOPMENT SURFACE**.

## Frozen v1 and isolation

GATE v1 remains a frozen terminal development negative at tag
`cj0-b-locally-gated-arrow-gate-v1-frozen-negative`. Its protocol,
implementation, results, audits, tags, and handoff are immutable and are not
reclassified by this fresh protocol.

The candidate physical module remains byte-identical at SHA-256
`ef0de37a9ac54b632b991f0d4647a5ee78c23810084d61497c88d6f757ec2188`.
GATE v2 adds an evaluator only, uses fresh physical identities, and ends the
development lane at GATE regardless of outcome. PX3, PX-C, and PX4--PX8 remain
unchanged.

## Mechanically unique correction

The sole schedule correction is uniform and mirrored: every changed-world
round uses within-round offset `2` instead of v1's offset `4`. The alternating
order remains A+D then C+B on even rounds and C+B then A+D on odd rounds. The
same offset applies to every variant and both timing strata; no route, identity,
or expected outcome is inspected to choose it.

All other flat schedules remain unchanged:

- S0 initial spacing/offset `4/2`, changed spacing `10`;
- S1 initial spacing/offset `6/3`, changed spacing `12`;
- the post-initial ordinary gap remains `25` ticks;
- acquisition counts, later gap, observation order, pressures, effects, and
  assertions remain unchanged.

For S0 the changed world begins at tick 55. The first alternating cluster is
A+D at 55 with physical delivery at 57 and C+B at 57 with physical delivery at
59, both before the ordinary pressure boundary at 60. For S1 the corresponding
ticks are 70/72 and 72/74, before boundary 80. The correction changes phase
opportunity only; it does not change the law or pressure.

## Eight fresh cells

Execute the same four normal/mirror/allocation/permutation variants crossed
with S0/S1, using fresh namespaces `0x9_b710_0000` through
`0x9_b780_0000`. Each cell reruns the complete GATE v1 flat, reversal,
self-evidence, recursion, ordinary convergence, temporal, replay, recurrence,
quiescence, fingerprint, work, and storage surfaces. The preregistered
GATE v1 claims G0--G9 remain unchanged except that G0 binds this protocol and
fresh evaluator and G3 expects the uniformly corrected changed schedule.

## Timing-boundary characterization

Each row also builds fresh anonymous local matter under the same physical law:
a threshold-3 source at position 0, threshold-3 destination at mirrored
position 2, an ordinary return ARROW, and an outward crossing. No initial
source-to-destination ARROW exists.

The following stages are independently serialized:

1. **first proposal:** at tick 8, destination state 2 and an external source
   firing create one generic coupling-1/resistance-1 proposal with delay 2;
2. **pressure crossing:** its queued delivery is due at tick 10, exactly on an
   ordinary pressure boundary, which applies one pressure update and
   deallocates that weak proposal;
3. **generation check:** the queued stale generation is checked but cannot
   fire the destination or cross outward;
4. **replacement:** at tick 11, ordinary destination state plus another
   external source firing creates exactly one generic replacement;
5. **maturation:** delivery at tick 13 fires the destination, ordinary return
   activity matures the live replacement to coupling 2 and resistance 4;
6. **held-out execution:** fresh ordinary activity at tick 14 uses that same
   replacement and produces exactly one outward crossing.

The control must record proposal count, pressure updates, deallocation count,
generation checks, destination firings, crossings, arrow record/live count,
generation, resistance, coupling, stage fingerprints, work, replay, and
quiescence. This control characterizes the v1 boundary mechanism; it does not
remove, defer, or special-case ordinary pressure.

## Clauses and execution

G0--G9 are the frozen GATE v1 clauses. G10 requires every timing-boundary stage
above, exact replay, natural quiescence, and fresh identities. All eleven
clauses must pass in all eight rows.

Before the sole run require format, focused tests, strict all-target Clippy,
zero dependencies, exact frozen hashes, physical-source vocabulary audit,
neutral refusal behavior, no-cell preflight, artifact absence, and changed-path
isolation. The sole evidence command is:

```text
cargo run --release --manifest-path arms/cj-b-locally-gated-arrow/Cargo.toml \
  --bin gate_v2 -- --gate-v2
```

It atomically publishes one fresh CSV/report pair. It may not be rerun,
regenerated, tuned, or repaired. A pass freezes development readiness and a
compare-ready physical port contract; a failure freezes another development
negative. No later scientific surface may be created.
