# Architecture

```text
world
  |
  v
input fires -> links meet at a junction -> paths form -> one path is chosen
                                                        |
                                                        v
                                                    output fires
                                                        |
                                                        v
outcome returns on the used path -> used links strengthen
                                                        |
                                                        v
                                      later input reuses the path
```

## Status

This is the architectural oracle for the physical organism runtime.

PXR0/PX-C, Physical Body V1, and Boundary Buffers V1 are accepted as of
2026-08-24. Boundary Buffers V1 remains the global oracle parent. Connected
Outcome Product V1 is accepted as of 2026-08-28 as the scoped parent for the
next developmental-hand ladder. Generic Workstation Opportunity V1 is accepted
as of 2026-08-28 as a scoped workstation-boundary fact. R1-R6 are development
or engineering evidence unless named otherwise below.

Do not infer a new law, mechanism, or authority from this document's forward
design. Start every successor from an explicit protocol and frozen parent.

## Authority

| Result | Pin | Evidence | Standing |
|---|---|---:|---|
| PXR0 v2 | `b76f4f1c276555a6f80ff697dbc9b9ef850df76e`, `pxr0-v2-accepted-pxc-parent-v1` | 466/466 | accepted parent |
| PX-C | `ec87c438aa8c52389fd2734667363ef43acaef93`, `pxc-continuous-organism-authority-v1` | 524/524 twice | accepted law |
| Physical Body V1 | `4b2c77331708c7b6314ca3dd56d0c0607b6beff7`, `physical-body-v1-authority-positive-v1` | 540/540 | accepted body |
| Body V1 audit | `90e0328d5ca38ad6fa90ac5dc0b3eb215d819a79` | positive | accepted audit |
| Boundary Buffers V1 | `a712850` | 548/548 | current parent |
| Buffer audit | `4b5a85c411655f3f8866cbf5107e40ad6ad1231f` | positive | accepted audit |
| Connected Outcome Product V1 | `7be8bbc3009fe3131622a6ee21e9aa260d649aa1`, `hand-causal-topology-product-candidate-v1` | positive authority; one, two, and five closed; both removals failed at their frozen walls | accepted hand-ladder parent (scoped) |
| Generic Workstation Opportunity V1 | `d18f8700b9e09b6d9fb65dfdace640868bcb149d`, adopted by `db65bb0` | positive one-shot authority; 10 isolated-finger steps across all five digits; 0 five-finger steps; exact replay | accepted workstation boundary (scoped) |
| R1-R5 mechanics | `3411aba95485a309d0d4f74ec8824c5029681c82`, `r1-r5-mechanical-optimization-development-v1` | 80/80 pairs, 536/536 clauses | development only |
| R6 partitioning | `r6-partition-invariance-development-v1` | 36/36 comparisons, 2/2 controls | development only |

The accepted PX-C kernel is
`crates/pxr0-physical-runtime/src/lib.rs` at SHA-256
`e34a9442205fa63d4bde3d286fb7c0c6e722ba04b64c403535e1db71cf3fb8aa`.

The historical Physical Body V1 production surface was:

- `truelearner/crates/core/src/lib.rs` at SHA-256
  `8a0f0c862a9aa6bfaf74a3a09ca5ee0eb6b3dc95e75ce76e5a136c9a8890ff0a`.
- `truelearner/crates/arena-format/src/lib.rs` at SHA-256
  `8c35c3c07fe95b2cc76cbe9ceb47d83f250c5e0c7c40481e7371583afa48a812`.

The current pre-release production path is
`truelearner-workstation -> truelearner-body`. `WorkstationHarness` owns the
compact `Body`; its junction handles and persistence details remain private.
The historical `truelearner-core` and embodiment layers are no longer workspace
members or production dependencies.

## Accepted law

- Run one local physical law during development and use.
- Admit anonymous physical input. Do not pass action IDs, answers, capability
  names, or evaluator knowledge into the organism.
- Let links carry activity, junctions form and choose paths, and actual
  participation leave the path by which an outcome can return.
