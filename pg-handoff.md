# Academy — Post-R6 Consolidated Handoff

## Current state

R6 Partition Invariance is complete and development-frozen.

Accepted runtime state:

```text
ReferencePhysics
    Vec
    GlobalScan
    FullScan
    AoS
    Scalar

MechanicalConfig::PRODUCTION
    TimingWheel
    Adjacency
    Frontier
    AoS
    Opportunistic exact batching
```

R6 additionally established:

```text
ArenaId
    = durable identity/content unit

ResidentArenaId
    = disposable execution placement
```

At zero admitted latency, changing resident partitioning does not change organism physics.

R6 evidence:

```text
partition comparisons       36 / 36
checkpoint controls          2 / 2
total clauses               38 / 38
prior R1-R5 differential    80 / 80
accepted clauses           536 / 536
exact replay                PASS
natural quiescence          PASS
strict Clippy               PASS
```

Do **not** begin R7 yet.

The next priority is to make development visible and scientifically inspectable through Academy.

---

# 1. Correct the current Academy status

The existing Playground is useful, but it is **not yet the full Academy V0**.

Freeze what exists as:

```text
A0 — Playground Shell + Physical Boundary
```

A0 genuinely includes:

```text
Playground → AcademyCore → TrueLearner dependency separation

Rust-owned raster input/output surfaces

text/image/file/drawing admission

bounded worker channel
TrueLearner off UI thread

physical input recording
body fingerprints
physical work
quiescence

deterministic replay of most recent interaction

capability catalog
capability evidence storage

history/inspector shell

same-process live checkpoint restore

no Academy semantics inside TrueLearner

no foveation semantics inside TrueLearner
```

Do not continue claiming the previous “V0 20/20” acceptance.

The developmental Academy remains incomplete.

---

# 2. Immediate roadmap

Use this order:

```text
A0    Playground shell + physical boundary          DONE

A1    Genuine teaching + fresh probing
A1-V  Episode recording/replay/video

A2    Curriculum graph + frontier + retention
A3    Development velocity + persistent history
A4    Genuine shared raster world
A5    Ready-world compatibility spike

then

R7    explicit non-residence/storage latency
R8    transport
F0    foveation curriculum/world
```

Academy and runtime engineering remain separate programs.

No Academy milestone advances TrueLearner physical-law authority.

---

# 3. Core architectural separation

Maintain:

```text
TRUELEARNER
    CELL / ARROW / SPIKE physical organism

ACADEMY
    curriculum
    teaching cases
    probes
    capability evidence
    developmental records
    evaluation

PLAYGROUND
    human-facing Dioxus UI
```

Dependencies:

```text
playground → academy-core → truelearner

truelearner ✗→ academy
truelearner ✗→ playground
academy-core ✗→ Dioxus
```

The Playground may be semantic for humans.

Academy may be semantic for teaching/evaluation.

TrueLearner remains semantic-free physical machinery.

---

# 4. Dioxus and raster boundary

Keep Dioxus Desktop as the Playground shell.

Dioxus is appropriate for:

```text
chat
rich messages
files
images
dashboards
history
development visualizations
```

But Dioxus must not become part of TrueLearner.

The canonical organism-facing visual world remains raster:

```text
VisualSurface
    width
    height
    RGBA pixels
```

Conceptual boundary:

```text
HTML / Dioxus / document renderer / simulator
                    ↓
              raster framebuffer
                    ↓
             physical sensory boundary
                    ↓
                 TrueLearner
```

HTML is a renderer.

Canvas is a possible compositor/display mechanism.

Raster is the organism-facing visual truth.

Do not depend on experimental HTML-in-Canvas APIs for V0. The architecture should permit adopting them later without changing AcademyCore or TrueLearner.

The Rust-owned framebuffer remains canonical.

---

# 5. A1 — Genuine Teaching and Fresh Probing

This is the immediate implementation gate.

The current Probe path is scientifically invalid because text submission is also used as its expected output.

Remove this behavior.

A Probe must never amount to:

```text
input = "abc"
expected = "abc"
```

