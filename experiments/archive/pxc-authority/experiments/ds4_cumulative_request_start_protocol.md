# DS4 cumulative request/start de-supply development protocol

Protocol identifier: `ds4-cumulative-request-start-v1`

Status: **AMENDED BEFORE DEVELOPMENT EVIDENCE; NO DS4 EVIDENCE SPENT**.

This protocol governs PROBE, MICRO, and GATE only. It cannot create M4 and it
does not authorize a definitive DS4 execution. A separate definitive matrix
must be frozen after development readiness.

Pre-evidence amendment: the frozen M3 port exposes chunk count and persistent
bytes but deliberately does not expose its private persistent map. Therefore
the non-plasticity check below freezes those complete public summaries plus the
byte-audited `acquire=false` call path, rather than adding a new M3 fingerprint
accessor. P4 retains its existing exact fingerprint. No target, linker, seed,
stage, or pass threshold changed.

## Frozen question

Starting from authoritative M3, can a byte-frozen learned request-role
mechanism select the request-bearing anonymous occurrence and initiate the
frozen downstream recurrent computation when initiation is driven only by a
learned M3 event-completion output, with no request/query marker and no
supplied `START` meaning?

This is a conjunctive removal:

```text
learned M3 event organization
        -> ordinary completion activity
        -> learned P4 request-role selection
        -> frozen downstream recurrence
```

Selecting the request without eliminating supplied `START`, or eliminating
`START` while supplying the request identity separately, is negative.

## Frozen ancestry and mechanism

- authoritative M3 commit/tag:
  `ffcdfe8b36fc62348b7ebcb09aaf4797f6146ba8` /
  `m3-cumulative-event-boundary-authoritative`;
- M3 cumulative development source SHA-256:
  `c4fc7aca11a5925effeb5a84b90184a70da0f66da7c063d0f87ba46ca36addf3`;
- M3 definitive core SHA-256:
  `d4c3ea6e671d1812e35e34ef7fa46a77f6a577c9e6a1d77d2c30e7d570017840`;
- M3 definitive CSV SHA-256:
  `ac8c0a6c9b7badfa263ceb054ffe59c11162b1ca256c56cc6df5f0d378179401`;
- M3 definitive report SHA-256:
  `ab77bd12b705b8620b6315260f8bb5b4df6efc961f1d20a0dd521af403e1ac5f`;
- frozen isolated P4 commit/tag:
  `51cf918e0b6eda77ccef6386ff1150db42cea6fd` /
  `p4-discovered-request-role`;
- frozen P4 mechanism/harness SHA-256:
  `2dbde723b394bcb3d788c796aa1745cd1cea392a64ab61497bb97474866144b8`;
- frozen P4 CSV SHA-256:
  `b1e8ee07be2fa425e7ec5cdfee54ea77abde97d0160a77ca6b5b550126d46c5d`;
- frozen P4 report SHA-256:
  `ced485b93072e4ffba6b1889175be130dcda27926b6b8685b4e6d3ade31cc2bd`;
- DS4--DS8 target-freeze commit/tag:
  `fcfd0413da37eac6eeb1a49c6f387e090229e605` /
  `ds4-ds8-target-freeze-v1`;
- target-freeze SHA-256:
  `f10f9d7b16106b6014767ff6188a6d556145ba3e5b4335e28de245c7622a7595`.

The P4 source is included byte-for-byte as the downstream mechanism. DS4 may
add only composition glue, source/information-flow audits, fixtures, controls,
and reporting. It may not alter the P4 request learner, M3 learner, M0--M2
mechanisms, recurrent executor, plasticity values, or still-supplied terminal
credit.

## Mechanically forced linker edge

The static dependency scan found one first absent path:

```text
M3 learned completion activity
        X
P4 learned request-role selection
```

The DS4 port may close only this edge. Completion is the frozen physical
source because P4 selects across the fully observed anonymous occurrence set;
starting before container completion would consume a partial request encoding.

The linker must carry only ordinary M3 completion activity. It may not carry a
request flag, target identity, expected answer, success bit, route identity,
or host `START` instruction. The P4 learner sees the ordinary occurrence set;
the target identity is not a separate argument to selection or execution.

## Sequential supplies left intact

DS4 does not remove supplies scheduled for later stages. The frozen P4
downstream may therefore continue to use:

- its learned request-role plasticity and the terminal semantic credit that
  updates an already-active request trace (removed at DS8);
- its frozen recurrence and supplied finish/output route (removed at DS5);
- its current temporary/permanent state treatment (removed at DS6);
- its current plasticity proposal/targeting policy (removed at DS7).

None of these later supplies may select the request or initiate execution.

## Development populations

Development namespaces are disjoint from all prior definitive namespaces and
from any future DS4 definitive matrix:

