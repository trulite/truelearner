# TrueLearner

CORE1 writing follows the small vocabulary in [LANGUAGE.md](LANGUAGE.md).

This branch contains the authoritative Physical Body V1 production organism
and separates it from its research history.

- `truelearner/` is the complete production Rust workspace.
- `experiments/` contains archived research code, protocols, evaluators,
  generated evidence, and audit tooling.
- `arch.md` is the accepted PXR0/PX-C + Physical Body V1 architectural oracle.

The current pre-release core preserves the accepted physical transitions behind
a harness: inputs enter, owned observations leave, and the body and resident
arena remain private. Checkpoints are opaque and carry no public body format.
Boundary Buffers V1 adds bounded FIFO staging for
`SpikeInput` and outward `Crossing` values, transactional backpressure, and
exact buffered live continuation. Cold storage, visual framebuffers, and
distributed execution remain future successors.

## Fast Rust development

Cargo automatically uses `sccache` when it is installed. Development and test
profiles keep many code-generation units but disable rustc incremental output,
which `sccache` cannot reuse.

For E2B development, keep the compiled target and compiler cache in one
explicitly reusable worker:

```sh
./experiments/tools/e2b_rust_command.py \
  --state-file .e2b-dev-state-rust \
  'cargo check --locked --manifest-path truelearner/Cargo.toml'
```

Terminate that worker when it is no longer needed:

```sh
./experiments/tools/e2b_rust_command.py \
  --state-file .e2b-dev-state-rust \
  --terminate-state
```

Omit `--state-file` for a fresh, self-terminating authority worker.
