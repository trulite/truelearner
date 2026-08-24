# PX3-R6 return-triggered trace readout protocol v1

Status: **PREREGISTERED; DEVELOPMENT EVIDENCE UNSPENT; PX3 AUTHORITY NEGATIVE**.

Start: frozen R5-B commit `425fd8f`, tag
`px3-r5-three-factor-return-attribution-carryover-negative-v1`.

| frozen input | SHA-256 |
|---|---|
| authoritative PX0 law | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| R5 source | `13ad86ab078e5fb72bdcbd0b5bff87f85f4cca0a493cda33d22f8dac647ac4fe` |
| R5 CSV | `947257da74420ca4d8a1dc4f49402ddd63bb7ae6d1fe758362c6779365bc73cf` |
| R5 report | `c88dcec19cd18de0723ec927dcaec0829efff1b5c7c4f867b598161b9d61acd1` |
| R5 audit | `49870f078da9b28d222b456ebea54bb72babf017dfdbde76be4e590bf105cbfe` |

## Question

> Can genuine return trigger a one-time read of separate live P and X
> footprints, so partial causal evidence never enters the attribution cell at
> all, using only authoritative PX0 CELL/ARROW/SPIKE physics?

## Existing-law boundary

CJ-B's “locally gated ARROW” is not imported. It is a separate development
substrate law, absent from authoritative PX0. R6 uses only ordinary cells,
arrows, thresholds, decay and firing.

The frozen geometry is:

```text
P ordinary unit trace --delay 1--> FP(threshold 2)
X ordinary unit trace --delay 0--> FX(threshold 2)

ordinary normalized R trace --unit--> FP
                            --unit--> FX

FP --unit--> M(threshold 2)
FX --unit--> M
M --delay 1, unit--> P(threshold 2)
```

P and X each leave one unit in separate footprint cells at the aligned read
tick. Without R, neither footprint fires and M receives zero arrivals. R adds
one unit to each footprint. A still-live footprint reaches two, fires once and
resets; only those R-triggered readouts can reach M.

Repeated footprints arrive at most once per tick per physical path. Under the
unchanged decay-one-per-tick and source refractoriness, a footprint without R
must remain subthreshold rather than accumulate across adjacent happenings.
R6 tests this for 100 consecutive episodes rather than assuming it.

## Exact matrix

Fresh seeds `3701,3709` use normal and mirrored allocation/topology. Six rows
execute in order for each seed:

1. `px-no-return-100-adjacent` — 100 consecutive real P/X episodes, no R;
   FP/FX may refresh but neither readout nor M may fire and M must receive zero
   arrivals;
2. `complete-pxr` — one P/X episode plus timely real normalized R; FP and FX
   each fire once, M fires once, unit echo credits the candidate without P
   refiring;
3. `p-only-r` — P footprint plus R, X blocked; only FP may fire, M must not;
4. `x-only-r` — physically driven X footprint plus R, P absent; only FX may
   fire, M must not and no candidate may form;
5. `px-late-r` — R trace arrives after FP/FX decay; no readout or attribution;
6. `adjacent-current-r` — adjacent P/X histories followed by R aligned to the
   second currently-live footprints; exactly one FP/FX/M readout and one weak
   echo.

Exactly 12 rows execute from fresh state twice for exact replay.

## Required measurements

Every row serializes actual primitive/P/X/R participation; FP/FX arrival,
firing and impulse; all arrivals at M; M firing; echo crossing; P firing;
candidate traversal/resistance/liveness; proposal/return work; storage;
fingerprints; quiescence and replay.

The decisive invariant is physical, not semantic:

```text
R absent -> FP fires 0, FX fires 0, M arrivals 0, M fires 0
R timely + both footprints live -> FP 1, FX 1, M arrivals 2, M fires 1
```

Late R and either missing footprint cannot produce M. The unit echo may update
eligible structure but may not itself refire threshold-two P.

## Classification

- **R6-A POSITIVE:** all 12 rows pass, especially zero M arrivals/firings after
  100 adjacent no-return episodes.
- **R6-B NEGATIVE:** any no-return accumulation, missing/late-factor alias,
  failed timely read, P refiring, replay/quiescence failure or matrix invalidity.

## Evidence discipline

All Rust work occurs only in E2B. Preflight is non-propagating and creates no
artifact. After implementation and a separate execution protocol are frozen
and tagged, `--r6` executes exactly once and its atomic result is preserved
without tuning, rescue or rerun. R6 cannot alter PX0 or retry PX3 authority.
