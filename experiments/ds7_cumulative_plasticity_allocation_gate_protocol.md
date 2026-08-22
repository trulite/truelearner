# Cumulative DS7 learned plasticity-allocation GATE

Protocol identifier: `ds7-cumulative-plasticity-allocation-gate-v2`.

Status: **PREREGISTERED DEVELOPMENT GATE; OUTCOME UNSPENT**.

Version 2 freezes one pre-execution load-scaling linker. Authoritative M4
applies pressure on completed physical event propagation, not once per local
candidate inside an event. The PROBE and MICRO used one encounter per event,
where those clocks were identical. Loaded GATE events require the explicit
equivalent batching:

```text
begin one physical event          -> one event-clock update
visit its local encounters        -> zero additional event-clock updates
complete event                    -> pressure follows the frozen M4 period
```

The existing single-encounter call remains exactly `begin event + one local
encounter`, so prior behavior is unchanged. This batching changes no snapshot,
proposal, representation, value, admission, eligibility, scalar increment,
pressure period, or removal law. Version 1 is preserved at its tag and no GATE
evidence was spent under it.

Exact development-readiness commit:
`9e7d197e2915fa0c550160d2cdd3dbb04884f168`.

Authoritative M4:
`8db47281a7c9c97cbb52ced6fc3dcff0e7efa9b2`.

Frozen inputs:

- development handoff SHA-256:
  `e2cd6556b31c15457e6e0accf6b1369d0140378138fc063c417372da09fb3a1c`;
- MICRO result SHA-256:
  `fa3d0510e0d821d32b2e3dd8c5bc9f357fca653f6e96f7d9d6b8fa19e61a377b`;
- MICRO audit SHA-256:
  `bc4e2cd797fb31133b315d8844301ca9734835176713cced0b049508475e84ff`;
- M4-linked allocator source SHA-256:
  `d1e14001902b6bafe6049ef01a9990ffcc91c22180543672c30a13fdb3ee7dc9`;
- MICRO source SHA-256:
  `3fd46245614c17476b8fa44887e4a710d0c358ecc4f7ebe7da5fbdb5bb9459ec`;
- retained-route execution (RT0) source SHA-256:
  `16ef4e2a691e22251d109860ac055c5a1ee78f586ad9335a375589336ad78ed0`.

## Question

> Can the frozen M4-governed encounter allocator acquire, retain, execute, and
> repair a multi-edge local route across fresh activity and increasing
> distractor load, while learned history—not an evaluator site policy—controls
> where structural variation is attempted?

## Fixed matrix

Six explicit blank-start seeds:

```text
22_000_000
22_500_000
23_000_000
23_500_000
24_000_000
24_500_000
```

Each seed runs exactly three distractor loads: `8`, `32`, and `128`, for 18
cells total. Every cell uses a fresh opaque identity namespace, fresh absolute
layout, zero proposals, zero encounter prototypes, zero value records, and
zero eligibility traces.

## Physical task

Each cell contains four locally adjacent substrate-arrow opportunities that,
if retained together, physically propagate an anonymous spike through a
four-edge route to terminal activity. Distractor opportunities are also local,
coactive, and initially indistinguishable as sites where variation may occur.

Productive and distractor encounters differ only in their ordinary observed
endpoint activity/adjacency histories. No route, role, target, candidate,
answer, phase, or future-utility bit enters their snapshot.

Training begins broad from blank state:

1. every locally active opportunity may form a variation;
2. a live variation may execute and produce ordinary route aftermath;
3. the still-supplied DS8 semantic outcome reaches only that active trace;
4. encounter prototypes and value learn which recurring physical histories
   tend to support useful variations;
5. all proposals, prototypes, and value records obey the single M4 scalar
   use/pressure lifecycle.

The four route edges are stable substrate cells, as in P2/RT0. Fresh held-out
occurrence identities carry activity through them but are never stored in the
route or encounter representation.

## Frozen phases per cell

1. **Blank acquisition:** eight training sweeps, with distractor encounters
   presented before route encounters so semantic outcome cannot preselect a
   proposal site.
2. **Held-out execution:** 32 fresh anonymous activity occurrences must
   traverse all four retained live edges and produce terminal activity with no
   structural update.
3. **Allocation/economy evaluation:** one fresh sweep of four productive plus
   all distractor encounters. Compare admissions against an always-open local
   baseline over the identical sweep.
4. **Value shuffle:** swap productive/distractor learned values while leaving
   physical snapshots and routes fixed; admission preference must reverse.
5. **Missing-edge repair:** withhold use from one retained route edge while
   ordinary pressure continues and the other route structures remain active.
   After that edge physically disappears, the correct learned allocator must
   reacquire it from local coactivity and restore route execution. The shuffled
   allocator must not restore it on its matched first non-exploratory
   encounter.
6. **Controls:** no-coactivity, outside-radius, inactive-feedback, stale edge,
   opaque-identity permutation, and absolute-layout permutation.
7. **Cumulative anchor:** unchanged authoritative M4 gate cell with exact
   lifetime vector `[1, 3, 6, 13, 27]`.

## Conjunctive criteria

Every one of 18 cells must satisfy:

- all four route proposals and at least one distractor proposal arose before
  their first delayed outcomes;
- all four route edges remain live after training and all 32 held-out physical
  traversals reach terminal activity;
- the learned allocator admits all four productive opportunities and no more
  than `ceil(load / 8)` distractor opportunities in the economy sweep;
- proposal admission is at least 50 percent lower than always-open at every
  load;
- shuffled value reverses productive/distractor admission;
- ordinary pressure removes the withheld route edge and its stale handle
  cannot execute;
- correct learned allocation reacquires the missing edge and restores 32/32
  held-out traversal; shuffled allocation remains incomplete on the matched
  first non-exploratory encounter;
- unused distractor proposals consume less retained allocation than the
  always-open baseline, while recurrent route proposals/prototypes/value
  remain live;
- all locality, eligibility, fresh-identity, and layout controls pass;
- source/information audit and unchanged M4 anchor pass.

The full 18-cell report must be duplicate-exact. The source audit must find no
supplied endpoint class, route/candidate/site input, probation TTL,
temporary/permanent or consolidation lifetime class, evaluator deletion, or
pre-proposal semantic outcome flow.

## Interpretation

PASS establishes cumulative DS7 development readiness for a separately frozen
definitive matrix. M4 remains authoritative until that one-shot matrix passes.

FAIL freezes the first failed phase. Mechanical path absence may be repaired
and retried under a new protocol. A new persistent representation, semantic
site choice, or multiple equally supported allocation laws is a scientific
stop.
