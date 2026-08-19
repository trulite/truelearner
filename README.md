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

## v18: Renaming-invariant composition

V18 tests the frozen v16 learner without adding a traversal controller,
temporary binding system, or task-specific graph machinery.

Each training episode is serialized into the ordinary token interface. It
contains shuffled directed link statements, an unrelated distractor chain, a
query symbol, and the correct final symbol.

Training uses chains two to four links long and surface symbols from receptors
0 through 95. Held-out evaluation uses completely disjoint symbols from
receptors 96 through 239 and chains five, eight, sixteen, and thirty-two links
long. No learning is permitted during evaluation.

A separate hard-coded walker validates the dataset. It is an upper bound, not
part of the learner. A variable-context trie receives the same serialized
training episodes as the event learner.

Results:

- the validating walker solves all 32 held-out episodes,
- the event learner solves 0 of 32,
- the trie solves 0 of 32,
- the event learner fabricates answers on branch, cycle, and missing-query
  controls,
- permanent event patterns grow from 1,786 after 16 examples to 12,834 after
  128 examples,
- held-out evaluation adds no permanent cells or arrows.

Spike work rises with prompt length, but because accuracy remains zero this is
input processing rather than evidence of learned reasoning.

The result is negative:

> The existing one-learner substrate does not discover renaming-invariant
> reusable composition.

The raw stream still supplies fixed edge, query, answer, and end markers.
Adding a specialized graph walker would solve the benchmark but would not
count as the learner discovering the procedure.

Recorded measurements are in
[`results/v18_composition.md`](results/v18_composition.md).

## v19: Temporary binding

V19 adds one substrate facility after the negative v18 result: cells and
arrows may exist only for the lifetime of one episode.

The input boundary exposes:

- a relation containing opaque identities in slot 1 and slot 2,
- a query containing one opaque identity.

It does not create an answer arrow or match the query against relation
statements. Opaque identities expose equality and hashing only. Their numeric
contents, ordering, and spelling are unavailable to the learning rule.

During an episode:

- relation and identity occurrences become temporary cells,
- symmetric structural arrows record slot membership,
- `SAME` compares the query identity with temporary occurrences,
- four permanent role-routing arrows compete,
- terminal answer supervision strengthens routes that produced the complete
  correct outcome and weakens alternatives.

After the episode, all temporary cells, arrows, relation records, and owned
container capacity are released.

Results:

- six permanent cells and four permanent arrows remain constant from ten
  through ten thousand training episodes,
- validation is 100 of 100 at every checkpoint,
- twenty thousand held-out episodes produce 20,000 correct answers,
- those episodes contain 400,000 completely fresh identities,
- zero identity-specific permanent cells remain,
- peak temporary structure is 31 cells and 20 arrows for ten relations,
- temporary structure and owned capacity return exactly to zero,
- reverse lookup and missing identities return `NOT_FOUND`,
- conflicting outputs return `AMBIGUOUS`,
- repeated identical relations return one answer,
- the canonical permanent fingerprint is unchanged across all held-out
  evaluation and across the ten-thousandth novel episode.

A persistent identity memorizer retains 100,000 entries from training and
answers zero held-out episodes.

This is a narrow temporary-binding result. The substrate still supplies
opaque equality, slot positions, temporary lifetime, output cardinality, and
the four possible role-routing arrows. V19 does not demonstrate composition.

Recorded measurements are in
[`results/v19_binding.md`](results/v19_binding.md).

## v20: Iterable lookup

V20 freezes the learned v19 lookup and asks one narrower question:

> Can the output of that operation become its next input through one learned
> reusable feedback route?

Each episode contains a shuffled two-link chain, distractor relations, a fresh
opaque query identity, two identical `APPLY` events, and one `READ` event.
Every training episode uses exactly two apply pulses.

The host does not pass an identity into `APPLY`, does not assign the lookup
result to the next query, and exposes no pulse number. One apply pulse enters
the same permanent apply cell every time. The machine reads its temporary
current identity, invokes the frozen v19 lookup, places the answer in a
temporary result role, and may update current only through one of three
candidate permanent feedback arrows.

Terminal supervision provides only the complete expected outcome after both
pulses. It does not identify the correct feedback arrow or reveal an
intermediate identity.

Results:

- ten permanent cells and seven permanent arrows remain fixed from ten through
  one thousand training episodes,
