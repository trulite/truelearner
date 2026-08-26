# CV0-E/J0 junction-lifetime resume protocol v1

Status: frozen before any CV0-J0 feature, evaluator, or executable change.

Parent: J0 positive `972d03a`.

This resumes the unchanged CV0 proposal form and frozen Gates E--J under the
J0 junction-lifetime model. It introduces no new physical law.

## Cumulative model

```text
variation
    -> P -> C+ -> X, outgoing coupling +1
    -> P -> C- -> X, outgoing coupling -1

CELL / junction
    -> activation, threshold, refractory, generation, live
    -> no consequence consolidation
    -> retained while live incident topology requires it
    -> orphan deallocates and its generation-safe slot becomes reusable

ARROW / link
    -> coupling, resistance, continuous participation, generation, live
    -> qualified local consequence consolidates actually participating
       incident Drive links
    -> unsupported links decay
```

The `cc0` model and CELL-resistance update must not be enabled in this build.

## Sequential execution

### Stage 1 -- Gate E only

Both generated branches traverse. Consequence physically returns only at C+.

Required:

```text
P -> C+       resistance 1 -> 4
C+ -> X       resistance 1 -> 4
C+ CELL       no resistance change

C- links      remain resistance 1
age 10        C- links gone; C- orphan gone
               C+ and both required links live

frozen probe  P -> C+ -> X executes again
```

Reference and Production, same-mechanics replay, phase/identity permutations,
and natural quiescence must all be exact. Stop on any failure.

### Stage 2 -- remaining frozen gates

Run only after Stage 1 is frozen positive:

- **F negative consequence:** swap the physical return to C-. Only the
  negative relation survives and re-executes.
- **G permutation:** exchange sign order, IDs, slots, translated physical
  identities/positions, and which sign is useful. Consequence determines
  retention.
- **H neither/both:** with no qualified consequence both relations disappear;
  with independent consequence at both contacts both may survive.
- **I shared-contact control:** deliberately place `+1/-1` alternatives
  behind one ordinary junction. One local consequence consolidates both
  actually participating alternatives, reproducing the old attribution
  granularity honestly.
- **J representation:** Reference and Production agree exactly on the frozen
  ordered history, proposals, deliveries, participation, resistance changes,
  deallocations, generation/reuse, final body, pending activity, clock,
  PhysicalWork, replay, and quiescence.

Gates A--D are rerun cumulatively: symmetric two-junction/four-link genesis,
bounded creation, unsupported decay, orphan cleanup, and generation-safe slot
reuse must remain exact.

## Prohibitions

No:

```text
CC0 CELL consolidation
new CELL or ARROW law
preferred sign or inhibitory role
selected candidate ID, winner flag, reward, or error
sign-specific Modulation
incoming lookup beyond J0's ordinary participating-incident interaction
special contact lifetime or evaluator cleanup
```

## Decision

- Gate E positive permits the preregistered full CV0 stage.
- Full A--J positive establishes CV0 development readiness under J0 and
  unblocks the separately frozen SV1 replay.
- Any failure stops CV0 without rescue.

RS2, CE1, FD2, ARC, authority, the oracle, and `arch.md` remain unchanged.
