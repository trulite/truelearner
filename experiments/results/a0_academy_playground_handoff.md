# A0 Academy Playground handoff

## Frozen boundary

A0 is ready to use as the external developmental instrument over frozen R6.
It is not part of organism cognition and must remain mechanically downstream
of `truelearner-core`.

## Entry points

- UI: `academy/crates/playground/src/main.rs`
- UI system: `academy/crates/playground/src/styles.css`
- Headless Academy: `academy/crates/academy-core/src/lib.rs`
- Design contract: `DESIGN.md`
- Run instructions: `academy/README.md`
- Frozen protocol: `experiments/protocols/a0_academy_playground_v0.md`
- Result: `experiments/results/a0_academy_playground_v0.md`

## Run

```sh
cargo run --locked --manifest-path academy/Cargo.toml -p academy-playground
```

## Next boundary

Do not start R7 from UI code. Storage residency and physical load latency remain
runtime work. Academy may observe those mechanics later through the body-level
interface, but Dioxus and Academy types must never enter TrueLearner.
