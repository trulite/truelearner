# SSA0.3 pre-commitment support integration protocol

Status: **PREREGISTERED DEVELOPMENTAL SUCCESSOR**.

This protocol is frozen before any SSA0.3 evidence-bearing execution. It is a
separately named successor to immutable Classification C at
`34277893201c1a72765b143de4b3da1912b6e3b6`, tagged
`ssa0-spatiotemporal-affordance-micro-v1-negative`. It does not reinterpret or
rerun SSA0--SSA0.2. It cannot advance M6, create M7, use Lane A evidence or
mechanisms, or be consumed by Lane A. No definitive execution, SSA1, or SSA2 is
authorized.

## Frozen parent and exact physics

- frozen parent/result commit:
  `34277893201c1a72765b143de4b3da1912b6e3b6`;
- frozen parent protocol commit/tag:
  `8ae8dc78bb6ecf6172f1d69cb41472beac327df4` /
  `ssa0-spatiotemporal-affordance-protocol-v1`;
- frozen parent implementation commit/tag:
  `0f759047353e9afe00bd2883c4a8f42e94192129` /
  `ssa0-spatiotemporal-affordance-implementation-v1`;
- frozen parent protocol SHA-256:
  `92ff3f758977a575e2f8ca651f7a45756e15241a6d4bf829012a266bae9489fc`;
- frozen parent implementation SHA-256:
  `180b24f6b682ec5d274e44b0c680062d10b1f68b6fddeb4d857ec599b32f6299`;
- frozen parent runner SHA-256:
  `fb693aa098e45617deefd5ae9b9de1003528d4b7fbfa87078545fbda5e90fa7f`;
- authoritative M6:
  `aa4e22efd8a65b7694956a53cfaa970582695215`;
- frozen DS-A1, M6 authority, and research-runtime SHA-256 values:
  `b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1`,
  `c2a95199139828e360713320ad57c77a100fc0135ba06b9219624d4f16e1d18d`,
  and `e570b3cd0fcff759a02a38a685f22f33bc28de65e25b7beb34f77d138f3fd711`;
- pre-SSA0.3 results-tree digest:
  `6a70480a01aabe508238859c46ab5b8dfffb7f78e8b1cfbd7ca7fa7921907d29`.

SSA0.3 copies the exact parent `SSA0_PHYSICS_BEGIN` through
`SSA0_PHYSICS_END` propagation loop byte-for-byte and checks that equality in
focused validation. CELL, ARROW, SPIKE, firing threshold `4`, impulse
integration, stable physical queue order, one-shot firing, effect propagation,
and immediate mutual inhibition remain exact. The fixture alone supplies
ordinary supporter paths with fixed integer ARROW delays, phases, impulses,
generations, and liveness. No commitment cell or organism-visible commitment
value is added. The evaluator observes commitment as the first contender CELL
firing in the actual trace.

There is no RNG, choice primitive, harness sampling, probability, supplied
support score, precommit score, semantic action label, aggregate route score,
softmax, temperature, noisy argmax, or evaluator-selected winner. Route labels
exist only after execution.

## Fixed physical worlds

Every case contains two independently executable routes. In each arm, the
`target` contender has four base supporter arrivals at ticks `[4, 5, 6, 8]`;
the competing contender has four at `[4, 5, 6, 7]`. With no added supporter,
the competitor physically commits at tick 7. An added supporter is an ordinary
relay CELL and two ordinary ARROWs. Unless explicitly varied, its impulse is
one and its contender-arrival phase is zero.

The core paired intervention is fixed before execution:

- extra early: one target supporter arrives at tick 6, before the observed
  baseline commitment;
- extra late: the identical count increase arrives at tick 10, after the
  observed baseline commitment.

Every world is mirrored by targeting the other physical route. No threshold,
base schedule, phase, acceptance rule, or expected direction may be tuned
after execution.

The four required crossed comparisons are:

1. base four-versus-four versus one extra early supporter: the early addition
   must change the realized route from competitor to target;
2. base versus the same one-supporter count increase at tick 10: the late
   addition must not change the realized route;
3. equal total count five-versus-four, early versus late delivery: the winner
   must differ while static counts are identical;
4. total target count four versus five, with identical support reaching the
   target before commitment because the fifth arrives late: behavior must be
   equivalent.

Behavior in comparison 4 means realized contender/effect and commitment
identity, not an identical post-commitment trace: the late physical spike is
expected to remain observable after behavior has committed.

## Ordered stages

Stages are evidence-bearing and must execute in order. The protocol and exact
implementation are committed and tagged before PROBE. Each stage's raw CSV and
Markdown interpretation are written atomically, hashed, committed, and tagged
before the next stage. Scientific failures are frozen without rerun or rescue.
A mechanical fault may only be repaired in a separately frozen retry that
preserves all earlier artifacts.

### PROBE

Run the base, extra-early tick-6, and extra-late tick-10 cases in both physical
A/B directions, each as an exact duplicate. PROBE passes only if:

