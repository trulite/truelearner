# PX3-D1-R3 downstream participation attribution protocol v1

Status: **PREREGISTERED; DEVELOPMENT EVIDENCE UNSPENT**.

Start: frozen R2 execution-collapse record
`444f2364368ca4d4acfee06911406c27f49340da`. The authoritative PX0 law,
frozen D1 result, R2 collapse and D2 result remain unchanged.

## Question

> Can candidate participation and downstream-effect participation jointly gate
> a real PX0 return to the candidate source, while keeping that return below
> the source's execution threshold so that credit does not initiate another
> traversal?

R3 tests attribution and the feedback safety margin. It does not test candidate
formation, persistence, reproposal, reversal, D2, MICRO, GATE or authority.

## Frozen dynamical invariant

Returning evidence may physically reach the source required by PX0's existing
local plasticity law. It must not be capable of firing that source:

```text
candidate source P threshold = 2
ordinary attributed return impulse = 1

eligible candidate + return arrival at P
  -> candidate resistance/coupling may change
  -> P does not fire
  -> candidate does not traverse again
  -> activity quiesces
```

This supersedes the rejected stronger wording “no edge may return to P.” No
substrate law changes. `PlasticSubstrate::apply_local_return` remains the only
credit operation.

## Exact physical geometry

All six unordered trace-pair opportunities among A/B/C/D exist uniformly.
Each pair has the same cells and arrows:

```text
two unit primitive participation traces
  -> O(threshold 2)
O -> coupling 2 -> P(threshold 2)
P -> weak candidate(resistance 1, coupling 1, delay 2) -> effect(threshold 2)
context -> unit delayed input -> effect

P executes
  -> participant-local PX1 normalization
  -> one P participation trace
effect executes
  -> identical participant-local PX1 normalization
  -> one effect participation trace

P trace, delay aligned
+ effect trace
+ one ordinary global return
  -> attribution cell M(threshold 3)
M -> P, delay 1, coupling 1
```

Primitive, P and effect normalization all use the same motif: outlet execution
sends one unit directly to a threshold-two trace and one unit through an
ordinary participant-local hub to that trace. The trace fires exactly once.

P fires initially only because O supplies coupling two. Late primitive input
reaches O, not P. A valid attribution cell emits one unit to P at the last tick
of the candidate's four-tick eligibility window. PX0 applies local return before
integrating that impulse; the candidate changes from resistance/coupling
`1/1` to `4/2`, then P remains at subthreshold state one.

There is no effect-to-P or relay-to-P arrow. Effect participation can influence
P only through effect trace + P trace + ordinary-return coincidence at M.

## Frozen timing

For a normal episode beginning with primitive sources at tick 2:

```text
tick 3  O and P fire; candidate traverses and becomes eligible through tick 7
tick 4  normalized P trace fires
tick 5  candidate + context fire effect
tick 6  normalized effect trace fires;
        delayed P trace + effect trace + ordinary return fire M
tick 7  M's unit attribution pulse reaches P;
        candidate strengthens; P does not fire
```

The high-return diagnostic changes only M->P coupling `1 -> 2`. It must make P
fire a second time and re-execute the now-coupling-two candidate/effect route
once. No second ordinary return is supplied, so this risk control must still
quiesce rather than recreate R2's unbounded oscillator.

## Frozen matrix

Seeds `3201,3209`, normal and mirrored insertion order; load `0`. Exactly these
nine rows execute in order for each seed:

1. `return-only` — global return without P/effect participation;
2. `ab-real-subthreshold` — AB, context and in-window return;
3. `ab-blocked-late-a` — AB/P participate, effect context is blocked, late A
   and global return occur;
4. `ab-blocked-effect` — AB/P participate, effect context is blocked, global
   return occurs without late A;
5. `no-ab-independent-effect` — effects fire independently with global return,
   but no P participates;
6. `ab-real-no-return` — P and effect participate without ordinary return;
7. `ab-real-late-return` — ordinary return arrives after trace coincidence and
   candidate eligibility have expired;
8. `ab-then-cd-two-completed` — separated AB and CD episodes each complete
   their own P/effect traces and receive lawful unit attribution;
9. `ab-suprathreshold-return-control` — the real AB geometry with only M->P
   coupling raised from one to two.

## Required distinctions

- `ab-real-subthreshold`: AB P/effect/P-trace/effect-trace/M each fire once;
  one unit attribution reaches P; candidate is `1 -> 4`, P firing remains one,
  candidate traversal remains one, and the world quiesces.
- `ab-blocked-late-a`: late A may touch O and strengthen the fixed O->P
  connector, but M does not fire, no attribution reaches P, and candidate
  remains one before ordinary pressure removes it.
- `ab-blocked-effect`: P trace alone plus global return cannot fire M.
- `no-ab-independent-effect`: effect trace alone plus global return cannot fire
  M; no dormant candidate strengthens.
- `ab-real-no-return`: P and effect traces without ordinary return cannot fire
  M or credit the candidate.
- `ab-real-late-return`: expired trace state cannot fire M or rescue the
  expired candidate.
- `ab-then-cd-two-completed`: only AB and CD P/effect/attribution routes fire;
  both candidates become four; crossed candidates remain one.
- `ab-suprathreshold-return-control`: one coupling-two attribution reaches P;
  P, candidate and effect each execute exactly twice, proving the safety margin
  is dynamical. The run must still naturally quiesce.

Every row serializes primitive source/trace firings; O and P firings; candidate
crossings and impulses; P/effect normalization arrivals and firings; attribution
inputs/firings/pulses; candidate and O->P connector resistance; local-return
updates; work; storage; fingerprints; quiescence and exact replay. Resistance
is never used to infer coupling or firing.

## Verdict

- **R3-A positive:** all 18 rows pass, including zero P refiring under unit
  attribution and exact P refiring under the coupling-two risk control.
- **R3 negative:** trace attribution aliases a control, lawful unit credit
  re-executes P/candidate, the high-return control does not expose the margin,
  any world fails to quiesce, or the exact matrix otherwise fails.

The sole evidence command and write-once artifact paths must be frozen in a
separate execution protocol after implementation. Preflight must construct no
world, propagate nothing and emit no evidence marker.