- held-out depth-two accuracy is 1,000 of 1,000,
- without retraining, depth one, three, and four are also each 1,000 of 1,000,
- all successful pulses reuse apply cell 6, frozen lookup arrow 1, and learned
  feedback arrow 4,
- every result becomes the next temporary current identity inside the machine,
- lookup work is 23 spikes per apply pulse,
- seventy-four thousand held-out identities create no permanent change,
- missing intermediate links return `NOT_FOUND`,
- conflicting intermediate links return `AMBIGUOUS`,
- duplicate identical links still return one answer,
- all temporary structure and owned capacity are released,
- the canonical permanent fingerprint is unchanged by held-out evaluation.

An unrolled two-stage baseline passes depths one and two but scores zero at
depths three and four. A baseline given the correct feedback route passes all
depths but receives no learning credit.

This supports the narrow claim that the selected lookup operation is iterable.
It does not show learned stopping or continuation. V20 supplies apply timing,
temporary working roles, the frozen v19 lookup, and three candidate feedback
routes.

Recorded measurements are in
[`results/v20_iteration.md`](results/v20_iteration.md).

## v21a: Autonomous continuation

V21a removes the repeated external apply events used by v20.

The evaluator now contributes only:

- one query identity,
- one external start spike,
- a safety cutoff that can kill execution after a requested number of
  successful lookups.

After start, every event is generated inside one queued runtime. The learned
path repeatedly activates the same apply cell, fans queued comparison spikes
over temporary relation cells, collects one result, follows the frozen v20
feedback arrow, updates current, and fires one learned self-trigger arrow back
to apply.

Training uses only two-link chains and complete terminal outcomes. Three
candidate self-routes are supplied: apply again, read now, or become quiet.
Supervision does not identify the correct route.

Held-out evaluation uses completely fresh identities. Every test episode
contains the same forty-link chain and eight distractor relations, regardless
of cutoff. This keeps temporary graph size constant while reasoning depth
changes.

Results:

- twelve permanent cells and thirteen permanent arrows stay fixed from ten
  through one thousand training episodes,
- one external start produces one, two, four, eight, sixteen, or thirty-two
  completed lookups,
- every depth scores 200 of 200,
- a supplied self-trigger baseline also passes every depth,
- a v20-style baseline without self-trigger passes one step and scores zero
  beyond one,
- every iteration reuses apply cell 6, lookup arrow 1, feedback arrow 4, and
  self-trigger arrow 10,
- internal queued spikes are 103, 205, 409, 817, 1,633, and 3,265,
- all sixty-eight thousand four hundred held-out identities remain temporary,
- the canonical permanent fingerprint is unchanged,
- branches remain ambiguous and duplicate identical relations produce one
  result,
- no activity limit is reached.

The spike curve is linear because fixed-size held-out episodes require 102
additional internal spikes per successful lookup, plus one initial
start-to-apply spike.

V21a demonstrates autonomous continuation, not learned finishing. The
evaluator still supplies the start event and safety cutoff. The frozen v20
operation, temporary working roles, and three candidate self-routes remain
supplied.

Recorded measurements are in
[`results/v21a_continuation.md`](results/v21a_continuation.md).

## v21b: Learned finish

V21b freezes the successful v21a path and removes the evaluator cutoff.

When queued lookup produces no successor, the generic runtime now emits one
neutral no-result event. Four supplied candidate routes compete:

- emit the current identity as an explicit answer,
- apply again,
- clear current,
- become quiet.

Terminal supervision provides only the expected final identity. It does not
identify the finish route. Scoring accepts only an explicit answer spike; the
evaluator never reads current as the answer.

Training uses finite chains one through four links deep. Held-out evaluation
uses completely fresh identities and chains five, eight, sixteen, and
thirty-two links deep. Every depth sweep episode contains forty total
relations, so the temporary working set stays fixed while execution depth
changes.

Results:

- fifteen permanent cells and eighteen permanent arrows remain fixed,
- all four held-out depths score 200 of 200,
- a supplied finish baseline also passes every depth,
- the frozen v21a machine without finish scores zero at every depth because it
  emits no explicit answer,
- each run receives one start, uses no cutoff, emits one answer, and reaches an
  empty queue naturally,
- the final semantic trace is exactly terminal current, lookup, no result,
  finish arrow 14, and explicit answer,
