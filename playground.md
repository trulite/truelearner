# Academy Playground — Post-R6 Requirements Spec

## Status and scope

Implement this **after R6 Partition Invariance is complete and frozen**.

This work does **not** modify the accepted TrueLearner learning law.

The invariant organism remains:

```text
CELL
ARROW
SPIKE

Drive
Modulation
Traversal
Trace
Eligibility
Resistance
Pressure
Proposal
Crossing
Quiescence
```

The purpose of this work is different:

> Build the human-facing developmental environment in which we can teach TrueLearner, interact with it, see what it appears to understand, and measure whether it is actually developing.

The Playground is not part of organism cognition.

The Academy is not part of organism cognition.

They are external world, teacher, evaluator, and instrumentation.

---

# 1. Architecture

Use three conceptual layers:

```text
TRUELEARNER
    physical organism

ACADEMY
    curriculum
    capability evidence
    probes
    developmental history
    diagnostics

PLAYGROUND
    human-facing multimodal environment
```

Dependency direction:

```text
playground → academy → truelearner

truelearner ✗→ academy
truelearner ✗→ playground
```

Do not introduce semantic curriculum concepts into TrueLearner.

---

# 2. UI technology

Use **Dioxus Desktop** for the first Playground.

Reasons:

* Rust application and TrueLearner remain native.
* Excellent fit for chat-style interfaces.
* HTML/CSS is appropriate for rich messages, files, dashboards and development panels.
* Easier path to a future browser version.
* The UI can evolve quickly without coupling TrueLearner to the frontend.

Do not put Dioxus types inside `truelearner`.

Suggested structure:

```text
truelearner/
    crates/
        core/
        arena-format/
        ...

academy/
    crates/
        academy-core/
        playground/
```

`academy-core` must remain headless.

`playground` owns Dioxus.

---

# 3. Primary visual world is raster

The organism-facing visual boundary is a **canonical raster framebuffer**.

Do not make SVG, DOM, HTML, widgets, PDF structure or file metadata the organism's visual representation.

Conceptually:

```rust
VisualSurface {
    width,
    height,
    rgba_pixels,
}
```

The exact Rust API may differ.

The principle must not.

Everything visual eventually becomes pixels before entering the organism.

---

# 4. HTML is a renderer, not the perceptual representation

The Playground may use rich HTML/CSS internally:

```text
chat bubbles
markdown
tables
file cards
code
document viewers
controls
```

But TrueLearner must not receive:

```text
Paragraph
ImageElement
Heading
Button
Document
ChatMessage
```

It sees the rendered surface.

Conceptually:

```text
Dioxus DOM
    ↓
render/composite
    ↓
raster framebuffer
    ↓
sensory boundary
    ↓
TrueLearner
```

---

# 5. HTML-in-Canvas future path

There is emerging browser work around rendering HTML content into canvas-like surfaces.

Design so this can be adopted later, but **do not depend on it for V0**.

Current architectural rule:

```text
HTML
    = world/UI renderer

Canvas
    = possible compositor/display mechanism

Framebuffer
    = organism-facing physical visual surface
```

If HTML-in-Canvas becomes broadly supported later, it should be possible to replace the renderer without changing TrueLearner or Academy.

Do not make V0 depend on experimental browser support.

---

# 6. Canvas implementation

Use a raster canvas/widget in the Playground for the shared perceptual workspace.

Canonical pixel state should be owned by Rust.

Browser/WebView Canvas may be used to display and edit it.

Conceptually:

```text
Rust framebuffer
       ↕
Canvas display/edit surface
       ↓
human sees world
       ↓
TrueLearner receives raster changes
```

Canvas is an implementation detail.

Framebuffer semantics belong outside Dioxus.

---

# 7. Playground layout

Initial UI can use three major regions:

```text
┌────────────────────────────┬──────────────────────┐
│ CONVERSATION / WORLD       │ ACADEMY              │
│                            │                      │
│ human messages             │ capability frontier  │
│ organism messages          │ mastery states       │
│ images                     │ retention            │
│ files                      │ transfer             │
│ drawings                   │ physical work        │
│                            │ developmental stats   │
├────────────────────────────┴──────────────────────┤
│ DEVELOPMENT TIMELINE                              │
└───────────────────────────────────────────────────┘
```

Do not over-design V0.

