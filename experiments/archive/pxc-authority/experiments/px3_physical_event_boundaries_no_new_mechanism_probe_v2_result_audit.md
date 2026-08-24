# PX3 physical event-boundary no-new-mechanism PROBE v2 result audit

Status: **FROZEN FIRST-CLAUSE FAILURE; PX3 ABSENT; PROBE EVIDENCE SPENT**.

## Sole execution

The preregistered command executed exactly once from frozen implementation
commit `b33a104ad762b836d59c0ef34a2285d0b5bb349c`, tag
`px3-physical-event-boundaries-no-new-mechanism-probe-v2-implementation`:

```text
cargo run --release -p px0-physical-correspondence \
  --example px3_physical_event_boundaries_probe -- --probe
```

It emitted exactly one
`PX3_PHYSICAL_EVENT_BOUNDARIES_NO_NEW_MECHANISM_PROBE_EVIDENCE_SPENT`
marker and exited `1` with classification `FIRST_CLAUSE_FAILURE`. There was no
rerun, rescue, regeneration, source change, or parameter change after the
marker.

## Frozen artifacts

- CSV:
  `results/px3_physical_event_boundaries_no_new_mechanism_probe_v1.csv`;
  SHA-256
  `5ae68dad569f943d08d945014ed5491a93eb30021b1afc78966ed39fd15b4cc4`;
- report:
  `results/px3_physical_event_boundaries_no_new_mechanism_probe_v1.md`;
  SHA-256
  `941c1754004b43eee9a99b969bfbad9e1fd75257091fc475e4dbb196084eef66`;
- executed source SHA-256:
  `a15f2b1b5070d3fc707b68d0a4f7135834efbd9fc919e6a3c27d60f7751afad9`;
- organism-visible block SHA-256:
  `ac11bd435098469cdf2a16b3d75dddf4285396c3a75aa31a87bff1f775142fee`;
- protocol SHA-256:
  `cae4d2b03b0c094a48348fc34ba49fa16c2ecf47847850e01d66c936efd83a52`.

Both staging paths are absent. The artifacts are preserved unchanged,
including the generated report's over-broad interpretation paragraph; this
audit is the authoritative diagnosis of the failed conjunction.

## First collapse

The frozen CSV shows:

- reference A direction resistance/live:
  `17|17|0|0` / `true|true|false|false`;
- reference B direction resistance/live:
  `17|0|17|0` / `true|false|true|false`;
- both reverse-allocation replicas match those outcomes exactly;
- every route's continuation fired `12` times, but routes in the second
  temporal cluster produced only one consequence, trace, return, and outward
  crossing before deallocation;
- blocked return yielded `0|0|0|0` live directions;
- the single-recurrence post-gap control yielded no outward crossing;
- all acquisition/training/held-out runs quiesced, all source refiring was
  zero, and every duplicate was exact;
- ledgered work was `631,187` operations.

The second cluster's first traversal occurred late enough that ordinary
ten-tick pressure at tick `80` reduced its weak resistance-`3` direction
candidate to zero after emission but before the returned participation trace
arrived. The first cluster returned before that pressure edge and survived.
This is the already-authoritative PX2 O1 opportunity-window behavior.

Therefore matched per-route participation marginals failed, and trained versus
crossed held-out behavior was not a valid PX3 discriminator. This result does
not establish either emergence or absence of relation-specific reusable
organization.

## Program consequence

The mechanically unique smallest correction is to move the complete
environmental schedule two ticks earlier while retaining every substrate law,
CELL, ARROW, SPIKE, topology, threshold, coupling, delay, resistance,
recurrence count, gap, and comparison. A fresh protocol, source commit,
namespaces, and artifact paths are required. This v2 result remains an
immutable negative and may never be rerun.

