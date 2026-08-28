```text
one shared learner + N anonymous joint surfaces
                    |
                    v
        one physical step over all joints
                    |
          +---------+---------+
          |                   |
       N = 2 works          N = 2 fails
          |                   |
          v                   v
       N = 5 test       preserve first break
```

# Compose the reflected hand through two and five joints

## Outcome

Build one development adapter in which a single harness owns two, then five,
anonymous reflected joint modules. Require every joint to retain the demonstrated
one-joint loop, survive a proximal-state change, replay exactly, become naturally
quiet, and keep work bounded by actual joint activity. Stop before five joints if
two joints fail. This is development evidence only; accepted physics and authority
remain unchanged.

## Authority

- Path: `research/campaigns/hand-activity-normalized-build-v1/convergence.toml`
- Revision: `sha256:a83340005269f22cefea49bb0985551c6597a69175aec25ccf2a0da0df34396a`

## Model

The world state is a finite product of anonymous reflected positions. One step is
one shared-harness arrow from the current product state to the next: pending real
movement returns for every changed component fire first, then current anonymous
surface incidence fires, outward motor effects are grouped only outside the harness
by their physical source, and every component applies the same reflected force law.
The serial pose is the cumulative sequence of component positions; changing a
proximal component therefore changes every downstream pose without naming a joint
inside the learner.

The one-joint adapter is the identity control. Adding modules composes the same
local arrow in one harness, rather than running independent learners. Registration
order is a representation choice and must not change physical histories after
mapping by stable physical source. A two-joint survivor is the prerequisite for the
conditional five-joint run.

## Invariants

- One harness owns all modules and uses
  `RecursiveLearnerReturnBearingContinuation`; no per-joint learner copy exists.
- Every module exposes the same anonymous sensor, motor, outcome, reflection, and
  contact physics with disjoint physical sources; no joint index, order, target,
  desired direction, score, or capability enters the learner.
- One-module behavior exactly matches the existing complete one-joint trajectory.
- For every admitted rung, each joint reaches and leaves both limits, uses both
  signs, and retains control when a proximal position changes.
- Forward and reverse module registration preserve per-source physical behavior.
- Exact replay, natural quiescence, zero propagation exhaustion, predecessor
  digests, inherited core controls, and activity-normalized work remain required.
- Five joints are not run after a failed two-joint prerequisite.
- No accepted default, core physics, frozen predecessor, or authority record is
  changed.

## Scope

- Add `research/experiments/developmental-hand-multi-joint/` with one shared-harness
  product world, two-joint evidence, conditional five-joint evidence, controls, and
  lossless first-failure evidence.
- Add a development campaign and convergence record for identity, two-joint serial
  composition, registration-order invariance, proximal-change isolation, and the
  conditional five-joint rung.
- Add factory candidate and independent verification receipts plus durable lessons.
- Exclude core learner changes unless the two-joint trace first localizes a missing
  physical transition and a successor plan is written; exclude semantic anatomy,
  separate harnesses per joint, action teaching, accepted-default promotion,
  authority adjudication, opposing digits, grasping, and morphology transfer.

## Development style

TDD. First make the one-module identity, registration-order, and two-joint closure
predicates executable. Run two joints. Only if it survives, enable and test five
joints. If two joints fails, preserve its full per-step physical evidence and stop.

## Focused tests

- `cargo test --locked --manifest-path research/experiments/developmental-hand-multi-joint/Cargo.toml one_module_matches_existing_hand`
- `cargo test --locked --manifest-path research/experiments/developmental-hand-multi-joint/Cargo.toml two_joint_serial_composition`
- `cargo test --locked --manifest-path research/experiments/developmental-hand-multi-joint/Cargo.toml registration_order_is_inert`
- `cargo test --locked --manifest-path research/experiments/developmental-hand-multi-joint/Cargo.toml proximal_change_preserves_distal_control`
- `cargo test --locked --manifest-path research/experiments/developmental-hand-multi-joint/Cargo.toml five_joint_composition`
- `cargo clippy --locked --manifest-path research/experiments/developmental-hand-multi-joint/Cargo.toml --all-targets -- -D warnings`

## Development loop

The representative warm regression suite is
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core`.
It must remain strictly under 10 seconds; cold bootstrap is recorded separately.

## Controls and evidence

Identity control compares one shared-world module with the exact sixteen-arrow
one-joint predecessor. Composition controls compare forward and reversed module
registration, independently perturb the first component after development, retain
every component's local positions and cumulative serial pose, and require no output
source to move another component. Negative controls retain disjoint physical-source
mapping, unchanged sample-versus-transition incidence, the old protocol suite, and
zero semantic input. The held-out case changes the proximal component only after a
shared development prefix and checks the distal component from the cloned state.
Cost is recorded per admitted input, batched item, comparison, scan, construction,
and joint count.

Killing falsifiers are one-module drift; either two-joint component losing a limit,
escape, sign, perturbation recovery, replay, quiet, or bounded work; registration
order changing source-mapped behavior; proximal change destroying distal control;
cross-source movement; or five-joint failure after two-joint success. A two-joint
failure stops five-joint execution and records the earliest failed component, step,
and physical transition stage.

Because the worktree has no clean named pre-benchmark commit, this turn may produce
validated development evidence but must not claim frozen benchmark authority.

## Risks and rollback

Shared coactivity may create cross-module paths or broad motor firing; physical
source attribution and isolated perturbation expose that. Product scheduling may
grow superlinearly; activity-normalized counters expose it. Adapter identity could
drift from the official one-joint world; exact trajectory comparison kills the run.
Rollback removes only the new experiment, campaign, receipts, and lesson; core and
predecessor evidence remain untouched.

## Open decisions

None.
