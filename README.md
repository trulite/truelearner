# organism-v0

Minimal executable experiments for the CELL / ARROW / SPIKE idea.

## Independent review

This project is structured as a library and executable so reviewers can add
held-out integration tests without editing the learner.

- [Review protocol](REVIEW.md)
- [Reproducibility record](REPRODUCIBILITY.md)
- `tests/reviewer_api.rs`: example reviewer-controlled topology and actions
- `.github/workflows/ci.yml`: formatting, tests, and strict Clippy

The project name is historical. The experiments do not claim biological life,
consciousness, or unrestricted general intelligence.

## Capability ladder: initial motion

The original smoke test used a hand-wired receptor-to-motion graph. The current
experiment removes that graph and asks a falsifiable question:

> Can a neighbor-only local learning rule discover directional structure from
> repeated sensory motion without receiving a direction label?

Eight receptors form a one-dimensional sensory surface. A bright point moves
from receptor 0 to receptor 7 and is briefly occluded between receptors 3 and
4. Recent activity leaves a short eligibility trace. When a neighboring
receptor fires, the arrow between them gains evidence:

```text
recent source activity + neighboring destination activity
    -> strengthen source-to-destination arrow
```

Immediate transitions receive more evidence than delayed transitions. There is
no built-in preference for left or right.

The first layer reports:

- the learned transition graph,
- right-versus-left directional evidence,
- next-receptor prediction on a held-out rightward sweep,
- a deterministic random-flash control,
- a stationary-input control.

The second layer implements the first explicit `COMPRESS` operation. It reduces
position-specific learned arrows:

```text
0 -> 1
1 -> 2
2 -> 3
```

to a relative template:

```text
relative_step = +1
```

The inverse pattern becomes `relative_step = -1`. The learner receives only
receptor identities, event times, local topology, and episode boundaries. It
does not receive right/left labels.

Both templates are placed in a concept bank. At evaluation time, the bank sees
the first transition, selects the matching relative template, and must predict
the rest of the episode. The test harness does not tell it which direction is
active.

Training is limited to one half of the receptor strip. Held-out tests use:

- receptor positions never used for that direction during training,
- different and irregular event speeds,
- a world-time gap of seven ticks, beyond the local learner's three-tick
  eligibility window,
- the opposite direction as a rejection test.

## What this demonstrates

It tests whether repeated temporal structure can modify persistent arrows and
whether repeated absolute arrows can be compressed into a reusable
position-independent first-order motion template.

It does **not** yet demonstrate:

- object discovery,
- object identity across an occlusion,
- extrapolation when several receptor positions are skipped,
- hierarchical compression beyond one relative transition,
- `NEAR` or `JOIN`,
- useful learning in a realistic sensory world.

## Higher-order dynamics experiment

The v3 experiment no longer compresses motion into a hard-coded `+1` or `-1`
index rule. Sensor IDs are deterministically permuted and therefore carry no
spatial meaning.

The learner receives a known `LOCAL` two-dimensional topology and observes
constant-motion episodes in four unit-speed cardinal directions. From
successive velocities it learns the dominant acceleration:

```text
next_velocity - previous_velocity = (0, 0)
```

This is an inertia-like rule: velocity usually remains unchanged.

The learned rule is transferred to a new 20-by-20 topology with a different
sensor-ID permutation. It must extrapolate two trajectories with:

- diagonal/oblique directions absent from training,
- velocity components and speeds absent from training,
- four hidden world ticks,
- opaque sensor IDs unrelated to the training grid.

A deterministic random-walk curriculum is the negative control and must not
produce a high-confidence inertia rule.

This still assumes a single isolated point and a supplied 2-D topology. It
does not yet learn object identity, bind multiple objects, or discover spatial
topology from raw sensors.

## Persistent identity experiment

The v4 experiment turns learned dynamics into latent object tracks. Input at
each world-time step is only a shuffled list of opaque sensor IDs. Ground-truth
object identities are available to the evaluator but never to the tracker.

For up to three objects, the tracker:

- predicts each latent track with the learned inertia rule,
- exhaustively evaluates viable track-to-detection assignments,
- updates identities only when the minimum-cost assignment is unique,
- keeps tracks alive while detections are missing,
- refuses detections beyond a local plausibility gate,
- reports ambiguity when multiple assignments are equally supported.