- Change strength through ordinary outcome and accepted local physics.
- Run to natural quiescence. Keep exact replay and outward-only observation.
- Keep physical time, checkpoint metadata, and wall time separate.
- Keep wall time out of learning.
- Treat `region` as physical boundary data, not meaning.
- Treat `Work` and `resident_bytes` as causally inert observers.
- Preserve the accepted pressure phase: epoch zero, period ten.
- Preserve junction liveness. Keep generation private as resident safety state.
- For the developmental-hand parent, let connected physical outcome sources
  define one ordinary field of alternatives. Keep output competition and
  bounded fresh-opportunity allocation ordinary inside that component, and
  compose both independently across disconnected components.
- This scoped law is `RecursiveLearnerCausalTopologyProductComposition`. It
  authorizes the next hand ladder; it does not change the global default or
  assert fingers, grasping, workstation use, or arbitrary morphology.
- At the workstation boundary, represent one generic chance to move as one
  physical phase and origin shared across its distinct motor targets. Preserve
  separate origins for genuinely separate causes. This is an accepted boundary
  fact, not a new learner law, and establishes unguided digit separation only.

Use the small words in [LANGUAGE.md](LANGUAGE.md) when explaining this law.
Use longer names only for exact Rust symbols or frozen evidence.

## Accepted body

- Keep durable identity independent of resident slots and storage addresses.
- Preserve bounded allocation, generation safety, compaction invariance,
  corrupt-checkpoint rejection, and exact restart.
- Keep checkpoints opaque; do not expose body or resident arena structures.
- Keep queued input and output bounded and FIFO.
- Admit input atomically.
- Apply output backpressure transactionally: if every crossing cannot fit,
  leave the body and queued input unchanged.
- Save the core checkpoint, outward region, buffer capacities, queued input,
  and queued output together.
- Never lose or duplicate an admitted boundary event across save and reload.

Boundary buffers carry activity only. They do not label, combine, interpret, or
synthesize it.

## Mechanics

The frozen historical mechanics remain evidence for the accepted law. The
compact body now implements the production law directly; the removed
`MechanicalConfig` API is not part of the current surface.

R6 established that partitioning can preserve physical history at zero added
latency. The pre-release runtime makes no public arena identity or partitioning
commitment; arena and foveation design remains future work.

Keep serial causal semantics while allowing parallel physical execution.
Partition, layout, cache, queue, and scheduling choices remain replaceable only
when they preserve the reference result.

## Boundaries

- Keep Academy, world, and review semantics outside the organism.
- Keep evaluator knowledge outside input, outcome, and state.
- Keep observation causally inert.
- Keep experimental code out of the production dependency graph.
- Keep the development factory outside organism physics.
- Preserve negative controls, exact replay, Production/reference equality, and
  natural quiescence.

## Forward design

The following ideas are design input, not current behavior or implementation
authority:

- Use a raster input surface and geometrically foveated sensing.
- Keep sensory position separate from where a learned path lives.
- Let active paths create semantic and compute hotspots; add no attention
  pointer.
- Use arenas as storage and mechanical-locality units, never semantic modules.
- Let disk hold the durable body and RAM hold active structure.
- Represent transient time with bounded local timing structures; do not scan the
  whole body.
- Make real storage, network, and device delay visible in physical time.
- Expose only dumb physical affordances; let useful use be learned.
- Keep robot, document, and world meaning outside the organism.
- Share a frozen body across private live contexts when serving.
- Preserve each context's experience; replay experience through the real law
  instead of merging mutation deltas.
- Use a small causal sequencer before introducing a distributed database.

## Successor gate

Require all of the following before changing accepted law or body authority:

1. Explicit authorization.
2. A frozen protocol and exact parent.
3. Development evidence with unchanged controls.
4. Disjoint authority evidence.
5. Exact replay, natural quiescence, and reference equality.
6. A deliberate update to this oracle after acceptance.

Stop if a change adds a cognitive noun, semantic adapter, hidden mechanism,
new substrate law, or unmeasured physical delay.
