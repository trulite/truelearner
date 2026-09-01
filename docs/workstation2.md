# Workstation2

Workstation2 is one external touchscreen and one ordinary application. It adds
no learner state or learner law.

## Boundary

```text
organism output: hand and eye movement
organism input:  retinal light, hand contact, proprioception

hand -> screen sensor -> DeviceEvent -> application -> screen pixels -> eyes
```

`Workstation2Session` admits exactly one `WorldSample` through
`WorkstationHarness::step`. `DeviceEvent`, text, scale, virtual-key layout, and
screen contact tracks remain external. There are no returned motor parents,
application parents, progress parents, or semantic action inputs.

## External structures

```text
Touchscreen: five opaque physical contact tracks
Application: active contacts, virtual-key text, visible scale
Workstation2: touchscreen + application
Workstation2Session: opaque body harness + Workstation2
```

The contact identifier is a device track, not a finger meaning visible to the
organism. A virtual key is a lit rectangle interpreted by the application only
after a generic touch ends within it.

The remaining public values are `ScreenPoint`, opaque `TouchId`, generic
`DeviceEvent`, and the frozen `Workstation2Observation`. Course-only evidence is
`PhaseEvidence` inside `CourseRun`; it is never organism state.

## External laws

1. A fingertip crossing the screen plane starts contact; lateral motion moves
   that contact; withdrawal ends it.
2. One short contact ending inside a visible key changes application text.
3. Exactly two active contacts may change visible scale through their relative
   separation; one contact cannot.
4. Eye position changes the local retinal view.
5. Application effects return only as later light, and surface reaction returns
   only as later hand contact.

## Course

The course runs development from an opaque checkpoint, exactly replays it, then
runs a fresh probe with the virtual keyboard shifted horizontally. It records
gaze movement, touch starts, visible-key changes, pinch-scale changes, physical
work, natural quiet, and the first missing rung. It never demonstrates or
chooses an organism action.
