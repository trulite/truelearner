# CV0 bounded local contact genesis resumed Gate-E negative v1

Status: stopped negative at the first failed resumed gate. No repair was made.

Parent: CC0 positive `6ea3e82`.

Protocol: `cv0-bounded-local-contact-genesis-resume-protocol-v1` layered on
the unchanged `cv0-bounded-local-contact-genesis-protocol-v1`.

Frozen candidate: `83a8fdc7c9d7d9a8cb271c5a55fc345f9664b9c8`.

## Result

CV0 resumed successfully through its former blocker:

- **Gate A PASS:** every opportunity created two ordinary CELL contacts and
  four ordinary weak Drive ARROWs. Contact construction was symmetric; the
  outgoing alternatives were exactly `-1/+1`.
- **Gate B PASS:** without consequence, both contact CELLs and all four ARROWs
  died through ordinary local decay.
- **Gate C PASS:** repeated source activation while candidates were live
  remained bounded at exactly two CELL and four ARROW proposals.
- **Gate D PASS:** both ordinary contact slots and all four ARROW slots became
  reusable; old CELL generations remained stale.

Gate E stopped negative. With consequence physically returning only to the
positive contact, the ordinary local laws produced:

```text
positive contact CELL       resistance 1 -> 4
positive outgoing C+ -> X   resistance 1 -> 4
positive incoming P -> C+   resistance 1 -> 1

negative contact and edges  resistance remains 1
```

At the unsupported weak lifetime boundary:

```text
positive relation live = (CELL true, incoming stem false, outgoing true)
negative relation live = (CELL false, incoming stem false, outgoing false)
```

Thus spatial selection itself was correct, but the selected causal relation
was no longer executable from `P`: local consequence at `C+` could consolidate
the participating contact and its outgoing structure, but it had no ordinary
physical path to consolidate the already-traversed incoming stem.

The failure was identical for every root, clock phase, and mechanics:

- cases: 100/100;
- rows: 200/200;
- clauses: 1,880/1,960;
- Gates A--D: PASS;
- Gate E: FAIL;
- failed Gate-E rows: 40/40;
- exactly two failed clauses per Gate-E row:
  `positive_stem_consolidated` and
  `positive_relation_remains_executable`;
- Reference/Production ordered histories: exact;
- same-mechanics replay: exact;
- natural quiescence: true; and
- maximum PhysicalWork: 35.

## Classification

This is not a CELL-lifetime, signed-variation, contact-locality, or
consequence-selection failure. CC0 solved useful contact retention, and the
outgoing signed candidate was selected correctly.

The missing affordance is narrower:

> Local consequence at a generated contact cannot yet close through the
> already-participating incoming stem required to keep the whole candidate
> relation executable.

PQLC can repeat local closure only where suitable ordinary closure topology
already exists. CV0's frozen proposal form creates only the two forward Drive
connections and therefore supplies no such return topology. Adding a return
edge, incoming-arrow lookup, stem special case, or implicit backward credit
inside CV0 would change the frozen candidate and is forbidden.

## Evidence and provenance

- targeted formatting/check/strict-Clippy worker:
  `ifk44bxtlfjlci644r63m` (preserved as the requested compilation worker)
- fresh matrix worker: auto-terminated by the one-shot runner; its client
  output was not retained after the expected non-zero Gate-E stop, while both
  result artifacts were downloaded successfully

No Rust or project audit ran locally.

Hashes:

```text
core lib.rs   04cd3a29db755d477522e39ded7c3562d05070f21bd00ad207b94c320bfe0173
evaluator     13013ede14969c19e30a7d4aef283939fac8e45fd1cb1163b8ec5b687c4e66ce
matrix        f2b57317b7b8f9a6755062cf067501f54c26d85d6da8f745434be2c3043fbc1e
report        03d56a1fb00034cd096e09759dd3937eeb5a5270baa1e30085dd059308771322
```

## Boundary

CV0 remains negative. Gates F--J, SV1, RS2, CE1, FD2, and frozen ARC A2 were
not run. Authority, the oracle, and `arch.md` remain unchanged.
