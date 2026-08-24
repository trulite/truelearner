# Physical Organism Runtime — Architectural Oracle and Forward Runtime Spec

This document is the architectural oracle for the physical organism runtime.

The PXR0/PX-C scientific lineage and its Physical Body V1 successor are complete and authoritative as of 2026-08-24. Sections 1–3 describe the accepted organism, its resolved runtime boundary, and its current production body. Sections 4–24 preserve forward-runtime intent except for the narrowly accepted V1 arena, identity, checkpoint, and physical-clock facts called out below; they do not authorize further implementation or substrate-law changes.

The authority pins are:

```text
PXR0 v2 accepted parent
    commit  b76f4f1c276555a6f80ff697dbc9b9ef850df76e
    tag     pxr0-v2-accepted-pxc-parent-v1

PX-C continuous-organism authority
    commit  ec87c438aa8c52389fd2734667363ef43acaef93
    tag     pxc-continuous-organism-authority-v1

canonical PX-C kernel ancestor
    crates/pxr0-physical-runtime/src/lib.rs
    SHA-256 e34a9442205fa63d4bde3d286fb7c0c6e722ba04b64c403535e1db71cf3fb8aa
    485 lines
    13 types
    16 functions/methods
    one active Rust file
    zero runtime dependencies

Physical Body V1 authority evidence
    commit  4b2c77331708c7b6314ca3dd56d0c0607b6beff7
    tag     physical-body-v1-authority-positive-v1

Physical Body V1 result audit
    commit  90e0328d5ca38ad6fa90ac5dc0b3eb215d819a79
    tag     physical-body-v1-authority-result-audit-v1

canonical production body
    truelearner/crates/core/src/lib.rs
    SHA-256 e6767845f27ddb9bb57bfb1fcab6dd1663178449faddc4a630b628e3d1148a8d
    truelearner/crates/arena-format/src/lib.rs
    SHA-256 8c35c3c07fe95b2cc76cbe9ceb47d83f250c5e0c7c40481e7371583afa48a812
    two production packages
    three production Rust files including the seven-line composition root
    zero dependencies on experiment code
```

PXR0 v2 passed its frozen `466/466` development matrix. PX-C independently passed `524/524` in development and `524/524` in its disjoint authority execution. Physical Body V1 then passed a fresh `540/540` successor authority matrix: all `512` cumulative PX0–PX8 row clauses, all `12` cumulative globals, and all `16` body clauses. Exact replay, natural quiescence, outward-only boundary observation, work and memory bounds, stable identity across compaction, canonical persistence, exact clocked restart, bounded allocation, generation safety, and corrupt-input rejection passed.

Physical Body V1 is now the oracle parent. Future work must begin as an explicitly preregistered successor. After successor authority is established, update this oracle deliberately; do not silently reinterpret it from an experimental branch.

---

# 1. Central idea

We are not building a conventional model server.

We are building a continuously executing physical organism whose long-lived body is a CELL/ARROW graph and whose momentary life is SPIKE activity.

The desired external loop is:

```text
WORLD
  ↓
input framebuffer
  ↓
foveated sensory surface
  ↓
change → spikes
  ↓
physical organism
  ↓
output framebuffer
  ↓
WORLD
```

There is no fundamental training/inference distinction inside the organism.

The same laws always run:

```text
activity
→ traversal
→ transient participation
→ eligibility
→ physical consequence/modulation
→ structural change
→ pressure/forgetting
→ further activity
```

Development means more durable structure changes.

Frozen inference means durable changes are suppressed or placed in a private CoW overlay.

---

# 2. Authoritative physical kernel and production body

The canonical production workspace is:

`truelearner/`

It contains two packages with a strict membrane:

```text
truelearner-core
    accepted CELL/ARROW/SPIKE transitions
    resident arena execution
    stable identity → disposable slot resolution
    clocked quiescent and live checkpoints
    seven-line production composition root

truelearner-arena-format
    canonical little-endian arena blocks
    immutable body manifests
    content hashes and validation
    no firing, learning, pressure, or experiment behavior
```

The PX-C kernel is retained inside `truelearner-core`; the production-body
successor changes residence, identity, and persistence representation without
changing the accepted local transition laws.

Current conceptual state is roughly:

