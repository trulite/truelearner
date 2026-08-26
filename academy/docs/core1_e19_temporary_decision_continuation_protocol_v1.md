# CORE1-E19 — Temporary Decision Continuation Protocol v1

## Status

Preregistered. No E19 runtime result exists when this protocol is frozen.

## Question

Can one unresolved local interaction remain physically OPEN across positive
route-completion returns, reactivate its ordinary decision junction, and close
only when ordinary consequence arrives, without storing an action, attempt,
episode, timeout, or alternative set?

## Candidate primitive

The only new retained state is:

```text
DecisionOpenness = CLOSED | OPEN
```

It is local to one experimental ARC3 organism. It contains no payload.

The physical loop is:

```text
ordinary unresolved junction activity
        -> OPEN

ordinary local variation exposes current routes
route-local refractory disfavors a route after it participates
selected route physically completes
        -> a positive completion SPIKE reaches a neutral local receptor

completion arrived AND marker is OPEN
        -> ordinary junction activity is admitted again
        -> ordinary variation may regenerate expired routes

useful route physically completes
        -> completion SPIKE arrives while still OPEN
        -> later ordinary consequence is admitted
        -> PQLC updates ordinary route material
        -> CLOSED
        -> quiescence
```

Completion and consequence are different physical admissions. Completion does
not consolidate and cannot close. Consequence cannot be inferred from silence.

The neutral completion receptor is ordinary CELL/SPIKE physics and has no
outgoing action, learning, or consequence connection. Its positive firing is
the only event that permits another junction activation while OPEN.

## Frozen body and retained physics

The candidate composes, without changing their laws:

- frozen CORE1-B/E16 ARC3 body;
- E17-R route-local refractory state;
- ordinary generic local variation and regeneration;
- ordinary physical traversal;
- existing consequence return and PQLC consolidation;
- existing resistance, pressure, generation safety, replay, and quiescence.

E18 in-flight protection remains disabled. A completion receptor is fixture
matter, not a lifetime exception. OPEN does not protect a route or alternative.

## Frozen opaque screen

Use the eight E17 opaque action/order permutations unchanged. For each seed,
the useful route remains hidden from organism physics and is used only by the
world to decide whether to admit ordinary consequence.

Target for the candidate arm:

- first physical action: `8/8`;
- useful route physically encountered: `8/8`;
- useful policy learned after two opportunities: `8/8`;
- bounded attempts: at most four route completions per opportunity;
- exact replay: `8/8`;
- Reference/Production equality: `8/8`;
- natural quiescence: every admission.

## Mandatory controls

1. **CLOSED:** ordinary silence plus a CLOSED marker produces no action.
2. **Missing completion:** after one failed route, withholding the completion
   SPIKE produces no second junction activation and no second attempt.
3. **Completion while CLOSED:** the physical completion SPIKE may arrive, but
   it cannot reactivate the junction.
4. **Completion is not consequence:** failed completion leaves OPEN and
   produces zero consequence/PQLC updates.
5. **Consequence closes:** a useful route completes while OPEN; only subsequent
   admitted consequence changes OPEN to CLOSED.
6. **Closed quiescence:** after closing, another physical completion cannot
   produce an action.
7. **Regeneration, not preservation:** an expired current route may reappear
   only through a fresh ordinary junction/variation admission. OPEN contains no
   stored route, alternative, or action menu.
8. **Schema audit:** the retained marker is exactly one payload-free two-state
   enum. Action identities exist only in ordinary route-local refractory state.

## Hard rejection screen

Reject E19 if any of the following is present or observed:

- the OPEN marker stores action identity;
- it stores an attempt number, iteration, index, or count;
- it expires or reactivates by elapsed time or timeout;
- missing completion produces another try;
- it stores or protects alternatives;
- it self-loops without a returned positive completion SPIKE;
- completion itself produces consequence/PQLC consolidation;
- successful consequence fails to destroy OPEN;
- a route is retained merely because the interaction is OPEN;
- any evaluator success, action meaning, or terminal-result value enters the
  marker, completion receptor, refractory law, variation law, or runtime.

## Representation audit

The experiment must report separately:

- marker state before and after each completion;
- completion SPIKE deliveries and their physical work;
- junction reactivations caused by delivered completion;
- attempted and physically participating routes;
- route-local refractory observations;
- consequence admissions and PQLC updates;
- marker state immediately before and after consequence;
- live-route regeneration observations;
- final autonomous action and quiescence.

Source inspection must find no marker field other than the two-state enum and
no timer/count/action/container reachable from it.

## Execution boundary

Development may use focused compile, unit, and single-seed preflight commands.
The frozen eight-seed evidence command may run once only after:

- the protocol is committed;
- the candidate and all controls compile;
- strict focused Clippy passes;
- single-seed Reference/Production preflight is exact;
- the schema/prohibition audit passes.

If the eight-seed target or any mandatory control fails, freeze a stopped
negative. Do not widen OPEN, add a timeout, preserve alternatives, add an
action/episode identity, or change route lifetime inside E19.

## Success claim

Only if every target and control passes:

> Temporary local causal openness, combined with positive physical completion,
> route-local refractory state, ordinary variation, and existing consequence
> learning, is sufficient for bounded decision continuation in the frozen
> opaque four-route world.

This is a narrow development primitive, not an exploration mechanism, planner,
or authority result. ARC and authority remain unchanged.
