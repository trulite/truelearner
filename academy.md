# TrueLearner Playground + Academy

*A multimodal developmental interface, curriculum engine, and evidence system*

> The Playground is where people interact with the organism. The Academy is the external developmental teacher and evaluator. TrueLearner itself remains only physical law: CELL, ARROW, SPIKE and their accepted local dynamics.

## Status

Forward product/runtime design. It does not add learning mechanisms to the organism and does not authorize changes to PX-C physical law.

## Design goal

Let a person talk, teach, upload images and files, draw, and watch the organism develop — while a rigorous external system continuously measures what it has actually acquired, generalized, retained, and made cheaper.

# 1. System boundary

The product has four conceptual layers. Only one of them is the organism.

```text
PLAYGROUND
human-facing conversation and shared workspace
↓
ACADEMY
curriculum, probes, capability evidence, developmental scheduling
↓
BOUNDARY / WORLD
turns semantic UI objects into physical surfaces and consequences
↓
TRUELEARNER
CELL / ARROW / SPIKE physical organism
```

> **Invariant** Semantics may exist freely in Playground and Academy. They must not leak into TrueLearner orchestration or physical state.

# 2. Playground: a multimodal conversation surface

The public experience should look familiar: a chat with attachments. Internally it is a shared perceptual world, not a collection of privileged understanding APIs.

## Inputs
- Text messages.
- Images and screenshots.
- Files such as PDFs, documents, spreadsheets and code.
- A shared drawing/canvas surface.
- Later: camera, audio, video and robot sensors.

Where practical, the first embodiment can reduce these to visual surfaces:

```text
text → rendered glyphs / pixels
image → visual surface
PDF → navigable page viewport
spreadsheet → navigable grid viewport
code → rendered text surface
```

The boundary may know that something is a PDF or image so it can provide physical navigation. TrueLearner does not receive a semantic Document, Image, Paragraph, Object or Query type.

## Outputs
- Text/glyph output for practical early conversation.
- A drawing framebuffer or canvas.
- Pointing, marking and selection on the shared workspace.
- Later: document manipulation, audio, network actions and robot actuation.

> Text output can initially be an explicit embodiment affordance. We do not need to force the earliest organism to rediscover typography from pixels before people can interact with it.

# 3. Shared workspace

Chat should gradually become a shared world the organism can inspect and act on.

```text
┌──────────────────────────────┬──────────────────────────────┐
│ Conversation │ Shared workspace │
│ │ │
│ You: look at this │ [image / page / drawing] │
│ [uploaded file] │ │
│ │ organism may gaze, point, │
│ Organism: ... │ draw, scroll or zoom │
└──────────────────────────────┴──────────────────────────────┘
```

Files should become manipulable physical objects. A PDF is not injected as parsed knowledge; it can be presented as pages with affordances such as scroll, zoom, next page and previous page. The organism can eventually learn when those actions are useful.

# 4. Academy: the external developmental system

Academy is the analogue of an adaptive learning system. It is allowed to reason in semantic capability terms because it is outside the learner.

```text
capability graph
↓
estimate developmental frontier
↓
choose teaching experience / probe / review
↓
organism lives through physical world
↓
observe behavior and physical cost
↓
update capability evidence
↓
advance, interleave, remediate or retest
```

## Capability graph

Each node is an external claim we want evidence for, not an internal module. The graph records prerequisites and the next plausible developmental frontier.

```text
turn taking
├─ copy symbol ─────────┐
└─ distinguish symbols ┤
↓
novel binding
↓
delayed recall
↓
simple naming
↓
relational language
↓
conversation
```

## Capability definition

```text
id: novel_binding
prerequisites:
- distinguish_symbols
- delayed_recall
teach: generated novel-binding worlds
probe: fresh held-out worlds
transfer: altered context / timing / distractors
retention: re-probe after intervening experience
controls: fresh identities, remapping, negative worlds
```

The organism never sees the capability name, expected answer, mastery threshold or prerequisite graph.

# 5. Teach, probe and transfer are different worlds

