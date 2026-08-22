# DS6 cumulative lifetime MICRO protocol

Protocol identifier: `ds6-cumulative-lifetime-micro-v1`

Status: **PREREGISTERED BEFORE MICRO IMPLEMENTATION OR EVIDENCE**.

This is a two-seed development hardening step. It cannot create M4 or
authorize a definitive run.

Frozen parent and candidate:

- M3: `ffcdfe8b36fc62348b7ebcb09aaf4797f6146ba8`;
- selected PROBE result: `32fe96ba94f0676489244f4feec77f3e6505dd7c`;
- selected implementation: `493d56bf44c23bd2d174a3ac35bf247dd54fac38`;
- candidate: Arm A recurrence/use competition only.

The `ScalarLifecycle` persistent fields and update law are byte-frozen from
the selected implementation:

```text
one map: anonymous M3 signature -> i32 strength
new local recurrence: strength = 1
existing local recurrence/use: strength += 2
each fourth ordinary completed event: every strength -= 1
strength <= 0: physical allocation removed
```

The extraction into a reusable MICRO module must be a mechanical byte-audited
move. No constant, field, branch, clock, input, or update order may change.

## Matrix

Two fresh development seeds are frozen:

```text
107_000  alternating useful recurrence, then distractor pressure
108_000  interleaved useful/distractor recurrence and reversed allocation
```

Each cell contains:

- four useful signatures with recurrence counts `4, 6, 8, 10`;
- sixteen one-off signatures;
- two recurring signatures whose later causal continuation contradicts;
- twelve ordinary intervening events with no target identity or cleanup cue;
- relabelled returns for every still-lawful useful signature;
- reacquisition of two physically removed one-offs;
- fresh occurrence handles, shapes, local times, consequences, and allocation
  order on every return.

The evaluator may know these groups only for scoring. No group label or count
may enter organism acquisition, execution, pressure, or removal.

## Conjunctive MICRO gates

Both cells must pass all gates:

1. exact M0--M3 ancestry and selected scalar-lifecycle source hashes;
2. no lifetime class, structural tier, task boundary, rest/cleanup call,
   request/start, finish/output, correctness, or evaluator flow;
3. all lawful useful signatures remain physically allocated and execute exact
   M3 spans and ordinary consequences on relabelled return;
4. all 32 one-off allocations physically disappear before return;
5. changed causal signatures do not execute stale learned paths and generic
   reopening remains functional;
6. all four removed one-offs reacquire through the identical ordinary path;
7. final persistent bytes and records are lower than paired keep-all;
8. no-pressure over-retains, shuffled recurrence does not preserve useful
   state, and boundary scheduling changes produce an exact snapshot;
9. duplicate execution is byte-exact and all occurrence-local state is zero;
10. the process writes a create-new Markdown artifact atomically before exit.

There is no aggregate percentage threshold. First failure stops GATE
eligibility but the complete MICRO cells and controls remain reported.

## Discipline

Formatting, compilation, static audits, and unit tests that do not execute a
MICRO seed are permitted before freeze. The MICRO source, runner, and result
schema must be committed and tagged before the single command. The command
runs the complete two-seed matrix once in the dedicated E2B development
sandbox and writes `results/ds6_cumulative_lifetime_micro.md` with create-new
semantics. It must fail if that path already exists.

PASS makes unchanged-mechanism GATE preregistration eligible. FAIL follows
only mechanically missing connections already present in M3 or the selected
arm; changing the scalar lifecycle requires a separately named diagnostic and
cannot rescue this MICRO.

