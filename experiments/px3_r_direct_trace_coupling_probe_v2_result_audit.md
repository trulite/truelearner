# PX3-R direct physical trace coupling PROBE v2 result audit

Status: **FROZEN MATERIAL NEGATIVE; EVIDENCE SPENT; PX3 ABSENT**.

## Sole execution

The registered retry command ran exactly once from implementation commit
`15fc87716e5239de3628f4617c20d695d9878fd2`, tag
`px3-r-direct-trace-coupling-probe-v2-implementation`. It emitted one
evidence-spent marker, published atomically, classified `FROZEN_NEGATIVE`, and
exited `1`. There was no rerun, tuning, rescue, or regeneration.

## Matched-marginal physical result

All prerequisites and negative controls passed:

- correspondence resistance: `23|23|23|23`;
- direction resistance/live: `18|18|18|18` / all live;
- training continuation, consequence, trace, route-return, and outward counts:
  `12|12|12|12` each;
- candidate arrivals/impulse per route: `12|12|12|12` /
  `23|23|23|23`;
- only recurrent `0<->1` and `2<->3` ARROWs persisted, resistance `17` in
  each directed orientation;
- absent opportunity, six-tick delayed return, four-tick non-overlap,
  correlation without route participation, and stale single occurrence left
  no live inter-trace structure;
- all finite executions quiesced, autonomous source refiring was zero, and
  duplicate complete replay was exact.

A positive cannot be explained by individual-route strength: all four vectors
are exact. Nevertheless the held-out discriminator failed.

## Decisive trained-versus-crossed failure

During a trained held-out occurrence, each active trace received the matured
impulse `2` from its pre-existing recurrent neighbor. During crossed held-out
occurrences `0+3` and `2+1`, each route still emitted down its old trained ARROW
whenever that individual route fired, even though the old neighbor was not its
contemporary co-participant. The same crossed occurrence also exposed a fresh
weak crossed ARROW. Consequently:

- trained held-out local impulse: `2|2|2|2`;
- crossed held-out local impulse: `3|3|3|3`;
- trained post-use matrix retained only resistance-`17` old edges;
- crossed post-use matrix retained resistance-`17` old edges plus
  resistance-`4` fresh crossed edges.

Direct ordinary ARROWs therefore remember recurrent physical adjacency, but
their transmission is driven by individual source firing, not by contemporary
joint participation. Crossed use activates the old organization and creates a
new weak one; it does not isolate the contemporary organization from the sum
of individually firing routes. The mandatory held-out physical discriminator
is false.

Repair would require coactivity-gated transmission, a recruited convergence,
or another conditional substrate law. Those are additional mechanisms outside
direct-edge ARM A and are forbidden as rescue. The result stops before MICRO,
swap/relearning, GATE, definitive evidence, or authority. No swap behavior is
claimed; executing it after the failed core discriminator would spend evidence
on an ineligible mechanism.

## Frozen artifacts and accounting

- executed evaluator SHA-256:
  `de2f55f74f0bc8a9b12cd0ad07c403278ecc72bcf61fa097f470be77f3a8569e`;
- arm law SHA-256:
  `0027d8356170a673c3045980fed8a5a3f1509277072753bba03cb3b43143f6c9`;
- physical world SHA-256:
  `abec17d32b110538133044f2366b27f4104b632101ed2d0f34e13f10d484a409`;
- CSV SHA-256:
  `421008eefe9e6a6ae82c0871b5f85bc31a3e722b4304f5b7433974200fcb89bb`;
- report SHA-256:
  `ce52593ea917b638e77f923746af6ef42638bf3a40e0a528f4dbf428ac650c28`;
- result storage: `3160` bytes;
- ledgered work: `12,489,546` operations;
- staging paths: absent.

Authoritative PX0--PX2, the Class-D PX3 negative, the generic collapse, and the
failed v1 candidate PROBE remain byte-identical. No shared code, broad
historical suite, definitive evidence, or authority matrix was touched.
