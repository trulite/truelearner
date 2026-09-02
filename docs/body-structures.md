# Body Structures — Executable Contract

> The complete persistent conceptual model. This describes the production
> Rust state; it is not a second model beside it.

## Ownership

```text
truelearner-core: identities + local physical state + legal transitions
truelearner-body: graph + scheduler + reaction/composition algorithms
workstation:      closed morphology around the generic body
```

Core has no scheduler, workstation, Academy, evaluator, trace, benchmark,
task, authority, or I/O dependency. Body contains no translated core state.

## Physical and learned state

```rust
type Time = u64;
type Impulse = i32;
struct JunctionId(NonZeroU32);
struct LinkId(NonZeroU32);

enum Retention { Integrating, Sampled { lifetime: Time, range: u32 } }
enum Trigger {
    SourceFires,
    RisesThrough(Impulse), FallsThrough(Impulse),
    Rises, Falls,
}

struct Junction {
    threshold: Impulse, retention: Retention,
}
struct Link {
    from: JunctionId, to: JunctionId, delay: Time,
    impulse: Impulse, trigger: Trigger,
}
struct Path {
    surface: JunctionId, middle: JunctionId, output: JunctionId,
    first: LinkId, second: LinkId,
}
struct Occurrence { at: Time }

struct ArrowState { active: bool, kind: ArrowKind }
enum ArrowKind {
    Propagation {
        mode: PropagationMode,
        last_transmission: Option<Occurrence>, evidence: PathEvidence,
    },
    Witness { kind: WitnessKind, last_transmission: Option<Occurrence> },
    Return { path: Path, opened_at: Time, status: ReturnStatus },
    Membership,
}
enum PropagationMode {
    Entry,
    Drive {
        boundary_crossing: bool, locally_plastic: bool,
        factors: Option<[LinkId; 2]>,
    },
}
enum WitnessKind { Progress, Closure { offers_choice: bool } }

struct PathEvidence {
    participation: u64, last_participation: Occurrence,
    outcome_at: Time, outcome_present: bool,
    outcome_changed_world: bool, outcome_available: bool,
    boundary_closed: bool, boundary_inhibited: bool,
    supported_closures: u8, strength: i64,
}

enum ReturnStatus {
    Open { switched_from: Option<LinkId> },
    Closed { at: Time, support: ClosedSupport, motif_parent: Option<LinkId> },
    Ambiguous { at: Time },
    Expired,
}
struct ClosedSupport { source: JunctionId, witness: LinkId }
```

`participation == 0` means no last occurrence; `outcome_present == false`
means no outcome. Public accessors reconstruct those optional logical views.
A composite is an ordinary `Drive` with retained `factors`, not another role.

An integrating junction holds subthreshold potential for four physical ticks.
Only arrivals at that same junction inside the window can sum. Sampled
junctions retain their separate declared lifetime. Neither mechanism stores an
episode or cause identifier.

`locally_plastic` is fixed drive morphology, not learned `PathEvidence`.
`last_transmission` is the link's bounded local eligibility mark. A returned
event walks the physically connected backward cone of every open path at that
source. Each `locally_plastic` propagation link in those cones whose last
transmission is no more than eight physical ticks old strengthens once, up to
strength two. Fixed and saturated links carry the traversal but do not change.
This creates no ancestry, return, or choice; the plasticity bit is persistent
body state and survives attachment and checkpoint restore.

## Body-owned retained and derived state

```rust
struct Body {
    graph: Arena + Vec<ArrowState>,
    consolidation: Option<Box<Consolidation>>,
    reentry: Option<Box<ReentryCache>>,
    derived: ReturnIndex + has_composites + has_local_plasticity,
    transient: schedule + current moment + reusable scratch,
}

struct Consolidation {
    closure_maintenance: bool,
    witnesses: Vec<AutomaticWitness>, evidence: Vec<AutomaticEvidence>,
    work: AutomaticityWork,
}
struct AutomaticWitness {
    returned: LinkId, path: Path, pairs: Vec<AutomaticPair>,
}
struct AutomaticEvidence {
    owner: LinkId, pair: AutomaticPair, supported_closures: u8,
}
struct AutomaticPair { first: LinkId, second: LinkId }

struct ReentryCache { epochs: Vec<u64>, shortcuts: Vec<ThoughtShortcut> }
struct ThoughtShortcut {
    start: Path, condition: JunctionId, routes: Vec<ReentryTrace>,
    dependencies: Vec<ReentryDependency>, rehearsals: u8,
}
struct ReentryDependency { junction: JunctionId, epoch: u64 }
```

