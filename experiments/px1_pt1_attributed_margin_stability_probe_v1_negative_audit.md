# PX1-PT1 attributed-margin stability PROBE v1 negative audit

Outcome: **FROZEN DEVELOPMENT NEGATIVE — COLLAPSE BEFORE ATTRIBUTION**.

The frozen PROBE executed once from implementation commit
`6d217e3deb43b34f0f828da15678b9667f8f96f6`. It was not rerun or rescued.

- PX0 correspondence resistance: `[22,22]`;
- supported/unsupported branch firings: `[8,0]`;
- outlet firings: `[0,0]`;
- continuation resistance: `[0,0]`;
- trace arrivals/firings: `[0,0]` / `[0,0]`;
- local branch returns: `[0,0]`;
- extra source firings: `0`;
- training/held-out/post-gap quiescence: all true;
- duplicate replay: exact.

## First collapse

Static replay of the frozen tick and pressure law identifies the first missing
physical edge:

```text
continuation installed after acquisition at resistance 2
tick 66 ordinary pressure                     2 → 1
supported branch fires at tick 69
continuation spike scheduled for tick 70
tick 70 ordinary pressure runs before
queued-arrow generation validation            1 → 0
queued continuation spike becomes stale and is discarded
outlet receives only its one support impulse
outlet never fires
```

Thus the physical target-execution trace was never exercised. This negative
does not support or reject the attribution hypothesis.

The minimum mechanical retry is separately named and changes only the equal
anonymous continuation opportunity reserve from `2` to `3`. No topology,
threshold, delay, coupling, support, return, pass rule, or PX0 law may change.
The extra unit is not route-specific; it permits either opportunity to survive
the two demonstrated ordinary-pressure events long enough for contemporary
physical evidence to arrive.

- CSV SHA-256:
  `0fc41bcfa2a74f21cd05364fe1199f78b1a10c14616f21b68696816c746d55d5`;
- report SHA-256:
  `1149d2fdba4d07fc462893f742dc29be0a68f228b57fc1aea0b8d75277af0eee`;
- implementation SHA-256:
  `767b6804faed71913bbbc15794b5a5e2a1c3fd0a1437716ac2aa54e35795236c`;
- definitive evidence: none;
- PX1 authority: absent.
