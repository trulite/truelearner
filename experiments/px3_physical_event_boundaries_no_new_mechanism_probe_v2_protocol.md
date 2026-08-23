# PX3 physical event-boundary no-new-mechanism PROBE v2 protocol

Status: **PREREGISTERED AMENDMENT; PROBE EVIDENCE UNSPENT; PX3 ABSENT**.

This protocol preserves and supersedes the unexecuted v1 protocol at commit
`d5229be8bac68dd4661facb40f53bcc0d4ae2cd2`, tag
`px3-physical-event-boundaries-no-new-mechanism-probe-v1-protocol`.
Every v1 scientific clause, authority restriction, organism-visible
vocabulary restriction, source hash, no-old-M3 rule, measurement, conjunctive
interpretation, atomic-publication rule, and no-rescue rule remains exact
except for the timing constants explicitly amended below.

## Pre-execution mechanical collapse

No CELL was entered and no probe, preflight, compilation, or simulation was
run under v1. A read-only schedule calculation against the frozen PX2 O1
boundary found that alternating two clusters with v1 round spacing `32` and
between-cluster gap `14` creates alternating per-route reuse intervals of `18`
and `46` ticks (including the retained PX2 propagation offset). The weak PX2
direction opportunity begins at resistance `3`; ordinary ten-tick pressure
can therefore deallocate it during the `46`-tick interval before recurrence
can act.

That schedule would diagnose the already-frozen PX2 O1 opportunity window,
not the PX3 relation-specific-state question. This is a mechanically unique
fixture correction, not a substrate or representational choice.

## Sole amendment

The exact v2 constants are:

- recurrence rounds: `12` (unchanged);
- within-cluster spacing: `0` (unchanged);
- between-cluster physical gap: `8` ticks;
- round spacing: `18` ticks;
- first acquisition use after direction opportunity formation: `18` ticks;
- held-out gap: `14` ticks;
- post-gap: `34` ticks;
- subthreshold control observation: `54` ticks after its sole presentation.

Alternating early/late placement now yields per-route reuse intervals `10` and
`26`, both inside the measured PX2 opportunity regime once the first ordinary
return occurs. An eight-tick gap remains outside the four-tick local return
window and allows ordinary CELL state decay. The cadence remains matched
between reference A (`01 | 23`) and reference B (`02 | 13`), and all four
routes still receive identical recurrence counts and matched early/late
opportunities.

The subthreshold clause is evaluated after its fixed `54`-tick pressure gap;
the weak once-returned opportunity must then be physically deallocated and
must emit no outward crossing. No evaluator cutoff or reset occurs.

## Exact execution and artifacts

After the v2 implementation is committed, tagged, and validated without
entering a CELL, exactly one command may spend evidence:

```text
cargo run --release -p px0-physical-correspondence \
  --example px3_physical_event_boundaries_probe -- --probe
```

The evidence marker and final/staging artifact paths remain exactly those
preregistered in v1. Any final or staging artifact pre-existence is a hard
refusal. A positive, frozen negative, or first-clause failure is published
atomically and never rerun.

