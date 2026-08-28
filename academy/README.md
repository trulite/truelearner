# Academy episodes

Academy develops and probes the frozen TrueLearner physical organism headlessly.
It records canonical evidence first, then derives review videos. The native
Dioxus application is only an episode gallery and player; the organism never
depends on Dioxus, video encoding, or Academy semantics.

## Generate episodes

From the repository root:

```sh
cargo run --locked --manifest-path academy/Cargo.toml \
  -p academy-runner -- output/academy-episodes
```

This writes a catalog plus six A1 episodes: one development run, one fresh
learned-relation test, and four negative controls. Each episode contains its
canonical JSON record, frame evidence, poster, and derived MP4.

## Run Body Discovery

The development-only visual-touch body has two independently movable foveated
eyes and one five-finger hand. Body Discovery is reported as four courses: Eye
Control, Hand and Finger Control, Eye-Hand Coordination, and Workstation
Contact. Eye and hand foundations can advance independently; dependent courses
remain not reached when their prerequisites fail.

Eye Control includes a bounded binocular-depth lesson. A target is projected
with equal-and-opposite horizontal disparity, and the probe requires repeated
opposing horizontal movements of the independent eyes with actual light
consequences in both eyes. No vergence motor or fused depth enters the learner.
This tests a stereo sensorimotor contingency, not general 3D understanding or
depth-directed reaching.

Opposing motor effort is integrated per physical axis, then
signed position, actual velocity, effort, joint limits, and touch return as
ordinary input. Its headless Academy course develops and probes body control
without selecting movements for the learner or counting canceled effort as
movement. Receptors are local only to their anatomical eye, hand, or digit axis
and are equally distant from both motor directions. Gaze contingency requires
repeated net movement with a recorded visual consequence:

```sh
cargo run --locked --manifest-path academy/Cargo.toml \
  -p academy-body --bin academy-body-course -- \
  --seed 31001 --output output/body-course-fixture
```

The output contains an immutable receipt and a content-addressed transcript.
Development commits learning; probes use cloned checkpoints and cannot teach
the durable learner. A preserved first failure is an honest course result.

## Physical workstation world

`academy-workstation` composes the binocular one-hand body with a headless
standard ANSI 104-key keyboard, continuous touchpad, and monitor containing a
fixed photographic image. Key travel, touch pressure, cursor motion, tap
release, visible text, and exact world-plus-organism checkpoint replay are
physical external-world mechanics. The learner still receives only pixels,
touch, and proprioception; key, cursor, click, character, image identity, and
evaluator state remain outside it.

To record an actual headless workstation run and derive an observer MP4:

```sh
cargo run --release --locked --manifest-path academy/Cargo.toml \
  -p academy-workstation-review --bin academy-workstation-record -- \
  output/workstation-run --steps 48 --seed 82001
```

The command freezes a checksummed `recording.tlwr`, decodes and exactly replays
that file, then writes `episode.mp4`, deterministic source frames, and a review
manifest. The video shows both organism eye fields plus external movement,
contact, device-event, work, quiescence, and fingerprint annotations. Review
labels and video timing never return to the learner.

## Run on macOS

From the repository root:

```sh
cargo run --locked --manifest-path academy/Cargo.toml -p academy-playground
```

The application opens a native window titled `Academy Episodes` at
1440 × 900 logical pixels (minimum 1080 × 720).

## Crates

- `academy-core`: headless capability evidence, physical admission records,
  deterministic raster surfaces, checkpoints/replay, and the bounded body
  worker.
- `academy-body`: generated Body Discovery development, probes, controls,
  replay, and evidence through the public `WorkstationHarness`.
- `academy-workstation`: headless keyboard, touchpad, monitor, collision,
  binocular rendering, and joint world/body replay.
- `academy-workstation-review`: optional frozen workstation recording review,
  deterministic observer frames, and MP4 encoding.
- `academy-review`: portable catalog and episode descriptions for causally
  inert review tools.
- `academy-episodes`: canonical episode catalog plus deterministic review-frame
  and video generation.
- `academy-runner`: headless development/test/control runner.
- `playground`: causally inert Dioxus Desktop episode gallery and player.

