# PX3 physical event organization definitive execution protocol v1

Status: **PREREGISTERED; DEFINITIVE EVIDENCE UNSPENT; PX3 AUTHORITY ABSENT**.

- package: `arms/px3-physical-event-organization-definitive`;
- preflight:
  `cargo run --manifest-path arms/px3-physical-event-organization-definitive/Cargo.toml --release -- --preflight`;
- sole evidence command:
  `cargo run --manifest-path arms/px3-physical-event-organization-definitive/Cargo.toml --release -- --definitive`;
- artifacts: `results/px3_physical_event_organization_definitive.csv` and
  `.md`, with corresponding hidden `.staging` paths.

Exactly 64 cells execute in seed-major order `0..15`; within each seed the
world order is `full-recursive`, `recurrence-no-return`, `return-no-joint`,
`same-path-amplitude-repeat`. Every cell executes twice for exact complete-state
replay. Every row serializes P0--P9 separately; the conjunctive threshold is
`64/64` cells and `640/640` claims.

Preflight and invalid arguments construct no world, propagate nothing, write
nothing and emit no evidence marker. Evidence emits
`PX3_PHYSICAL_EVENT_ORGANIZATION_DEFINITIVE_EVIDENCE_SPENT` exactly once and
atomically publishes even a scientific failure.

The implementation must be committed and tagged before execution. Validation
and evidence use a fresh dedicated E2B sandbox whose state file is
`/Users/satya/.cache/truelearner/px3-physical-event-organization-authority-e2b.json`.
The development sandbox is forbidden.

No correction, rescue, regeneration or rerun follows the sole definitive
execution. A positive result permits only a separately committed PX3 authority
handoff; PX4 remains unexecuted.
