# CV0/J0 bounded local contact genesis result audit v2

Status: development positive.

Gate E parent: `6af342c` (`cv0e-j0-gate-e-positive-v2`).

E2B full-matrix worker: `idlj212ugl6t0lrd7klkc`.

## Result

- cases: `240/240`;
- Reference/Production rows: `480/480`;
- clauses: `5480/5480`;
- Gates A--D: PASS;
- Gate E: PASS;
- Gates F--I: PASS;
- Gate J Reference/Production equality: exact;
- replay: exact;
- natural quiescence: true;
- maximum PhysicalWork: `35`.

The cumulative result establishes:

- one opportunity creates exactly two ordinary junctions and four ordinary
  links with symmetric `+1/-1` outgoing alternatives;
- a consequence at C+ retains only the positive relation;
- swapping the return to C- retains only the negative relation;
- identity, slot, geometry, sign-order, and anchor permutations do not change
  selection;
- neither useful removes both relations; both useful may retain both;
- unsupported links decay, orphan junctions deallocate, generations invalidate
  stale references, and slots are reused;
- placing both signed alternatives behind one shared junction causes both
  participating alternatives to consolidate, preserving the honest physical
  attribution limit;
- boundary anchors do not act, generate candidates, receive credit, or alter
  selected outcomes.

No CELL resistance was consolidated. Useful junctions remain only because
their live incident links require them.

## Artifact hashes

- matrix CSV: `ff1e4631a518c6f56861b5fa21dac74580fae250b36b5cc633ee545dd8c87abc`;
- report: `a4cf0418a6f7503657109e161af8cb166e574f8620609f84c73087c94fce28c4`.

No SV1 or later stage had executed when this result was frozen.

