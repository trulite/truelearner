# RS0 recurrent stability characterization result audit v1

Status: development-positive characterization; outcome B.

## Execution and integrity

The frozen RS0 matrix executed exactly once in fresh E2B worker
`ings3j8djoetx2ququjeg` from preflight commit `6375a30`.

```text
physical cases                         400 / 400 PASS
mechanics rows                         800 / 800 PASS
same-mechanics exact replay            800 / 800 PASS
Reference / Production physical match  800 / 800 PASS
frozen predicates                      800 / 800 PASS
absolute-phase classification          invariant
persistent discriminators              10 / 10
maximum PhysicalWork                   288
classification                         B — strong recurrence generally persistent
```

Evidence hashes:

```text
matrix  b86f465040238e7685b4d42f978e984d77360d3e87e4a7d612b0ac21c1e52a4b
report  5f8cdacd7b498e2bc7b42a956a4ea5f94a3f4f4f65beb7fa5d2f16afad5813eb
```

No plasticity, Modulation, CE0 efficacy update, candidate stabilizer, local
forgetting termination, or authority operation occurred.

## Empirical stability boundary

Activity died naturally in every topology lacking an executable recurrent
return:

- one-way coupling 2, threshold 2: two firings, one traversal;
- reciprocal coupling 1, threshold 2: one firing, one subthreshold traversal;
- reciprocal coupling 2, threshold 3: one firing, one subthreshold traversal;
- all zero-delay cycles: one complete lap, then the initiating CELL's existing
  same-tick refractory state stopped the return;
- both eight-CELL acyclic chains: exactly eight firings and seven traversals.

Activity remained exactly periodic whenever a recurrent cycle had efficacy at
or above threshold and any positive total cycle delay:

```text
reciprocal c1 / threshold1 / delays 1+1       period 2 firings / 2 ticks
reciprocal c2 / threshold2 / delays 1+1       period 2 firings / 2 ticks
reciprocal c2 / threshold2 / delays 2+2       period 2 firings / 4 ticks
reciprocal c2 / threshold2 / delays 3+3       period 2 firings / 6 ticks
reciprocal c2 / threshold2 / delays 0+1       period 2 firings / 1 tick
cycle length 3 / delay1                       period 3 firings / 3 ticks
cycle length 4 / delay1                       period 4 firings / 4 ticks
cycle length 8 / delay1                       period 8 firings / 8 ticks
```

Relative ARROW phase did not change the class. Absolute clock phase did not
change any class. Coupling 1 at threshold 1 was periodic, proving that the
numerical value 2 is not special; persistence follows whether transmitted
efficacy reaches the next CELL's threshold after refractoriness has ended.

Every periodic family consumed the complete 256-delivery observation segment
and complete 32-delivery continuation segment. Each recorded 256 first-segment
firings, 32 continuation firings, 287 ARROW traversals, and PhysicalWork 288.
All ARROWs remained live above resistance 999,000, so local forgetting did not
terminate or classify the probe.

## Classification

CE0's oscillator was not a special artifact of one reciprocal delay or phase.
Existing refractory physics stops recurrence only when the entire loop closes
within the same physical tick. Once the loop's total delay is positive and its
efficacy reaches threshold, ordinary cycles of lengths 2/3/4/8 remain periodic.

RS0 therefore supports outcome B: strong executable recurrence is generally
self-sustaining under the existing substrate. This independently justifies
investigating a missing local activity-limiting affordance, but RS0 proposes or
selects none.