The dependency direction is:

```text
playground -> academy-review
academy-runner -> academy-episodes -> academy-core -> truelearner-core
academy-episodes -> academy-review
academy-body -> academy-core -> truelearner-core
academy-body -> truelearner-workstation -> truelearner-core
academy-workstation -> truelearner-workstation -> truelearner-core
academy-workstation-review -> academy-workstation
```

The Playground reads `catalog.json` at startup. Its native webview requests
posters lazily, streams only the selected video through a confined local media
protocol, and requests a record only when its download link is followed.
Playground remains a workspace member but is excluded from default members so
headless Academy checks do not compile the desktop UI stack.

## ARC-AGI-3 compatibility

The first external-world adapter lives in `academy-arc3`. It normalizes the
official 64×64, sixteen-color, turn-based ARC-AGI-3 boundary into recorded
Academy evidence and renders review videos. ARC game/action/score semantics stay
outside TrueLearner. See the
[protocol](../experiments/archive/academy-docs/arc3_compatibility_protocol_v1.md)
and
[first live result](../experiments/archive/academy-docs/arc3_compatibility_result_v1.md).

The next ARC3-A1 slice closes the loop through the physical organism. A fixed
raster sensor admits an official frame, organism crossings select actions, and
visible changed-raster outcome can strengthen the path that was used. The
headless suite includes development, a frozen learned probe, retention,
shuffled-boundary, and blocked-return controls. See the
[ARC3-A1 protocol](../experiments/archive/academy-docs/arc3_a1_sensorimotor_protocol_v1.md)
and
[development result](../experiments/archive/academy-docs/arc3_a1_sensorimotor_result_v1.md).

### Official capstone boundary

The first-class ARC-AGI-3 capstone uses a separate teaching-free agent and a
pinned `uv` adapter. The adapter may see official evaluator state; the agent
receives only the current 64×64 palette frame and a typed catalogue of available
physical actions. Nullary actions produce unit calls; coordinate actions are
represented as bounded point calls and remain silent until foveation supplies
their arguments. Every game and every replay starts a fresh agent process.

Build the release agent and exercise the public, non-scoring fixture:

```sh
cargo build --release --locked --manifest-path academy/Cargo.toml \
  -p academy-arc3 --bin academy-arc3-capstone-agent
uv run --locked --project academy/capstones/arc3 \
  academy/capstones/arc3/capstone.py \
  --mode fixture \
  --agent academy/target/release/academy-arc3-capstone-agent \
  --output output/arc3-capstone-fixture
```

Run every anonymously accessible public game locally, without a key or an
official scorecard:

```sh
uv run --locked --project academy/capstones/arc3 \
  academy/capstones/arc3/capstone.py \
  --mode public \
  --agent academy/target/release/academy-arc3-capstone-agent \
  --output output/arc3-capstone-public
```

After the candidate has independent verification, is committed, and the tree
is clean, run the complete server-selected official suite:

```sh
ARC_API_KEY=... uv run --locked --project academy/capstones/arc3 \
  academy/capstones/arc3/capstone.py \
  --mode official \
  --agent academy/target/release/academy-arc3-capstone-agent \
  --output output/arc3-capstone-official
```

Official mode has no game-subset argument. It refuses a dirty tree, requires a
registered key, leaves RHAE scoring to the SDK, scrubs credentials from the
receipt, and writes the content-addressed transcript and receipt atomically.
Use only `uv run`, `uvx`, or other `uv` project commands for this Python lane.

To review the frozen ARC3-A1 gallery locally:

```sh
ACADEMY_EPISODE_DIR=results/arc3_a1_v1/gallery \
  cargo run --locked --manifest-path academy/Cargo.toml -p academy-playground
```

## Linux development prerequisites

The E2B desktop smoke uses Xvfb and the native WebKit/GTK packages required by
Dioxus Desktop, including `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`,
`libayatana-appindicator3-dev`, `librsvg2-dev`, and `libxdo-dev`.

Xvfb is useful for a Linux launch smoke, but the product runtime is the native
desktop window. The final V0 visual review and targeted validation were also
run locally on macOS at the user's explicit request.
