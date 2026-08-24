# TrueLearner Academy Playground

The Academy Playground is an external developmental instrument for the frozen
TrueLearner physical organism. It is a native Dioxus Desktop application; the
organism never depends on Dioxus or on Academy semantics.

## Run on macOS

From the repository root:

```sh
cargo run --locked --manifest-path academy/Cargo.toml -p academy-playground
```

The application opens a native window titled `Academy` at
1440 × 900 logical pixels (minimum 1080 × 720).

## Crates

- `academy-core`: headless capability evidence, physical admission records,
  deterministic raster surfaces, checkpoints/replay, and the bounded body
  worker.
- `playground`: Dioxus Desktop UI, file/image selection, shared raster canvas,
  human controls, and causally inert instrumentation.

The dependency direction is:

```text
playground -> academy-core -> truelearner-core
```

## Linux development prerequisites

The E2B desktop smoke uses Xvfb and the native WebKit/GTK packages required by
Dioxus Desktop, including `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`,
`libayatana-appindicator3-dev`, `librsvg2-dev`, and `libxdo-dev`.

Xvfb is useful for a Linux launch smoke, but the product runtime is the native
desktop window. The final V0 visual review and targeted validation were also
run locally on macOS at the user's explicit request.
