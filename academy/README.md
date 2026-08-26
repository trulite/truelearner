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
- `academy-episodes`: canonical episode catalog plus deterministic review-frame
  and video generation.
- `academy-runner`: headless development/test/control runner.
- `academy-storage`: immutable content-addressed Academy evidence in private
  S3 storage. It remains outside TrueLearner physics.
- `playground`: causally inert Dioxus Desktop episode gallery and player.

The dependency direction is:

```text
playground -> academy-episodes -> academy-core -> truelearner-core
academy-runner -> academy-episodes
academy-storage -> AWS S3
```

Storage configuration and its object contract are documented in
[`docs/s3_storage_v1.md`](docs/s3_storage_v1.md).

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
