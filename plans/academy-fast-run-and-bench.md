# Separate Academy run, review, integration, and performance lanes

```text
                    ┌─> evidence bundle ─> review media ─> Playground
worlds -> Academy run
                    └─> stable fingerprint
                              |
                              v
                       release performance sample
```

## Outcome

The normal Academy loop compiles and runs only headless curriculum code. A run
freezes compact, reconstructable evidence before a separate command renders
posters or videos. Playground and performance measurement are explicit lanes
with independently disposable target directories.

Performance samples time already-built release code around fixed headless workloads; they
do not include Cargo, JSON layout, video encoding, or Playground unless a
separately named integration sample asks for that effect. Performance clocks
remain observers and never enter physical input, time, outcome, or state.

This is an engineering and evidence-layout change. It changes no organism law,
Academy claim, teaching case, probe, control, or replay criterion. The repository
is pre-release, so the local episode record layout has no compatibility
successor requirement; exact information and replay are preserved instead.

## Authority

- Path: `academy.md`; `arch.md` Boundaries; `academy/README.md`;
  `academy/docs/episode_review_v1.md`; `AGENTS.md`
- Revision: commit `b0e61e9e591fdff648a8394be6300e6f05494103`;
  `academy/Cargo.toml` SHA-256
  `c079cfe2af4ae64c617e599e8bdd3ac3933c5b8ae02cc66d7574441d10f0b734`;
  `academy/crates/academy-runner/src/main.rs` SHA-256
  `142d3b22270cabb261ed23736da8e15c1361119f4b0a8ec9cf72c8f5081658c7`;
  `academy/crates/academy-episodes/src/lib.rs` SHA-256
  `e5149f1d2728114a9f75915983c5c3ef0eea048d0d6095ce93f09d84b4ad15c2`;
  `academy/docs/episode_review_v1.md` SHA-256
  `68920e6653f8de6eed518dfc509ada2323c89000c5663da65d2d393b5a2c269c`

## Model

- A headless suite is a pure description of ordered development, test, and
  control runs plus effectful execution through the harness. Its output is a
  fixed collection of complete `A1Experience` values and stable fingerprints.
- `academy-evidence` maps each experience to a local content-addressed bundle:
  compact metadata, raw checkpoint blobs, compact inputs/crossings, and lossless
  PNG surfaces. Its inverse reconstructs the same experience or returns a typed
  integrity/decoding error. A manifest names and hashes every component.
- `academy-runner run` executes the suite and atomically publishes evidence
  manifests only after every referenced component is durable. It invokes no
  renderer, network client, or UI.
- `academy-render` maps frozen evidence to review frames, posters, catalog, and
  optional MP4 derivatives. It cannot invoke a physical run. Rendering one
  manifest at a time permits incremental reuse when the evidence hash is
  unchanged.
- `academy-perf` maps fixed suite configurations to repeated release samples
  and a JSON measurement receipt. It validates the expected final fingerprint
  on every sample before reporting time and work distributions.
- Cargo target lanes are `headless`, `playground`, and `perf`. Each lane
  owns one cache and can be cleaned without evicting the others. Exact commands
  are documented; no command silently builds every lane.

## Invariants

- Inputs remain anonymous physical arrivals and evaluator knowledge remains
  outside the organism.
- Development, fresh tests, transfer cases, and negative controls keep their
  existing order, seeds, assertions, work, outputs, fingerprints, and natural
  quiescence.
- Writing then reading an evidence bundle reconstructs equal checkpoints,
  inputs, crossings, surfaces, observation, replay verdict, and fingerprints.
- Every manifest reference is content-addressed and verified before use; a
  missing, corrupt, swapped, or truncated component fails closed.
- A review is a function of frozen evidence. There is no edge from rendering,
  ffmpeg, Playground, or performance time back into a run.
- The dependency graph contains no AWS SDK or S3 adapter, and the default graph
  contains no Dioxus or Wry.
- The performance graph contains no Dioxus, Wry, AWS SDK, ffmpeg invocation, or
  evidence serialization in a headless timing sample.
- Performance samples are rejected if their physical result differs; faster wrong
  work is never reported as an improvement.
- The representative warm regression remains strictly under 10 seconds.

## Scope

- Add a small headless suite module/crate that owns A1 orchestration now embedded
  in episode generation; make the runner and performance sampler consume it.
- Add `academy-evidence` for local manifests, content-addressed components,
  atomic publication, integrity checks, and exact A1 reconstruction.
- Make `academy-runner` default to run-only evidence production and add an
  explicit command for composing run plus review when desired.
