# CJ0-OR ordinary convergence GATE v1

Outcome: **POSITIVE**.

- rows: `72/72`;
- independent clauses: `1008/1008`;
- exact replays: `72/72`;
- simultaneous refractory-suppressed rows: `12`;
- positive-skew two-output rows: `60`;
- external SPIKEs / source firings: `8640/2160`;
- convergence / downstream / crossing totals: `1056/1056/1056`;
- stale-route deallocations / incidental proposals: `288/0`;
- constructed CELL / ARROW instances: `6624/4680`;
- aggregate persistent substrate bytes: `617472`;
- ledgered work: `96097` operations.

Each isolated route reached the ordinary convergence CELL and downstream CELL. Both routes together reached them at least once and at most twice. Simultaneous cardinality suppression is attributed to the frozen refractory rule; the threshold-2 `0,0,1` controls are excluded as saturation/conjunction rather than disjunction. Every queue drained naturally, every idle follow-up was inert, and there was no autonomous source refiring or runaway propagation.

This stage does not change PX0-PX2, reinterpret a PX3 negative, add an OR law, or advance authority.
