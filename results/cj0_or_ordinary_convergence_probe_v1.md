# CJ0-OR ordinary convergence PROBE v1

Outcome: **POSITIVE**.

- rows: `8/8`;
- independent clauses: `112/112`;
- exact replays: `8/8`;
- simultaneous refractory-suppressed rows: `6`;
- positive-skew two-output rows: `2`;
- external SPIKEs / source firings: `960/240`;
- convergence / downstream / crossing totals: `108/108/108`;
- stale-route deallocations / incidental proposals: `32/0`;
- constructed CELL / ARROW instances: `736/520`;
- aggregate persistent substrate bytes: `68608`;
- ledgered work: `10434` operations.

Each isolated route reached the ordinary convergence CELL and downstream CELL. Both routes together reached them at least once and at most twice. Simultaneous cardinality suppression is attributed to the frozen refractory rule; the threshold-2 `0,0,1` controls are excluded as saturation/conjunction rather than disjunction. Every queue drained naturally, every idle follow-up was inert, and there was no autonomous source refiring or runaway propagation.

This stage does not change PX0-PX2, reinterpret a PX3 negative, add an OR law, or advance authority.
