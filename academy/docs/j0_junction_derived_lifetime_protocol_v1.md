# J0 junction-derived lifetime discriminator protocol v1

Status: frozen before any J0 substrate, evaluator, or executable change.

Parent: CV0 resumed Gate-E negative `8311e27`.

## Question

Is an ordinary CELL best treated as an independently learned durable object,
as demonstrated by CC0, or as a junction whose lifetime is derived entirely
from the live causal ARROW topology that requires it?

CC0 remains an immutable comparison arm. J0 is not cumulative with CC0 and
may supersede it only if the complete discriminator passes.

## Competing models

### CC0 model

```text
CELL firing
    -> CELL participation

qualified Modulation at CELL
    -> CELL resistance increases

time
    -> CELL resistance decays
```

### J0 candidate model

```text
CELL
    activation / threshold / refractory / generation / live
    no consequence-supported resistance learning
    no independent learned lifetime

ARROW
    coupling / resistance / participation / generation / live
    remains the durable learned causal structure

qualified Modulation at junction C
    -> every actually participating incident ARROW at C may consolidate

at least one live incident ARROW requires C
    -> C remains live

no live incident ARROW requires C
    -> C deallocates
    -> generation advances
    -> resident slot becomes reusable
```

The incident rule is physically honest at junction granularity. If two
incoming or outgoing ARROWs actually participated, both may change. An
incident ARROW that did not participate receives no support.

J0 retains no contact/junction CELL class. The rule applies to every ordinary
CELL. Test fixtures may use high-resistance ordinary anchor topology so the
source and target remain structurally required while the junction is tested.

## Frozen gates

### 1 -- useful two-link relation

For `P -> C -> X`, both ARROWs actually traverse and qualified Modulation
arrives at C. Both participating incident ARROWs consolidate. C receives no
durable CELL update and remains live because the consolidated topology still
requires it.

### 2 -- unsupported relation

Neither weak ARROW receives qualified consequence. Both decay ordinarily; C
then has no live incident topology and deallocates. No independent CELL timer
or learned resistance is involved.

### 3 -- one surviving incident ARROW

Exactly one incident ARROW remains live beyond the weak lifetime. C remains
live while that one ordinary structural connection requires it.

### 4 -- all incident ARROWs gone

Once the last live incident ARROW disappears, C deallocates at the same local
physical time and becomes reusable.

### 5 -- generation-safe reuse

After junction deallocation, its resident slot can be reused with a fresh
`CellId` and advanced generation. The old `CellRef` remains permanently inert.

### 6 -- two participating incoming ARROWs

Two incoming ARROWs actually traverse into C before local consequence. Both
may consolidate. This records the true physical resolution of the junction.

### 7 -- one incoming did not participate

Two incoming ARROWs exist, but only one traverses. Local consequence at C may
consolidate only the traversed ARROW and any other actually participating
incident structure.

### 8 -- nearby unrelated topology

Equally recent activity on nearby but non-incident ARROWs is unaffected by
Modulation at C.

### 9 -- model discriminator

Run the same frozen physical worlds under separate `cc0-model` and
`junction-model` builds. Record where their physical histories and retained
topology differ. J0 is positive only if the junction model passes Gates 1--8
and the CC0 comparison reproduces its independently learned-CELL behavior; no
runtime switch may choose a model inside one organism.

### 10 -- representation and replay

Within each model, Reference and Production must agree exactly on ordered
physical transitions, deliveries, resistance changes, deallocations,
generation/reuse, final durable body, pending activity, clock, PhysicalWork,
natural quiescence, and same-mechanics replay.

## Prohibitions

J0 may not introduce:

```text
ContactCell / JunctionCell / TemporaryCell
CELL consequence resistance or learned CELL persistence
incoming path identity, predecessor pointer, or backward traversal
selected candidate or useful branch identity
incident support without actual ARROW participation
neighbor support, reward, task role, or evaluator cleanup
```

## Decision

- **J0 positive:** freeze the discriminator and explicitly supersede CC0 as
  the preferred development model. CC0 remains preserved evidence of a
  possible but unnecessary mechanism. Only then may CV0 be rerun.
- **J0 negative or ambiguous:** preserve CC0 and stop. Do not combine the two
  models or rescue J0 inside this gate.

SV1, RS2, CE1, FD2, ARC, authority, the oracle, and `arch.md` remain unchanged.
