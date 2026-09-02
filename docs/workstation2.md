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
Application: keyboard, target, or replaceable pixels
Workstation2: touchscreen + application
Workstation2Session: opaque body harness + Workstation2
```

The contact identifier is a device track, not a finger meaning visible to the
organism. A virtual key is a lit rectangle interpreted by the application only
after a generic touch ends within it.

The remaining public values are `ScreenPoint`, opaque `TouchId`, generic
`DeviceEvent`, and the frozen `Workstation2Observation`. A pixels-only
application accepts a `LightField` replacement and ignores device events so an
external adapter can consume them. `Workstation2Session::step_traced` adds only
observer evidence to the ordinary step. Course evidence is never organism
state.

## External laws

1. A fingertip crossing the screen plane starts contact; lateral motion moves
   that contact; withdrawal ends it.
2. One short contact ending inside a visible key changes application text.
3. Exactly two active contacts may change visible scale through their relative
   separation; one contact cannot.
4. Every gaze retains coarse context for the complete screen; eye position
   changes foveal detail.
5. Application effects return only as later light, and surface reaction returns
   only as later hand contact.
6. Replacing a pixels-only frame changes no body or touch-track state.

## Display and vision

```text
2048x2048 display raster
  |-- fixed 8x8 global means
  |-- signed 16x16 transient localization, retained for 32 steps
  `-- gaze-centred 17x17 fovea, 8 display pixels per sample

0..1023 body reach <-> display mapping
ARC viewport: [256,256)..[1792,1792), 64x64 at 24x
```

Display pixels, body coordinates, and application source pixels are separate
types. Full-screen applications use the full display. A 64x64 bounded game uses
the central 1536-square viewport, leaving a 256-pixel margin for a physical
bezel. Nearest-cell scaling preserves source colors and point coordinates.
Global means and the fovea sample the same physical raster, including the dark
hand occluder. Outside-screen space contributes zero only to foveal receptors
whose fields extend past an edge.

## Course

The course runs development from an opaque checkpoint, exactly replays it, then
runs a fresh probe with the virtual keyboard shifted horizontally. It records
gaze movement, touch starts, visible-key changes, pinch-scale changes, physical
work, natural quiet, and the first missing rung. It never demonstrates or
chooses an organism action.
