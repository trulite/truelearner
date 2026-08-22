# DS-A0 mechanism and source fingerprints

Protocol: `ds-a0-anonymous-boundary-action-formation-v1`

## Lineage

- required clean parent: `85b01a50d0f85995632bbd7e604d6d2ff554f0b7`, tag
  `ds1-after-e0-cumulative-parent-audit-amendment`;
- preregistration: `85ae765af5ddc4d7749b00965a0ff26a7e718dce`, tag
  `ds-a0-anonymous-boundary-action-formation-protocol`;
- initial implementation: `4821be0f7705e1b60b88ed61926e00f064e9e3f0`, tag
  `ds-a0-anonymous-boundary-action-formation-implementation`;
- formation-mechanics amendment: `1ee6fef33aae6d8a443d58c8eb6fed87fa15a015`, tag
  `ds-a0-anonymous-boundary-action-formation-implementation-amendment`;
- mechanically-derived forbidden-path amendment:
  `375bf2c3f7da3f2695cc4910347826c3b5b37278`, tag
  `ds-a0-anonymous-boundary-action-formation-implementation-amendment-2`.

The first two implementation tags are superseded development history. The
protocol tag was not moved or overwritten. The implementation frozen for this
readiness handoff is `375bf2c`.

## Frozen implementation fingerprints

| Source | SHA-256 |
|---|---|
| `src/ds_a0_anonymous_boundary_action_formation.rs` | `3eb802f394a225a4ad7f0938b4a672723da2c1303ff95e805423de8161057527` |
| `src/bin/ds_a0_anonymous_boundary_action_formation.rs` | `524157266f7860ff6b3dabb8ddfd2c5c67a2446d39bdfc6443aef42e23b80374` |

The mechanism has one raw-coactivity traversal, one plastic route installer,
one bridge constructor, and one root-SPIKE executor. There is no
`RouteInstance` or preassembled executable path descriptor. Raw propagation
observations are stored in a type that the executor cannot access as ARROW
adjacency. Supported local coactivity installs three fresh bound CELLs and two
fresh live ARROWs per route before the bridge runs. Execution resolves only an
opaque root reference, injects a SPIKE at that root, and discovers every next
CELL from live ARROW endpoints and generations in the cloned temporary
substrate.

## Read-only parent continuity

| Frozen boundary | Parent and current SHA-256 |
|---|---|
| `src/ds_e0_anonymous_event_formation.rs` | `fc5d426cc8a5116dbd2749b914e6c30db88529d3070a844a20fc76ac88782615` |
| `src/ds1_after_e0_cumulative_composition.rs` | `a4deadedfde7b9896d64d0cacd41560441ea85cf3bda119a5d09aa3aaddcd7a0` |
| `src/ffs_same0.rs` | `50cf169bb293177a35270adde656f28f98e68c83a4d39d2876399261b7ee697c` |
| `src/ffs_same0/cs0a.rs` | `430cd2206c8baa7106c4de7f203d4d0c48b544290e6266596ebcdb91d02655c9` |
| frozen DS1 marked learner text | `adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e` |

No frozen/shared mechanism or prior artifact changed. M0 at `1d74c0e` remains
authoritative; M0+DS-E0 remains an enabling ancestor only; the prior unchanged
DS1 collapse remains stage 4; M1 does not exist.
