# Composable focused sensor fields

```text
spatial field + physical focus -> bounded local refinement -> sensor transduction
           |                              |                         |
     values stay external       geometry sees no values       explicit local law
```

## Outcome

Extend `truelearner-embodiment` with one sensor-independent focused-field
construction. A complete ordered field plus zero or more physical focus
coordinates becomes a complete, bounded partition with finer regions around
each focus and coarser regions elsewhere. An explicit local transducer may then
reduce each region to one available-or-unavailable receptor value. Prove the
same construction with one-dimensional spectra, two-dimensional luminance,
depth and touch, and a held-out three-dimensional field; project the retained
real workstation eye rasters through it. This is reusable focused-sensing
infrastructure, not active search, learned attention, glyph recognition, depth
inference, hand control, or Production adoption.

## Authority

- Path: `LANGUAGE.md`; `research/constitution.md`; `lessons.md` lessons 0a, 0b,
  35, 66, 68, 69, 71, and 72;
  `plans/composable-active-sensing-effects.md`;
  `research/campaigns/workstation-monitor-cue-local-field-v1/convergence.toml`;
  `factory/receipts/composable-local-spatial-sensor-fields-verification.json`
- Revision: HEAD `dfe933886d4a030d7775356f78e908e8531c2fc2`;
  `LANGUAGE.md` SHA-256
  `2b1954b161358c4a259198b0b9e4c66a93e47350d749d7c3baf3ddcef7bb8a41`;
  convergence SHA-256
  `a4293f1addd9e76f0d0ed7109a8ff134aed315718a464eb7600c24f122696f09`;
  prior verification SHA-256
  `3871e16616760629224e9753908d47e7e2424f3555a0da4e961665c5d5636cb7`

## Model

The existing `SpatialField` remains the complete physical sensor product.
`FocusProfile` contains only a refinement depth and a maximum focus count.
Focus coordinates are supplied separately from actual sensor or actuator state;
the profile cannot inspect field values. Construction rejects an excessive
focus count, an out-of-field coordinate, or a block-count overflow before
consuming the field. Duplicate coordinates are one physical focus and focus
order has no meaning.

`focus_partition` consumes the field, starts with one region covering it, and
repeatedly subdivides every region containing a focus. Each splittable axis is
divided symmetrically: equal outer halves and, for an odd extent, one explicit
centre region. Siblings not containing a focus remain coarse. Refinement stops
at the profile depth or unit regions. Every source site moves into exactly one
region, so the result reconstructs the original field exactly. With dimension
count `D`, focus bound `F`, and depth `L`, the checked region bound is one plus
`F * L * (3^D - 1)`. Zero focus coordinates produce the single coarse region.

This is repeated product reassociation along a focus-selected branch. The
geometry arrow depends on coordinates but is natural in sensor values: mapping
values before focusing equals mapping every focused region afterward. Focus-set
union is commutative and idempotent, mirroring the field and coordinates mirrors
the partition, and unchanged field, profile, and focus reproduce exactly.

`transduce_complete` is a second, explicit arrow. It maps each available source
value into a receptor value and folds a region using caller-supplied identity and
associative combination. If any site in that region is unavailable, its receptor
is unavailable; no stale or neighboring value is invented. Refinement is
consistent: combining the transduced children equals transducing their parent
when the supplied physical operation obeys its declared laws. The output retains
region origin and shape, making resolution and locality observable.

Multiple eyes, fingertips, frequency bands, or future sensors use independent
fields and profiles through the existing parallel and bank composition. The
shared crate owns field geometry and lawful reduction only. World adapters own
the meaning of coordinates and the physical transducer; actuators own changes
to focus; the learner receives only resulting physical signals.

The research projection uses one unchanged seven-level profile per real eye,
the retained eye's actual gaze converted to raster coordinates, and unsigned
intensity addition as the frozen transducer. A fixed regular gaze grid, declared
before cue comparison, separately measures how resolution changes across the
field. Evaluator comparisons and distinguishing-region locations remain outside
the organism.

## Invariants

- Focus geometry never reads sensor values, learner state, target identity,
  correctness, reward, or evaluator results.
- Focus coordinates come from the caller's physical state and are validated
  against the field; the library never moves or selects a focus.
- Every source site belongs to exactly one focused region, and reconstruction is
  exact after every valid focus set, depth, dimension, and edge split.
- Empty focus, duplicate focus, and permuted focus sets have defined identity,
  idempotence, and commutativity behavior.
- Mapping preserves availability, geometry, origin, and composition and
  commutes with focused partitioning.
- Mirrored fields and coordinates produce mirrored partitions without an
  outward-only preference.
- Transduction is separate from geometry, uses an explicit identity and
  associative combination, and never fills an unavailable region.
- Region count is checked before allocation and remains within the profile's
  dimension, focus, and depth bound regardless of input field size.
- One-, two-, and three-dimensional fields and different value types use the
  same public construction; the shared library contains no sensor or device
  nouns.