```text
PROBE  base seed 94_000; one learner; smallest path-existence assay
MICRO  base seed 95_000; two learners; 8 fresh held-out episodes each
GATE   base seed 96_000; six learners; 32 fresh held-out episodes each
```

M3 acquisition uses ordinary cumulative M2-to-M3 streams and remains separate
from held-out DS4 request episodes. P4 request acquisition rotates depths
`1,2,3,4`; held-out depths rotate `5,8,16,32`. Every request occurrence,
opaque identity, surface receptor, serialization order, and M3 occurrence is
fresh. Held-out evaluation is non-plastic.

The future definitive protocol must choose fresh namespaces not used here.

## Ordered development stages

The port reports the first collapse in this exact order:

1. **P0 frozen-source audit** -- M3, P4, target, result, and protocol hashes
   match; authoritative M3 exists; all frozen source audits pass.
2. **P1 physical initiation path** -- a learned M3 completion produces
   ordinary activity that reaches P4 selection; no host `START`, request flag,
   or separately supplied target identity crosses the linker.
3. **P2 learned request acquisition** -- the request role consolidates from
   pre-answer active traces under the still-supplied DS8 credit channel.
4. **P3 held-out functional transfer** -- fresh anonymous requests at depths
   `5,8,16,32` initiate and complete the frozen recurrence with exact output,
   explicit emission, and natural quiescence.
5. **P4 controls 1--12** -- every control below passes conjunctively.
6. **P5 determinism/work/lifecycle** -- duplicate runs are exact, M0--M3 and
   request work is nonzero and attributable, held-out state is non-plastic,
   and occurrence-local state is erased.

Stages after the first collapse are blocked. A mechanical collapse is followed
to its first missing physical edge under the frozen sprint policy.

## Frozen controls

1. **Learned event required.** Identical request occurrences without learned
   M3 completion activity cause zero request selection, execution, or update.
2. **Subthreshold M3.** A generic completed span with zero learned M3 use does
   not activate P4; generic reconstruction alone cannot masquerade as M3.
3. **Missing close.** An incomplete M3 candidate causes zero activation.
4. **Invalid transition/interruption.** Broken event activity causes zero
   request initiation and leaves a later valid event able to reenter.
5. **Fresh M3 identities/allocation.** Relabelled M3 occurrences and reversed
   allocation preserve learned activation without a stable handle.
6. **Fresh request serialization.** The learned request transfers across all
   six positions, fresh receptor identities, and transferred ordering.
7. **Symmetric impossible requests.** Two observably identical identity
   occurrences form no stable preference and do not become competent.
8. **Pre-answer information flow.** The selected trace exists before output;
   expected answer and terminal polarity do not reach selection or initiation.
9. **No separate target channel.** The selected opaque identity is recovered
   only from the chosen anonymous occurrence; no target/request field crosses
   the composition boundary.
10. **Frozen-source leak audit.** Organism-visible persistent M3/P4 state and
    the DS4 linker carry no evaluator answer, request/start signal, stable
    occurrence identity, or truth-derived key. Evaluator/harness names in the
    byte-frozen source are not organism-visible values.
11. **Held-out non-plasticity.** M3 chunk count and persistent bytes and the
    exact P4 fingerprint are unchanged across every held-out episode; M3 is
    called only through its frozen `acquire=false` path and temporary state
    returns to zero.
12. **Duplicate and disjoint population.** Duplicate execution is byte-exact;
    acquisition and held-out namespaces are disjoint and no development seed
    is claim-eligible.

## PROBE stopping rule

PROBE asks only whether the missing edge physically exists:

```text
learned M3 use > 0
completion activity > 0
request selection activations > 0
request update activations > 0
same occurrences without learned completion -> all three are zero
```

If this path is zero, follow the first missing edge and retry the unchanged
target. Do not harden MICRO/GATE controls around a zero path. If making it
nonzero requires a new persistent representation or forbidden semantic signal,
stop for scientific review.

## Readiness and authority boundary

GATE readiness requires all six ordered stages, all twelve controls, exact
frozen hashes, byte-identical M3/P4 mechanism copies, a clean worktree, and no
DS4 definitive artifact. Readiness is development evidence only.

A separate authority phase must preregister a single write-once DS4 matrix,
verify hashes from the clean readiness ancestor, run it exactly once on E2B,
and freeze the result without rescue. Only a full definitive pass may create
M4 and make cumulative DS5 eligible.

All Rust formatting, compilation, Clippy, tests, and execution run only on E2B
through `/Users/satya/work/br/truelearner/scripts/e2b_persistent.py` with
`/Users/satya/work/br/truelearner/.env.e2b`. Local work is limited to source and
document editing plus read-only Git, text, and hash inspection.
