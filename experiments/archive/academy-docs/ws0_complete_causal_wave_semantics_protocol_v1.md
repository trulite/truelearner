# WS0 complete causal-wave semantics protocol v1

Status: frozen before any WS0 runtime or evaluator change.

Parent: RS2 v5 cumulative-integration negative `23de525`, whose canonical
runtime is byte-identical to AH0 development-ready `6ab8a15`.

RS2 is stopped. WS0 does not retry or modify RS2.

## Question

Can one complete causal-wave rule cover all already-existing CELL/ARROW/SPIKE
effects—Drive, Modulatory consequence, and participation-qualified local
closure—without allowing arbitrary handles, insertion order, or observer order
to become physics?

## Physical wave

A wave is one physical moment identified by:

```text
arrival tick
phase
causal generation
```

All valid arrivals in that wave are admitted together. At every junction:

- all same-wave Drive impulses form one signed local Drive incidence;
- all same-wave Modulatory arrivals form one local Modulatory incidence whose
  multiplicity is retained;
- Drive changes junction activation;
- Modulation acts on the local pre-existing participation/plastic state;
- these local effects form one synchronous incidence stage;
- threshold and refractory state are evaluated once after that incidence
  stage, so a CELL fires at most once in the wave.

Every transmission caused by that local moment—ordinary SourceFires Drive,
ordinary SourceFires Modulatory, or QualifiedLocalParticipation Modulatory—
enters the next causal wave when its delay is zero and its phase is unchanged.
Positive delay or a later phase supplies its own physical separation and
starts at causal wave zero there.

No event created by a wave may re-enter the incidence stage of that same wave.

## Simultaneous Modulation

The retained substrate treats each ordinary Modulatory arrival as one physical
consequence. If several arrive at the same junction in one wave, their local
effects are evaluated as an unordered multiset against the participation state
that existed at the start of the wave. Multiplicity is preserved; arrival
identity and insertion order are not.

Any QLP transmissions caused by those coincidences are scheduled only into the
next causal wave. Modulation never excites or fires the CELL.

## Prohibitions

Causal execution must not inspect or prefer:

- numeric CELL/ARROW/physical identity;
- resident slot or arena placement;
- insertion or recording order;
- mechanically assigned serial within one wave;
- Drive-first or Modulatory-first packet order;
- a path, predecessor, credit, reward, depth, or backward mode.

Canonical ordering remains permitted only for storage, hashing, debugging, and
observer normalization.

## Frozen WS0 matrix

Run every family under Reference and Production, exact same-mechanics replay,
fresh disjoint identities, handle/insertion permutations, and logical
wave-normalized observation.

1. multiple signed Drives at one junction;
2. multiple Modulatory arrivals at one junction;
3. simultaneous Drive and Modulatory incidence at one junction;
4. simultaneous Drive and Modulatory incidence at different junctions;
5. SourceFires Drive causing a later Modulatory arrival;
6. SourceFires Modulatory transmission from an ordinary firing CELL;
7. Modulatory incidence causing PQLC continuation;
8. zero-delay Drive chain;
9. zero-delay Drive fan-out/merge;
10. zero-delay Modulatory/PQLC chain;
11. recurrent Drive topology under existing refractory physics;
12. mixed recurrent topology with ordinary Modulatory traffic;
13. CELL/ARROW/physical-handle renaming;
14. reversed CELL/ARROW/input insertion order.

The mixed same-junction controls must establish:

- Modulation cannot contribute activation or firing;
- Drive cannot masquerade as consequence;
- pre-existing participation can receive simultaneous Modulation;
- participation caused only by a firing in the current wave is unavailable to
  Modulation in that same wave;
- a CELL fires at most once from the complete signed Drive incidence;
- all caused zero-delay activity is observed strictly in a later causal wave.

## Exact comparison

After logical renaming, compare:

- wave key and complete incidence/event multisets;
- Drive and Modulatory delivery multiplicity;
- fires;
- resistance/coupling changes where enabled;
- QLP traversals;
- proposals and deallocations;
- PhysicalWork;
- physical clock;
- normalized durable body;
- canonical pending physical activity;
- natural quiescence/observation-ceiling classification;
- identical continuation from live checkpoint;
- exact replay.

ExecutionCost and raw observer-recording order are diagnostic only.

## Prefix and stop rule

After targeted format/check/Clippy, the complete WS0 matrix executes once in a
fresh E2B worker. Any failure freezes WS0 negative without repair or rerun.

Only a complete WS0 positive permits unchanged replay of:

```text
SI0 v2 -> PQLC0 -> PQLC1 -> retained cumulative corpus
```

RS2 may be retried once only after WS0 and that retained prefix are positive.
CE1, FD2 v2, frozen ARC A2, authority, oracle status, and `arch.md` remain
unchanged in WS0.
