# PX3-D1-R2 closed-loop return attribution protocol v1

Status: **PREREGISTERED; EVIDENCE UNSPENT**.

Start: frozen D1 result commit `c01bdb42cc1a146284154ab3f8aa801df36f9383`.
Authoritative PX0--PX2 and all spent PX3/D1 artifacts are read-only.

## Question

> Can ordinary physical route separation make candidate credit require a
> completed downstream loop, so later upstream input cannot masquerade as
> return while candidate eligibility is live?

This diagnostic adds no return label, provenance ID or substrate law.

## Exact topology

Each unordered trace pair converges on an ordinary threshold-two opportunity
`O`. `O` reaches a separate threshold-one continuation `P` through an identical
fixed unit ARROW. The weak candidate is the dormant resistance-one ARROW
`P -> consequence`, not an outgoing ARROW of `O`. Candidate traversal therefore
opens eligibility at `P`. Each consequence returns through its own ordinary
relay to a physical `P` target.

```text
A trace + B trace -> O_AB -> P_AB --weak candidate--> E_AB
                                               E_AB -> relay -> P_AB
```

Late A reaches `O_AB`, not `P_AB`. The fixed `O->P` connector may itself receive
ordinary PX0 updates and must be serialized; the D1-R2 claim concerns whether
the weak selectable candidate is credited only through the closed downstream
route.

All six pair loops exist symmetrically. No priming or generic proposal occurs.

## Frozen matrix

Seeds `3001, 3007`; raw couplings `1/2/4`; load `0`.

1. return reaches every `P`, no pair traversal -> all candidates remain `1`;
2. AB traversal + timely real consequence loop -> AB `1 -> 4`;
3. AB traversal + blocked consequence + late A within eligibility -> AB remains
   `1` and expires;
4. AB traversal + genuine consequence return after eligibility -> no credit;
5. no AB traversal + genuine consequence activity/return -> no credit;
6. AB traversal with its physical return route swapped to dormant `P_AC` ->
   neither AB nor AC candidate strengthens;
7. temporally separated genuine AB and CD loops before pressure -> both may
   strengthen, crossed candidates may not;
8. amplitude variants `AB(2+1)` and `AB(4+4)` preserve the same route result.

Serialize primitive PX1 traces, O/P firings, weak-candidate crossings and
impulses, consequence/relay/return crossings, connector and candidate
resistance, pressure/deallocation, fingerprints, work, replay and quiescence.
Never infer coupling from resistance.

## Verdict

- **R2-A:** all route controls pass; closed physical route separation is
  sufficient for this provenance case.
- **R2-C/D/E:** freeze any timing-only, route-insensitive, new-law-dependent or
  complete interpretable failure.

This diagnostic makes no candidate-formation, D2, recursion, reversal or
authority claim. Implementation requires a separate frozen execution surface,
E2B preflight and one write-once run.
