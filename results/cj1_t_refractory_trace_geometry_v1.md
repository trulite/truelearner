# CJ1-T refractory/trace window geometry v1

Outcome: **POSITIVE GEOMETRY**.

- geometry: **R < T; ordinary local return closes the old trace before retraversal**;
- first actual same-path retraversal offset: `1`;
- last offset with first trace observed live at second arrival: `4`;
- same-path threshold-2 firings across sweep: `0`;
- distinct same-tick traversals/local firings: `2/1`;
- independent A/B live-trace probes: `1/1`;
- rows: `7/7` passed;
- exact replay: `true`;
- all naturally quiescent: `true`;
- native work: `183` operations;
- authoritative PX0 law changed: `false`;
- candidate/CJ1 MICRO/GATE executed: `false`.

The unchanged unit-coupling physics admits a second same-path traversal at offsets 1 through 4 while the first eligibility is live immediately before the source arrival. That arrival performs ordinary local-return closure before the source fires and writes the new traversal trace. The receiving unit contribution has also decayed before every possible retraversal, so same-path repetition never fires the threshold-2 locus. Two distinct paths traverse at the same tick, retain independently observable live traces, and fire the locus once. This geometry resolves repetition, but it does not repair CJ1's separate mature coupling-2 amplitude substitution.