```text
CELL
    physical identity
    physical position / region
    threshold
    transient activation
    refractory state
    resistance / generation / liveness

ARROW
    from → to
    delay / phase
    coupling
    resistance
    generation / liveness
    eligibility window
    Drive | Modulatory

SPIKE
    arrival time
    phase
    physical origin
    target
    impulse
    traversed arrow provenance

PHYSICAL BODY
    stable ArenaId + CellId/ArrowId + Generation
    disposable CellSlot / ArrowSlot resident packing
    bounded CELL and ARROW capacity
    canonical immutable ArenaBody blocks
    timeless BodyVersion manifests
    PhysicalClock derived from substrate tick
    QuiescentCheckpoint = body + clock
    LiveCheckpoint = body + clock + transient physical state
    pending load availability admitted as a physical tick
```

Core physics:

```text
Drive
→ activation
→ threshold
→ firing
→ propagation
→ traversed structure becomes temporarily eligible

Modulation
→ cannot excite/fire
→ may alter live eligible structure
→ consumes eligibility

Use + useful physical consequence
→ resistance can rise

non-use / unsupported use
→ pressure
→ resistance falls
→ zero resistance deallocates structure

physical activity
→ may create weak local structural possibilities

queue empty
→ natural quiescence

anonymous physical inputs
→ arrive
→ enter the same spike queue
→ existing physics runs to quiescence
→ already-produced crossings are filtered by outward region
```

This should remain expressible exhaustively on one page.

No active cognitive nouns should be necessary:

```text
no Episode
no History
no Query
no Role
no Cause
no Event
no Correct/Wrong
no Start
no Finish
no Answer
no AND/OR/NOT
```

---

# 3. Frozen PXR0 review resolutions

The pre-PX-C Rust review resolved all five questions below. These resolutions are part of the accepted interpretation of the canonical file.

## A. Structural variation on `external_arrival`

Proposal formation restricted to externally arriving physical activity is a legitimate retained boundary law, not residual experimental orchestration. It supplies weak local structural possibilities when the world physically perturbs the organism; subsequent traversal, modulation, resistance, and pressure decide what survives.

Changing or broadening that rule requires a separately preregistered successor.

## B. `region`

`region` is strictly physical and causally inert except for constructing and filtering ordinary boundary observations:

```text
this traversal crossed a physical boundary
→ emit Crossing
```

It must not quietly become:

```text
visual region
language region
memory region
semantic module
```

It does not select visual, language, memory, or semantic mechanisms. Future semantic organization must be learned.

## C. Work/accounting

`Work` counters and `resident_bytes` are causally inert observers retained for frozen evidence compatibility. They are not organism state transitions. A future optimized successor may move them behind an observer or feature boundary, but the accepted runtime leaves them unchanged.

## D. Cell liveness/generation/resistance

`Cell.live` is causal. `Cell.generation` is read but currently fixed and redundant. Stored `Cell.resistance` is dormant scaffolding after construction.

The latter two remain in the accepted runtime because removing them would create a successor surface. Any removal must be separately preregistered and must preserve behavior and fingerprints.

## E. Pressure phase

Pressure phase is intrinsic physical substrate time: epoch zero, period ten.

Aligned time translations should preserve behavior.

Different phase relationships may lawfully produce different physical histories.

Equivalent translations initialize the empty substrate at origins congruent modulo ten. Other phase relationships may lawfully differ. Pressure must not be reinterpreted as wall-clock housekeeping.

## F. Physical Body V1

The accepted production body adds no cognitive or learning law. It establishes
where the already-authoritative physics lives and how that body survives
movement, compaction, and restart:

```text
identity
    ArenaId + CellId/ArrowId + Generation
    independent of resident CellSlot/ArrowSlot

execution
    explicit mutable RAM arenas
    deterministic resident compaction
    no directly mutable mmap state

durability
    canonical immutable arena bytes
    canonical BodyVersion manifests
    content-addressed validation

restart
    BodyVersion is structural and timeless
    QuiescentCheckpoint adds physical clock
    LiveCheckpoint adds transient state, queued activity,
    and admitted pending-load availability ticks
```

The V1 authority does **not** claim a cold arena cache, actual asynchronous
storage service, network transport, replication, distributed scheduling,
foveated sensors, or organism-visible fetch/prefetch/pin behavior.

---

Everything from Section 4 through Section 24 remains forward-runtime intent
unless it is exactly one of the Physical Body V1 facts frozen above. Those
sections are not general implementation authorization.

---

# 4. Foveation-first embodiment

The first runtime embodiment should be framebuffer-based.

Use:

```text
one input framebuffer
one output framebuffer
```

The input framebuffer is the physical surface through which the visual world reaches the organism.

The eye/camera is **geometrically foveated**.

Near gaze:

```text
high receptor density
high spatial detail
many possible change spikes
```

Far from gaze:

```text
coarse receptor density
lower detail
fewer change spikes
```