- Make `academy-episodes` consume frozen evidence only; add `academy-render` as
  the explicit media command and retain the portable `academy-review` catalog.
- Remove the unused Academy S3 crate and AWS SDK dependency closure entirely.
- Add a dependency-light `academy-perf` release executable with fixed warmup,
  sample count, fingerprints, work counters, and machine-readable output.
- Document separate target directories and commands for headless, Playground,
  and performance lanes. Keep full-workspace CI as an explicit cold gate.
- Update the episode-review document to describe the new local evidence manifest
  and separate review derivative.

Exclude changes to TrueLearner physics, harness behavior, accepted checkpoints,
ARC3 world semantics, research authority, capability capstones, remote benchmark
services, Criterion, speed-driven algorithm changes, Playground visual design, and automatic
deletion of an existing target directory.

## Development style

TDD. First freeze current suite outputs and add evidence round-trip, corruption,
dependency-boundary, render-from-frozen-input, and performance-determinism tests.
Then separate the effects and commands without changing the frozen assertions.

## Focused tests

- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-evidence --lib`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-core -p academy-arc3 --lib`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-runner`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-episodes -p academy-render`
- `cargo check --locked --manifest-path academy/Cargo.toml`
- `cargo check --locked --manifest-path academy/Cargo.toml -p academy-playground`
- `cargo build --release --locked --manifest-path academy/Cargo.toml -p academy-perf`
- `academy/target/perf/release/academy-perf --suite a1 --warmup 2 --samples 3 --verify`
- `cargo fmt --all --manifest-path academy/Cargo.toml -- --check`
- `cargo clippy --workspace --all-targets --locked --manifest-path academy/Cargo.toml -- -D warnings`

The tests establish exact evidence reconstruction, fail-closed corruption,
unchanged physical behavior, run/render separation, explicit UI builds,
deterministic performance results, format, and lint cleanliness.

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path academy/Cargo.toml -p academy-core -p academy-arc3 --lib`

Pre-change warm baseline: `1.19 seconds`, strictly under 10 seconds. Record cold
bootstrap, evidence writing, review rendering, and performance execution as four
separate measurements; none may be substituted for the warm correctness gate.

Measured diagnostic baselines on the current macOS host:

- clean current default check without AWS: `7.99 seconds`, `53 MiB` target;
- warm direct six-episode run plus review: `5.02 seconds`;
- same run with ffmpeg replaced by a no-op: `3.44 seconds`;
- generated gallery: `114 MiB`, of which `113 MiB` is JSON;
- one record: about `19 MiB` pretty JSON, `6.8 MiB` compact JSON, and
  `70 KiB` when gzip-compressed;
- the removed pre-change target had `30,719` files and occupied `3.7 GiB`.

Treat these as local engineering baselines, not portable scientific thresholds.

## Controls and evidence

- Held-out cases: run the ignored Core and ARC3 adversarial tests, reconstruct a
  bundle in a fresh process, render from that reconstruction, and run a release
  sample after unrelated experience.
- Negative controls: missing component, wrong content hash, truncated checkpoint,
  swapped surface, renderer without evidence, and deliberately wrong
  performance fingerprint all fail without changing the organism or accepted data.
- Laws: evidence write/read identity; rendering composition gives the same review
  metadata as rendering the original experience; repeated performance samples
  preserve the same physical result; adding an observer changes no run.
- Falsifiers: changed physical output/work/fingerprint/replay/control, renderer
  access to a live harness, performance time entering state, AWS/Dioxus in default
  dependencies, a non-reconstructable bundle, or warm regression at or above 10
  seconds rejects the candidate.
- Evidence: validated plan, candidate receipt, dependency graph check, exact
  before/after target and phase timings, performance receipt, and independent
  verification tied to the candidate digest.
- Not applicable because this is engineering isolation and measurement: do not
  create a research arm or use performance speed as algorithmic authority.

## Risks and rollback

- Splitting a record can lose self-containment; require a manifest-completeness
  check and exact reconstruction before publication.
- PNG encoding must preserve RGBA bytes and dimensions; compare decoded surfaces
  and fingerprints, not visual similarity.
- A convenience command can accidentally render before evidence is durable;
  compose only the two completed commands over a frozen manifest.
- Speed measurements can reward removed work; validate fingerprints, physical work, and
  controls before recording each sample.
- Separate target lanes duplicate some shared dependencies; keep only three named
  lanes and make each independently cleanable rather than creating per-command
  caches.
- Roll back crate edges and commands to the accepted candidate before this plan;
  evidence produced during development is pre-release and regenerated from the
  frozen suite.

## Open decisions

None.