Function is more important than polish.

---

# 8. Multimodal conversation

The Playground must support:

```text
text input
image upload
file upload
human drawing
organism text output
organism drawing output
```

Design the interface so later it can add:

```text
audio
video
camera
simulated worlds
robot embodiment
```

without changing the Academy model.

---

# 9. Text input

The human types normal text using ordinary UI controls.

For organism perception, text should eventually be renderable into the shared visual surface.

For early V0, a direct byte/glyph physical boundary may be retained if required to make the system playable quickly.

If used, mark it explicitly as an **embodiment affordance**, not a semantic language mechanism.

Do not embed tokenization or language concepts into the physical learner.

---

# 10. Images

Uploaded images should become raster world surfaces.

Flow:

```text
image file
→ decode outside TrueLearner
→ raster surface
→ display in Playground
→ sensory input
```

TrueLearner should not know:

```text
JPEG
PNG
image metadata
object labels
```

unless those are physically presented to it.

---

# 11. Documents and files

Files belong to the world.

Examples:

```text
PDF
text document
code
spreadsheet
markdown
```

The Playground may understand file formats in order to render them.

TrueLearner should normally experience rendered content.

Example:

```text
PDF
→ page renderer
→ raster page
→ viewport
→ TrueLearner
```

Later expose physical document affordances:

```text
scroll
next page
previous page
zoom
pan
```

These are mechanical abilities the organism can learn to use.

Do not provide semantic operations like:

```text
find relevant paragraph
retrieve answer
summarize document
```

---

# 12. Shared canvas

Provide a common visual workspace where:

```text
human can draw
organism can draw
images can appear
documents can appear
future simulated worlds can appear
```

The human and organism should increasingly share the same visible world.

This becomes the bridge to later embodiment.

---

# 13. Organism drawing

Organism drawing should ultimately modify a raster output surface.

Do not require SVG semantics.

Potential early physical output affordances:

```text
set pixel
draw short stroke
move drawing cursor
pen up/down
```

Do not expose:

```text
DRAW_CIRCLE
DRAW_TRIANGLE
DRAW_FACE
```

Those are semantic actions.

The organism should learn how physical drawing operations produce visible outcomes.

---

# 14. Causally inert overlays

The Playground may draw debugging overlays using DOM/SVG/Canvas:

```text
gaze position
fovea boundary
active regions
Academy labels
probe indicators
performance measurements
```

These must be causally inert by default.

They must **not** enter the organism-visible framebuffer unless intentionally configured as part of the world.

Keep:

```text
human/debug view
```

separate from:

```text
organism visual surface
```

---

# 15. Academy capability graph

Academy maintains an external capability graph.

Example:

```text
interaction
    ↓
copying
    ↓
symbol distinction
    ↓
novel binding
    ↓
delayed binding
    ↓
composition
    ↓
conversation
```

This graph describes **our evidence about development**.

It is not a representation inside the organism.

---

# 16. Initial capability states

Use approximately:

```text
UNKNOWN
EMERGING
ACQUIRED
GENERAL
STABLE
AUTOMATIC
```

These are Academy labels only.

They should be derived from evidence, not from organism introspection.

---

# 17. Each capability requires multiple evidence modes

At minimum:

```text
TEACH
    examples allowed to change organism

PROBE
    fresh held-out examples

TRANSFER
    structurally related but changed world

RETENTION
    test after intervening experience

AUTOMATICITY
    measure falling physical cost
```

A successful chat answer is not sufficient evidence of mastery.

---

# 18. Development frontier

Academy should maintain the current frontier:

```text
stable capabilities
frontier capabilities
not-ready capabilities
```

Spend most curriculum activity near the frontier.

Avoid:

```text
repeating already automatic abilities endlessly
```

and:

```text
teaching abilities whose prerequisites are missing
```

This borrows the useful outer-loop idea from systems such as Math Academy:

> maintain a prerequisite graph and continuously work near the learner's current frontier.

The graph remains external to TrueLearner.

---

# 19. Example capability evidence

Clicking a capability should eventually show something like:

```text
Novel binding

teach experiences       8
fresh probes           15 / 16
transfer probes         6 / 8
retention @ 100         PASS

median physical work
early                  411
current                 93

status                 STABLE
```

We want evidence, not labels alone.

---

# 20. Development velocity