unless the capability being explicitly tested is copying/echoing.

Likewise, stop hardcoding all text interactions to fixed capability IDs such as:

```text
interaction-response
copy-symbol
```

Capability evidence must correspond to the capability actually being taught/tested.

---

# 6. TeachingCase and ProbeFamily

Academy should explicitly represent a teachable external hypothesis.

Conceptually:

```rust
TeachingCase {
    capability_id,
    seed,
    teaching_world,
    teaching_experiences,
    consequence_policy,
}

ProbeFamily {
    same_structure,
    reverse,
    distractor,
    delayed,
    transfer,
}
```

Exact API is not prescribed.

These types belong entirely in Academy.

TrueLearner never receives:

```text
capability_id
expected_answer
correct
wrong
reward
loss
```

---

# 7. First genuine teaching example

Support something equivalent to:

```text
show novel visual object X
+
present novel symbol "dax"
+
ordinary physical interaction/consequence
```

Later generate independently:

```text
Probe 1
    same learned relationship
    different position/context

Probe 2
    "dax"
    → identify corresponding visual thing

Probe 3
    visual thing
    → produce/identify "dax"

Probe 4
    distractors

Probe 5
    unrelated intervening experiences
    → retention

Probe 6
    changed visual presentation
    → transfer
```

The teaching and probe instances must not be the same physical fixture.

---

# 8. A1 negative controls

A1 must prove that trivial strategies fail.

At minimum:

```text
echo control
fresh identity control
remapped identity control
distractor control
wrong-context control
```

A naive echo strategy must fail at least one required negative control.

Fresh identifiers and generated contexts should be used aggressively.

Do not advance a capability merely because the organism repeated the latest visible symbol.

---

# 9. A1 acceptance gates

A1 is development-positive only when:

```text
1. No general Probe uses submitted input as its own expected answer.

2. Teaching cases create a genuine physical relationship/consequence.

3. Probe instances are generated independently from teaching instances.

4. Fresh identities/contexts are used.

5. Probe execution does not teach unless deliberately configured.

6. Evidence is attached to the capability actually tested.

7. Text capabilities can pass/fail genuine probes.

8. Raster/visual capabilities can pass/fail genuine probes.

9. Echo/memorization controls can fail.

10. Fresh/remapped negative controls exist.

11. Academy can show the exact evidence causing a state transition.

12. TrueLearner receives no privileged evaluator semantics.
```

Until this is earned, do not present capability status as genuine mastery.

---

# 10. A1-V — Episode Recorder

Implement this alongside A1.

Every Teach, Probe, Transfer and Retention experience should become something we can **watch**.

The authoritative artifact is not a screen recording.

It is a deterministic recorded experience from which a human-viewable episode can be rendered.

Record:

```text
experience ID

body/checkpoint before
body/checkpoint after

physical clock range

admitted physical input stream
WorldSurface changes
organism physical outputs
crossings
PhysicalWork

Academy mode
capability being investigated
generator seed
Academy result/evidence
```

---

# 11. Three episode views

Support deriving three visual views from one ExperienceRecord.

## Organism View

Show exactly the physical visual surface the organism received.

No observer annotations.

This must answer:

> What did the organism actually see?

## Shared World View

Show the ordinary raster environment in which human/world and organism acted.

## Observer View

Show the same episode with causally inert annotations such as:

```text
Teach / Probe / Transfer / Retention
capability
tick
phase
work
body version
crossings
future gaze marker
future active arenas
result
```

Observer annotations must never feed back into the organism-visible framebuffer.

---

# 12. Video/replay output

From an ExperienceRecord:

```text
ExperienceRecord
      ↓
deterministic replay/render
      ↓
frames
      ↓
preview/video
```

Produce a human-viewable format such as MP4/WebM where practical.

Do not use WebView repaint timing or arbitrary UI screen recording as scientific evidence.

UI screen recording may exist for demos, but the replay-derived episode is the inspectable artifact.

Conceptual episode bundle:

```text
episodes/<experience-id>/
    metadata
    physical record
    before checkpoint/ref
    after checkpoint/ref
    preview/video
    thumbnail
```

