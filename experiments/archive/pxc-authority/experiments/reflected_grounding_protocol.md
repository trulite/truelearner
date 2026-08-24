# Frozen protocol: RG0a reflected grounding

Status: preregistered before RG0a implementation, smoke, or execution.
Protocol version: `reflected-grounding-rg0a-v1`.

Disposable branch: `research/reflected-grounding-rg0a`. Frozen parent:
`rp0b-reflected-program-economics-technical-negative`
(`37806c75b44108a7bc8e79afceeaec599b406cef`). The parent tag, RP0a positive,
RP0b negative, TrueLearner trunk, tau/d2 work, and every F0 artifact remain
unchanged. RG0a is a new functional gate, not a reinterpretation or rescue of
RP0b.

Frozen parent references:

- `src/reflected_program_discovery.rs`:
  `13776296d745ed6f758b80086e5ee0ed4764b86c4fa8529be9e967ea262ef4b4`;
- RP0a protocol:
  `87bfc89d031c1c1ccefa48160581fa931ba333b329fe3be7d8590b973c378eaf`;
- RP0a CSV:
  `215f8d18e611585f6d7416ab57ce5072450fad18d4d03750c6199ced2fbf5235`;
- RP0a generated report:
  `8318fec3e5144083ec79051e4556d179f795cbfb2a9537ac0275c7e18858b8f2`;
- RP0a audit:
  `81b6676eca0e3325fb0f0c51c5eab6a4d0cccf4489e0f7ce33eb37b5288ba1a7`;
- RP0b protocol:
  `de2fc225b48ab44472d12e5c75633f602365135417d027c578b537b3f0f6f9b9`;
- RP0b CSV:
  `10e52f45b24f595268112f027e14581655e3ec5d42363dca53f2a7bdc9edb0b3`;
- RP0b generated report:
  `68261b1159e3989405d1a0cea18efb57f1829e803e32dfd8546ed4e52567e478`;
- RP0b audit:
  `9e6b1218e5bda0b12a2590afecdf401d53a6aa1ce9ca42ef44b0aef0cfc4765e`.

The preregistration commit adds only this protocol and README navigation. It
contains no RG0a implementation, smoke output, learned state, runtime result,
or conclusion.

## Question and permitted claim

Can a frozen learned RP0a reflected program use temporary structural bindings
to act on the current lower-level working state and produce the correct
concrete continuation without executing or falling back to the original
lower-level route program?

```text
fresh current lower state
    -> anonymous provenance recognition
    -> temporary learned-role <-> lower-cell binding
    -> frozen learned reflected arrow fires
    -> binding dereference
    -> ordinary spike reaches bound lower cell
    -> frozen lower cell physics handles the spike
    -> explicit concrete answer
```

The permitted positive claim is narrow:

> A frozen learned reflected program grounded its role activations into fresh
> concrete lower cells through temporary structural bindings and produced the
> correct concrete continuation with the original lower route program absent.

RG0a does not claim runtime reduction, acquisition break-even, general
compilation, arbitrary state grounding, recursive reflection, or learning over
learning.

## The single new capability

RG0a adds exactly one substrate operation:

> A learned reflected role may be temporarily associated with the currently
> active lower occurrence whose anonymous provenance activates that role; when
> the role is activated, the association may be dereferenced to deliver an
> ordinary spike to that concrete lower cell.

The association is episode-local, bidirectionally traversable, read-only after
construction, and erased at quiescence. It stores opaque identities only. It
does not store a semantic role, opcode, value, answer, route label, or callback.

Binding assignment must be the unchanged RP0a structural match: the frozen
role learner compares a current anonymous provenance signature with its
persistent anonymous patterns. The evaluator may retain a separate map for
scoring, but no evaluator role or correct mapping may enter the integrated
grounding path.

Binding dereference is deliberately stupid plumbing:

```text
concrete source cell fires
    -> source location's temporary reflected role activates
    -> learned reflected arrow fires
    -> target role's unique temporary lower binding is read
    -> ordinary spike is enqueued at that opaque lower location
```

The target lower cell supplies all subsequent physics. The binding layer may
not interpret why the cell fired or what the target does.

## What remains frozen

RG0a reconstructs the eight frozen integrated RP0a endpoints exactly. It does
not change:

- role signatures, recurrence threshold, supports, or fresh-identity transfer;
- the P0 proposal, probation, eligibility, credit, consolidation, or pruning
  physics that produced the reflected program;
- any learned RP0a role pattern, arrow, strength, or topology;
- v19-v21 opaque identity equality, relation lookup, queue, feedback,
  continuation, no-result, finish, or answer behavior;
- the chain family, irrelevant-relation count, correctness criterion, activity
  limit, or held-out depths;
- the RP0a and RP0b artifacts or conclusions.

