# DS7 GATE v2 withheld-edge collapse handoff

Status: **MECHANICALLY LOCALIZED; NO SCIENTIFIC AMBIGUITY**.

Frozen negative:
`f80db49` / `ds7-cumulative-plasticity-allocation-gate-v2-negative`.

The v2 implementation used an arbitrary 400-event withholding cap. The exact
frozen scalar arithmetic is:

```text
route edge after first creation + execution             3
seven further acquisition encounters + executions     +28
twenty retention encounters + executions               +80
six pressure boundaries before repair base              -6
                                                        ---
withheld-edge resistance entering repair               105

400 withholding events / pressure period 4             100 pressure ticks
final resistance in each matched branch                   5
```

Therefore the v2 result follows exactly:

```text
edge removed                    false
stale execution blocked         false
"correct repair" 32/32          reuse of still-live edge
shuffled repair blocked         false, because no repair was required
```

The correct and shuffled proposal lifecycles are byte-identical before the
post-removal encounter. Learned value affects admission only after an edge is
absent; it cannot change the scalar decay of the withheld existing edge. There
is no remaining diagnostic branch ambiguity.

Starting from completed-event clock 28, the 105th pressure boundary occurs
during withholding event 417. A fixed 424-event window therefore contains
exactly enough ordinary pressure to remove the edge with seven events of
margin, without changing the pressure period, starting resistance, reuse law,
or removal threshold.

The mechanically forced retry delta is:

```text
withholding cap 400 -> 424 events
report correct and shuffled removal separately
report entry and final resistance separately
```

No snapshot, representation, proposal, value, admission, eligibility, route,
credit, lifecycle, or M4 mechanism may change.