Only real returned closure changes `Consolidation`. `ReentryCache`,
`ReturnIndex`, `has_composites`, candidates, traces, receipts, and scratch are
derived and never causal evidence. During one reaction, `CandidatePath` holds
the current physical candidate and its evidence; its `ContinuationResult` holds
only the temporary result of read-only continuation inspection. Neither is
checkpointed.

Local resolution produces one transient warrant:

```rust
enum ChoiceWarrant {
    ReturnedConsequence,
    RetainedContinuation,
    Reentry,
    Exploration,
    LocalIncidence,
}
```

The warrant says which physical evidence class uniquely selected the path. It
is not stored learner state, a reward, or a second decision. Traces project the
same warrant. Only a selected `Reentry` carrying one exact returned reentry may
participate in later membership formation.

## Workstation morphology and public boundary

```rust
struct BodyControl { axis: BodyAxis, direction: Direction }
// BodyAxis = eye | palm | finger flexion
// Direction = Decrease | Increase

struct ApproachLine { strength: u8, pending: u8, inhibited: u8 }
struct VisualApproach { lines: Vec<ApproachLine> } // fixed 16x16 anatomy

struct Rgb { red: u8, green: u8, blue: u8 }
struct ChromaticSignal { red_green: i16, blue_yellow: i16 }
struct ColorField { width: u16, height: u16, pixels: Vec<Rgb> }
struct ChromaticField {
    width: u16, height: u16, pixels: Vec<ChromaticSignal>,
}
```

`BodyControl` serializes directly; there is no parallel command representation.
The workstation competition quotient follows physical incidence. Left eye,
right eye, planar palm transport, palm depth, local thumb motion, and ordinary
digit flexion are declared morphology components, but the body still identifies
paths that arise on the same current physical surface. A morphology claiming
two independent products must therefore expose distinct physical incidence; a
component number alone cannot override the learner's locality law.
The workstation also owns one ordinary exploration surface for each of those
six components. The external sequence clock fires exactly one surface per
admitted opportunity; that surface exposes its local motor alternatives but
stores no task, direction, preference, or learned evidence.
The workstation also retains the crossings that met a joint stop on the last
step (`pending_stops: Vec<MotorEffect>`, reported as `joint_stops`). They join
the world's boundary parents in the next boundary wave and are checkpointed
like `pending_transitions`.

Gaze and approach use separate physical branches. `VisualAttention` retains a
visible region for eye orientation. `VisualApproach` owns one adaptive line for
each fixed 16x16 visual subregion. A completed touch opens the active line; a
coincident fresh visual change raises its bounded strength from one to two,
while expiry inhibits it for 32 samples. The inhibition gates hand reach and
depth entry without removing eye focus. The array position is anatomical
incidence, not an event, object, action, or cause identifier.

Workstation vision has separate physical scales. A fixed 8x8 mean field covers
the complete screen. Four signed transient subregions per mean field localize
change on a 16x16 lattice. A gaze-centred 17x17 field at four body units per
sample supplies local detail in four mirror-symmetric interleaves. The original
9x9 receptors remain coarse foveal receptors so version-14 identities and
learned links retain their physical meaning. Eye position advances in 32-unit
quanta under the existing 128-unit velocity cap; planar palm motion advances in
8-unit quanta under its existing 64-unit cap.

The screen owns one RGB raster. Retinal projection derives Rec. 709 luminance
plus signed red-green and blue-yellow opponent responses. Luminance continues
to own global layout, salience, gaze, and approach. Each original 9x9 foveal
location also owns two opponent sensor junctions with the same local motor
incidence. Gray light is zero on both opponent axes. Any RGB change raises the
ordinary spatial transient even when luminance is unchanged.

The pointer finger has one flexion axis. Its fingertip shares the palm's
horizontal and vertical position and has depth relative to the palm. Screen
pressure is local to finger flexion; tangential slip is local to planar arm
motion. Arm depth and finger flexion are distinct actuators in the same normal
contact component. Planar motion is independent.
Palm depth moves at most one 16-unit quantum per body step, so discrete motion
cannot place the arm beyond the finger's withdrawal range at first contact.

Checkpoint version 20 stores the opponent receptors, finger position, and
separate visual-approach lines. Older incompatible checkpoint versions are
rejected. The public API creates ordinary
junctions and drives, supplies arrivals, runs time, observes frozen traces, and
checkpoints/restores. Narrow internal constructors create entries, witnesses,
returns, and memberships. No public API sets a raw role or submits the internal
edit transaction.
