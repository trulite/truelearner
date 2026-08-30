# Replace the Workstation sensor quotient with local physical sensors

```text
left receptors  ----> left-eye motors
right receptors ----> right-eye motors
touch sites     ----> nearby hand motors
proprioception  ----> its own axis motors
                         |
                         v
  compact Body -> effects
```

## Outcome

Make `WorkstationHarness` admit a fixed product of ordinary local visual,
touch, and proprioceptive readings. Remove the exact-`255` center rule and the
whole-`WorldSample` hash. Preserve the existing motor, opportunity,
state-integration, transaction, checkpoint, and Academy evidence paths while
factoring the global outcome witness into left-eye, right-eye, and hand
components. This
repairs the external sensor adapter; it does not change learner physics or
claim a new Academy capability.

## Authority

- Path: `academy.md`, `arch.md`, `LANGUAGE.md`, `algo.md`, and
  `lessons.md`'s TAME boundary lesson.
- Revision: Git `ded2e725622e270ad0d414dc433d1ee965f8145d` plus the current
  compact-body cutover candidate in the working tree.

## Model

The input object is the product `left eye × right eye × touch × proprioception`.
Construction attaches a fixed nine-by-nine receptor grid to each anatomical
eye, pressure and slip sensors to each touch site, and six scalar sensors to
each body axis. Eye receptors are equally near both directions of that eye's
horizontal and vertical motors. Each proprioceptive sensor is equally near
both directions of its own axis. Touch sensors are equally near the relevant
hand axes. The lowering arrow reads each component independently and sends its
value to its persistent handle; it performs no classification, hashing, target
detection, or motor selection. Left eye, right eye, and hand retain separate
ordinary outcome witnesses so their independent choices are not merged by a
global quotient. Sampled sensors are primed once with an
out-of-domain value so the first ordinary reading is observable and later
unchanged readings are identity.

`WorkstationHarness::transition` remains the partial transactional arrow from
one harness state and one `WorldSample` to the next harness state and owned
observation. Body mutation remains inside Workstation; Academy sees only the
public harness.

## Invariants

- No pixel value, position, target convention, capability, evaluator field, or
  whole-sample digest selects a sensor or motor family.
- A change in one eye, touch site, or proprioceptive field changes only that
  component's reading.
- Eye receptors attach only to their own eye and are equally near opposing
  directions; proprioception attaches only to its own axis.
- Touch and proprioception enter the Body as ordinary input.
- Motor opportunities remain unchanged; actual movement returns only through
  the corresponding left-eye, right-eye, or hand outcome component.
- Invalid samples remain transactional; successful waves reach natural quiet.
- Exact checkpoint replay and the 28 compact-body laws remain unchanged.
- Existing Academy failures are preserved rather than repaired with a hidden
  action route.
- The representative warm regression stays strictly under ten seconds.

## Scope

- Change `truelearner/crates/workstation/src/harness.rs` and its focused tests.
- Add no dependency, learner mechanism, capability condition, random mapping,
  persistence redesign, metric redesign, or checkpoint optimization. The only
  outcome change is the product-preserving split of the existing witness.
- Do not change Academy worlds, evaluators, capability expectations, Body
  physics, or archived research evidence.

## Development style

Use TDD: add pure component-preservation and target-convention negative tests,
then replace the sensor construction and lowering. Preserve any unchanged
Academy failure and debug only its first physical arrow.

## Focused tests

- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-workstation`
  proves component-local lowering, transactional waves, movement return, and
  checkpoint replay.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body --test body_laws`
  preserves learner identity, composition, choice, outcome, and learning laws.
- `cargo test --manifest-path academy/Cargo.toml -p academy-body -p academy-workstation`
  runs unchanged semantic-firewall, development, control, and replay tests.
- `cargo clippy --manifest-path truelearner/Cargo.toml -p truelearner-workstation --all-targets -- -D warnings`
  establishes Rust cleanliness.

## Development loop

The representative warm regression is
`cargo test --manifest-path truelearner/Cargo.toml -p truelearner-workstation --lib`.
Its measured wall time must remain strictly under 10 seconds.

## Controls and evidence

Negative controls vary a center pixel across ordinary intensities, alter only
the right eye, and alter only one touch site; none may reroute another
component. Held-out controls are the unchanged Academy Body course, physical
workstation session, corrupt input, exact replay, and compact-body law suites.
The change is falsified by any pixel convention branch, global sample hash,
cross-component reading change, evaluator field entering Body input, replay
difference, non-quiescent success, or warm regression at ten seconds.
Candidate and verification receipts are written under `factory/receipts/`.

## Risks and rollback

Removing the hidden routing may expose a genuine Academy capability failure.
Preserve it and identify the first missing physical transition; do not restore
the classifier. The fixed receptor grid intentionally bounds input size while
remaining independent of frame dimensions. Git can restore the prior adapter
if the local lowering itself violates replay or Body laws.

## Open decisions

None.
