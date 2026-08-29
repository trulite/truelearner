# Workstation binocular alignment development

```text
left retinal offset -> left horizontal eye loop  \
                                                   product -> external alignment evidence
right retinal offset -> right horizontal eye loop /
```

## Outcome

Build and test a research-only binocular body composition in which two separate
eye fields retain signed horizontal retinal position, return only light changes
caused by actual eye movement, and factor ordinal light changes through crossed
thresholds. Require both independent eye loops to reduce their own target error
across generated disparity bands. This establishes bounded binocular alignment
development only; it does not add a fused depth input, vergence motor, target
coordinate, desired direction, distance estimate, reaching, or general 3-D
claim.

## Authority

- Path: `arch.md`, `academy.md`, `LANGUAGE.md`, `algo.md`, `lessons.md`
  lessons 35, 54-58, and 66-70; `research/constitution.md`; campaign protocol
  `research/campaigns/workstation-binocular-alignment-v1/protocol.toml`
- Revision: `dfe933886d4a030d7775356f78e908e8531c2fc2`; retained failing Body
  Discovery transcript SHA-256
  `dd7daf1cd81d799668b5a6d99861370c9ed93b3b98ec1e4dd1e2b6b37d5709bf`

## Model

Each eye has an independent object consisting of its light field, retinal
samples, horizontal axis, and physical movement return. Within one eye, the
arrows are signed retinal sampling, threshold-preserving input, ordinary path
choice, bounded eye movement, changed light, and output-caused return. The
binocular system is the product of the left and right loops; neither eye may
read the other and no fused value enters the Harness.

`ResearchVisualComposition` is an orthogonal research-boundary value selecting
a finite retinal layout and the independently removable laws of signed
placement, movement-caused visual return, and threshold factorization. It
composes with the existing output-specific, aligned, causal-return research
profile without adding another monolithic combination variant. Academy-owned
evidence reads the two owned before/after eye poses and the two admitted light
fields, then measures each eye's target error outside the organism.

Failure is explicit: invalid input fails before transition; missing movement is
`MissingExploration`; movement without reduced error fails alignment; a
non-quiescent, non-replayable, over-budget, or semantically leaking run is
rejected.

## Invariants

- The organism receives only separate raster pixels, touch, proprioception, and
  generic motor opportunity through the public Harness.
- Target position, disparity band, error, improvement, verdict, and capability
  name remain evaluator-only.
- Left and right sensor histories, axes, outputs, outcomes, and movement returns
  remain distinct; binocular behavior is their product, not a fused channel.
- Signed placement changes physical receptor locality but does not encode a
  target, desired direction, or answer.
- Only a retinal change caused by the same eye's actual movement may carry that
  output's physical origin as a transition.
- Threshold factorization preserves one final transition and ordinary samples
  at intermediate crossed bins.
- Existing production construction and retained workstation traces remain byte
  identical; all new behavior is research-only.
- Exact replay, Production/reference behavior, natural quiescence, semantic
  firewall, checkpoints, bounded work, and the under-ten-second warm loop remain
  intact.

## Scope

- `truelearner/crates/workstation/src/harness.rs` and public research exports
- `academy/crates/academy-body/` external alignment measurement and honest
  course expectation
- `research/experiments/workstation-binocular-alignment/`
- `research/campaigns/workstation-binocular-alignment-v1/`
- affected Cargo lockfiles and `factory/receipts/`
- Excludes core learner-law changes, production-default adoption, authority
  promotion, fused depth, a vergence actuator, hand movement, and device use.

## Development style

TDD. First encode the honest alignment predicate and show the inherited body
fails it. Then add the complete evidence-backed visual composition, climb from
one-eye signed response to two-eye alignment and disparity transfer, and only
after success run downward removals.

## Focused tests

- `cargo test --release --locked --manifest-path research/experiments/workstation-binocular-alignment/Cargo.toml --lib`
  runs the complete candidate, its mechanism removals, replay, quiescence,
  leakage, and disparity controls.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-body`
  preserves honest curriculum failure and external-only evidence.
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment -p truelearner-workstation`
  preserves driver laws, morphology wiring, body state, checkpoints, and the
  production Harness.
- `cargo run --release --locked --manifest-path research/experiments/workstation-binocular-alignment/Cargo.toml --bin binocular_alignment_trace -- research/campaigns/workstation-binocular-alignment-v1/artifacts/development-trace.json`
  emits the retained causal and external alignment trace.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world organism_sample_contains_no_device_or_evaluator_fields`
  preserves the semantic firewall.

## Development loop

Representative warm regression suite:
`cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-embodiment -p truelearner-workstation && cargo test --locked --manifest-path academy/Cargo.toml -p academy-body --lib`.
Its measured warm duration must remain strictly under 10 seconds; cold bootstrap
is recorded separately.

## Controls and evidence

The positive reference is the complete product over near, middle, and far
generated disparity bands. Held-out cases use distinct seeds and mirrored
starting relations. Negative controls include zero disparity, a uniform light
field, unchanged-body resampling, same-direction or one-eye-only motion, and
the inherited production Harness. Downward removals independently collapse
signed placement, remove movement-caused visual return, and remove threshold
factorization. A complete-candidate failure stops the upward ladder and keeps
all removals unrun; success requires each removal to fail its predicted rung.

Evidence consists of validated campaign manifests, a retained development
trace, a validated Rust candidate receipt, and independent verification. No
research authority or production adoption follows from software success.

## Risks and rollback

A retinal lattice can accidentally mirror the fixed fixture, an evaluator can
leak target knowledge, or a research profile can alter production construction.
Detect these with generated disparity bands, mirrored and zero-disparity
controls, serialized-input audits, exact production traces, and explicit
profile-only constructors. Roll back the research visual composition and
experiment together; checkpoints and production defaults remain unchanged.

## Open decisions

None.

## Result

The preregistered three-factor candidate was over-composed and did not survive
the mirrored relation. The smallest surviving spatial product is signed
retinal placement times movement-caused visual return; ordinal threshold
factorization belongs to intensity discrimination and was removed from this
binary spatial rung. The reduced research-only composition reached exact joint
alignment for both outward and inward far, middle, and near relations, with
best-alignment separations of 32, 96, and 160. Collapsed placement, removed
visual return, and Production controls failed the closed product. The Academy
predicate now requires actual reduction of each eye's external target error, so
the unchanged Production body honestly retains `BinocularDepth` as its first
failure. Full evidence and the next discriminator are recorded in
`research/campaigns/workstation-binocular-alignment-v1/convergence.toml`.
Stable fixation is deliberately deferred; the bounded acquisition trace retains
the first post-alignment departure for the next causal localization.
