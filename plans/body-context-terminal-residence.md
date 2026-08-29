# Body-context terminal residence

## Outcome

Replace no existing arm. Add one successor test that uses the retained
fixed-context trajectory to distinguish passing through an external norm from
finishing and holding there. The unchanged complete candidate must finish its
last four observations inside shifted `[2, 3]`. The fixed centered calibration
must finish its last four outside `[2, 3]` and inside `[-1, 1]`.

This can establish body-context necessity only in the bounded scalar fixture. It
does not change production calibration or promote morphology or authority.

## Authority

- Path: `research/campaigns/body-curried-component-calibration-ablation-v1/convergence.toml`;
  `research/campaigns/body-curried-component-calibration-ablation-v1/artifacts/remove-body-context.log`.
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`.
- Convergence SHA-256:
  `65fede4fd46cf805273513f7f230596d5e461cdf89b7f545bee821d7ccc4c17b`.
- Failed-arm evidence SHA-256:
  `aaa18213ae086d89da00c0ff6fa53033c25c944903480b232fd57e4f46c4e3a2`.

## Model

The retained trajectory `[3, 3, 2, 2, 1, 1, ...]` contains two different
objects: a transient crossing of `[2, 3]` and a terminal fixed point at `1`.
The old fold selected any length-four subobject. The successor selects only the
terminal length-four subobject. No learner or world arrow changes.

## Invariants

- The falsified `calibration_ablation_fixed_context` test remains unchanged and
  ignored as frozen negative evidence.
- The successor changes only the observer projection from any four-position
  window to the final four-position window.
- Complete and removed runs begin from the same checkpoint and replay exactly.
- The only physical difference remains the context curried into `Normalizer`.
- Natural quiescence and the semantic firewall remain required.

## Scope

- `truelearner/crates/embodiment/tests/runtime_attachment.rs`: one new ignored
  complete shifted-terminal reference, one new ignored fixed-context successor,
  and a pure terminal-window helper if useful.
- `research/campaigns/body-context-terminal-residence-v1/`: frozen protocol,
  evidence, and convergence.
- Research lessons, program record, and factory receipts after convergence.
- No production Rust changes.

## Development style

TDD. Add and compile the successor oracle, rerun the unchanged complete
reference, then execute the successor once by exact name. Preserve any failure;
do not alter the old arm or adapt the terminal window after execution.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment regulation_body_curried_calibration -- --exact`
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment calibration_terminal_shifted_context_reference -- --ignored --exact`
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment --test runtime_attachment calibration_ablation_fixed_context_terminal_residence -- --ignored --exact`

## Development loop

The representative regression is
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core -p truelearner-embodiment`
and must remain strictly under 10 seconds.

## Controls and evidence

The unchanged complete candidate, the frozen failed any-window arm, disjoint
shifted context, fixed centered context, exact replay, and natural quiescence are
negative controls or references. New held-out types are not applicable because
this successor changes only an observer projection over an already-retained
trajectory. Evidence is the final four positions of complete and removed runs.

## Risks and rollback

Terminal residence is still bounded to thirty-two observations and does not
prove asymptotic stability. The test must say final four, not eventual forever.
Rollback removes only the successor test, campaign, records, and receipts.

## Open decisions

None.
