# Academy workstation world

This headless external world composes the development `WorkstationHarness` with
a physical workstation:

- a standard ANSI 104-key keyboard with unequal key sizes and key travel;
- a continuous touchpad with maintained-contact cursor movement and tap release;
- a monitor raster containing an immutable photographic asset, visible text,
  cursor, and click selection;
- separate binocular renders of the workstation and hand;
- collision-derived palm and fingertip pressure; and
- one opaque checkpoint spanning both organism and world state.

The checked-in monitor photograph is `assets/coastal-monitor.png` with SHA-256
`a25049d016cd28d70b464040c57997fe9aa69db1502a1ff25e071b95b6768a47`.
It was generated once, then frozen as ordinary world content. Rendering is
deterministic and reduces it to the grayscale light accepted by the current
body.

Device names and state remain outside the learner. The organism receives only
two raster light fields, six contact samples, proprioception, and ordinary
physical outcomes. A key changes only after a fingertip crosses its press
depth; the cursor moves only during maintained touch with actual displacement;
and device consequences become visible on the following physical step.

This crate provides the workstation surface. It is not evidence that the
organism can separate its digits, reach, point, click, type, or understand
images.
