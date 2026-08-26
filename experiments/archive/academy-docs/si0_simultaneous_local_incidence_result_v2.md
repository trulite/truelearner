# SI0 simultaneous local incidence result v2

Status: development-positive. No authority or downstream experiment advanced.

Frozen observer/runtime candidate: `518cbc9`
(`si0-simultaneous-local-incidence-v2-frozen-v1`).

Sole evidence execution: E2B `ig7mkj5escd275ica27y6`.

## Result

- Families: `10/10`.
- Rows: `120/120`.
- Identity/insertion baseline equality: `120/120`.
- Reference/Production equality: `120/120`.
- Exact replay: `120/120`.
- Preregistered firing behavior: `120/120`.
- Natural quiescence: `120/120`.
- Pending activity and loads: zero throughout.
- Maximum PhysicalWork: `37`.
- First divergence: none.

The matrix covers same-junction signed incidence, all three threshold
compositions, pre-existing activation, independent junctions, parallel ARROWs,
CELL/ARROW/physical-name and insertion permutations, zero-delay chain,
zero-delay fanout/merge, and a zero-delay cycle.

## Claim

Simultaneous local Drive incidence is invariant to arbitrary handle naming and
insertion order in the tested development matrix. Causal ordering arises from
successive physical waves rather than numeric identity. Same-wave signed Drive
arrivals combine at their junction, threshold/refractory is evaluated once,
and a CELL fires at most once. Zero-delay consequences remain observable in the
next causal wave.

The observer represents each `(tick, phase, wave)` as independent multisets of
incidences, fires, and effects. It does not infer incidence/fire ownership from
recording adjacency.

## Runtime identity

The SI0 physical candidate is byte-identical before and after v2:

- feature declaration: `d7d34bb477bc74657d8d1486d2c04fef759bb5f91ce5b08b805891f0bd75819c`;
- physical law: `f19a89ac92c12cc4910047021c8bdedfa42b4c4dc2f5c3fcfa83e2a0b2a4c978`;
- mechanics: `5f1172a0eaa0628d1775029c44e7a1b5bb2c4525c713b468f756a0705ef822a4`.

## Evidence hashes

- matrix: `4db4e9026f39205155bcba13a88880280e7518026f9680aa1676bb97439ced57`;
- report: `8ff869df75952bdbcd016b10def965bdda9939fa5e33b250ad3ac2752ae896cc`.

SI0 v2 does not itself remove handle ordering from general causal code. That
architectural hardening remains the next separately reviewed change. RS2, CE1,
FD2, ARC, authority, oracle status, and `arch.md` remain unchanged.