- the successful apply, lookup, feedback, and self-trigger route IDs remain
  unchanged from v21a,
- the v21a permanent fingerprint remains unchanged while only finish-route
  confidence learns.

Two scaling curves are recorded:

```text
reasoning depth       5     8      16      32
internal spikes      515   773   1,461   2,837

temporary relations   8    16    32     64      128
internal spikes      197   341   629   1,205   2,357
```

At fixed working-set size, each additional successful lookup adds 86 spikes.
At fixed depth eight, each additional temporary relation adds 18 spikes to the
complete run.

Branches remain ambiguous, duplicate identical links produce one result, and
a zero-link chain answers its query identity. A cycle emits no answer and
reaches the safety limit, confirming that cycle memory remains unsolved.

The later substrate also solves 32 of 32 episodes from the original v18 depth
distribution. This does not erase v18's negative result: v21b additionally
supplies opaque equality, relation slots, temporary lifetime, a no-result
event, and candidate route families that v18 did not have.

Recorded measurements are in
[`results/v21b_finish.md`](results/v21b_finish.md).

## Discovery d0: Discover one route

The execution ladder v19 through v21b is frozen. D0 starts a separate
discovery ladder and asks whether the v19 routing direction can emerge without
receiving four task-specific route candidates.

The parser exposes only opaque identities in two sensory slots and a query.
Permanent role cells firing within a short temporal window propose arrows in
both directions. This generic proposal rule creates useful and useless
connections.

Each training episode executes one competing arrow. Only the arrow that
actually carried the attempted answer receives a recently-used trace.
Terminal supervision then supplies one scalar fact: success or failure. It
does not reveal the correct identity, route, or connection. Repeatedly
successful arrows survive; the other provisional arrows are pruned.

Results:

- 18 arrows are proposed generically, 17 are rejected, and one becomes stable,
- forward experience retains `Slot1 -> Slot2`,
- reversed experience from the same prior retains `Slot2 -> Slot1`,
- eight independent seeds in each direction reach perfect held-out accuracy,
- random answer labels create no stable route,
- twenty thousand further renamed episodes leave permanent state unchanged.

An irrelevant-cue control exposes a limitation: 15 of 32 learners selected a
predictive cue shortcut and failed after the cue was removed. D0 learns a
reliably rewarded topology, not necessarily a causal one.

D0 still supplies opaque equality, sensory role cells, episode boundaries, a
coactivity window, one-route competition, traces, and the consolidation rule.
It discovers one role-routing arrow, not the complete v19-v21b program.

Recorded measurements are in
[`results/d0_topology_discovery.md`](results/d0_topology_discovery.md).

## Discovery d1: Contrasting experience

D1 leaves the d0 learner unchanged and changes only the information in its
experience.

Observation-only and contrasting learners receive identical opaque
identities, relations, query, relation order, answer changes, learner seeds,
and 792-episode budgets. In the observation-only stream, an irrelevant cue
always marks the target relation. In the contrasting stream, matched episodes
move the cue without changing the answer and change the true right-side value
while holding the cue fixed.

The contrasting curriculum counterbalances every observable cue location:
each of ten relative relation positions and cue absence occurs 72 times.
Changed and unchanged answers each occur 396 times. No intervention marker or
explanation reaches the learner.

Results across 32 seeds:

- observation-only learners fail 10 times after cue removal,
- contrasting learners fail 0 times after cue removal,
- all 32 contrasting learners retain `Slot1 -> Slot2`,
- the representative cue shortcut rises to strength 1 and later falls to -1,
- the true route reaches strength 4 and consolidates at episode 83,
- random labels still produce no stable topology,
- held-out evaluation leaves permanent state unchanged.

The separate d0 curriculum remains recorded at 15 shortcut failures in 32
runs. D1's 10-of-32 observation result belongs to the exactly paired
comparison.

D1 demonstrates intervention-robust prediction under a supplied curriculum,
not causal understanding. The learner does not choose which contrast to
create.

Recorded measurements are in
[`results/d1_intervention.md`](results/d1_intervention.md).

## Discovery d2: Learned epistemic action

D2 lets unresolved topology choose among three opaque actions and no action.
Action meanings are permuted between runs:

- one moves the cue while preserving the real slot relationship,
- one corrupts both competing predictions,
- one changes nothing useful.

Every real action has the same small cost.

