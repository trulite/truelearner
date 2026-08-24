# LR2 dense topology and PX0--PX2 successor conformance result audit v1

Status: **DEVELOPMENT SPLIT; ARM C SELECTED AS SUCCESSOR CANDIDATE; NO NEW AUTHORITY**.

## Frozen execution

Both development commands ran once, concurrently, from frozen implementation
commit/tag `c23d34eea0701f7dd719f215d8007c79037f0971` /
`lr2-bc-dense-conformance-implementation-v1` in fresh isolated E2B sandboxes:

- B sandbox `i885hmknyzybqg2h87nqs`;
- C sandbox `iy3vz1gr2fyx8t5j611vo`.

Both commands exited normally, published atomically, replayed every row exactly
and quiesced naturally. The two CSVs each contain one header plus 116 rows and
exactly 28 columns on every line.

Artifact SHA-256 values:

- B CSV: `6b335619bd72fb9130bf9cdaf6679c0648970507ff778025a6639a0dfc256106`;
- B report: `48fb92d5052dfa9303e9ed2857af37cc2ca588b2f4200414d9799f56dcaf0bdd`;
- C CSV: `d888db91b390849ad2bbbb61705af324c3e8892f4299a68902aef9547ff7f520`;
- C report: `540a523d9dad59db8069c060086e738848b574f79172917166bb0563d39e5f58`.

## Result

| Surface | Arm B | Arm C |
|---|---:|---:|
| Dense specificity | 32/40 rows; 312/320 clauses | 40/40; 320/320 |
| PX0 lifecycle conformance | 24/24; 192/192 | 24/24; 192/192 |
| PX1 participation conformance | 24/24; 192/192 | 24/24; 192/192 |
| PX2 direction conformance | 28/28; 224/224 | 28/28; 224/224 |
| Total | **108/116 NEGATIVE** | **116/116 FUNCTIONAL POSITIVE** |
| Total/max work | 36,540 / 1,102 | 37,760 / 1,259 |
| Maximum persistent bytes | 2,880 | 2,784 |
| Added transmission distinction | none | `Drive` / `Modulatory` |

C used 3.34% more total work and 14.25% more worst-row work. In the matched
dense fixtures it nevertheless used 96 fewer persistent bytes because B's
separate physical compartments cost more storage than C's ARROW mode field.
This accounting does not erase C's conceptual cost: it adds one persistent
physical distinction to every ARROW, whereas B adds topology.

## Exact B counterexample

B failed exactly two registered dense worlds in every seed/reflection stratum:

1. `unrelated-nearby-no-lawful-return`; and
2. `cross-and-distractors-no-return`.

In all eight rows the target candidate genuinely traversed once, no lawful
target return occurred, and unrelated ordinary activity entered its adjacent
compartment. B accepted one return update and changed the target candidate
from resistance/coupling `1/1` to `4/2`. The only failed clause was the
registered zero-plasticity predicate. Replay and quiescence remained exact.

This is the dense-compartment contamination hypothesis, not an accounting or
oscillation failure. Spatial separation prevents renewed execution drive from
crediting P, but an undifferentiated crowded learning compartment still cannot
tell lawful downstream feedback from unrelated physical traffic that reaches
the same place.

C rejected those same ordinary arrivals: accepted updates stayed zero and the
candidate stayed `1/1`. In the matched lawful-return world its modulatory path
produced exactly one accepted update and `1/1 -> 4/2`. Thus C's positive rests
on the registered physical transmission distinction, not hidden suppression.

## Selection and boundary

Arm B is not selected. It is an informative development negative: topology
alone, as instantiated by one undifferentiated neighboring learning
compartment, is insufficient under dense topology.

Arm C is selected only as the **LR0 successor candidate**. It is the cheapest
registered arm that simultaneously:

- accepts genuine downstream feedback;
- rejects renewed upstream drive and dense unrelated traffic;
- remains quiescent;
- and conforms to all fresh compact PX0--PX2 behavioral surfaces.

This does not make the C law authoritative. PX0--PX2 authority is unchanged,
their spent matrices were not rerun, and PX3 remains negative. The next lawful
step is a separately frozen successor-law conformance/authority workflow for
the cumulative PX0--PX2 stack before any PX3 integration resumes.
