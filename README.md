# TrueLearner

CORE1 writing follows the small vocabulary in [LANGUAGE.md](LANGUAGE.md).

This branch contains the compact production organism and separates it from its
research history.

- `truelearner/` is the complete production Rust workspace.
- `truelearner/crates/body/` implements the compact physical organism.
- `truelearner/crates/workstation/` adapts physical worlds to that body while
  keeping junction handles private and device meaning outside the organism.
- `experiments/` contains archived research code, protocols, evaluators,
  generated evidence, and audit tooling.
- `arch.md` is the accepted PXR0/PX-C + Physical Body V1 architectural oracle.

## Agent workflows

```text
.agents/skills/ -> agent judgment and workflow
factory/        -> engineering templates, runners, and validators
research/       -> research programs, campaigns, evidence, and E2B runtime
```

- [Agent skills](.agents/skills/)
- [Factory as code](factory/README.md)
- [Research program as code](research/README.md)
- [Agent routing](AGENTS.md)

Rust changes flow through `$rust-plan -> $rust-implement -> $rust-verify`,
with `$categorical-rust` as their modeling discipline. Academy work uses
`$academy`; benchmark families advance through `$benchmark-climb` without
letting benchmark knowledge enter the organism.

The current production path is
`Academy -> WorkstationHarness -> truelearner_body::Body`. Inputs enter as
ordinary physical sensor events, outward motor effects leave, and all junction
mapping stays private. Checkpoints remain opaque.

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