An action trace snapshots the currently plausible routes and their strengths.
Ordinary d0 updates continue until every snapshotted route has been exercised.
The trace then classifies its own consequence:

- one route weakened while another remained supported,
- all routes weakened,
- nothing useful separated.

No evaluator information score or correct route enters this process. Cleanup
waits while the trace is active, and an unresolved strength tie cannot
consolidate through fixed arrow ordering.

Across 32 runs and all six action permutations:

- all 32 learn the correct topology,
- all 32 prefer the informative action,
- none prefer the disruptive action,
- paid action use falls to zero after topology resolves,
- random labels produce neither stable topology nor positive action value.

In the representative run, disruption changes both route strengths from 1 to
0 and receives action value -3. The informative action changes the real route
from 1 to 2 while the shortcut falls from 1 to 0, receives value +2, and is
selected again. Its second use raises the real route from 3 to 4, the shortcut
falls from 1 to 0, and topology consolidates. No further paid action occurs.

Random action search also solves all 32 runs and uses slightly fewer paid
actions on average: 2.6 versus the learned policy's 3.0. D2 therefore
demonstrates conditional epistemic preference, not superior search
efficiency.

Recorded measurements are in
[`results/d2_epistemic_action.md`](results/d2_epistemic_action.md).

## Discovery d2.1: Amortized epistemic action

D2.1 keeps the d2 learner frozen and asks when remembering where useful
evidence comes from becomes cheaper than searching again.

Each ambiguity receives a new d0 topology workspace and fresh opaque
identities. The workspace is consumed and destroyed after resolution. Only
the action policy persists; it contains action values, tried flags, and an
exploration cursor, but no problem identities or topology references.

The environment is swept across 4, 8, 16, 32, and 64 total choices. Every
mapping contains one informative action, one disruptive action, inert actions,
and no action. Meanings stay fixed across one hundred problems but are
permuted between runs.

Results:

```text
choices                   4     8     16     32     64
first learned cost       2.0   3.8   10.0   15.6   41.4
mature learned cost      1.0   1.0    1.0    1.0    1.0
mature random cost       2.5   5.1    8.2   17.9   36.0
oracle cost              1.0   1.0    1.0    1.0    1.0
break-even problem         1     1      2      1      2
```

All learned, random, and oracle runs remain correct. At 64 choices, average
learned work is 3,684 spikes and 100 episodes, versus 35,548 spikes and 965
episodes for random search.

All 12,000 fresh workspaces are destroyed, no workspace remains live between
problems, and the persistent policy retains zero problem identities.

D2.1 supplies fresh workspace boundaries and a fixed action mapping across
problems. Action remapping and continual topology contexts remain untested.

Recorded measurements are in
[`results/d2_1_amortization.md`](results/d2_1_amortization.md) and
[`results/d2_1_amortization.csv`](results/d2_1_amortization.csv).

## Discovery d2.2: Silent action remapping

D2.2 keeps the d2.1 action policy unchanged and silently changes which opaque
action is informative after 10, 50, or 100 maturity problems.

Two remaps are tested:

- the new informative action was never tried,
- the new informative action was previously tried and weakened.

Mature and fresh policies receive the same new mapping and fresh topology
problems. Adaptation requires five consecutive correct problems using one paid
action. No forgetting, value decay, or exploration-reopening rule is added.

Unknown replacements are eventually found, but prior confidence creates
increasing adaptation cost. At 16 choices, mature adaptation cost rises from
36.5 after 10 maturity problems to 216.5 after 100. At 64 choices it rises
from 85 to 265.

Previously rejected replacements are never reconsidered in any of 12 mature
runs within 500 problems. All matched fresh policies adapt in five problems.
All 7,547 fresh topology workspaces are destroyed.

D2.2 therefore records rigidity rather than hiding it:

> The frozen policy can exhaust obsolete positive evidence and re-explore an
> unknown action, but it cannot overturn old negative evidence once no-action
> is preferred.

Recorded measurements are in
[`results/d2_2_remap.md`](results/d2_2_remap.md) and
[`results/d2_2_remap.csv`](results/d2_2_remap.csv).

## Discovery d2.3: Expectation-triggered reopening

D2.3 adds one generic plasticity mechanism without changing the topology
learner or the meaning of useful evidence.

Historical action outcomes are retained separately from current-regime
evidence. A trusted action remains consolidated through isolated failures.
Three consecutive missing expected consequences mark only that action mapping
unreliable and reopen previously rejected alternatives.

