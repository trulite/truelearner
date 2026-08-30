# Add attachment and calibration to the compact body

```text
quiet Body + quiet open Body --declared port joins--> one Body
body context ------------------calibrate------------> reading -> residual
```

## Outcome

Add exactly two capabilities to `truelearner-body`:

1. Move a quiet body with exposed junction ports into another quiet body,
   preserve all physical and learning memory, remap its identities, align the
   quiet clocks, and add only explicitly declared directional port joins.
2. Curry local body context and a relation into a memory-free calibration
   transformation from an optional reading to an optional non-negative
   residual.

The claim excludes automatic path formation, sensor or actuator types, motor
mapping, coordinates, draining, persistence, and durable attachment handles.

## Authority

- Path: `LANGUAGE.md`, `lessons.md`,
  `truelearner/crates/body/src/`,
  `truelearner/crates/core/src/attachment.rs`, and the calibration laws at
  `truelearner/crates/embodiment/src/lib.rs`
- Revision: Git `bedcbe54208bd8dc2df9d1bdde976f6c28c4ea7a`; current compact-body
  engine digest `655154f72bb1e6800c85a4f30227e565723b675276401e37be3f6136936cdab1`

## Model

The attachment objects are `Body`, `OpenBody`, its ordinal `Port`s, directed
`Join`s, and the resulting port-to-junction `Attachment`. The attachment arrow
is `attach(&mut Body, OpenBody, &[Join])`. It accepts only naturally quiet
bodies. Validation is pure and complete before mutation; failure returns the
unchanged open body and leaves the host unchanged. Success appends the part's
arena and link memory, remaps its local identities once, advances both quiet
clock domains to their maximum moment, and adds the declared ordinary links.

The calibration objects are body context, reading, and `Residual`. The arrow is
the curried form “body context to a function from an optional reading to an
optional residual.” `None` remains `None`; zero residual is the quiet identity;
independent residuals compose by saturating addition. Calibration owns no
temporal memory and does not attach anything by itself.

## Invariants

- Attachment preserves held junction state, sampled memory age, internal link
  behavior, learning memory, and causal identity.
- Attachment adds no link except each declared join and changes no existing
  host or part link law.
- The host is unchanged on every validation failure; the caller receives the
  open part back.
- Both bodies must be quiet; exclusive ownership is the input gate.
- IDs are remapped only for the appended part and returned ports; no host scan
  is required.
- Empty attachment is identity, and disconnected attachments preserve behavior
  across order and grouping.
- Calibration preserves absence, uses only supplied body context and relation,
  and has no device-specific branch.
- Existing propagation slots remain 32 bytes and the physical kernel remains
  unchanged apart from the narrow quiet-clock and appended-junction hooks.

## Scope

Add attachment and calibration modules and focused tests inside
`truelearner/crates/body`. Update only the compact body's exports and the arena
and engine hooks required to append a quiet body. Do not change old core,
embodiment, checkpoint, new-harness acceptance behavior, workstation, Academy,
research, or workspace membership.

## Development style

Use TDD. Write attachment laws, calibration laws, failure controls, and one
composition test before implementing the two modules. Keep concrete structs
and functions; add no traits, registries, protocols, serialization, or generic
sensor framework.

## Focused tests

- `cargo test -p truelearner-body --test attachment` establishes quiet atomic
  composition, port direction, state and cause preservation, identity, and
  disconnected order behavior.
- `cargo test -p truelearner-body --test calibration` establishes currying,
  absence preservation, zero identity, association, saturation, and structured
  reading transfer.
- `cargo test -p truelearner-body` establishes integration with the unchanged
  physical kernel.
- `cargo clippy -p truelearner-body --all-targets -- -D warnings -A clippy::obfuscated-if-else`
  establishes a warning-free compact API while leaving one pre-existing engine
  style lint outside this change untouched.

## Development loop

The representative warm regression suite is `cargo test -p truelearner-body`.
It must complete in under 10 seconds; record cold compilation separately.

## Controls and evidence

The unchanged checkpoint suite is held-out evidence that the resulting body
still clones and resumes correctly. Negative controls cover active host, active
part, unknown host junction, foreign part port, duplicate open ports, and an
unavailable calibration reading. The candidate is falsified by invented links,
lost held or sampled state, changed cause, partial host mutation, calibration
relation calls on absent input, or a propagation slot-size change.

## Risks and rollback

The main risk is corrupting linked-list identities while appending arenas; test
pre-existing multi-link propagation before and after remapping. Clock alignment
may age older sampled memory, which is intended physical elapsed time; test a
still-live sample. Rollback removes the two modules and their narrow arena and
engine hooks.

## Open decisions

None.