Prefer change/event encoding:

```text
previous receptor state
vs
current receptor state
→ change
→ spike
```

A stationary world should become relatively quiet.

---

# 5. Do NOT geometrically map the retina onto memory

There are two distinct foveations.

## Sensory foveation

Fixed physical geometry:

```text
where the eye looks
→ more sensory resolution
```

## Semantic foveation

Emergent graph activity:

```text
sensory spikes
→ learned correspondence/routing
→ particular graph regions become active
→ semantic hotspots form
```

The retinal coordinate of an object must not dictate where its memory lives.

Example:

```text
object in left periphery
→ coarse spikes
→ some learned graph activity

gaze moves
→ object enters fovea
→ completely different receptor pattern

learned correspondence
→ overlapping semantic structure
```

The graph learns that these changing sensory patterns belong to the same useful physical thing.

This is central.

---

# 6. Semantic hotspots

Do not implement `ATTENTION`.

A hotspot is simply a region of the graph currently carrying unusually concentrated physical activity:

```text
high traversal
high recent participation
high local eligibility
high useful consequence support
```

Hotspots should form and cool through ordinary physics.

Several may coexist:

```text
visual hotspot
language/social hotspot
goal/action hotspot
retrieved-memory hotspot
```

There is no global attention pointer.

Compute should follow activity.

Dormant knowledge should cost almost nothing.

---

# 7. Disk-first arena architecture

The organism may eventually be far larger than RAM.

Use arenas as **storage/mechanical locality units**, never semantic modules.

An arena means:

```text
a compact chunk of physical graph state
```

Not:

```text
DOG_ARENA
LANGUAGE_ARENA
VISION_ARENA
```

Conceptual hierarchy:

```text
COLD
disk/NVMe only

WARM
RAM-resident arenas around current semantic neighborhoods

HOT
active CELL/ARROW subsets in CPU cache / SIMD batches
```

The desired principle is:

> Disk holds the organism. RAM holds the current semantic fovea. SIMD operates on the active fovea.

---

# 8. Memory layout

Separate **physical graph geometry** from **storage address**.

A CELL has a physical/logical graph position.

It separately has storage location:

```text
ArenaId
slot
generation
```

Storage may repack without changing organism physics.

Try to co-locate physically interacting structure for performance, but storage layout must never define semantics.

For hot arenas prefer SoA/AoSoA-style layouts suitable for SIMD:

```text
cell_state[]
threshold[]
refractory[]
generation[]

arrow_from[]
arrow_to[]
coupling[]
resistance[]
eligibility[]
mode[]
```

Do not scan the whole graph.

Process compact active frontiers:

```text
spikes
→ bucket by arena
→ compact active CELL/ARROW sets
→ SIMD
```

---

# 9. Timing rings / transient activity scheduling

Transient SPIKE activity should not be represented as a globally scanned unsorted vector in the production runtime.

Use bounded **per-arena timing rings / timing wheels** for near-future activity.

Conceptually:

```text
current tick
    ↓
┌────┬────┬────┬────┬────┬────┐
│ t  │t+1 │t+2 │t+3 │t+4 │... │
└────┴────┴────┴────┴────┴────┘
  ↑
process current bucket
```

An ARROW with delay `d` schedules its outgoing SPIKE into:

```text
ring[(tick + d) mod W]
```

The ring advances physical time.

`phase` still orders activity within the same tick.

A practical bucket may therefore contain:

```text
tick bucket
    ├─ phase -200
    ├─ phase 0
    ├─ phase 100
    └─ ...
```

or equivalent deterministic partitioning.

The ring is an implementation of transient time, not learned memory.

Durable CELL/ARROW structure remains in arenas.

Do not create separate semantic event queues.

## Arena execution

At each tick:

```text
timing ring
    ↓
active arena buckets
    ↓
compact arrivals by target CELL
    ↓
SIMD CELL update
    ↓
firing mask
    ↓
traverse live outgoing ARROWs
    ↓
schedule future SPIKEs into arena timing rings
```

This should be the main CPU execution pipeline.

## Why rings matter

They provide:

```text
bounded active frontier
deterministic physical ordering
cheap delayed propagation
natural per-arena parallelism
SIMD-friendly batches
```

without scanning all pending activity.

## Eligibility / refractory / traces

Do not automatically give every transient state its own ring.

Prefer compact timestamp state where possible:

```text
eligible_until
refractory_until
last_update_tick
```

A dedicated expiry structure is justified only if profiling demonstrates that scanning/checking timestamps dominates runtime.

## Distributed arrivals

Remote/network/disk delay should eventually enter the same physical timing model.