The hard d2.2 case now adapts in every run:

```text
choices  maturity  violations to reopen  problems to adapt  paid actions
16       10        3                     8                  21
16       50        3                     8                  21
16       100       3                     8                  21
64       10        3                     8                  69
64       50        3                     8                  69
64       100       3                     8                  69
```

Adaptation time is independent of how long the old policy had been correct.
Isolated noise and an unchanged environment cause zero false reopenings.

The first switch to a previously rejected action costs 21 paid actions in the
16-choice diagnostic. Switching back costs 9, and switching again also costs
9, because historical evidence from both regimes remains available.

A full reset adapts in six problems, faster than d2.3, but discards all
history. D2.3 preserves history and reduces later rediscovery cost.

All 2,162 fresh topology workspaces are destroyed.

Recorded measurements are in
[`results/d2_3_plasticity.md`](results/d2_3_plasticity.md) and
[`results/d2_3_plasticity.csv`](results/d2_3_plasticity.csv).

## Discovery d3: Model-based epistemic action

### D3a: Role-relative action models

D3a trains outside any ambiguity task. It receives opaque actions and
temporary role occupants before and after acting. Generic equality produces
changed or preserved events for each role.

Coactivity proposes persistent action-to-role outcome arrows. Across sixteen
action permutations:

- held-out effect predictions are 6,144 of 6,144,
- shuffled action/outcome training produces zero confident models,
- 196,608 opaque identities are observed,
- zero identity-specific cells remain permanent,
- model size remains fixed as identity experience grows.

### D3b: Predict a useful experiment

D3a models are frozen. D3b receives two novel competing route activity graphs.
Generic set and connection comparison exposes shared and route-specific
structure without naming a distinguishing role.

Every action model predicts changed and preserved roles before action. The
supplied epistemic preference chooses an action that changes route-specific
evidence while preserving shared and competing evidence.

First-action results:

```text
model-based selection   48 / 48
empty action history    11 / 48
random selection        18 / 48
change-everything        0 / 48
```

All 48 complete decision traces exist before execution. The selected action is
then executed once on fresh identities. Frozen model fingerprints remain
unchanged.

Recorded measurements are in
[`results/d3_model_based_epistemic_action.md`](results/d3_model_based_epistemic_action.md)
and [`results/d3_pre_action_traces.csv`](results/d3_pre_action_traces.csv).

## Discovery d4a: Composable action transformations

D4a learns more than which roles change. For every opaque action and output
role, it learns which input role supplies the resulting occupant.

Training contains individual actions only. Held-out evaluation repeatedly
applies frozen learned models to fresh temporary structures:

```text
exact sequence predictions   848 / 848
changed-role mask baseline    32 / 848
swap twice                    16 / 16
order-sensitive pairs         16 / 16
```

Unseen sequence lengths range from one through sixteen. Permanent model size
and temporary role count remain fixed while the number of generic model
applications grows with sequence length.

Two opposite rotations provide the decisive control: they change exactly the
same roles but obtain occupants from different input roles. The changed-role
mask cannot distinguish them; the learned provenance models can.

Recorded measurements are in
[`results/d4a_composable_transformations.md`](results/d4a_composable_transformations.md)
and [`results/d4a_composition_traces.csv`](results/d4a_composition_traces.csv).

## Discovery d4b: Counterfactual sequence search

D4b freezes learned provenance models and supplies a generic shortest-first
sequence enumerator. No real action is taken during planning. Every candidate
is simulated only through learned models and tested with the supplied
structural separation criterion.

Fresh problems require shortest distinguishing sequences of exactly one, two,
three, four, or eight actions:

```text
model prediction and real execution   40 / 40
true-model oracle                     40 / 40
equal-budget random order             30 / 40
one-step selector                      8 / 40
changed-role-mask planner              0 / 40
unreachable reported                   8 / 8
```

Permanent model size remains 507 entries. Average candidates examined grow
from 1.9 at depth one to 6,969.8 at depth eight. The corresponding model
applications grow from 1.9 to 50,850, exposing the cost of exhaustive search.

Recorded measurements are in
[`results/d4b_counterfactual_planning.md`](results/d4b_counterfactual_planning.md)
and [`results/d4b_planning_traces.csv`](results/d4b_planning_traces.csv).

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
