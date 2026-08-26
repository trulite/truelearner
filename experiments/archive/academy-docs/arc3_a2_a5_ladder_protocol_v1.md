# ARC3 A2-A5 developmental ladder protocol v1

Status: frozen before implementation and live behavioral execution.

This is one sequential Academy development run against the official ARC-AGI-3
`ls20` environment. It does not alter TrueLearner's physical law. Each gate is
evaluated in order and the run stops at the first failure while preserving a
reviewable episode for that failure.

## Fixed sensory boundary

The organism receives no ARC object, player, target, action, score, level,
terminal, or reward representation. Its visual context is a fixed mechanical
compression of the complete 64x64 palette raster:

```text
SHA-256(the 4096 raw palette bytes)
    -> first unsigned 64 bits
    -> modulo 1024 physical context sites
```

Hashing preserves no ARC semantics and is fixed for every development, probe,
replay, and transfer row. A context collision is a gate failure, not something
the evaluator may repair after observation. ARC action identifiers are mapped
to four outward motor crossings only after the crossing leaves the organism.

The spatial sensor is deliberately a sparse whole-raster fingerprint, not yet
an object retina. Passing the early gates therefore establishes contextual
sensorimotor retention, not spatial generalization.

## Shared physical learning geometry

Every context/motor pair has the same weak opportunity used by ARC3-A1:

```text
raw-raster context -> weak Drive candidate -> motor -> outward crossing
         |                                    |
         +-------------- traces -------------+
changed official raster return ---------------+
                                               -> Modulatory credit
```

Only the candidate that physically traversed can receive the changed-raster
return. Ordinary renewed raster Drive cannot strengthen it. A development-only
ordinary motor-babbling pulse may complete a motor threshold; it is never used
in frozen probes.

## Frozen curriculum

- official game: `ls20`, version `9607627b`;
- development seed: `205`;
- held-out seed: `206`;
- external motor map: `[1, 2, 3, 4]`;
- four-turn curriculum: `[1, 4, 2, 3]`;
- context sites: `1024`;
- autonomous A4 ceiling: `64` further actions;
- all frames must contain exactly 4096 values in `0..15`;
- every organism transition must quiesce naturally.

The curriculum is Academy scaffolding. It is not evidence that the organism
discovered a goal. The official next raster is the only developmental return
admitted to the body.

## A2 — four contextual action-effect relations

One retained body experiences the four curriculum actions in sequence. For
each action:

- a distinct raw-raster context must be observed;
- one babbled motor crossing must cause the official action;
- the subsequent official raster must differ;
- exactly one qualified plasticity update must support that traversed route;
- all four motor identities must occur exactly once;
- no context collision or ambiguous output is allowed.

A2 passes only after all four relations exist in the same body.

## A3 — state-conditioned autonomous replay

Reset the official environment but retain the body. From the reset raster,
run four turns with:

- no babbling;
- no changed-raster return or other modulation;
- exactly one outward motor crossing per turn;
- the emitted motor sequence exactly `[1, 4, 2, 3]`;
- each crossing produced by the context learned at the corresponding A2 turn.

This is a contextual action-selection result. It is not yet goal discovery or
planning.

## A4 — autonomous level completion

Continue from a fresh official reset with the retained body, no curriculum,
no babbling, and no modulation. The private Academy evaluator checks the
official `levels_completed` field, which never enters TrueLearner.

A4 passes only if at least one official level completes within 64 actions.
Silence, ambiguous output, repeated non-progress, timeout, loss, or exhausted
work fails A4 and stops the ladder. The failure frame and organism observation
must still become a reviewable episode.

## A5 — held-out-seed transfer

A5 runs only after A4 passes. The same body receives a fresh `ls20` environment
at seed `206`, with no additional teaching, babbling, or modulation. It must
complete one level within 64 actions. Context collisions or exact-frame
memorization that does not transfer are lawful scientific negatives.

## Evidence and replay

The one command runs A2 through A5 sequentially, stops at the first failed
gate, and serializes:

- every admitted official raster;
- every outward motor crossing and executed ARC action;
- babbling and modulation flags;
- physical work, clock, pressure phase, resistance, coupling, and quiescence;
- private official state and level count only in Academy evidence;
- gate status, first failure, and skipped gates;
- one video per executed gate;
- a second complete run with the same seed that must be byte-identical after
  normalization.

No source under `truelearner/` may change. A2-A5 are development evidence only;
no organism authority is advanced.
