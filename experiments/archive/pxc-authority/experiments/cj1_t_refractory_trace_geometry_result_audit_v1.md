# CJ1-T refractory/trace geometry result audit v1

Status: **VALID POSITIVE DEVELOPMENT GEOMETRY**.

## Frozen execution

- executed implementation commit:
  `40289692f81af4471f8632532be28806bbc4f340`;
- E2B persistent sandbox: `i6x9gykt9tvp6xfz5z8ra`;
- authoritative PX0 SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- protocol SHA-256:
  `4c904459c7684261d1a5c63b1ff16eb3a6dc47dbf8596ec23386f254834c8762`;
- executed runner SHA-256:
  `010227f7bd1bcd140dd08e83d77b3e9b1748f2f59d3fa02511005798a669665c`;
- result CSV SHA-256:
  `d1b646d2689045adec746652392ff870354bcfa7505ab85c280d9eae4ebc8e70`;
- result report SHA-256:
  `879f33846c74d60b37304a1e3bafb78d51246b3029f3b1efc9fe8ce69be1670e`.

The exact preregistered command executed once in E2B. The result contains seven
unique rows plus one header, no staging remnants, exact duplicate replay and
natural quiescence in every row. Independent schema and native-accounting
checks pass. The authoritative PX0 law remained byte-exact and no local Rust
process ran.

## Window geometry

The result establishes `R < T`, with an ordinary close interposed before
retraversal:

| second offset | first eligibility at arrival | return close | second source firing | second traversal | locus firing |
|---:|---|---:|---:|---:|---:|
| 0 | live | 1 | 0 | 0 | 0 |
| 1 | live | 1 | 1 | 1 | 0 |
| 2 | live | 1 | 1 | 1 | 0 |
| 3 | live | 1 | 1 | 1 | 0 |
| 4 | live | 1 | 1 | 1 | 0 |
| 5 | expired | 0 | 1 | 1 | 0 |

The source is first traversable again at offset `1`; the first ARROW-local
eligibility remains live through arrival offset `4` and expires before offset
`5`. Therefore the nominal refractory interval is shorter than the eligibility
interval.

However, at every retraversable live offset, the second arrival first performs
PX0's ordinary local-return update, which clears the old eligibility. Only then
does the source fire and the ARROW traverse again, writing a fresh eligibility.
There are two actual traversals within the nominal eligibility time interval,
but there is no second traversal that retains the first same-path eligibility
through the traversal.

Separately, one bounded unit at the receiving CELL decays before the first
possible retraversal. Consequently no same-path sweep row fires the threshold-2
locus.

## Distinct-path control

The simultaneous A+B control records two source firings, two actual crossings
through two separately allocated physical ARROWs, two eligibility writes and
one threshold-2 locus firing. Independent post-event A and B probe clones each
perform exactly one native local-return close, establishing that both separate
path traces were live in the same post-event state.

## Interpretation boundary

For coupling-one repeated participation, unchanged PX0 physics already
separates the cases:

- same A path repeated: source refractoriness blocks offset `0`; at later
  offsets ordinary return closes the old path trace and unit local state has
  decayed, so no two-unit coincidence forms;
- distinct A and B paths: both traverse at the same tick, both path traces are
  live independently, and the local coincidence fires.

This validates the repetition geometry the invalid CJ1 fixture failed to
instantiate. It does not establish full CJ1 Classification A or B: the frozen
unchanged-physics mature coupling-2 row still shows that one strong traversal
can supply a threshold of two. Repetition is handled by existing physics;
amplitude substitution remains a separate boundary.

CJ1-T is development-only. It authorizes no mechanism change, CJ1 MICRO/GATE,
definitive evidence, authority claim, PX3 restart or PX-C.