The Academy dashboard should make **development speed** highly visible.

Track:

```text
experiences to acquisition
experiences to stable mastery
transfer ratio
retention ratio
physical work per success
durable body growth
ticks per success
```

Most importantly track whether learning itself accelerates.

Example:

```text
comparable novel capability

early organism      80 examples
later organism      19 examples
later still          4 examples
```

This distinguishes:

```text
knows more
```

from:

```text
has become better at learning
```

This is a primary research metric.

---

# 21. Experience distance, not just wall time

Retention/review scheduling should primarily use developmental distance:

```text
10 intervening experiences
30
100
300
1000
```

and possibly:

```text
physical work
spike count
body versions
```

Do not make human wall-clock time the primary developmental clock.

---

# 22. Interleaving

Do not train curriculum blocks as:

```text
AAAAAAAA
BBBBBBBB
CCCCCCCC
```

Prefer mixed experience:

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

Fresh identities and contexts should be generated aggressively.

We want stable physical regularities, not curriculum-fixture memorization.

---

# 23. Human Teach mode

Add a clear interaction concept such as:

```text
Teach
Probe understanding
```

Example:

Human:

```text
This shape is called dax.
```

Later Academy can generate hidden/fresh tests:

```text
same shape elsewhere
same shape with distractors
delayed question
reverse mapping
visual variation
```

Then show:

```text
Immediate       PASS
New context     PASS
Delayed         PASS
Reverse         PASS
Transfer        NOT YET
```

Do not expose Academy's expected answers as privileged reward signals to TrueLearner.

---

# 24. Semantic teacher versus physical organism

Academy may know:

```text
expected answer
capability being tested
whether probe passed
```

TrueLearner must not directly receive:

```text
CORRECT
WRONG
reward
loss
capability_id
```

unless such feedback is physically embodied in the world through ordinary outcomes.

Evaluator semantics remain outside.

---

# 25. Development timeline

Every human interaction should produce a development record.

Conceptually:

```text
ExperienceRecord {
    body_version_before
    physical inputs
    user/world actions
    organism outputs
    body_version_after
    physical_work
    clock range
    Academy annotations
}
```

Exact API can differ.

The semantic annotations remain Academy-side.

---

# 26. V0 capability curriculum

Do not start with English comprehension.

Start with a small, testable capability set.

Suggested first group:

```text
interaction/response

copy simple symbol
distinguish two symbols
repeat short sequence

bind novel symbol A ↔ B
retrieve binding
reverse binding
retain binding after delay
replace binding

simple sequence continuation

visual difference
visual ↔ symbol binding

short conversational recall
```

Approximately 10–20 capabilities is sufficient for V0.

---

# 27. Playground V0 goal

The first useful version should let a person:

1. Start TrueLearner.
2. Type messages.
3. Upload an image.
4. Upload a file.
5. Draw on a shared canvas.
6. Receive text and/or raster output.
7. Teach a simple novel association.
8. Trigger or observe fresh Academy probes.
9. See capability status change.
10. See physical work/ticks/body changes.
11. Restart from a body version/checkpoint.
12. Continue interaction.

This is enough to make development visible.

---

# 28. Do not build foveation yet

The Playground is intentionally built **before** the foveation experiment.

But its raster world contract must make foveation easy to add later.

Future path:

```text
full framebuffer
→ gaze
→ foveated sampling
→ change detection
→ SPIKE input
```

Then MiniWorld/ProcTHOR/robot camera can use the same visual boundary.

---

# 29. Future foveation panel

Design UI space so later Academy can show:

```text
gaze movement
pixels sampled
sensory bandwidth
active arena count
physical work
task success
```

and capabilities such as:

```text
FOV-0 eye motion changes perception
FOV-1 orient to useful change
FOV-2 visual search
FOV-3 ignore distractors
FOV-4 inspect several places
FOV-5 remember inspected location
FOV-6 anticipatory gaze
```

Do not implement these in V0.

---

# 30. UI independence

The architecture must permit:

```text
Dioxus Desktop today

Dioxus Web later

other native frontend later
```

without changing:

```text
TrueLearner
AcademyCore
VisualSurface
ExperienceRecord
CapabilityEvidence
```

Do not allow Dioxus to become the runtime architecture.

---

# 31. Threading

Never run the physical organism on the UI event loop.