Do not block organism time waiting for a remote arena.

Instead:

```text
remote dependency requested
→ other hotspots continue
→ remote block/activity arrives later
→ enqueue arrival into appropriate arena/timing state
```

The actual delay remains physically observable.

## Framebuffers

Input visual history initially needs only double buffering:

```text
previous foveated sample
current foveated sample
→ delta spikes
```

Do not add a large visual-history ring unless a later physical experiment requires it.

Output framebuffer may use ordinary double/triple buffering for rendering consistency; this is not organism memory.

## Experience journal

Production inference experiences may later be appended to a durable journal/ring/log for offline development, but this is runtime bookkeeping outside organism physics.

Do not confuse it with SPIKE timing rings.

---

# 10. REBOL-style distributed runtime philosophy

Do not put a distributed database underneath the organism unless forced.

Prefer a very small runtime-native block/value protocol.

The same durable arena representation should be usable, as far as practical, for:

```text
disk
network
cache
checkpoint
```

Think:

```text
immutable/versioned arena blocks
+
transient spike traffic
+
tiny causal development journal
```

Not:

```text
SQL
distributed query planner
ORM
database schema
```

Potential runtime protocol forms are conceptually:

```text
get arena/version
send spikes
need arenas
commit experience
checkpoint version
```

These are runtime/control protocol messages, not organism-visible semantics.

---

# 11. Distributed from the start

Distribution boundary = arena, not CELL.

A worker owns/hosts hot arena state.

Long-range physical ARROW activity may require another arena.

If it is remote:

```text
request arena
→ real latency
→ arrival occurs later
```

Do NOT freeze organism time while waiting.

Other hotspots continue.

When the arena arrives, that delay is part of the organism's physical experience.

This is deliberate.

---

# 12. Do not hide hardware latency from the organism

A local structure and a remote structure may have very different effective physical delays.

That is useful.

The organism should be able to learn the machine it inhabits.

Example:

```text
route A
→ useful result in 3 ticks

route B
→ equivalent result in 80 ticks
```

Over life, the organism may learn routes and structures better suited to its computational body.

This is computational proprioception.

The runtime may transparently optimize caching only when behavior is preserved, but it must not virtualize away every meaningful latency difference.

---

# 13. Mechanical locality primitives should be teachable

Provide dumb physical affordances such as:

```text
FETCH
PREFETCH
PIN
UNPIN
EVICT
REPLICATE
MOVE/COPY BLOCK
SEND
WAIT
CANCEL
```

These are allowed because they describe what the computational body can physically do.

Do not provide:

```text
remember important thing
retrieve relevant memory
optimize cache
prefetch likely knowledge
```

Those are policies/intelligence.

The organism should learn:

```text
what to keep nearby
what to fetch early
what to evict
what to replicate
what structures should co-locate
```

through observed delays, capacity pressure, and consequences.

There may still be an invisible behavior-preserving OS/runtime cache beneath this.

Keep the two levels distinct.

---

# 14. Physical robot embodiment later

The same principle extends to a robot.

Do not expose semantic robot actions like:

```text
WALK
GRASP
LOOK_AT_FACE
```

Expose physical channels:

```text
motor current
joint torque
camera movement
gripper tension
speaker/output surface
```

Then the organism discovers what the body can do through physical consequence.

The cognitive architecture must not change when moving from:

```text
computer body
→ simulated body
→ robot body
```

Only the available physical affordances and experience distributions change.

---

# 15. No training/inference distinction inside the organism

The runtime always executes the same physics.

Development:

```text
more structural proposals
more plasticity
more durable changes
more churn
```

Maturity:

```text
more traversal
less churn
more stable routing
more precise semantic foveation
```

Same executable.

No:

```text
train()
backward()
optimizer.step()
```

The world/teacher changes experience, not the learning machinery.

---

# 16. Frozen inference

Production should support a frozen durable organism.

Shared durable body:

```text
read-only arenas
```

Each inference context gets a private transient overlay:

```text
CELL activation
refractory state
participation traces
eligibility
pending spikes / timing-ring buckets
temporary structures
optional query-local plasticity
```

This allows many concurrent inference contexts to share the same huge organism.

Three operational modes may eventually be useful:

```text
LIVE
durable learning enabled

FROZEN
durable graph read-only

FORKED
durable graph read-only
private query-local CoW plasticity
```

FORKED is likely the safest serving mode.

---

# 17. Preserve inference-time learning

For every production inference context, optionally retain:

```text
base organism version
physical input/event stream
world/user responses
outward crossings
timing
arenas touched
CoW structural changes
```