Exact encoding may differ.

---

# 13. Development timeline playback

History should evolve from a log into a filmstrip:

```text
V63  Teach novel binding      [▶]

V64  Fresh probe              [▶] PASS

V65  unrelated experience     [▶]

V66  Retention @ 30           [▶] PASS
```

Clicking `[▶]` must replay that historical episode—not merely the latest interaction.

Eventually support synchronized comparison:

```text
EARLY EPISODE              LATER EPISODE

work 941                   work 183
ticks 312                  ticks 71

        synchronized playback
```

This will make automaticity visible.

Hard principle:

> Every claim that TrueLearner learned something should be traceable to experiences we can replay and watch.

---

# 14. A2 — Real capability graph

Once A1 is positive, implement the actual Academy curriculum engine.

Each capability should eventually define:

```text
id
prerequisites

teach generator
probe generator
transfer generator

retention schedule
mastery criteria
```

Capability states:

```text
UNKNOWN
EMERGING
ACQUIRED
GENERAL
STABLE
AUTOMATIC
```

These are external Academy labels derived from evidence.

Never organism introspection.

---

# 15. Genuine frontier

The current stable/frontier count is not enough.

Implement an actual frontier policy.

Conceptually:

```text
current evidence
      ↓
prerequisites satisfied?
      ↓
frontier candidates
      ↓
choose next developmental action
```

Start deterministic.

Suggested initial policy:

```text
1. due retention review
2. weak prerequisite
3. frontier probe
4. frontier teaching
```

Do not add a learned curriculum optimizer yet.

The Academy scheduler itself can be optimized later.

---

# 16. Retention scheduling

Schedule mainly by **developmental distance**, not wall time.

Suggested initial distances:

```text
10 intervening experiences
30
100
300
1000
```

Potential later distance measures:

```text
physical work
spikes
body versions
```

Wall time is operational metadata, not developmental semantics.

---

# 17. Interleaving

Avoid curriculum blocks such as:

```text
AAAAAAAA
BBBBBBBB
CCCCCCCC
```

Prefer:

```text
A
C
A
B
D
A
C
E
```

Use fresh generated identities and contexts.

The goal is stable physical regularity, not memorization of curriculum ordering.

---

# 18. A3 — Development velocity

Development velocity is a primary research output, not polish.

For every capability track:

```text
experiences → first success

experiences → acquisition

experiences → generalization

experiences → stability

fresh probe success

transfer success

retention success

early physical work / success

current physical work / success

early ticks / success

current ticks / success

durable body growth
```

And globally measure:

```text
new capabilities acquired / N experiences

new stable capabilities / physical work

median examples-to-acquisition

transfer ratio

retention ratio

automaticity improvement

durable bytes per acquired capability
```

---

# 19. Learning-to-learn metric

Explicitly track whether comparable new capabilities become cheaper to acquire.

Example:

```text
first comparable novel capability      80 experiences
later                                  19
later still                             4
```

This distinguishes:

```text
organism knows more
```

from:

```text
organism has become better at learning
```

This should eventually be one of the headline Academy graphs.

---

# 20. A3 persistence

Current `Option<Vec<u8>>` same-process checkpoint storage is insufficient.

Academy development must survive app restart.

Persist at least:

```text
current body/checkpoint reference

experience records

capability evidence

curriculum/frontier position

retention schedule

development timeline
```

A lightweight file/journal implementation is sufficient initially.

Do not introduce a database unless evidence demands one.

---

# 21. Historical replay

Every persisted experience should remain independently replayable.

Do not make Replay mean:

```text
replay latest input
```

It must mean:

```text
replay selected historical ExperienceRecord
```

A replay should distinguish:

```text
physical replay
observer rendering
Academy re-evaluation
```

where appropriate.

---

# 22. A4 — Genuine shared raster world

The current separate human surface and decorative organism crossing raster are sufficient for A0 only.

Do not call the decorative crossing path “organism drawing.”

Move toward one canonical:

```text
WorldSurface
```

Both human and organism can physically affect it.