Conceptually:

```text
Dioxus UI
    ↕ commands/events

Academy runtime
    ↕

TrueLearner execution worker
```

Long physical runs, probes and rendering must not freeze the UI.

Use bounded channels/backpressure.

Do not introduce unbounded event queues.

---

# 32. Determinism

UI timing must not enter organism physics accidentally.

Human actions are admitted into physical time explicitly.

Do not let:

```text
WebView repaint timing
mouse event scheduling jitter
async task completion order
```

silently define physical history.

The Playground may be nondeterministic.

The admitted physical input stream must be recordable and replayable.

---

# 33. Filesystem boundary

Uploaded/opened files should remain outside TrueLearner until explicitly rendered or otherwise physically exposed.

Avoid leaking paths, MIME metadata, parsed semantic structure or filesystem concepts into the organism unless intentionally presented.

---

# 34. Logging and inspection

Academy/Playground may record rich semantic/debug logs.

TrueLearner core remains semantic-free.

Provide a development inspector capable of showing:

```text
body version
physical clock
pending activity
crossings
physical work
arena counts
durable bytes
recent experiences
Academy capability changes
```

Do not require TrueLearner to explain itself semantically.

---

# 35. V0 repository shape

Start small.

Suggested:

```text
academy/
├── Cargo.toml
└── crates/
    ├── academy-core/
    │   └── src/
    │
    └── playground/
        └── src/
```

Do not create many crates prematurely.

`academy-core` owns:

```text
capability graph
evidence
probes
experience records
curriculum policy
```

`playground` owns:

```text
Dioxus
chat UI
file selection
canvas
rendering
human controls
```

---

# 36. Acceptance gates

V0 is complete only if:

1. TrueLearner has no dependency on Academy or Dioxus.
2. AcademyCore has no dependency on Dioxus.
3. Playground launches as a desktop app.
4. Text conversation works.
5. Image upload works.
6. File upload and basic rendering works.
7. Human raster drawing works.
8. Organism raster output can be displayed.
9. Academy can define and display capabilities.
10. A capability can move through evidence states based on actual probes.
11. Teach and probe experiences are distinguishable in Academy.
12. Probe semantics do not enter TrueLearner.
13. Every admitted physical input can be recorded.
14. A recorded interaction can be replayed deterministically at the physical boundary.
15. Physical work and body-version change are visible.
16. UI activity does not silently alter organism physical time.
17. Debug overlays can be hidden from the organism framebuffer.
18. No foveation semantics are hardcoded into TrueLearner.
19. No cognitive abstractions are reintroduced into Rust orchestration.
20. Existing TrueLearner physical equivalence tests remain green.

---

# 37. Non-goals for V0

Do not implement yet:

```text
MiniWorld
ProcTHOR
foveated vision
GPU rendering
SIMD
distributed Playground
robot body
audio
video
production authentication
cloud multi-user hosting
complex document editing
semantic retrieval
LLM assistance inside TrueLearner
```

The goal is developmental visibility.

---

# 38. Central design rule

Freeze this rule:

> The Playground may be semantic for humans. Academy may be semantic for teaching and evaluation. The boundary turns those semantics into physical experience. TrueLearner itself remains only the physical organism.

And for visual embodiment:

> HTML and UI structure are renderers. The organism-facing visual truth is raster.

The Playground should become the microscope through which we watch the organism develop.

# 39. Ready-world compatibility spike

After Academy Playground V0 is working, but **before implementing TrueLearner foveation**, test the external embodied worlds we already identified.

The purpose is not to train TrueLearner in all of them.

The purpose is to answer:

> Which existing world gives us the cleanest framebuffer + action boundary for the first foveation curriculum?

Test these in order.

## A. MiniWorld — first-choice wind tunnel

MiniWorld is a minimal 3D RL/robotics simulator with a Gymnasium interface and ready environments including `OneRoom`, `RoomObjects`, `Sign`, `ThreeRooms`, `Maze`, `PickupObjects`, and `PutNext`. It installs with `pip install miniworld`.

Actually install and run it.

Verify:

```text
reset world
obtain egocentric RGB frame
step movement / camera actions
capture successive frames
run headless if possible
reset deterministically from seed
```

Exercise at least:

```text
OneRoom
RoomObjects
Sign
ThreeRooms
```