The deterministic scenarios check:

- reassociation after a two-frame occlusion,
- identity preservation while two trajectories cross,
- three-object tracking while one object disappears,
- explicit uncertainty for a symmetric, unidentifiable frame,
- rejection of teleporting detections.

This is still not raw vision. Detections arrive pre-segmented as point
locations, and the 2-D sensor topology is supplied.

## Raw-frame object concepts

The v5 experiment removes pre-segmented point detections. Input is a binary
value for every opaque sensor ID in a frame.

Using supplied four-neighbor `LOCAL` topology, the system:

- groups simultaneously active neighboring sensors into candidate regions,
- normalizes each region relative to its own origin,
- counts recurring unlabeled shapes across moving frames,
- promotes only high-support shapes into reusable concepts,
- recognizes those concepts at unseen positions on a larger grid with a new
  sensor-ID permutation.

The held-out checks include an object with one missing pixel, a novel shape
that should be rejected, and random binary visual noise that must not create a
stable high-support concept.

This is a first object-template learner, not general vision. Connectedness,
translation normalization, binary pixels, and non-touching objects are still
provided assumptions.

## Active causal learning

The v6 experiment adds neutral interventions without rewards, survival, or
external task labels. The learner observes:

```text
raw frame before
action issued
raw frame after
```

It infers a recent distribution of transformations for four actions:
`PUSH_LEFT`, `PUSH_RIGHT`, `ROTATE`, and `WAIT`.

The experiment selector prioritizes actions with insufficient evidence and,
after a rule changes, actions whose recent outcomes conflict. Tests compare
the number of interventions required to identify all rules against random
action selection and passive `WAIT` observation.

Learned action effects must:

- predict all four results for an unseen shape,
- transfer to a new grid and sensor-ID permutation,
- focus new experiments on a changed action,
- replace the obsolete rule using recent contradictory evidence.

The action vocabulary and transformation representation are still supplied.
The learner discovers which transformation belongs to each action; it does not
yet invent its own causal representation.

## Causal composition and planning

The v7 experiment connects learned action rules to `JOIN` and `ANSWER`.
Given only a raw start frame and raw target frame, breadth-first search uses the
learned causal model to compose opaque actions into a shortest supported plan.

Held-out targets require five to eight actions on a new sensor layout and an
unseen shape. Additional tests require:

- choosing the only action order that remains valid near a grid boundary,
- returning no plan for a target outside the learned action system,
- detecting a changed action effect during execution,
- learning the replacement rule and replanning from the resulting state.

Matched-length random action sequences provide the control baseline.

Planning search itself is currently supplied. The experiment tests whether
learned causal operators are accurate and composable enough to support
reasoning; it does not yet learn its own search procedure.

## Procedure discovery

The v8 experiment stores successful action traces and searches them for
recurring subsequences that produce a positive compression gain. Repeated
primitive subsequences become procedures, and repeated procedure tokens can
be compressed again into nested procedures.

The learned procedures are used as higher-level operators before falling back
to primitive planning. Evaluation requires them to:

- transfer to an unseen shape on a differently permuted sensor grid,
- preserve the primitive planner's solution length,
- reduce expanded search states by at least five times,
- compose a learned procedure into a nested procedure,
- reject seeded random action traces at the same support threshold,
- retain primitive planning for targets outside the learned procedure set,
- invalidate dependencies after an action rule changes,
- learn a replacement procedure from successful post-change experience.

Each procedure records the learned primitive transformations on which it
depends. This makes causal change observable instead of silently executing a
stale macro.

The recurrence threshold, compression metric, action vocabulary, transform
representation, and primitive fallback planner remain supplied. The machine
discovers reusable action chunks, but it does not yet invent its own procedure
representation or learning objective.

## v9: Structural representation discovery

The v9 learner no longer receives `dx`, `dy`, rotation, shape, or anchor
values. It sees raw sets of opaque active sensors, opaque action IDs, and a
supplied graph of local sensor ports.

For each action it maintains competing structural effects:

```text
stay
follow local port 0
follow local port 1
...
```

Before/action/after observations eliminate inconsistent effects. The learned
operators must move an unseen multi-sensor pattern on a larger graph with new
sensor IDs, predict a blocked boundary outcome, and reject contradictory
shuffled action/outcome evidence.

