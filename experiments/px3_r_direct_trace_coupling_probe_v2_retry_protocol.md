# PX3-R direct physical trace coupling PROBE v2 retry protocol

Status: **PREREGISTERED MECHANICAL RETRY; EVIDENCE UNSPENT; PX3 ABSENT**.

The v1 PROBE is frozen at commit
`005991d8a3dcf54e205fac478d2998e498f36693`, tag
`px3-r-direct-trace-coupling-probe-v1-first-clause-failure`. Its source,
artifacts, hashes, and `12,375,887` ledgered operations are immutable. V1 will
never be rerun or reinterpreted.

## Frozen v1 dependency

V1 failed before the candidate discriminator because weak PX2 direction
ARROWs for the second cluster met an ordinary-pressure boundary before their
first return. Routes `2/3` therefore had direction resistance `0`, while routes
`0/1` reached `17`. The arm opportunity, replay, and quiescence code did not
cause that mismatch.

## Mechanically unique retry construction

V2 uses a fresh isolated crate and fresh namespaces. It preserves the v1 arm
law byte-for-byte, adding only an explicit one-time installation method for
the already-preregistered numeric opportunity. Installation requires an empty
physical queue, requires the opportunity to be absent, and clears no substrate
state; recent-firing storage is still empty because the opportunity was absent.

The exact physical sequence is:

1. construct and acquire all four PX0 correspondence paths with the arm
   opportunity absent;
2. add the four weak PX2 direction ARROWs exactly as v1;
3. give every route one actual matched maturation occurrence while the arm
   opportunity remains absent: `0+1` at tick `64`, `2+3` at tick `72`;
4. after natural drain, install the preregistered opportunity
   radius/overlap/delay `8/1/1` (or the registered control variant);
5. begin candidate training at tick `84`.

The maturation occurrences are serialized as physical work but excluded from
candidate-training counts. They provide equal contemporary PX2 route strength
and cannot form inter-trace ARROWs because the arm opportunity is physically
absent. This is an unchanged-port preparation, not hidden candidate training.

Positive training remains twelve rounds at spacing `18`, paired clusters eight
ticks apart with alternating early/late order. The temporal-spacing control
uses four-tick within-combination spacing, ten-tick cluster spacing, and
twenty-tick rounds beginning at `84`. All four matured directions remain
inside the frozen PX2 opportunity regime.

Every v3 candidate-law clause, symmetric edge exposure, four-route marginal,
held-out discriminator, negative control, anti-representation boundary,
atomic publication rule, and no-authority restriction remains exact.

## Classification and sole command

V2 uses fresh atomic artifacts:

- `results/px3_r_direct_trace_coupling_probe_v2.csv`;
- `results/px3_r_direct_trace_coupling_probe_v2.md`.

After a new implementation commit/tag and no-CELL validation, exactly one
command may spend V2 evidence:

```text
cargo run --release --manifest-path arms/px3-r-trace-coupling-v2/Cargo.toml \
  --bin probe -- --probe
```

The classifications remain `DIRECT_TRACE_COUPLING_CANDIDATE`,
`FROZEN_NEGATIVE`, and `FIRST_CLAUSE_FAILURE`. Every outcome is frozen without
rescue. A positive permits only a separately preregistered MICRO.