Record:

```text
installation friction
startup time
headless support
frame format/resolution
camera controllability
action API
seed determinism
step throughput
custom-world difficulty
license/dependency burden
```

Do not let reward directly strengthen TrueLearner.

MiniWorld is the leading candidate for the first foveation wind tunnel because it is intentionally small and customizable.

---

## B. ProcTHOR / AI2-THOR — developmental-world candidate

Actually install and run a minimal ProcTHOR/AI2-THOR world.

Verify:

```text
load/sample house
obtain RGB observation
move camera/agent
interact with object if straightforward
reset/reproduce a world
capture frames into our VisualSurface-compatible form
```

ProcTHOR exists specifically to generate diverse interactive 3D environments and supports varied floorplans, objects, materials, lighting, state changes, manipulation and multi-agent interaction. Its published ProcTHOR-10K corpus contains large environmental and object diversity.

Record the same measurements as MiniWorld plus:

```text
scene-generation cost
asset/download size
Unity/process-management complexity
procedural variation controls
whether gaze can be controlled separately from locomotion
```

Do not yet integrate it deeply into Academy.

The question is whether it should become the richer world after MiniWorld.

---

## C. Habitat-Sim — scale/validation candidate

Perform an installation and minimal rendering spike.

Verify:

```text
load one supported scene
obtain RGB frames
configure agent/sensor orientation
step agent
headless execution
measure approximate frame throughput
```

Habitat-Sim is explicitly designed as a high-performance embodied simulator with configurable agents and sensors, and supports multiple 3D scene datasets.

Do not spend significant effort downloading large datasets merely for this spike.

Stop once we can estimate:

```text
integration complexity
sensor flexibility
gaze-control suitability
runtime weight
likely usefulness as a later external validation world
```

---

## D. TAVIS — inspect, do not integrate yet

Inspect and smoke-test only if installation cost is reasonable.

TAVIS is specifically an active-vision benchmark with controllable gaze, procedural ID/OOD splits, and a Gaze-Action Lead Time metric for anticipatory gaze; it is built on IsaacLab and includes released code/data/baselines.

Determine:

```text
whether it runs on our expected hardware
IsaacLab/GPU requirements
how gaze actions are represented
whether observations can pass through our raster boundary
whether its evaluation metrics could later be reused
```

Do not make TAVIS a V0 dependency.

Treat it as a future external benchmark.

---

# 40. Common world-adapter boundary

Do not write TrueLearner-specific code separately for every simulator.

Prototype a tiny external Academy-side abstraction conceptually like:

```rust
trait RasterWorld {
    fn reset(&mut self, seed: u64);
    fn frame(&self) -> &VisualSurface;
    fn apply_physical_action(&mut self, action: WorldAction);
    fn step(&mut self);
}
```

Exact API is not prescribed.

The important contract is:

```text
WORLD
    ↓
raster framebuffer
    ↓
later FoveatedSensor
    ↓
TrueLearner
```

and:

```text
TrueLearner outward physical action
    ↓
world adapter
    ↓
camera/body/world movement
```

World-specific semantic metadata must remain Academy-side.

---

# 41. World-spike deliverable

Produce:

```text
academy/docs/world_compatibility_spike_v1.md
```

with a comparison table:

```text
World        Runs?   Headless   RGB   Gaze   Procedural   Speed   Setup burden
MiniWorld
ProcTHOR
Habitat
TAVIS
```

Also include:

```text
recommended first foveation world
recommended second developmental world
recommended later external benchmark
blocking issues
```

Do not select based on visual realism.

Select primarily for:

```text
clean physical framebuffer boundary
controllable gaze/camera
deterministic seeded worlds
fast iteration
procedural variation
low semantic leakage
ability to run many Academy probes cheaply
```

Expected working hypothesis:

```text
MiniWorld   → first foveation wind tunnel
ProcTHOR    → richer developmental environment
Habitat     → larger-scale embodied validation
TAVIS       → later active-gaze benchmark
```

But the spike must be allowed to overturn this ordering.

---

# 42. Boundary

This world spike does **not** authorize:

```text
foveated sensing
gaze-learning curriculum
new TrueLearner physical laws
reward channels
semantic world APIs inside TrueLearner
```

It merely establishes which existing external world should be attached to the raster Playground once the foveation experiment begins.