Conceptually:

```text
Human action
    ↓
WorldSurface changes
    ↓
organism perceives

Organism action
    ↓
WorldSurface changes
    ↓
organism perceives consequence
    ↓
human sees same world
```

This closes the action/perception loop.

---

# 23. Primitive drawing actions

When exposing drawing to the organism, expose physical affordances only.

Good:

```text
move cursor
pen up
pen down
short stroke
pixel/patch change
```

Bad:

```text
DRAW_CIRCLE
DRAW_TRIANGLE
DRAW_FACE
```

Drawing intelligence must be learned.

The canonical result remains raster.

---

# 24. Documents/files

Images:

```text
decode externally
→ raster
→ WorldSurface
```

Documents:

```text
PDF / Markdown / code / spreadsheet
→ external renderer
→ raster viewport
→ WorldSurface
```

Later expose mechanical controls:

```text
scroll
zoom
pan
next page
previous page
```

Do not provide semantic operations such as:

```text
find relevant paragraph
summarize
retrieve answer
```

inside TrueLearner.

---

# 25. A5 — Ready-world compatibility spike

After the Academy has genuine A1 teaching/probing and the Playground shell is usable, actually run the candidate embodied worlds we previously identified.

Do not choose only from documentation.

The required output is:

```text
academy/docs/world_compatibility_spike_v1.md
```

---

# 26. MiniWorld

Actually install and execute MiniWorld.

Exercise at least:

```text
OneRoom
RoomObjects
Sign
ThreeRooms
```

Verify:

```text
reset
RGB observation
camera/agent actions
successive frame capture
headless operation if possible
seed determinism
step throughput
custom world effort
```

Measure:

```text
installation friction
startup time
frame format
camera control
action API
seed reproducibility
throughput
dependency burden
customization difficulty
```

Working hypothesis:

```text
MiniWorld = first foveation wind tunnel
```

But actual results may overturn this.

---

# 27. ProcTHOR / AI2-THOR

Actually install/run a minimal world.

Verify:

```text
load/sample house
obtain RGB frame
camera/agent movement
basic interaction
reset/reproduction
raster integration
```

Record additionally:

```text
scene-generation cost
asset/download size
Unity/process complexity
procedural variation
camera/gaze independence
```

Working hypothesis:

```text
ProcTHOR = richer developmental environment
```

---

# 28. Habitat

Perform a minimal Habitat-Sim rendering spike.

Verify:

```text
load one scene
RGB frame
sensor orientation
agent movement
headless execution
rough throughput
```

Do not spend excessive time downloading datasets for the spike.

Working hypothesis:

```text
Habitat = larger-scale embodied validation
```

---

# 29. TAVIS

Inspect and smoke-test if installation cost is reasonable.

Determine:

```text
hardware/GPU requirements
IsaacLab burden
gaze representation
raster compatibility
evaluation metrics
```

Do not make TAVIS a V0 dependency.

Working hypothesis:

```text
TAVIS = later external active-vision validation
```

---

# 30. Common world boundary

Do not write TrueLearner-specific simulator integrations.

Prototype an external Academy-side abstraction conceptually like:

```rust
trait RasterWorld {
    fn reset(&mut self, seed: u64);
    fn frame(&self) -> &VisualSurface;
    fn apply_physical_action(&mut self, action: WorldAction);
    fn step(&mut self);
}
```

Exact API is open.

The invariant architecture is:

```text
WORLD
   ↓
full raster framebuffer
   ↓
future foveated sensor
   ↓
ordinary physical arrivals
   ↓
TrueLearner
```

and:

```text
TrueLearner outward physical activity
   ↓
world adapter
   ↓
world/camera/body action
```

Semantic simulator metadata remains Academy-side.

---

# 31. World selection criteria

Do not select the first foveation world based primarily on realism.

Prioritize:

```text
clean framebuffer boundary

camera/gaze control

deterministic seeds

fast iteration

procedural variation

low semantic leakage

cheap large-scale Academy probing

headless execution

simple deployment
```

Expected comparison table:

```text
World       Runs  Headless  RGB  Gaze  Procedural  Speed  Setup

MiniWorld
ProcTHOR
Habitat
TAVIS
```

---

# 32. Foveation remains future work

Do not implement foveation in A0–A5.

But preserve the path:

```text
full framebuffer
        ↓
gaze position
        ↓
foveated sampling
        ↓
previous/current receptor delta
        ↓
SPIKE activity
```

Only sensory foveation begins geometrically.

Semantic, memory, compute and network foveation must emerge from organism/runtime activity.

---

# 33. Future foveation controls

When F0 begins, expect controls such as:

```text
FULL VISION
    full-resolution everywhere

FIXED FOVEA
    reduced sensory bandwidth
    gaze immobile

ACTIVE FOVEA
    reduced bandwidth
    gaze under organism control
```

Eventually measure:

```text
task success
pixels sampled
sensory bandwidth
gaze movements
physical work
ticks
development experiences
generalization
```

Desired eventual pattern:

```text
ACTIVE FOVEA ≈ FULL VISION success

ACTIVE FOVEA << FULL VISION physical/sensory cost

ACTIVE FOVEA > FIXED FOVEA
```

Do not implement these yet.

---

# 34. Runtime visibility in Playground

R6 now makes body-level instrumentation safe.

The human-facing inspector may show causally inert mechanics such as:

```text
body version
physical tick
pressure phase

durable arenas
resident arenas
active resident arenas

pending activity
cross-arena traversals

crossings
PhysicalWork

body bytes
resident bytes
```

Later R7 can add:

```text
resident
loading
cold
availability ticks
```

Later R8:

```text
local
remote
transport delay
prefetch
```

None of these names enter organism semantics.

---

# 35. Fix current inspector/accounting UI

The current runtime already stores more information than the top strip shows.

Expose, where available:

```text
pending inputs
pending outputs
replay state
backpressure
work categories
resident/arena information
```

Do not claim backpressure evidence if its counter can never currently change.

Instrumentation must distinguish:

```text
PhysicalWork
    invariant physical categories

ExecutionCost
    implementation-specific mechanics
```

---

# 36. Engineering cleanup

These are real issues, but secondary to A1 scientific correctness:

```text
modal focus handling
Escape close
focus restoration

aria-pressed for Teach/Probe
aria-live status

pointer/touch/keyboard drawing accessibility
larger touch targets

avoid whole-frame PNG/base64 encode/decode on every draw update

replace indefinite 48 ms polling where sensible

bound message history

file size limits

surface errors visibly
```

Do not spend polish effort while Probe remains scientifically self-answering.

Priority:

```text
1. scientific correctness
2. durable use
3. engineering/polish
```

---

# 37. Input determinism

UI timing must not silently enter TrueLearner physics.

Do not let:

```text
WebView repaint timing
mouse scheduling jitter
async task completion ordering
worker polling interval
```

define organism history.

Human/world events must be explicitly admitted to organism physical time.

Every admitted physical input must be recordable and replayable.

---

# 38. No R7 contamination

While building Academy A1–A5, do not accidentally implement R7 concepts inside the Playground.

R7 remains a separate future runtime experiment concerning:

```text
non-resident structure
explicit load request
host completion
deterministic availability tick
resumption of waiting activity
```

Academy may display such mechanics later.

Academy must not define them.

---

# 39. Immediate implementation instruction

Start from the current A0 Playground.

Do not rewrite the shell unnecessarily.

First implement:

```text
A1 Genuine Teaching + Fresh Probing
+
A1-V Episode Recording/Replay
```

Stop and report once these are development-positive.

Do not proceed directly into A2 without an explicit review.

The report must clearly show:

```text
teaching cases used
fresh probe families
negative controls
capability evidence transitions
physical work
body changes
deterministic experience replay
human-viewable recorded episode
```

The key question for the next gate is:

> Can Academy teach a genuinely novel relationship through ordinary physical experience, later test it with independently generated evidence, and let us replay and visually inspect exactly what happened?

If yes, then we finally have the beginning of the **Academy**, rather than only the Playground shell.