The unchanged inherited lower physics is limited to temporary identity and
`SAME`, relation storage, cell activation, spike delivery, queues, local
feedback/current-state effects, continuation, no-result, finish, and explicit
answer emission.

## Explicitly forbidden

No experimental grounded arm may use:

- `ProgramChoices` or any equivalent host-resolved route table;
- the frozen direct executor after the branch point;
- a direct lower route arrow, supplied lower continuation, or fallback;
- a semantic role name, source/target label, operation or lookup opcode;
- a host-side `if reflected_role == ...` semantic dispatch;
- a Rust callback implementing the requested operation;
- an evaluator-created jump, entry point, target, binding, route, or answer;
- a concrete identity retained across episodes;
- a context-to-entry index, cache, attention mechanism, second learner, new
  reflected arrow, or modification of the frozen learned program.

The lower cell registry necessarily contains each concrete cell's already
frozen local physics. Looking up a cell by opaque destination and letting that
cell handle an ordinary spike is permitted. Using the registry to infer or
select a destination is forbidden.

## Frozen starting state and branch point

Each episode creates a fresh v19-v21 chain with a query, `depth` relations, and
eight irrelevant relations. Every concrete cell location, activity-source
identity, relation identity, and temporary occurrence is fresh. Relation order
is independently permuted.

The common frozen state `S` contains:

- the fresh relation occurrences;
- the fresh current/query occurrence;
- the ten fresh lower cells and their unchanged local physics;
- an empty event queue at the external continuation boundary;
- no temporary reflected binding.

`S` is serialized once and cloned byte-for-byte into all arms. The branch point
is immediately before the lower route continuation begins. Relations and the
current/query occurrence are immutable input state; temporary current state,
queue state, and bindings belong to the branch.

### A. CONCRETE

From `S`, one external start pulse invokes the frozen direct lower route
program. Its direct route arrows, cells, spikes, queues, lookup, feedback,
continuation, finish, and answer behavior execute normally.

### B. GROUNDED REFLECTED

From the identical `S`, direct lower route arrows are absent. Current anonymous
provenance is observed, the frozen role learner constructs temporary bindings,
and one external start pulse invokes the same lower cells. Whenever execution
requires a route, the concrete source cell activates its bound reflected role,
the frozen learned reflected arrow fires, and the target binding delivers an
ordinary spike to the bound lower cell. No route is resolved in advance.

Residual lower cell physics is expected and counted. The forbidden lower
continuation is specifically the original direct lower route program: its route
arrows must fire zero times and its executor must never be called in this arm.

### C. NO BINDINGS

Identical grounded execution with the temporary binding table removed before
start. It must not be fully competent.

### D. SHUFFLED BINDINGS

The structurally produced bindings are permuted across current lower locations
using an independent deterministic seed before start. No evaluator mapping is
used. It must not be fully competent.

### E. ACTIVITY-ONLY GROUNDING

The same lower cells emit the same amount of anonymous activity without
consume/produce incidence. The frozen role learner therefore receives no
structural grounding. It must not be fully competent.

### F. RANDOM REFLECTED PROGRAM

The structurally correct bindings are retained, but a deterministic independent
random four-arrow program over the same learned roles replaces the frozen
learned topology. The random program is sampled without evaluator knowledge
and frozen before evaluation. It must not be fully competent.

### G. SHUFFLED-TERMINAL PROGRAM

Use the unchanged RP0a learning process and structural observations, but train
the reflected program with RP0a's shuffled terminal feedback control. Freeze
that resulting state before RG0a evaluation. It must not be fully competent.

### H. ORACLE BINDING AND PROGRAM

Positive diagnostic only. Supply the correct role/cell binding and correct four
arrows, then use the identical grounded executor and lower cell physics. Oracle
information never enters B-G and cannot establish the RG0a claim.

## Definitive matrix

Reconstruct frozen integrated RP0a seeds `0..8`. Evaluate 16 fresh episodes at
each unseen depth:

`5, 8, 16, 32, 64, 128`.

Every arm for a seed/depth/repetition receives the identical serialized `S`.
The definitive integrated matrix therefore contains `768` episodes and `7680`
fresh lower-role transfers. Duplicate read-only evaluation uses fresh copies of
the same serialized states and must be bit-for-bit deterministic.

For a depth `d` successful grounded execution is expected to fire `3*d + 2`
dynamic reflected arrows: lookup, feedback, and continuation for every
successful step, followed by final lookup and finish. This count is a frozen
structural invariant, not a runtime-success criterion by itself.

## Measurements

Record separately for every arm/seed/depth:

- correct answers, explicit answers, quiescence, and activity-limit hits;
- fresh concrete identities, fresh activity-source identities, role-transfer
  matches, false/ambiguous bindings, and bindings erased;
