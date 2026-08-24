# ARC3 A2 newborn-opportunity pressure diagnostic protocol v1

Status: frozen after the continuous-admission negative and before successor
implementation or execution.

## Question

Can an ordinary local structural proposal receive one lawful traversal
opportunity before ordinary global pressure may remove it, without changing
any other learning, execution, or pressure rule?

## Candidate physical rule

When local variation creates a new ARROW at physical tick `t` with delay `d`,
the ARROW records a transient first-opportunity deadline `t + d`.

```text
proposal exists
but its earliest scheduled traversal has not yet been attempted
    -> ordinary global pressure may inspect it
    -> ordinary global pressure may not lower its resistance

first scheduled traversal is attempted
    -> protection ends, whether delivery succeeds or is stale/blocked
    -> all later pressure is ordinary
```

The protection is not eligibility, credit, resistance, or guaranteed
execution. Modulation still requires actual traversal. Proposal resistance
remains 1, coupling remains 1, and pressure amount/period/phase remain
unchanged. Manually constructed and previously traversed ARROWs receive no
protection.

The deadline is transient physical state and must be preserved by an exact
live checkpoint if it can exist at a checkpoint boundary. It must not enter a
durable body version.

## Core discriminators

1. A resistance-1 ordinary ARROW present before a pressure epoch still dies.
2. A distance-1 proposal created at tick 9 remains live through pressure at
   tick 10, attempts its scheduled traversal, and then loses protection.
3. A proposal whose first traversal cannot execute loses protection after that
   attempt and can die under the next ordinary pressure epoch.
4. No proposal may receive modulation before actual traversal.
5. Reference and production mechanics must serialize the same physical state
   and produce the same physical history.

## Official A2 rows

Run only A2 against official `ls20`, seed 205, curriculum `[1,4,2,3]`, with
the candidate-free spatial geometry and same-tick continuous admission.

- phase-0 row: fresh body, no initial gap;
- phase-9 row: fresh body advanced nine physical ticks before the first raster.

Each row must replay exactly. PASS requires all four scaffolded actions, the
expected four changed rasters, exactly one qualified update per completed prior
route, natural quiescence, and live traversed candidates in both phase rows.

## Stop conditions

- Any core discriminator failure stops before official evidence.
- Either official row failing freezes a development negative.
- A2 success does not authorize A3-A5, authority, or architectural-oracle
  changes. Broader retained-runtime regression is required before this law can
  become a successor baseline.

All Rust compilation and execution occurs in the reusable E2B development
worker. No project Rust command runs locally.
