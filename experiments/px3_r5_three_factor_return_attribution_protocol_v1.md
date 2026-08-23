# PX3-R5 three-factor physical return attribution protocol v1

Status: **PREREGISTERED; DEVELOPMENT EVIDENCE UNSPENT; PX3 AUTHORITY NEGATIVE**.

Start: frozen R4 result commit
`d8c564071a335d8cda4e12eba8ca3077fa9d8582`, tag
`px3-r4-return-window-separability-uninterpretable-v1`.

| frozen input | SHA-256 |
|---|---|
| authoritative PX0 substrate law | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| R3 positive CSV | `62b34a64396728c28b617bab75cf1141ee2b2db53897ee655809b6180cb2a67b` |
| R3 result audit | `6f565cf8397afb55e28293360f1ade5aa51b89ba5fa8c19ce0eacaa23086e299` |
| R4 source | `88425c48a141c3dea5177c2581f35506a215c23166c5b90224cd4cf506fa6986` |
| R4 CSV | `81d3296ddda223486c3e3d00b01e590cc18889e5fffe85e59e1da825f143b82e` |
| R4 report | `bbf02161d4de8f5f64fec16af545e4758f8c8115a2b03c01e3a28cc430f8e25e` |
| R4 result audit | `75bb603ba21aa9af0d6fab644264b10f6db0d1bf8a1de4a34049e169550b3785` |

R5 changes no substrate law and cannot retry or repair PX3 authority.

## Question

> Can ordinary overlap of three independently normalized physical
> participations—candidate source P, downstream effect X, and a traversed
> world-return path R—attribute credit without semantic return labels or false
> attribution from renewed upstream activity?

## Frozen physical hypothesis

The P/X organization remains the unchanged stage-one PX3 geometry. The raw
global-return unit used by R3/R4 is replaced by an ordinary physical return
path using the identical PX1 normalizer:

```text
world input -> R source -> R outlet
R outlet -> unit direct trace input
R outlet -> local hub -> unit trace input
R trace threshold 2 -> exactly one unit participation firing

P trace unit + X trace unit + R trace unit -> M(threshold 3)
M -> P delay 1, coupling 1; P threshold 2
```

The evaluator may schedule an external world input at the R source. It may not
set an R flag, inject M directly, call plasticity directly, select a candidate,
or label the return as belonging to P. M receives only ordinary ARROW
traversals from three ordinary participation-trace cells.

For an episode starting at tick 0:

```text
tick 1  A/B traces overlap; O and P fire; native P->X candidate traverses
tick 2  X fires; P trace fires; world input traverses R source/outlet
tick 3  X trace and R trace fire; P/X/R units reach M; M fires
tick 4  M's unit echo reaches P; candidate strengthens; P does not refire
```

All PX0 timing, decay, refractoriness, proposal, eligibility, return,
resistance and pressure laws remain byte-identical.

## Critical carryover caveat

R4 showed that a threshold-three M can fire without world return when adjacent
episodes contribute four P/X trace units across two ticks:

```text
tick t    P1 + X1 -> M state 2
tick t+1  ordinary decay leaves 1
          P2 + X2 raises M to 3 -> false firing
```

Therefore normalization of R is not assumed sufficient. The exact offset-one
recurrence/no-return collapse is a decisive R5 control. If it still fires M,
R5 must freeze a carryover negative even if isolated three-factor rows work.

## Exact matrix

Fresh seeds `3601,3609` use normal and mirrored insertion order. Exactly eight
scenarios execute in order for each seed:

1. `complete-pxr` — P, X and a real R-path traversal; one credit, no P refire;
2. `px-no-return` — P and X only; no attribution or credit;
3. `pr-x-blocked` — P and R participate while X is blocked; no attribution;
4. `xr-p-absent` — X is driven through an ordinary physical driver and R
   participates while P is absent; no attribution and no candidate;
5. `px-late-a-no-return` — P and X participate, later A activity occurs, R is
   absent; no attribution or credit;
6. `adjacent-ab-no-return` — two real A+B episodes at offsets 0 and 1, no R;
   no attribution is required for a full R5 positive;
7. `collision-real-return` — first episode R participation and second A+B
   upstream arrival both reach the source-local causal region at the R4
   collision time; exactly one R-backed attribution may occur;
8. `two-completed-pxr` — two separated complete P/X/R episodes; exactly two
   lawful attributions and two unit echoes may credit the same physical route.

Each row executes from complete fresh state twice for exact replay, giving
exactly 16 rows.

## Required distinctions

- A return input counts only if the R source, outlet and normalized R trace
  physically fire.
- Isolated `P+X`, `P+R`, `X+R`, late A and return absence cannot fire M.
- `complete-pxr` produces one M firing, one unit echo, candidate resistance
  `1 -> 4`, and zero P refirings.
- `adjacent-ab-no-return` must not manufacture R, M or echo for R5-A.
- The collision contains one actual R trace and exactly one R-backed M firing;
  renewed upstream input supplies no R trace.
- Two separated completed chains produce two R traces, two M firings and two
  unit echoes without autonomous oscillation.
- Every world naturally quiesces, complete-state replay is exact, and storage
  remains bounded.

Resistance is serialized but never used to infer trace firing, attribution,
echo or P execution.

## Frozen classification

- **R5-A THREE-FACTOR POSITIVE:** all 16 rows pass, including zero M/echo in
  adjacent recurrence without R.
- **R5-B CARRYOVER NEGATIVE:** isolated three-factor and missing-factor rows
  pass, but adjacent no-return recurrence fires M from residual P/X activity.
- **R5-C CORE NEGATIVE:** complete P/X/R fails, any isolated missing-factor
  control aliases, unit echo refires P, collision/two-chain behavior aliases,
  replay/quiescence fails, or the matrix is otherwise invalid.

## Evidence discipline

Implementation, formatting, tests, Clippy, preflight and the sole evidence run
occur only in E2B. Preflight constructs no world, propagates nothing, emits no
evidence marker and writes no artifact. After implementation and a separate
execution protocol are committed and tagged, `--r5` may execute exactly once.
Its result is published atomically and frozen without rescue or rerun.