- the unmodified competitor wins and commits at tick 7 in both mirrors;
- early support is physically delivered before that boundary and the target
  wins;
- late support is delivered after the boundary and the competitor still wins;
- every repeated complete state, trace, result, end fingerprint, and work
  ledger is duplicate-exact;
- both routes remain independently executable.

A clean failure of the temporal discrimination freezes Classification C. A
failed validity/control observation freezes D. No later stage runs after a
stopped classification.

### MICRO

On fresh route and occurrence identities, run all four crossed comparisons in
both A/B directions. Also run handle/allocation reversal with inert layout
padding, exact duplicates, and blocked/stale-route controls. MICRO passes only
if all four crossed comparisons pass in both mirrors, all normalized results
survive the permutations, and all controls pass.

If controlled total-count variation with equal pre-commitment delivery changes
the realized behavior, freeze B. If neither temporal delivery nor static count
distinguishes behavior under valid controls, freeze C. Partial or confounded
patterns that cannot support exactly A, B, or C freeze D.

### GATE

Use three fixed transfer variants: canonical identities/layout; fresh physical
and occurrence identities with reversed CELL and ARROW allocation plus 17
inert padding CELLs; and a second fresh identity set with reversed CELL handles
plus 3 inert padding CELLs. Cross each with both physical A/B target directions.

The timing sweep places one extra unit supporter relative to the evaluator-
observed tick-7 commitment:

| timing category | arrival tick | phase | preregistered causal side |
|---|---:|---:|---|
| well before threshold | 2 | 0 | target |
| just before threshold | 6 | -10 | target |
| threshold neighborhood, before | 7 | -200 | target |
| threshold neighborhood, after | 7 | 200 | competitor |
| just after commitment | 8 | 0 | competitor |
| well after commitment | 11 | 0 | competitor |

The phase sweep at tick 7 is ordinary frozen queue physics and tests the
physical neighborhood without relying on route identity order.

Independently run these support-delivery sweeps in every transfer/mirror arm:

- number: zero, one, and two extra unit supporters, all early; and one and two,
  all late;
- physical strength: one extra supporter of impulse one or two, early and late;
- arrival spacing for two unit supporters: early schedules `[2, 6]`, `[5, 6]`,
  and `[6, 6]`, with matched late schedules `[9, 10]`, `[10, 11]`, and
  `[11, 11]`.

Comparable early deliveries pass the related-effect check when every listed
early number/strength/spacing condition realizes the target; matched late
deliveries must realize the competitor. No smooth curve is required. The
evaluator separately reports deterministic empirical realization counts over
transfer/mirror arms; no frequency enters the organism.

GATE controls require fresh route and occurrence identities, physical A/B
mirror, opaque handle permutation, allocation/layout permutation, exact
duplicates, independent execution of each route, blocked route cannot win,
stale route cannot win, unchanged result under inert padding, absence of any
route-index-0/fixed-layout/handle/allocation bias, byte-exact copied physics,
forbidden-primitive source audit, and unchanged frozen-parent hashes.

## Frozen classification rule

Stop with exactly one classification, using the first applicable rule after
the last permitted stage:

- **B — static count independently affects behavior** if valid matched worlds
  show any reproducible realized-route/effect change from total supporter count
  while pre-commitment delivery is held equal;
- **A — pre-commitment support law positive** if every validity control passes,
  all core crossed comparisons and temporal-window expectations pass in every
  transfer/mirror arm, comparable early delivery variations have the related
  effect, and no static-count effect is observed in matched late worlds;
- **C — neither distinction survives** if validity controls pass but neither a
  reproducible pre-commitment temporal distinction nor an independent static
  count distinction survives;
- **D — scientific ambiguity** for any remaining controlled but mixed pattern,
  or when current physics cannot separate the variables because a validity,
  permutation, mirror, or causal-boundary control fails.

B precedes A because any independent static-count effect contradicts the A
claim that equivalent late count is behaviorally inert. Stage stopping rules
above apply before GATE when an earlier stage already determines C, D, or B.
Do not optimize for A and do not rerun a scientific negative.

## Execution surfaces and focused validation

The only SSA0.3 evidence surfaces are `--probe`, `--micro`, and `--gate`.
`--definitive` must reject with exit 2 before harness entry. Focused validation
is limited to:

```text
cargo fmt --all -- --check
cargo clippy --release --bin ssa0_3_precommit_support -- -D warnings
cargo test --release --bin ssa0_3_precommit_support
cargo run --release --quiet --bin ssa0_3_precommit_support -- --probe
cargo run --release --quiet --bin ssa0_3_precommit_support -- --micro
cargo run --release --quiet --bin ssa0_3_precommit_support -- --gate
cargo run --release --quiet --bin ssa0_3_precommit_support -- --definitive
```

Only the stage currently authorized by the ordered procedure is run. No broad
historical suite is required because frozen/shared code must remain unchanged.
The final atomic artifact records classification, mechanistic reading,
protocol/implementation/stage/result commits and tags, exact hashes, trace and
state fingerprints, focused validation, isolation audit, worktree status, and
blockers.
