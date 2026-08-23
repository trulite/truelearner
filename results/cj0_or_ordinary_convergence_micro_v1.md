# CJ0-OR ordinary convergence MICRO v1

Outcome: **POSITIVE**.

- rows: `24/24`;
- independent clauses: `336/336`;
- exact replays: `24/24`;
- simultaneous refractory-suppressed rows: `0`;
- positive-skew two-output rows: `24`;
- external SPIKEs / source firings: `2880/720`;
- convergence / downstream / crossing totals: `360/360/360`;
- stale-route deallocations / incidental proposals: `96/0`;
- constructed CELL / ARROW instances: `2208/1560`;
- aggregate persistent substrate bytes: `205824`;
- ledgered work: `32237` operations.

Each isolated route reached the ordinary convergence CELL and downstream CELL. Both routes together reached them at least once and at most twice. Simultaneous cardinality suppression is attributed to the frozen refractory rule; the threshold-2 `0,0,1` controls are excluded as saturation/conjunction rather than disjunction. Every queue drained naturally, every idle follow-up was inert, and there was no autonomous source refiring or runaway propagation.

This stage does not change PX0-PX2, reinterpret a PX3 negative, add an OR law, or advance authority.