Every capability should have at least three generators.
- **TEACH —** experiences intentionally available during acquisition.
- **PROBE —** fresh same-structure experiences used only as evidence.
- **TRANSFER —** structurally related experiences with changed surface form, context, timing or distractors.

Static test sets are dangerous because the organism may eventually learn the fixture. Identities, placements, timings and irrelevant details should be regenerated aggressively.

# 6. Developmental frontier

Academy should spend most teaching effort near the boundary between stable capability and genuine novelty.

```text
STABLE
✓ copying
✓ short delayed binding
✓ simple sequence continuation
FRONTIER
◐ novel binding
◐ two-step composition
◐ naming across context
NOT READY
○ relational questions
○ long reference chains
○ open-ended planning through language
```

Too-easy worlds mostly rehearse. Impossible worlds provide poor developmental evidence. The frontier is where small amounts of experience can plausibly produce new verified capability.

# 7. Mastery is multidimensional

Accuracy alone is not enough. Each capability should accumulate an evidence profile.

| **Measure**         | **Question**                                                                                 |
|---------------------|----------------------------------------------------------------------------------------------|
| **Acquisition**     | How many experiences or units of physical work were required before the capability appeared? |
| **Generalization**  | Does it work on genuinely unseen identities and worlds?                                      |
| **Retention**       | Does it survive unrelated intervening experience?                                            |
| **Automaticity**    | Does the same behavior become cheaper in ticks, work and active structure?                   |
| **Robustness**      | Does noise, timing variation or context change break it?                                     |
| **Reversibility**   | Can it adapt when the governing relation changes?                                            |
| **Structural cost** | How much durable body growth was required?                                                   |

## Mastery levels
- UNKNOWN — no meaningful evidence.
- EMERGING — succeeds inside the teaching distribution.
- ACQUIRED — succeeds on fresh same-structure probes.
- GENERAL — transfers to altered worlds.
- STABLE — survives spaced probes and interference.
- AUTOMATIC — remains stable while physical cost falls substantially.

## Experience-distance retention

Review should initially be scheduled by developmental distance, not human wall time.

```text
learn capability
→ 10 unrelated experiences → probe
→ 30 unrelated experiences → probe
→ 100 unrelated experiences → probe
→ 300 unrelated experiences → probe
```

Physical work or spike count can also be used as the spacing variable. Successful retention expands the interval; failure reopens relevant prerequisites or teaching worlds.

# 8. Development velocity is the headline metric

The purpose of Academy is not only to show what the organism knows. It should show whether the organism is becoming better at learning.
- Capabilities acquired per 1,000 experiences.
- Capabilities stabilized per unit of physical work.
- Median examples-to-acquisition.
- Transfer ratio.
- Retention ratio.
- Automaticity improvement.
- Durable bytes or arrows per acquired capability.

> **Key test** The killer graph is acquisition cost for comparable new capabilities over development. If it falls sharply while transfer controls remain hard, the organism may be learning how to learn.

```text
early comparable capability 80 experiences
later comparable capability 18 experiences
later still 4 experiences
```

# 9. Multimodal developmental curriculum

The first public curriculum should be small enough to understand exhaustively. It can expand quickly as the organism demonstrates stable prerequisites.

## A. Interaction
- activity causes response
- turn taking
- repeat / copy
- two-way distinction
- simple continuation

## B. Binding
- associate two novel symbols
- short delayed recall
- retrieve either direction
- fresh identity binding
- replace an old binding

## C. Sequence
- continue simple pattern
- continue novel pattern
- multi-symbol copying
- delayed continuation

## D. Visual grounding
- distinguish simple shapes
- same thing after position change
- copy a shape
- word ↔ displayed object
- choose named object among distractors

## E. Composition
- use two learned bindings together
- simple visual relation
- two-step instruction
- description → drawing

## F. Conversation
- answer about visible context
- remember previous turn
- refer to earlier object
- learn a fact from a user
- retrieve or revise it later

## G. Documents
- find visible information
- scroll to locate target
- remember across pages
- compare two visible pages
- use a diagram or table

# 10. No privileged correctness channel