## v10: Hypothesis-driven intervention

The v10 experiment preserves all consistent causal hypotheses instead of
committing immediately. It selects an action and sensor state whose predicted
outcomes divide the remaining hypotheses most strongly.

The active learner must identify all five opaque action rules in fewer
interventions than deterministic random experimentation. A boundary
observation intentionally remains ambiguous until a more informative
intervention is selected.

## v11: Continual adaptation

The v11 experiment removes episode resets. A bounded recent-evidence memory
receives one continuous transition stream. One action changes its structural
effect while two other actions remain stable.

The learner must replace the changed rule within six relevant samples, retain
both unchanged rules, and keep memory bounded independently of stream length.

## v12: Autonomous abstraction

The v12 experiment supplies multiple opaque actions that have equivalent
effects. The machine groups them into operator classes based only on learned
causal behavior.

Successful traces deliberately use different action aliases, so no exact raw
action sequence repeats. After conversion to learned operator classes, the
same three-step structure appears eight times and becomes a compressed
procedure. Random class sequences are the negative control.

This creates two learned hierarchy levels:

```text
opaque actions -> causal operator classes -> recurring procedure
```

## v13: Self-directed curriculum

The v13 unified loop begins without an externally selected learning phase. It
first chooses interventions until primitive causal uncertainty is resolved,
then builds operator classes and selects from a mixed pool of task traces
according to expected recurrence and compression value.

It must discover the useful procedure after six selected traces, while random
trace sampling requires substantially more experience.

## v14: Cross-domain structural transfer

The v14 experiment transfers the learned operator library and class-level
procedure between two independently encoded sensory domains:

- a permuted binary-pixel encoding,
- an unrelated sparse tone-code encoding.

The target domain has new sensor IDs and new action IDs. One causal
calibration transition per reusable target action maps the new actions onto
the existing operator classes. The class-level procedure must then execute
without any target-domain task demonstrations. An action whose effect is
absent from the source library must remain unclassified.

This is structural transfer, not unrestricted cross-domain understanding. The
local port graph, effect candidate vocabulary, codecs, compression objective,
and task pool remain supplied.

## v14.5: Scaling harness

The v14.5 harness measures deterministic work rather than treating noisy wall
time as the scaling law. The production structural learner records:

- causal-hypothesis evaluations,
- active-sensor visits,
- topology states scanned while selecting experiments.

It sweeps:

```text
training observations: 256 -> 16,384
active context:        1 -> 256 sensors
topology size:         64 -> 4,096 sensors
```

Log-log regression estimates combined work for observation and topology
scaling, and active-sensor visits for context scaling. A queue-based
branching-process probe measures event-cascade size below and above the
critical branching ratio. A bounded associative-memory probe measures recall
accuracy from 0.25x through 4x slot load.

Wall-clock nanoseconds are recorded in the CSV as secondary hardware-specific
evidence. The deterministic operation counts, cascade error, and memory-load
curve determine pass/fail.

Run and record the scaling sweep with:

```bash
cargo run --release --bin scaling -- --output results/v14_5_scaling.csv
```

The current machine-specific measurements are recorded in
[`results/v14_5_scaling.md`](results/v14_5_scaling.md).

This is not a general prediction scaling law. It covers the local structural
learner, event cascades, and a bounded associative-memory probe.

## v14.6: Learned self-stabilization

The problem is repeated internal activity that keeps circulating after the
machine has already found the useful result.

The experiment begins with two kinds of recurring paths:

- a useful recurring path that eventually reaches the correct output,
- a useless loop that creates activity without adding information.

The machine is not told which connection is unstable. It sees whether the
episode reached the correct output and how much internal activity was used.

Repeated successful paths become short concept routes. Once a concept route
produces the same result, older activity that adds nothing new becomes weaker.
A temporary activity limit protects the experiment while this learning takes
place.

Three controls are required:

- prediction pressure alone remains correct but unstable,
- activity pressure alone becomes quiet but stops producing answers,
- combined pressure preserves correct answers, creates concept routes, and
  settles.

After learning, a new unstable loop is attached to the network. The machine
must detect its wasted activity through experience and recover without being
told which new connection caused the problem.

