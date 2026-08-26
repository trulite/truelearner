# RS1 inhibitory topology sufficiency implementation audit v1

Status: evidence-eligible topology-only candidate frozen before any RS1 matrix
execution.

## Scope

- parent: RS0 result `d1384e8`;
- protocol: `7ab0088`, tag
  `rs1-inhibitory-topology-sufficiency-protocol-v1`;
- physical-law additions: zero;
- learning-law additions: zero;
- core changes after RS0: zero;
- CE0 efficacy plasticity, FD2, ARC, authority, oracle, and `arch.md`:
  unchanged.

## Existing physical affordance

The accepted substrate already admits signed ordinary Drive. `ArrowSpec`
coupling and queued impulse are signed `i32`; Drive delivery adds that impulse
to CELL state; negative CELL state relaxes toward zero under ordinary CELL
decay; firing still requires reaching a positive threshold. RS1 therefore
uses only ordinary CELLs, SourceFires ARROWs, and Drive SPIKEs.

Each selected excitatory CELL drives an ordinary threshold-one relay CELL.
That relay returns an ordinary negative Drive to the same excitatory CELL.
The frozen main family uses magnitude 16; a separately frozen magnitude sweep
is characterization only. No result may change the magnitude or geometry.

## Observation boundary

RS1 inherits the already-tested, feature-gated RS0 observation ceiling. The
observer is causally inert: it pauses an active queue after a scheduled-
delivery ceiling, preserves pending physical activity, and resumes it on the
next call. Every settling family is also compared with ordinary unbounded
propagation. ExecutionCost and diagnostic allocation capacity remain outside
physical equivalence.

## Frozen candidate hashes

```text
core lib.rs       45dd6af368776d68574ff2b00dd4db109d469bfeedc99b57eb76ad6b26ca111c
core Cargo.toml   5d794eae058f5cdd896064b0a37a6dfb124d9d7b6d03f8cfa9c53651e58460ef
evaluator         c3d1b95ea1f568702230a4bc31832f6575bc4d3db0c5c31f8454c62f595ca786
arm Cargo.toml    677943e3b525101a5cdef8a0219a4a780e7d0cbc14fcf5a467c12f4fedba4016
protocol          bb175daa3a22fc03e99d8ad8f0a462054fa468a025469af4babadaf4ac6d8cee
```

## Frozen matrix

Twenty-two families cover the uninhibited RS0 reciprocal control; local H16
feedback; one-way chains; disconnected and untraversed feedback; cycles of
length 2/3/4/8; delays 0/1, 1/1, 2/2, and 3/3; executable and subthreshold
coupling/threshold boundaries; the frozen inhibition-strength sweep; and two
simultaneous loops with feedback in only one neighborhood.

Two fresh identity roots and all ten absolute clock phases produce 440 cases
and 880 Reference/Production rows. Each case also executes an exact
same-mechanics replay. The matrix has not run.

The standalone evaluator was remotely formatted and compiled in reusable E2B
worker `iz0fwqk6a9bkvd4fgbtp1`. No Rust or project command ran locally.

The first strict-Clippy preflight stopped before evidence because the local
ARROW-construction helper accepted eight mechanical arguments. Those existing
fields were grouped into the existing `ArrowSpec`; no world geometry, law,
predicate, or matrix dimension changed. The corrected evaluator then passed
formatting and strict Clippy in the same reusable worker.

The first static-audit invocation then stopped because its negative-decay
source sentinel named a nonexistent expression. The sentinel was corrected to
the accepted `saturating_add(decay).min(0)` expression. This audit-only repair
changed no executable Rust or experimental predicate.
