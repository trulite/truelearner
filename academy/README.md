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
receives only the current 64×64 palette frame and available physical actions.
Every game and every replay starts a fresh agent process.

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
