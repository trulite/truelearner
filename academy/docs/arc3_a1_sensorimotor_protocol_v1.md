# ARC3-A1 sensorimotor development protocol v1

Status: frozen before implementation and live behavioral execution.

This is an Academy development experiment. It does not change TrueLearner's
physical law and does not claim ARC-AGI-3 task solving.

## Claim

An official ARC-AGI-3 raster can enter through a fixed physical sensor, cause
an organism motor crossing, and return a visible physical consequence that
selectively preserves the recently traversed context-to-motor route. After
development, the retained body produces the same motor crossing from the same
coarse visual context without the motor-babbling scaffold.

The learned fact is deliberately small: in this visual context, this physical
motor path had a visible effect. It is not a goal, action-name, score, plan, or
game-rule claim.

## Semantic firewall

TrueLearner may receive only:

- raster-derived context spikes;
- a development-only ordinary Drive pulse used as motor babbling;
- a world-return spike when the subsequent 64x64 raster physically differs;
- ordinary time, pressure, Drive, Modulatory transmission, and crossings.

TrueLearner must not receive the ARC game identifier, action identifier,
score, terminal state, level count, expected behavior, pass/fail result, or
the Academy episode class. ARC action identifiers are assigned to outward
motor crossings only after those crossings leave the organism.

Raster admission uses one frozen, label-free sensor: the most frequent ARC
palette value. Ties resolve to the numerically smallest palette value. This is
a coarse physical sensor, not an ARC object detector.

## Physical geometry

For each of sixteen raster contexts and each of four motors, a weak candidate
route exists:

```text
context/action source --weak Drive--> motor --Drive--> outward boundary
        |                              |
        +---------- trace ------------+
                       \
real changed-raster return ------------> threshold-3 attribution relay
                                           |
                                           +-- Modulatory --> source
```

All candidate routes begin with coupling 1 and resistance 1. Motors require
two units. Therefore raster context alone traverses candidates but cannot
produce an action. During development only, one ordinary babbling pulse adds
the second motor unit. The resulting motor crossing is the action.

The context trace, the motor that actually fired, and the real changed-raster
return must overlap at one threshold-3 relay. Only that relay can send a
Modulatory arrival to the corresponding candidate source. Ordinary renewed
raster Drive cannot strengthen the route.

After qualified modulation raises the selected candidate coupling to two,
the same raster context can produce the motor crossing without babbling.

## Live loop

The official ARC environment remains owned by the Python bridge. A persistent
Rust agent owns the TrueLearner body. For every turn:

1. Python sends only the current 64x64 palette frame and available boundary
   actuator count to the Rust agent.
2. Rust admits the raster-derived context and runs to quiescence.
3. Rust returns zero or one outward motor crossing.
4. Python maps that crossing to one currently available ARC action and steps
   the official environment.
5. The next raster may supply physical changed-raster return.

The mapping is Academy boundary machinery. A shuffled mapping control must
make behavior follow the physical mapping rather than a hidden action label.

## Episode suite

The headless suite must generate reviewable evidence for:

1. **Initial exploration**: a fresh body uses motor babbling; the crossing is
   visibly marked as scaffolded.
2. **Development**: a changed official raster completes the physical loop and
   produces selective modulation.
3. **Frozen learned probe**: after an environment reset, the retained body
   emits the learned crossing with no babbling and no modulation.
4. **Blocked-return control**: adjacent babbled episodes with the return sensor
   blocked cannot mature the route; after pressure the body is silent.
5. **Shuffled mapping control**: the same learned motor crossing is decoded by
   a permuted external action map; behavior follows the map.
6. **Retention**: after a bounded physical gap, the supported route still
   produces the learned crossing without babbling.
7. **Exact replay**: a fresh complete run with the same seed produces
   byte-identical normalized evidence.

## Acceptance

Development is positive only if:

- every official frame validates as 64x64 values in 0..15;
- all actions are caused by outward crossings;
- the initial candidate is subthreshold without babbling;
- changed-raster return produces exactly one qualified plasticity update;
- the learned probe emits exactly one action with no babbling and no update;
- blocked return produces no update and no retained response;
- shuffled decoding follows the external map while the motor crossing stays
  unchanged;
- retention passes, every run naturally quiesces, and work remains bounded;
- exact replay is byte-identical;
- no source or dependency under `truelearner/` changes.

Stop on a physical-law ambiguity or if the official world cannot be driven
headlessly. Measurement and rendering defects may be repaired without changing
the frozen physical geometry.