The important durable artifact is the **experience**, not merely the mutation delta.

Do not merge inference deltas directly into the organism.

Offline development should replay experiences through the real physics.

If experience A and B were both observed against V42:

```text
do NOT:
V42 + ΔA + ΔB
```

Instead:

```text
V42
→ replay A
→ organism changes
→ replay B against changed organism
```

because development is path-dependent.

---

# 18. Parallel inference

Parallel inference is:

```text
shared frozen durable body
+
many private transient semantic foveas
```

Batch by hot arena.

If many contexts use the same resident arena, process their active local state in compact SIMD batches.

If contexts use unrelated arenas, schedule them independently across cores/machines.

Parallelism should follow semantic locality.

---

# 19. Development concurrency

A developmental organism must retain one coherent causal life.

Therefore:

> Serial causal semantics, parallel physical execution.

Within one experience:

```text
independent arenas/hotspots may run concurrently
```

Across durable development:

```text
experience histories must have a serializable causal order
```

Do not merge conflicting mutation sets.

Replay conflicting experiences.

---

# 20. Clock model

Keep three clocks separate.

## Physical time

Inside an experience:

```text
tick / phase / queue order
```

This belongs to CELL/ARROW/SPIKE physics.

Timing rings are an implementation of this physical time.

## Development version

A logical monotonic version defining durable organism history:

```text
V42 → experience A → V43 → experience B → V44
```

This is causal developmental order.

## Wall/HLC time

Only for:

```text
logs
operations
distributed tracing
human debugging
```

Never make wall time part of learning semantics.

---

# 21. Tiny causal sequencer instead of a distributed DB

For development, use:

```text
tiny authoritative append-only development journal
+
immutable/versioned arena blocks
```

Parallel workers may speculatively process experiences against the same base version.

If two experiences are independent, their arena changes may commit concurrently under one deterministic causal ordering.

If they conflict:

```text
do not merge mutations
→ replay losing experience against newer organism
```

The serial unit is the **experience**, not the delta.

This is Cockroach/FoundationDB-like serializability, but implemented as a much smaller organism-native runtime.

---

# 22. Runtime learns its own machine

The organism should be able to observe indirectly:

```text
local memory is quick
SSD is slower
remote arena is slower still
network congestion changes delay
cached structure arrives sooner
```

Do not tell it those names.

It experiences only physical timing and consequences.

Over development it may learn:

```text
better routing
prefetch
cache placement
replication
locality
shortcut structure
```

So the organism can adapt its computational anatomy to the hardware environment.

---

# 23. Long-term foveation stack

The final runtime should have several nested foveations:

```text
SENSORY FOVEATION
where physical sensory detail is concentrated

SEMANTIC FOVEATION
where graph activity is concentrated

MEMORY FOVEATION
which arenas are resident

COMPUTE FOVEATION
which active arenas consume SIMD/core time

NETWORK FOVEATION
which remote arenas are prefetched/replicated
```

Only sensory foveation is explicitly geometrical.

The rest should emerge from activity and learned policy.

---

# 24. Hard architectural principle

Do not hide the computational body from the organism.

The body has:

```text
finite memory
finite bandwidth
latency
locality
congestion
device costs
physical actuators
```

These should be learnable facts of its environment.

The organism should eventually become better at using the machine because it has lived in it.

---

# 25. Oracle governance and successor boundary

The PXR0/PX-C phase and Physical Body V1 successor are finished. The authority
pins at the top of this document define the accepted continuous organism and
its current production body.

This oracle governs future work as follows:

```text
current authority
    truelearner-core + truelearner-arena-format
    unchanged PX-C physical transition laws
    stable identity independent of resident slots
    canonical immutable body blocks
    clocked quiescent and live restart
    anonymous physical arrival
    continuous execution to natural quiescence
    outward physical crossings
    zero orchestration seams and guarded semantic surfaces

remaining future intent
    framebuffer/foveated embodiment
    cold arena residence and eviction
    asynchronous and network arena transport
    distributed execution and causal journaling
    organism-visible computational-body affordances
    design input only
    not current behavior
    not implementation authorization

successor change
    explicit user authorization
    frozen protocol and parent
    development evidence
    disjoint authority evidence
    oracle updated only after acceptance
```

No experimental branch, evaluator, primer, optimization, or deployment concern may silently add a cognitive noun, semantic adapter, hidden mechanism invocation, or new substrate law to the active organism.

When explicitly authorized, the next successor program may begin from the
remaining intent in Sections 4–24 and may extract a dedicated design note.
Until then, no further runtime implementation is implied.
