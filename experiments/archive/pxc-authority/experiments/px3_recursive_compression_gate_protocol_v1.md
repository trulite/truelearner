# PX3 recursive compression GATE protocol v1

Status: **PREREGISTERED; DEVELOPMENT EVIDENCE UNSPENT**.

Start: frozen integrated MICRO-A result
`927c4cb860383c742138723c3788d4f264081b8d`. The frozen D2-A result is imported
unchanged at commit `3da0698e8a514b3ddac0c77b2fecd9e7c2f2bae2`. Authoritative
PX0 and every spent PX3 artifact remain read-only.

## Question

> Can native PX3 organization recurse through `A+B -> X`, `X+C -> Y`, and
> `Y+D -> Z` while every primitive and derived participant uses one identical
> PX1 normalization interface and every level uses one identical conjunction,
> proposal, attribution and persistence mechanism?

This GATE integrates D2 normalization with MICRO-A formation and R3 credit. It
does not supply a mature candidate, a typed composite adapter, an event object,
a level flag or a special recursion operation.

## Level-blind physical topology

Primitive outlets A/B/C/D and derived outlets X/Y/Z all use the same helper and
the same physical motif:

```text
participant outlet
  -> unit local trace input, delay 1
  -> participant-local hub, delay 1 -> unit trace input, delay 0
trace threshold 2
  -> exactly one ordinary unit participation firing
```

The only participant interface consumed downstream is that trace CELL. A
derived outlet has no primitive/composite/event flag and its raw incoming
candidate coupling is not forwarded. One outlet execution emits two unit
normalization arrivals and exactly one trace firing.

Three instances of one joint-stage constructor receive trace CELL identities:

```text
stage 0: A trace + B trace -> O -> P -> candidate -> X outlet
stage 1: X trace + C trace -> O -> P -> candidate -> Y outlet
stage 2: Y trace + D trace -> O -> P -> candidate -> Z outlet
```

Every O has threshold two and unit inputs. Every P has threshold two. Every
output has threshold two and is the sole cell at physical distance one from its
P. No P->output candidate ARROW exists initially. Each stage has the same P
normalizer, threshold-three attribution M, and connections:

```text
P fires -> native distance-one P->output proposal and traversal
P execution -> one P trace
output execution -> its ordinary participant trace
P trace + output/participant trace + ordinary global return -> M
M -> P, impulse 1 -> candidate credit without P refiring
```

The output trace is simultaneously the stage's downstream participation trace
and its physically completed-effect trace. There is no second representation.
All fixed couplings, delays, thresholds and resistances are identical across
the three stages. Reflection changes only whether the output is one position
to the left or right of P.

## Uniform opportunity surface

At each possible depth tick the harness sends the same external unit
background to all three P cells and fires one shared context cell whose unit
output reaches all three stage outputs. It never sends an input to an
individual P or output.

```text
inactive stage: background 1 -> P does not fire or propose
active stage:   O 1 + background 1 -> P fires and may propose
weak candidate: candidate 1 + uniform context 1 -> output fires
mature candidate: candidate 2 -> output fires once without context
```

Ordinary global return is likewise broadcast to all three M cells. Only an M
with live P and output traces reaches threshold three. Its unit echo is below
P's threshold two.

## Frozen worlds and schedules

Seeds `3401, 3409, 3413, 3419` execute in order. They cross normal/reversed
cell and arrow insertion with forward/reflected distance-one proposal geometry.
Raw primitive coupling and background load are one and zero respectively.

For an exposure beginning at `s`, causal depth is expressed only through
physical timing:

```text
A and B sources: s       -> primitive traces: s+1
X outlet: s+2            -> X trace: s+3
C source: s+2            -> C trace: s+3
Y outlet: s+4            -> Y trace: s+5
D source: s+4            -> D trace: s+5
Z outlet: s+6            -> Z trace: s+7
```

Uniform all-P background and context occur at `s+1`, and additionally at
`s+3`/`s+5` for requested depths two/three. Global return occurs at
`s+3`, and additionally at `s+5`/`s+7`. Thus each active stage receives the
same four-tick participation-to-credit geometry used by MICRO-A.

The one continuing training world receives:

```text
AB depth-one exposures:   s = 0, 11
XC depth-two exposures:   s = 20, 31
YD depth-three exposures: s = 40, 51
final ordinary pressure boundary: tick 60
```

Expected native activity per phase, in stage order AB/XC/YD:

```text
phase       O/P/candidate/output/P-trace/output-trace/M/credit   candidate impulse
AB train    2|0|0                                                3|0|0
XC train    2|2|0                                                4|3|0
YD train    2|2|2                                                4|4|3
```

Exactly one native proposal occurs in each phase. Candidate resistance is:

```text
after first AB / recurrent AB:  4|0|0 / 6|0|0
after first XC / recurrent XC:  8|4|0 / 10|6|0
after first YD / recurrent YD: 12|8|4 / 14|10|6
after tick 60 pressure:         13|9|5
```

Clones taken immediately after each stage's first exposure advance through
ordinary pressure to ticks `50`, `70`, and `90`; the newly formed AB, XC, and
YD candidate respectively must reach zero. One experience may execute but may
not earn reuse.

## Context-free reuse and controls

Held-out clones never receive context or global return. They receive only
primitive source activity and uniform all-P background at the relevant depth
ticks. Therefore an output can execute only through an already mature
coupling-two candidate.

After AB training:

- timed AB produces one X outlet execution and exactly one X trace firing;
- C alone produces one C trace but no XC firing, proposal, Y or Y trace;
- AB followed by C after X trace expiry produces X and C traces but no XC
  firing, proposal, Y or Y trace.

After XC training:

- timed AB then X+C produces exactly one X and one Y outlet execution and one
  ordinary X and Y trace firing;
- D alone produces one D trace but no YD firing, proposal, Z or Z trace;
- the depth-two chain followed by D after Y trace expiry produces Y and D
  traces but no YD firing, proposal, Z or Z trace.

After tick 60, a timed depth-three held-out chain must produce exactly:

```text
primitive trace: A=1, B=1, C=1, D=1
stage O/P/candidate/output/P trace/M: 1|1|1
candidate raw impulse:                2|2|2
output normalization arrivals:        2|2|2
output normalization impulse:         2|2|2
ordinary derived trace X/Y/Z:         1|1|1
structural proposals and credit:       0|0|0
```

The held-out X+C and Y+D modules receive the same two unit trace inputs and
take the same O->P->candidate->output->normalizer path as primitive A+B. No
downstream cell may observe the raw coupling or construction history of X/Y.

## Serialization and verdict

Each row records exact per-phase firings, crossings, impulses, proposal and
credit counts; candidate ArrowIds/generations/resistances; one-exposure
deallocation; all context-free controls; final primitive/output normalization;
work, storage, fingerprints, replay and natural quiescence. A resistance value
may not stand in for execution or trace evidence.

- **GATE-A positive:** all four rows satisfy every formation, recurrence,
  normalization, timing-control, context-free reuse, level-blind recursion,
  replay and quiescence predicate.
- **GATE negative:** any level needs a supplied candidate or different physical
  API; any derived execution emits zero or multiple traces; raw mature coupling
  leaks through normalization; a gapped/single-sided stage forms or fires; one
  exposure survives; unit credit refires P; any final stage fails; or any exact
  row fails.

Implementation requires a separately frozen execution protocol, E2B preflight
and one write-once `--gate` execution. Any failure is frozen without tuning or
rerun. A positive result is development readiness evidence only; definitive
evidence and authority remain separate.
