# Playground

```text
human UI -> rendered raster -> physical input -> TrueLearner
    ^                                          |
    |                                          v
inspector <- inert record <- physical output <-+
                    |
                 Academy
```

## Purpose

Playground is the human-facing surface for teaching, conversation, shared
drawing, replay, and inspection. It does not own organism law or capability
judgment.

Read [arch.md](arch.md) for organism authority, [academy.md](academy.md) for
teaching and evidence, and [pg-handoff.md](pg-handoff.md) for current work.

## Ownership

- `truelearner` owns physical input, output, time, and body state.
- `academy-core` owns teaching cases, probes, evidence, and experience records.
- `playground` owns Dioxus, human controls, rendering, files, and views.

Keep `playground -> academy-core -> truelearner`. Do not add reverse
dependencies. Keep Dioxus out of `academy-core` and TrueLearner.

## Physical boundary

Use a Rust-owned RGBA raster as the organism-facing visual truth.

The UI may contain HTML, text, controls, document structure, and file metadata.
Render visible content before admission. Do not pass DOM nodes, paragraphs,
buttons, parsed documents, paths, MIME types, or simulator meaning into the
organism.

Text output may remain an explicit early body affordance. Keep it declared and
outside hidden evaluator channels.

Expose drawing as physical actions such as move, press, release, color, and
width. Do not expose semantic actions such as draw-object or select-answer.

## Interaction

Support:

- text input and output;
- image and file upload;
- rendered document pages with physical navigation;
- a shared raster surface for human and organism action;
- Teach, Chat, and Inspect modes;
- organism-visible, shared-world, and observer views.

Observer overlays may show labels, expected outcomes, work, crossings, body
identity, replay state, and capability evidence. Never composite them into
organism input.

## Execution

- Run TrueLearner away from the UI thread.
- Bound all worker channels and queues.
- Admit human and world events explicitly to physical time.
- Record admission order and content.
- Do not let repaint timing, mouse jitter, task completion order, or polling
  define organism history.
- Run to natural quiescence or a declared bounded stop.
- Keep the warm representative regression loop under 10 seconds.

## Records

For every admitted experience, retain enough to reproduce:

- initial body identity and checkpoint;
- ordered physical input;
- physical output and outcome;
- ticks, work, crossings, and body change;
- quiescence and stop reason;
- Academy annotations outside the physical record;
- exact replay result.

Build human-viewable replay from this record, not from WebView timing or a
screen capture. Keep historical episodes independently replayable.

## Inspector

Show only measurements the runtime actually records:

- body identity and version;
- physical clock and pending activity;
- input and output queue state;
- crossings, work, and durable bytes;
- quiescence and replay equality;
- recent experiences and Academy evidence.

Distinguish zeros from unavailable measurements. Do not claim backpressure,
latency, or storage behavior that cannot occur in the current runtime.

## V0 gate

V0 passes only when:

1. The dependency direction is clean.
2. The desktop app accepts text, image, file, and drawing input.
3. Visible material reaches TrueLearner only through declared physical
   boundaries.
4. Text and raster output are visible.
5. Academy can run genuine teaching and fresh probes.
6. Evaluator meaning never enters TrueLearner.
7. Every physical input is recorded and replays exactly.
8. UI activity does not alter physical time.
9. Observer overlays remain causally inert.
10. Existing physical-equivalence tests remain green.

## Deferred work

Do not add foveation, GPU rendering, distributed Playground, robot bodies,
audio/video input, production hosting, semantic retrieval, or LLM help inside
TrueLearner during V0.

After V0, compare MiniWorld, ProcTHOR, Habitat, and TAVIS behind one external
raster-world boundary. Select by deterministic setup, fast iteration,
procedural variation, controllable view, and low semantic leakage—not realism.
That comparison does not authorize foveation or new organism law.

Stop if a UI object, evaluator fact, simulator label, or wall-clock event
becomes organism state.
