# Measure actual digit separation

```text
owned body observations -> changed fingers per step -> isolated digit events
                                                           |
                                             two distinct digits required
                                                           |
                                             Academy verdict only
```

## Outcome

Make Academy's `DigitSeparation` claim mean that at least two distinct finger
axes each move without another finger moving in the same physical step. Whole-
hand flexion, whole-body coactivation, repeated motion of only one finger, and
silence cannot establish separation.

This changes external evidence only. It does not teach, select, suppress, or
reward an organism action and does not change learner or body physics.

## Authority

- Path: `academy.md` Body discovery and Evidence rules; `arch.md` Boundaries;
  `research/campaigns/motor-no-consequence-v1/convergence.toml`
- Revision: frozen parent commit
  `86d471654497ac27af0a1beb9b52cada7d52bd7d`; research protocol SHA-256
  `f9258e9e7e3b246de4cc9cb0be2fba00378abca8c5df9627478430c170ab74d4`

## Model

- One `HumanStepObservation` owns the physical movements from one completed
  harness step.
- A pure projection retains changed `FingerFlexion` axes from that step.
- An isolated digit event exists exactly when that set has cardinality one.
- `DigitSeparation` passes when isolated events cover at least two distinct
  finger axes during the experience. A changed movement with no such evidence
  remains `Failed`; no changed movement remains `MissingExploration`.
- Development, probe cloning, course prerequisites, world generation, replay,
  quiescence, work accounting, and all organism-visible samples remain
  unchanged effects around this evaluator-only projection.

## Invariants

- The evaluator reads only owned observations after harness execution.
- Capability name, desired digit, direction, isolation target, verdict, score,
  or evaluator state never enters `WorldSample`, outcome, or checkpoint.
- Moving all ten fingers together cannot pass. Moving the same isolated finger
  repeatedly cannot pass. Two different fingers moving alone can pass.
- Canceled effort and bound-blocked effort are not movement evidence.
- Probe mutation remains discarded; exact replay and natural quiescence remain
  mandatory.
- Learner core, human harness, body state, worlds, scheduling, thresholds other
  than the semantic meaning of `DigitSeparation`, and later capability rules
  are unchanged.
- The representative warm regression remains strictly under ten seconds.

## Scope

- Change only `academy/crates/academy-body/src/course.rs` to derive truthful
  per-step digit-separation evidence and add its unit controls.
- Strengthen `academy/crates/academy-body/tests/body_course.rs` to preserve
  `DigitSeparation` as the current first failure until the learner demonstrates
  the corrected claim.
- Record the corrected frozen-parent result and the rejected whole-body
  coactivation counterexample.

Exclude `truelearner-core`, `truelearner-human`, motor competition, outcome
return, proprioception, body integration, Academy worlds, curriculum order,
lesson duration, action injection, accepted authority, and later capability
evidence.

## Development style

TDD. Add failing controls for whole-hand coactivation and repeated one-finger
motion, plus a positive two-isolated-digits case, before changing the verdict.

## Focused tests

- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`
  establishes isolated-digit evidence and preserves all existing evaluator and
  world controls.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --test body_course`
  establishes the frozen-parent frontier, probe isolation, exact replay, and
  the semantic firewall.
- `cargo test --workspace --locked --manifest-path truelearner/Cargo.toml`
  establishes that learner and body code remain unchanged and passing.
- Academy workspace format, check, and clippy commands establish canonical
  warning-free Rust.

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`

Its measured warm budget is strictly under 10 seconds. Record cold compilation
and full course execution separately.

## Controls and evidence

- Held-out cases: all ten fingers together; two fingers together; the same
  isolated finger twice; two distinct isolated fingers; non-finger hand motion;
  canceled finger effort; seeds `31001` and `91003`.
- Negative controls: evaluator data remains absent from serialized samples;
  probe checkpoints remain unchanged; body and learner files are byte-identical
  to the frozen parent.
- Laws: step grouping is preserved; changed-axis set order and duplicates do
  not affect the verdict; side plus digit identifies a physical finger axis;
  concatenating steps cannot erase valid prior isolated evidence.
- Falsifiers: synchronous fingers pass; one special finger passes alone;
  evaluator state enters organism input; a learner/body file changes; replay or
  quiescence differs; the frozen parent advances beyond corrected separation;
  or the warm regression reaches 10 seconds.
- Evidence: validated plan, pure evaluator controls, frozen-parent course run,
  semantic-firewall test, exact replay, candidate receipt, and independent
  verification receipt.
- Not applicable because this corrects an Academy developmental claim rather
  than running an official external benchmark or promoting learner authority.

## Risks and rollback

- The stronger claim may keep the frontier at `DigitSeparation` longer. That is
  the intended honest result, not a regression in learner behavior.
- Requiring two distinct isolated digits is narrower than merely observing one
  independent finger. The explicit claim and controls make that boundary
  reviewable; later broader dexterity claims need separate evidence.
- Roll back the evaluator helper and tests together. Organism checkpoints and
  physical histories require no migration because they are unchanged.

## Open decisions

None.