The scaling harness also repeats this learning with one through sixteen
independent routes. It measures how training activity grows and verifies that
every learned route finishes with a small stable cascade.

## v16: One learner

V16 starts a fresh integration line. It does not import the frame, effect,
tracking, planning, or procedure machinery from the capability ladder.

What comes in:

- one opaque token at a time,
- with arrival order as the only relationship.

What happens:

- one receptor cell fires for the current token,
- previously active pattern cells join with that receptor,
- a new join recruits a pattern cell and later occurrences reuse it,
- arrows from active patterns strengthen toward the token that followed,
- queued spikes carry the strongest learned continuation back to token
  receptors,
- recent activity can be cleared while learned cells and arrows remain.

What comes out:

- the token receptor with the strongest unambiguous activity is returned as
  the next-token answer.

The exact same learner instance is evaluated on:

- a delimiter-separated repeated-sequence induction probe,
- thirty-two key-value associations absorbed in one stream,
- three facts hidden near the beginning, middle, and end of an 8,192-token
  noise stream,
- unknown-query and deliberately remapped controls.

A one-token-context version of the same learner is the induction baseline.
The full learner must improve on it by at least twenty percentage points.

This is the first direct integration of cells, arrows, and spikes into one
generic sequence learner, but important structure remains supplied:

- token boundaries and ordered arrival,
- separate joining and prediction phases,
- recruitment of a new pattern cell for a new join,
- preference for the deepest matching pattern,
- an external recent-activity reset,
- a fixed activity limit.

The learner demonstrates associative sequence memory. It does not yet
demonstrate reversal, sorting, learned phase control, or autonomous
compression of its growing pattern graph. The recorded v16 measurements are
in [`results/v16_one_learner.md`](results/v16_one_learner.md). Tiny Shakespeare
is not part of this milestone.

## v17: Consolidation

V17 asks whether v16 is more compact than a conventional variable-depth
lookup structure, and whether an offline rest phase can retain tested behavior
with less permanent structure.

The same synthetic curriculum is given to:

- the v16 cell-arrow-spike learner,
- a plain context trie with no cells or spikes.

The trie matches v16's induction and associative retrieval results. This is
important negative evidence: v16 currently behaves like an event-driven trie,
not a discovered abstraction system.

During rest, the event-driven learner:

- counts which pattern cells actually reactivate,
- proposes a new graph containing recurring patterns and their required
  parents,
- physically rebuilds its cell and arrow storage,
- runs the remembered behavior suite without permitting new learning,
- accepts the rewrite only when the tested behavior survives.

An arbitrary prediction rewiring keeps the same consolidated graph size but
destroys behavior. This verifies that reduced size alone is not the result.

The experience sweep increases retained input from 1,704 to 17,064 tokens.
Raw pattern storage grows from 9,810 to 110,445 cells. Consolidated storage
grows from 1,931 to 6,576 cells while induction remains above 95%.

The corrected comparison gives the trie the same activation counts, retention
threshold, rest timing, and read-only retest. It retains exactly the same
11,741 contexts and the same 97.8% induction accuracy. However:

- the rested event graph uses 44,818 links,
- the rested trie uses 33,077 links,
- estimated owned container storage is about 4,169 KiB for the event graph
  and 2,342 KiB for the trie,
- query work is 15.9 spikes for the event graph and 8.6 lookups for the trie,
- both learn both new post-rest associations, but the trie adds fewer links.

The storage estimate includes allocated Rust container capacity but excludes
hash-table control bytes and allocator bookkeeping.

Therefore v17 demonstrates a useful recurrence-based retention policy, not a
compression advantage for cells, arrows, and spikes. It is selective
retention, not structural abstraction. The recurrence threshold, rest timing,
rewrite operation, and replay acceptance suite are supplied. The replay set
is counted during rest and discarded after the candidate graph is accepted.

Recorded measurements are in
[`results/v17_consolidation.md`](results/v17_consolidation.md).

## Run

```bash
cargo test
cargo run --release
```

## Core runtime

- `CELL`: local slow state + firing threshold
- `ARROW`: persistent connection + weight
- `SPIKE`: transient event carrying source, destination, world-time and value
- `settle()`: drains the spike queue until quiescence

The original queue-propagation test remains in place while the learning
experiment develops independently.
