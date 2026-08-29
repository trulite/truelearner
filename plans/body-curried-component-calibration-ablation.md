# Body-curried component calibration ablation

```text
unchanged complete calibration
          |
          +-- fixed context ------> shifted norm must fail
          +-- one-shot drive -----> unresolved residual must stall
          +-- merged change ------> worsening must receive false closure
          `-- drive at zero ------> normal body must act or leave rest
```

## Outcome

Prove downward necessity inside the bounded scalar calibration world by removing
four factors independently from the already-supported complete candidate. Keep
the production `Residual`, `Normalizer`, `calibrate`, and physical calibration
trace unchanged. Express every removal only in the runtime test adapter, freeze
its predicted first failure, preserve the complete candidate as the positive
reference, and retain exact counterexample trajectories and trace decisions.

This establishes necessity only for the declared bounded regulation fixture. It
does not promote authority or establish real hand, eye, ear, voice, speech,
multisensor, or arbitrary-morphology competence.

## Authority

- Path: `plans/body-curried-component-calibration.md`;
  `research/campaigns/body-curried-component-calibration-v1/convergence.toml`;
  `factory/receipts/body-curried-component-calibration-verification.json`.
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`.
- Parent: `body-curried-component-calibration-v1` convergence SHA-256
  `b26f1cc512461e7ddc011689961760ee14327303d5efa10d55795459ed8bea4c`.
- Candidate receipt SHA-256
  `fbe476cb97b16e45f27259e779ca56eddf6463f1953aebbe8b90da05ea843208`.
- Verification receipt SHA-256
  `416655ff4c40e29ace60fa9319f6dc933d7d684a1b6555febfbfdcf92573450f`.
- Source revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`.

## Model

The complete arrow factorization is:

```text
body context -> typed observation -> residual
                                     |   |   |
                                  sample rise fall
                                     |   |   |
                                  persist no  outcome
                                          credit
zero residual -> identity
```

Each ablation removes exactly one arrow while retaining the others:

1. Fixed-context removal supplies the old centered minus-one-through-one
   relation while the external body evaluates residence in the disjoint shifted
   two-through-three relation. It must settle in the wrong band or fail the
   shifted band.
2. Persistence removal permits the first nonzero drive but removes later
   nonzero sample drive. A first useful action may close, after which a still
   nonzero residual must receive no new action and stall.
3. Directional-change removal copies motor-caused transition incidence onto the
   persistent drive surface, merging rise and fall at the exact closure
   boundary. The first worsening residual must write a false cycle closure.
4. Zero-identity removal supplies ordinary drive even at residual zero. A body
   beginning normal must emit an action or leave the normal relation.

The test adapter may filter or add already-public `PhysicalInput` values. It may
not modify core choice, physical trace topology, production calibration, motor
identity, correct direction, evaluator reward, or previous-value memory.

## Invariants

- The unchanged complete regulation test passes before and after every removal.
- Every removal is independent and test-only; no production branch or feature
  flag is introduced.
- The fixed-context arm changes only the context curried into `Normalizer`.
- The no-persistence arm retains initial drive and all real fall returns but
  removes later phase-seven sample drive.
- The merged-change arm changes only the late drive incidence from sample to the
  current physical incidence; trace rise and fall tissue remains unchanged.
- The zero-identity arm adds only one ordinary sample at the existing drive port
  while residual is zero.
- Assertions use external trajectories and retained physical trace events; no
  ablation supplies a correct action or hidden motor direction.
- Exact replay, natural quiescence, the semantic firewall, and the warm
  regression budget remain required.

## Scope

- `truelearner/crates/embodiment/tests/runtime_attachment.rs`: one test-only
  ablation strategy and four frozen negative-control tests.
- `research/campaigns/body-curried-component-calibration-ablation-v1/`: frozen
  protocol, arms, evidence, and convergence.
- `research/programs/learner/lessons.toml`, `research/programs/learner/program.toml`,
  and `lessons.md`: converged what-was-seen and what-solved-it record.
- `factory/receipts/`: candidate and independent verification evidence.
- No production Rust file changes.

## Development style

TDD. Add all four ignored ablation oracles and compile them before executing any
arm. Run the unchanged complete reference first. Then execute each frozen arm
once at its cheapest exact test name. A removal survives only when it produces
its predicted distinct failure and the positive reference remains unchanged.
Do not repair an arm after observing a different wall.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment regulation_body_curried_calibration -- --exact`
  preserves the positive reference.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment calibration_ablation_fixed_context -- --ignored --exact`
  requires shifted-context failure.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment calibration_ablation_persistent_drive -- --ignored --exact`
  requires the first unresolved post-closure stall.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment calibration_ablation_directional_change -- --ignored --exact`
  requires false closure on a worsening residual.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment calibration_ablation_zero_identity -- --ignored --exact`
  requires action or departure from an initially normal body.

## Development loop

Representative warm regression:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core -p truelearner-embodiment`.
It must remain strictly under 10 seconds.

## Controls and evidence

The unchanged complete candidate is the positive reference and the earlier
frozen uncalibrated stall is retained. Each of the other three removals is also
a negative control for the factor currently under test. Shifted context,
same-or-larger residual, residual zero, reflected effects, exact replay, natural
quiescence, and the semantic firewall remain explicit controls.

New held-out sensor types are not applicable because this is downward ablation,
not an upward generality claim. The parent's held-out spatial type remains
unchanged and the full embodiment regression reruns it. Evidence consists of
the exact external trajectory, output origins, new consequence-write events,
the first predicted failed arrow, replay equality, and quiescence for each arm.

## Risks and rollback

The main risk is accidentally implementing a second regulator inside the test.
The adapter therefore performs only input omission, incidence substitution, or
one input addition at an existing port. Another risk is calling any failure
necessity; each arm has one predicted distinct wall, and an unexpected wall is
falsification rather than support. Rollback removes only the test strategy,
ablation tests, campaign, research notes, and receipts.

## Open decisions

None.