- Repeated observation alone creates no transition. A physical focus movement
  may change the focused observation only through ordinary post-effect sensing.
- Existing spatial-field evidence, point-alias evidence, cue rendering, runtime
  retina, real-key hand behavior, stable fixation, replay, natural quiescence,
  Production behavior, and semantic firewall remain unchanged.

## Scope

- `truelearner/crates/embodiment/src/lib.rs`: focus profile, focused partition,
  checked refinement, exact reconstruction, mapping, and complete-region
  transduction.
- `truelearner/crates/embodiment/tests/focused_sensor_fields.rs`: categorical
  laws, bounds, focus effects, multiple dimensions, modalities, mirrors, and
  failure paths.
- `research/experiments/workstation-return-bearing-opportunity-composition/src/bin/`:
  compact retained-raster focused-field projection.
- `research/campaigns/workstation-focused-sensor-field-v1/`: frozen discovery
  protocol, parent control, focused-field arm, artifact, and convergence.
- `factory/receipts/`: candidate and independent verification receipts.
- Excludes core learner changes, runtime workstation retina changes, Academy
  world or renderer changes, feature recognition, salience, semantic attention,
  supplied target focus, retained last-seen values, active eye or hand policy,
  cue continuity, correspondence, reaching, checkpoint migration, authority
  promotion, and Production adoption.

## Development style

TDD. First require exact reconstruction, checked bounds, focus-set laws,
map/focus commutation, mirror preservation, unavailable-region behavior,
refinement consistency, real post-effect focus change, modality transfer, and
typed failure. Implement only the product refinement and explicit reduction
needed by those tests. Add the retained real-raster projection after the shared
laws pass.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test focused_sensor_fields`
  proves checked construction, exact reconstruction, bounded refinement,
  identity, idempotence, commutativity, mirror preservation, mapping laws,
  strict unavailability, and post-effect focus sensing.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment focused_sensor_field_transfers_across_dimensions_and_modalities`
  proves the same types handle one-dimensional spectral values,
  two-dimensional intensity, occluded depth and touch, and a held-out
  three-dimensional field.
- `cargo run --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --bin focused_sensor_field_projection -- research/campaigns/workstation-monitor-cue-reuse-v1/artifacts/monitor-cue-reuse.json research/campaigns/workstation-focused-sensor-field-v1/artifacts/focused-field-projection.json`
  projects the retained real cue pair, blank control, actual binocular gazes,
  and frozen regular gaze grid without changing organism state.
- `cargo run --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --bin monitor_cue_local_field_projection -- research/campaigns/workstation-monitor-cue-reuse-v1/artifacts/monitor-cue-reuse.json /tmp/local-field-parent.json`
  preserves the exact complete local-field parent.
- `cargo test --release --locked --manifest-path research/experiments/workstation-return-bearing-opportunity-composition/Cargo.toml --lib monitor_cue_action_outcome_reuse_retains_the_first_broken_arrow`
  preserves the full parent trace and honest first-failure classification.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields`
  preserves the semantic firewall.
- `cargo fmt --all -- --check`, `cargo check --locked`, and
  `cargo clippy --locked -- -D warnings` run in every affected workspace.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment -p truelearner-workstation`.
Its measured warm duration must remain strictly under 10 seconds; record cold
bootstrap separately.

## Controls and evidence

Held-out cases are an odd-shaped three-dimensional field, two independent
focused fields, five focus factors, mirrored odd and even fields, and a physical
focus move observed through `interact`. Negative controls are no focus, duplicate
focus, reversed focus order, unchanged focus, focus outside the field, too many
foci, zero refinement, unit fields, unavailable interior and edge sites, uniform
values, identical cue replay, blank cue, the complete local-field parent, and the
retained point alias. Falsifiers are lost, duplicated, reordered, or invented
sites; reconstruction failure; value-dependent geometry; focus-order effects;
broken mirror behavior; unbounded regions; a stale unavailable value; reduction
inconsistency; sensor vocabulary in the shared crate; target-supplied focus;
changed parent evidence, runtime behavior, or firewall; or a warm regression at
or above ten seconds.

Evidence is a validated candidate receipt, independent verification receipt,
immutable parent artifacts, and one compact focused-field projection containing
source digests, profiles, physical gazes, region counts and bounds, reconstruction
digests, cue and blank differences, and fixed-grid summaries. It contains no raw
duplicate raster, video, semantic region label, or evaluator feedback.

## Risks and rollback

Focused sensing could become hidden attention, make reduction silently semantic,
or allocate an unbounded tree. Keep geometry value-blind, keep transduction a
separate explicit fold, check the closed-form region bound before consuming the
field, and retain origins and shapes in every result. Stop if the real projection
requires a cue-specific focus, reducer, or branch. Rollback removes the new
focused types, tests, projection, and unadopted campaign while preserving the
established complete spatial field and all negative evidence; no persistence or
checkpoint migration is involved.

## Open decisions

None.
