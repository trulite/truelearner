# J0 junction-derived lifetime result audit v1

Status: development positive. J0 supersedes CC0 as the preferred development
model; CC0 remains preserved comparison evidence and has not been deleted or
rewritten.

Protocol: `j0-junction-derived-lifetime-protocol-v1`

Frozen candidate: `f8c97fc37d6af9a706dd680ebb68c9d6ee416699`

## Result

Both separately compiled discriminator arms passed their frozen expectations:

| Arm | Cases | Rows | Clauses | Max PhysicalWork |
|---|---:|---:|---:|---:|
| CC0 model | 160/160 | 320/320 | 1,880/1,880 | 9 |
| Junction model | 160/160 | 320/320 | 1,880/1,880 | 10 |

Both arms reproduced exact Reference/Production ordered histories, exact
same-mechanics replay, and natural quiescence across every root and phase.

## Decisive useful relation

For the same actual `P -> C -> X` traversal and local consequence at C:

```text
CC0 model
    CELL C                 1 -> 4
    incoming P -> C        1 -> 1
    outgoing C -> X        1 -> 4
    age-10 relation        (CELL live, incoming dead, outgoing live)
    fresh P pulse          cannot re-execute C

Junction model
    CELL C                 remains 1; no CELL update
    incoming P -> C        1 -> 4
    outgoing C -> X        1 -> 4
    age-10 relation        (CELL live, incoming live, outgoing live)
    fresh P pulse          re-executes C exactly once
```

The junction model retained the useful relation with fewer learned physical
quantities: the links learned, while the junction remained because its live
incident topology still required it.

## Remaining gates

The junction model also established:

- unsupported weak incident ARROWs died, after which the orphan junction
  deallocated at the same physical age;
- one surviving incident ARROW was sufficient to retain the junction;
- loss of the last incident ARROW deallocated it;
- resident-slot reuse advanced generation and left the old `CellRef` inert;
- two participating incoming ARROWs at one junction both consolidated;
- an existing incoming ARROW that did not traverse remained unchanged; and
- equally recent nearby but non-incident topology remained unchanged.

These outcomes were obtained without a contact/junction CELL class, CELL
consequence resistance, predecessor/path identity, backward traversal mode,
selected-candidate state, or evaluator cleanup. The frozen static audit
passed.

## Implementation boundary

Under the `j0` candidate feature, stored CELL resistance remains in the common
durable representation for comparison compatibility, but it has no lifetime
or learning effect. J0 does not yet delete that dormant field from the shared
format. Such deletion belongs to a cumulative integration/replay gate, not
this discriminator.

The first J0 command stopped before evidence because the evaluator submitted
two same-tick arrivals sequentially after the first had advanced physical
time. The corrected evaluator admits those simultaneous inputs atomically;
the candidate law and frozen worlds did not change.

## E2B provenance

- reusable formatting/check/strict-Clippy/static-audit worker:
  `ifk44bxtlfjlci644r63m`
- pre-evidence harness stop: `irgofngq2vnzboxywcnrk`
- final two-arm discriminator: `ik4temdp7zwo9t4aw8ph9`

No Rust or project audit ran locally. The reusable compilation worker remains
preserved; fresh workers were terminated by the runner.

## Hashes

```text
core lib.rs        4981b007e2ddb6c282854ab03453a3c3a76ad00518e55f8fb9397c582aa991e2
evaluator          53ad8846bfc6da40bee95e7bcbc71f5ee682903373b76e4e38c6e7458c1a145f
CC0 matrix         7096503704f959f4b689af556c448ed69edf094e9bc12eb5ddf1bc8def8a0966
Junction matrix    c581c71bd4871d51e8ed45ac916b553c8e867c3e276820d012730f551a37c5a1
CC0 report         7ab899375a48fa3d90049ddb45c9c6f5b1fb2a9d5af29ae237b8892849078024
Junction report    1fadfc8f3cd06dfa33441a732034ae16a386b3e8487c2b2851ad61883ef772fd
```

## Decision

J0 is development positive. The preferred model is now:

```text
Junctions compute.
Participating links learn.
Live topology retains its junctions.
Orphan junctions disappear.
```

CC0 is superseded rather than accumulated. CV0 may now be rerun from its
frozen Gate-E boundary using J0, but that cumulative result was not run here.
SV1, RS2, CE1, FD2, ARC, authority, the oracle, and `arch.md` remain unchanged.