Academy may know the expected behavior for scheduling and evaluation. That evaluator knowledge must not become a reward, loss, CORRECT/WRONG bit or semantic control signal inside TrueLearner.

```text
organism acts
→ world changes
→ user/world produces ordinary consequence
→ organism experiences that consequence
Academy separately records:
PASS / FAIL / evidence strength
```

The teacher may design worlds rich enough to create useful physical contrast. The learner still changes only through its accepted physical mechanisms.

# 11. Chat should expose evidence, not introspection

A second panel should answer “what did it understand?” using hidden external probes rather than trusting self-report.

```text
UNDERSTANDING
Shape copying AUTOMATIC
Word ↔︎ object binding STABLE
Reading simple labels GENERAL
Drawing from instruction ACQUIRED
Cross-page memory EMERGING
Development frontier
→ composition
→ three-turn reference
→ document navigation
```

The organism can still talk about itself if it learns to, but Academy evidence remains the authoritative developmental measurement.

# 12. User modes

The interface can stay simple while supporting three distinct intents.

| **Mode**    | **Purpose**                                                                                                                          |
|-------------|--------------------------------------------------------------------------------------------------------------------------------------|
| **Chat**    | Ordinary multimodal conversation. Academy watches for useful evidence but does not interrupt aggressively.                           |
| **Teach**   | The user deliberately teaches a word, object, rule or skill. Academy schedules fresh probes later and reports acquisition/retention. |
| **Inspect** | Shows capability evidence, frontier, retention, automaticity and recent developmental changes. Hidden probes can be run on demand.   |

# 13. Playable V0: 1–2 day target

The first release should prove the developmental loop, not the full future embodiment.

## Must have
- Chat UI with text, image and file upload.
- Shared image/canvas area.
- Organism output as text plus a simple bitmap/drawing surface.
- A boundary adapter that presents uploaded material through physical surfaces.
- A small capability graph (roughly 20–30 initial capabilities).
- Generated teach/probe/transfer worlds for the easiest capability families.
- Fresh-identity controls and negative worlds.
- Capability panel with mastery level and frontier.
- Development-velocity counters: acquisition cost, transfer, retention and physical work.
- Experience log sufficient to reproduce Academy evidence.

## Explicitly not required for V0
- General document understanding.
- Full natural-language competence.
- Production distributed arena transport.
- Learned caching and prefetch policy.
- Robot embodiment.
- A large hand-authored ontology of intelligence.

# 14. Suggested package boundary

```text
truelearner/
physical organism and production body
academy/
curriculum/
worlds/
probes/
evaluator/
scheduler/
evidence/
playground/
chat/
uploads/
canvas/
capability-panel/
academy, playground → truelearner
truelearner ✗→ academy, playground
```

Do not force this into many crates immediately. The important boundary is dependency direction and semantic separation, not folder aesthetics.

# 15. Acceptance principles

1.  A user can chat, upload an image/file and receive text or drawn output.

2.  Uploaded content reaches the organism through declared physical boundary surfaces, not semantic embeddings or hidden parsed knowledge.

3.  Academy can teach and evaluate a capability without adding a mechanism or semantic field to TrueLearner.

4.  Every claimed capability has fresh probes and at least one transfer or negative control.

5.  Capability evidence distinguishes acquisition, transfer, retention and automaticity.

6.  The UI can answer “what did it understand?” from external evidence rather than self-report.

7.  Curriculum scheduling is adaptive to the developmental frontier.

8.  Teaching is interleaved enough to resist fixture and temporal memorization.

9.  Development velocity can be plotted across body versions.

10. Comparable capability acquisition cost is tracked to detect learning-to-learn.

11. No evaluator-only semantic signal enters CELL/ARROW/SPIKE physics.

12. The public product remains useful and understandable even when the organism knows almost nothing.

# 16. Governing principles

> Playground makes the organism easy to live with. Academy makes its development measurable. Neither is allowed to become part of the learner.

The long-term product is not “a chatbot with tests.” It is a developmental environment in which people can teach, converse, show, draw, upload and manipulate things while the Academy continuously asks a stricter question: what new capability has this organism actually earned?
