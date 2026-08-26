# Let local outputs explore before body integration

```text
ready neighboring outputs
          |
          v
strongest route wins -> equal strength prefers less-used route
          |
          v
one output participates -> actual consequence -> used route strengthens
                                                |
                                                v
                                      later input reuses it
```

## Outcome

Add a meaning-blind local competition before neighboring ready output
junctions can cross the boundary together and cancel in a neutral body. Learned
path strength wins. Equal-strength routes prefer the one with less existing
physical participation; an exact initial tie is resolved only from physical
time and position.

This supplies alternative exposure, not an action interface or directional
policy. The body continues to integrate every crossing without selecting one.

## Authority

- Path: `arch.md` Accepted law and Successor gate; `LANGUAGE.md`; `algo.md`;
  `research/campaigns/motor-symmetry-v1/convergence.toml`;
  `research/campaigns/motor-participation-v3/protocol.toml`
- Revision: parent commit
  `bf633bc932be4684bc7070d1e6fb37b2614ef811`; frozen successor protocol
  SHA-256
  `a43fa59080ca97485ee8c5e16eddfda1fa5bdebf4db9a6e21bcf33ec2a14cafe`

## Model

- `choose_at` first identifies output incidences that will reach threshold in
  the current physical moment after the existing within-junction sign choice.
- Ready outputs form connected local groups using the existing
  `LOCAL_VARIATION_RADIUS`. Groups separated beyond that radius are independent.
- Each candidate is compared lexicographically by greater admitted path
  strength, then lower admitted-path participation. Physical time selects among
  candidates still exactly tied in the stable position order.
- Losing incidences are suppressed before `choose` records reuse, so only the
  actual participant can leave a return path and receive later outcome.
- Existing route formation, sign choice, output firing, consequence return,
  strengthening, body integration, and Academy evaluation remain unchanged.

## Invariants

- Competition uses only location, threshold, held activation, path strength,
  participation, and physical time. No axis, direction, action, capability,
  answer, reward, evaluator, or world meaning enters the learner.
- At most one ready output participates in a connected local group; nonlocal
  groups never suppress one another.
- A suppressed output link receives no reuse participation, return path, or
  outcome credit. Upstream links that actually carried activity retain truthful
  partial participation.
- Equal-strength outcome-free repetitions expose alternatives. Consequence can
  override that exploration only through ordinary strengthened path physics.
- Crossing, result, trace, checkpoint, and replay remain deterministic;
  execution reaches natural quiescence.
- The neutral human body does not arbitrate output. It continues to fold paired
  effort by signed addition and reports truthful proprioception.
- The representative warm regression remains strictly under ten seconds.

## Scope

- Change `truelearner/crates/core/src/choose.rs` only for the learner law.
- Add public-harness controls in
  `truelearner/crates/core/tests/harness_boundary.rs` for exploration, selective
  credit, reuse, nonlocality, and replay.
- Update the human public-harness regression to expect learner-owned movement
  and returned consequence rather than body cancellation.
- Preserve immutable campaign results and successor evidence under `research/`.

Exclude body-side arbitration, motor labels, readiness changes, new persistent
state, random state, world changes, Academy evaluator changes, semantic actions,
voice, ears, and checkpoint format changes.

## Verification

- Targeted causal law:
  `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core local_motor_competition`
- TrueLearner workspace:
  `cargo test --locked --manifest-path truelearner/Cargo.toml --workspace`
- Human boundary:
  `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-human --test human_harness`
- Academy compatibility:
  `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body`
- Formatting and lint:
  `cargo fmt --all --manifest-path truelearner/Cargo.toml -- --check` and
  `cargo clippy --workspace --all-targets --locked --manifest-path truelearner/Cargo.toml -- -D warnings`
- Held-out evidence: Body Discovery seed `31001`, disjoint seed `91003`, exact
  replay, natural quiescence, semantic-firewall test, and measured warm suite.

## Falsifiers and rollback

Reject the candidate if both local outputs cross, outcome-free stimulation
sticks to one alternative, a suppressed route is strengthened, a far output is
suppressed, replay changes, semantic data enters the learner, quiescence fails,
or the warm suite reaches ten seconds.

Rollback is the single core choice change plus its updated boundary tests. The
body topology, proprioception, Academy course, and pre-release checkpoint
envelope require no migration.

## Open decisions

None.
