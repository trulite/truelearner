# Playground Handoff

```text
A0 shell
   |
   v
A1 teach + fresh probe ----+
   |                       |
   +-> A1-V record/replay -+
                           |
                           v
                     stop and review
                           |
                           v
                  A2 -> A3 -> A4 -> A5
```

## Current state

R6 Partition Invariance is development-frozen.

- `MechanicalConfig::REFERENCE` uses Vec, GlobalScan, FullScan, AoS, and
  Scalar.
- `MechanicalConfig::PRODUCTION` uses TimingWheel, Adjacency, Frontier, AoS,
  and opportunistic exact batching.
- `ArenaId` is durable identity.
- `ResidentArenaId` is disposable execution placement.
- Zero-latency repartitioning preserved history in 36/36 comparisons and 2/2
  checkpoint controls.
- The prior R1-R5 result remains 80/80 differential pairs and 536/536 accepted
  clauses.

Do not begin R7. Academy work does not advance organism authority.

## A0: accepted shell

A0 already provides:

- `playground -> academy-core -> truelearner` separation;
- Rust-owned raster input and output;
- text, image, file, and drawing admission;
- bounded work off the UI thread;
- input records, fingerprints, work, and quiescence;
- recent-interaction replay;
- capability and inspector shells;
- same-process checkpoint restore;
- no Academy or foveation meaning inside TrueLearner.

Do not call A0 a complete developmental Academy.

## Current gate

Implement A1 and A1-V from the current shell. Avoid unrelated UI rewrites.

### A1: genuine teaching and fresh probing

Represent each teaching case outside TrueLearner with:

- the capability claim;
- a generated teaching world;
- teaching experiences;
- an ordinary consequence policy;
- a seed and exact record.

Generate independent probe families for:

- the same relation in a fresh context;
- reverse use;
- distractors;
- delay and unrelated intervening experience;
- transfer to changed presentation.

The first case should teach a novel relation between a fresh visual thing and a
fresh symbol, then test both directions with new positions, contexts, and
distractors.

Never use submitted text as its own expected output except in an explicit copy
test. Never attach all text input to fixed capability IDs.

### Controls

Require echo, fresh-identity, remapped-identity, distractor, and wrong-context
controls. At least one required control must defeat a naive echo or latest-item
strategy.

Do not let a probe teach unless its frozen protocol permits it.

### A1 acceptance

A1 is positive only when:

1. Teaching creates a real physical relation and ordinary outcome.
2. Probe instances are independent of teaching instances.
3. Fresh identities and contexts are used.
4. Evidence attaches to the capability actually tested.
5. Text and raster capabilities can both pass and fail.
6. Echo and memorization controls fail when expected.
7. Academy shows the exact evidence behind each state change.
8. TrueLearner receives no expected answer, correctness bit, reward, loss,
   capability ID, or evaluator state.

Until then, do not present capability state as mastery.

### A1-V: episode record

Record A1 while implementing it. Each episode must include:

- initial checkpoint and body identity;
- admitted input in physical order;
- output, outcome, crossings, and work;
- body changes and final fingerprint;
- quiescence and stop reason;
- exact replay evidence;
- external teaching, probe, and control labels.

Render three causally separate views:

- **Organism:** only what entered and left the physical boundary.
- **Shared world:** the raster that human and organism could affect.
- **Observer:** inert labels, expected outcomes, and measurements.

Build video or playback from the canonical record. UI screen capture is a demo,
not evidence.

## Report and stop

Stop when A1 and A1-V are development-positive. Do not proceed to A2 without
explicit review.

Report:

- teaching cases and seeds;
- fresh probe families;
- unchanged negative controls;
- capability evidence changes;
- physical work and body changes;
- exact replay;
- the human-viewable episode.

The gate question is: can ordinary physical experience form a novel path,
fresh input reuse it, and independent evidence show the result without hidden
authority?

## Later gates

- **A2:** capability graph, frontier, interleaving, and retention.
- **A3:** development velocity, persistent history, and learning-to-learn.
- **A4:** one shared raster world with primitive physical actions.
- **A5:** measured world compatibility behind an external raster adapter.
- **R7:** separately authorized non-residence and storage latency.
- **R8:** separately authorized transport.
- **F0:** separately authorized foveation.

Keep Academy persistence file-based until evidence requires a database. Keep
UI timing out of physical history. Keep R7 mechanics out of Playground.