- external invocations and current-state installations;
- provenance events/relations and persistent-pattern comparisons;
- role activations, binding writes, binding reads, and failed dereferences;
- reflected arrow evaluations and dynamic reflected arrow firings;
- direct lower arrow evaluations/firings and direct-executor calls;
- lower cell activations, spikes enqueued/dequeued, queue checks, relation
  scans, opaque identity comparisons, feedback/current updates, and finish;
- fallback/oracle calls, permanent fingerprints, immutable-state hashes,
  workspace creation/destruction, and maximum live workspaces.

One work unit is one actually executed primitive event. Work is diagnostic in
RG0a and cannot establish an economic claim.

## Conjunctive functional gate

RG0a passes only if all conditions hold:

1. **Frozen ancestry.** Parent commit and every hash above match.
2. **RP0a reconstruction parity.** All eight integrated states exactly match
   their frozen RP0a endpoints, including roles, arrows, acquisition work,
   held-out behavior, bytes, and fingerprints.
3. **Identical branch state.** All arms begin from byte-identical `S` for every
   seed/depth/repetition.
4. **Fresh anonymous grounding.** All integrated episodes use fresh concrete
   and activity-source identities, retain none permanently, and transfer all
   `7680/7680` anonymous roles correctly with no ambiguous binding.
5. **Grounded functional substitution.** GROUNDED REFLECTED answers `768/768`
   correctly with explicit answers, natural quiescence, no activity-limit hit,
   and exactly the expected dynamic learned-arrow firings.
6. **Downward causal path.** Every integrated routed lower spike is caused by a
   frozen learned reflected arrow followed by a temporary binding dereference;
   the four frozen arrow identities are all used.
7. **No lower-program fallback.** GROUNDED REFLECTED has zero direct lower
   route evaluations/firings, zero direct-executor calls, zero fallbacks, zero
   oracle calls, and no pre-resolved route table.
8. **State isolation.** The frozen learned fingerprint and immutable lower
   state are unchanged, every temporary binding is erased, and duplicate
   evaluation is deterministic.
9. **Necessary bindings.** NO BINDINGS and SHUFFLED BINDINGS are each less
   correct than GROUNDED REFLECTED and have `0/8` fully competent seeds.
10. **Necessary structural provenance.** ACTIVITY-ONLY GROUNDING is less
    correct than GROUNDED REFLECTED and has `0/8` fully competent seeds.
11. **Necessary learned topology and credit.** RANDOM REFLECTED PROGRAM and
    SHUFFLED-TERMINAL PROGRAM are each less correct than GROUNDED REFLECTED and
    have `0/8` fully competent seeds.
12. **Grounding upper bound.** ORACLE BINDING AND PROGRAM answers `768/768`
    through the identical grounded executor without a direct lower route.
13. **Opacity audit.** Integrated source contains no evaluator role-to-location
    assignment, semantic target resolution, operation callback, concrete
    answer, oracle route, or retained concrete identity.
14. **Accounting and lifecycle.** Every row reconciles, every state clone and
    binding workspace is destroyed, and no temporary or mutable evaluation
    state survives.

Any failed condition freezes RG0a as a valid functional negative. Correctness
that uses even one fallback, direct lower route, or evaluator-created binding
is invalid rather than positive.

## Interpretation and sequencing

- RG0a fails: RP0a remains one-level functional learning closure, but no
  demonstrated downward grounding/substitution exists.
- RG0a passes: the learned reflected program can act on fresh current lower
  state through temporary bindings; reflected grounding is positive.

RP0b remains its frozen technical negative under its own invocation mechanism
regardless of RG0a. A positive RG0a may unlock a separately preregistered
grounded-runtime experiment. It does not itself unlock F1. Recursive F1 remains
blocked until grounded substitution also demonstrates a computational reason
for the reflected level to persist.

## Permitted pre-definitive work and execution boundary

After this protocol is committed and tagged, implementation may perform source
edits, non-Rust local audits, E2B formatting/compilation/Clippy/tests, frozen
RP0a reconstruction parity, and one excluded diagnostic smoke. The smoke must
use a non-definitive seed and oracle-constructed role/program state; it may test
the grounded plumbing and controls but may not reconstruct or evaluate seeds
`0..8`, produce a result artifact, or establish RG0a.

All Rust work runs through the persistent E2B runner at
`/Users/satya/work/br/truelearner/scripts/e2b_persistent.py`. The runner must
reuse and leave its sandbox alive. Local work is limited to source/document
edits, read-only audits, and Git packaging. The definitive RG0a command may run
exactly once after the implementation and audit are committed and tagged.

An invalid smoke or definitive run is preserved and explained. A valid
negative is frozen without adding another capability, weakening controls,
enabling fallback, rerunning the matrix, or proceeding to economics or F1.
