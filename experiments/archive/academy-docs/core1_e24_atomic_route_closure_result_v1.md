# CORE1-E24 — Atomic Route Closure Result v1

## Status

**FALSIFIED AT GATE 2.** Atomic closure repaired complete-route formation, but
it did not produce an outward motor action. The full E14 learning regimen,
consequence gate, and autonomous probes were not run. No repair or rerun
occurred.

## Candidate behavior

The default-off E24 flag admitted newly proposed direct candidates to the same
source-firing event's unchanged subdivision loop. Reference, exact Reference
replay, and Production all produced the same first-turn observation:

```text
source fires                    1
context trace fires             1
babbler fires                   1
complete pairs before           0
paired proposals                2
complete pairs after            2
positive / negative pairs       1 / 1
outward action                  none
Modulatory deliveries           0
PQLC updates                    0
passive USED-PENDING            0
temporary E22 return edges       8
physical work                  70
physical tick                   3
natural quiescence           true
```

Gate 1 therefore passed exactly: same-event formation created live complete
source → contact → motor routes while preserving both generic signs.

## Decisive falsifier

Despite those complete routes, no motor fired and no outward action crossed.
Gate 2 therefore falsifies the E24 hypothesis as a sufficient explanation of
the E14 frontier:

> Non-atomic route formation was real, but repairing formation alone is not
> sufficient for motor participation.

The eight E22 return edges sharpen the result. E22 creates such an edge only
when an outgoing generated-contact Drive arrow actually emits. Thus local
contact-route emissions did occur across the four available motors, and the
participation-born return topology was created. Yet those emissions did not
integrate into a motor threshold crossing.

The causal boundary is now:

```text
context/source activity
-> atomic complete route formation       yes
-> contact-route Drive emission           yes
-> E22 temporary return topology          yes
-> motor threshold crossing                no
-> outward action                          no
-> changed-world consequence               no
-> Modulatory/PQLC                         no
```

## What E24 does not decide

The frozen observation contains one positive and one negative complete pair
for the inspected motor. It also retains the unchanged route and babbler
delays. E24 was not designed to discriminate balanced signed cancellation from
incidence-timing mismatch, or from another motor-local integration condition.
Selecting among those explanations would require a new explicit hypothesis.

Accordingly, E24 earns no route-closure physics. It moves the frontier one
physical step downstream:

> Why do actual complete contact-route emissions fail to produce a motor
> firing in the frozen E14 admission?

Credit remains out of scope. E22 continues to be earned candidate physics for
later consequence once an outward causal action exists.

## Evidence discipline

- protocol commit: `e94d2e3`;
- candidate implementation commit: `6b98fcb`;
- evidence marker: emitted once;
- Gate 1: passed;
- Gate 2: failed as preregistered;
- Gate 3/full E14: not run;
- matrix/autonomous probes: not created;
- post-result repair/rerun: none;
- exact replay and Reference/Production equality: passed;
- strict release Clippy: passed;
- focused core in-flight and Academy blocked-return controls: passed;
- formatting and `git diff --check`: passed.

Evidence:

- `experiments/results/core1_e24_atomic_route_closure_v1/preflight.csv`
- `experiments/results/core1_e24_atomic_route_closure_v1/report.md`

SHA-256:

- protocol:
  `314df605a6f1395e1b99d4849d7f36730080055f5e934e7041dcfa03caa93482`;
- implementation audit:
  `ca5ab1a48cdd4e9152e8630d1700e4df57f0707630e5dcd9c0da010ec996d1fa`;
- E24 core candidate:
  `d52798ab3eb23aa2c3507b5fdf678fe31a81ee77c3ccfa966adb9fc1e0b7c449`;
- Academy enable surface:
  `4fb80ce765033cba3ce6e44615fc40834d20d0fcc0a3e222383dca1c61cd3693`;
- evaluator:
  `9377feb73eff8f2150b9385317d3e232f6d7498a01c3b2a5bfd415605e706fd1`;
- bounded preflight:
  `3cfb4d93150b1327c8619940b9244e5512807d43aab1c906f5b6e48c4094866e`;
- generated report:
  `94b83b3096510135315108c6afacc908f8f4203323e01b3673a800e36173b879`.
