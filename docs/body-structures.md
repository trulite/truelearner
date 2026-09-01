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
type Cause = u64;
struct JunctionId(NonZeroU32);
struct LinkId(NonZeroU32);

enum Retention { Integrating, Sampled { lifetime: Time, range: u32 } }
enum Trigger { SourceFires, SourceOpens, SourceCloses }

struct Junction {
    threshold: Impulse, potential: Impulse,
    retention: Retention, last_source: Cause,
}
struct Link {
    from: JunctionId, to: JunctionId, delay: Time,
    impulse: Impulse, trigger: Trigger,
}
struct Path {
    surface: JunctionId, middle: JunctionId, output: JunctionId,
    first: LinkId, second: LinkId,
}
struct Occurrence { cause: Cause, at: Time }

struct ArrowState { active: bool, kind: ArrowKind }
enum ArrowKind {
    Propagation {
        mode: PropagationMode,
        last_transmission: Option<Occurrence>, evidence: PathEvidence,
    },
    Witness { kind: WitnessKind, last_transmission: Option<Occurrence> },
    Return { path: Path, cause: Cause, opened_at: Time, status: ReturnStatus },
    Membership,
}
enum PropagationMode {
    Entry,
    Drive { boundary_crossing: bool, factors: Option<[LinkId; 2]> },
}
enum WitnessKind { Progress, Closure { offers_choice: bool } }

struct PathEvidence {
    participation: u64, last_participation: Occurrence,
    outcome_at: Time, outcome_present: bool,
    outcome_caused_transition: bool, outcome_available: bool,
    boundary_closed: bool, boundary_inhibited: bool,
    exact_closures: u8, strength: i64,
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

## Body-owned retained and derived state

```rust
struct Body {
    graph: Arena + Vec<ArrowState>,
    consolidation: Option<Box<Consolidation>>,
    reentry: Option<Box<ReentryCache>>,
    derived: ReturnIndex + has_composites,
    transient: schedule + current moment + reusable scratch,
}

struct Consolidation {
    closure_maintenance: bool,
    witnesses: Vec<AutomaticWitness>, evidence: Vec<AutomaticEvidence>,
    work: AutomaticityWork,
}
struct AutomaticWitness {
    returned: LinkId, path: Path, cause: Cause, pairs: Vec<AutomaticPair>,
}
struct AutomaticEvidence {
    owner: LinkId, pair: AutomaticPair, exact_closures: u8,
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
// BodyAxis = eye | palm | wrist | spread | opposition | digit
// Direction = Decrease | Increase
```

`BodyControl` serializes directly; there is no parallel command representation.
Prerelease checkpoints contain only the current model. Incompatible changes
bump the version and old artifacts are rejected. The public API creates ordinary
junctions and drives, supplies arrivals, runs time, observes frozen traces, and
checkpoints/restores. Narrow internal constructors create entries, witnesses,
returns, and memberships. No public API sets a raw role or submits the internal
edit transaction.
