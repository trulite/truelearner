# CPC1 local continuous temporal participation protocol v1

Status: frozen before candidate implementation or execution.

Parent: CPC0 development positive at
`fc501ec1252b8650d9490196c11b2e7b74a30b8c`.

## Narrow question

Within the fixed CPC0 contact topology, can actual ARROW traversal leave a
continuously relaxing local physical state such that Modulatory activity at
that contact produces a graded local plastic response without a deadline or
eligibility predicate?

```text
P --> Ca --A--> X
P --> Cb --B--> Y
```

CPC1 changes neither contact topology nor pressure. It does not run ARC,
chain credit, form contacts autonomously, or advance authority.

## Candidate

The candidate is feature-gated behind `cpc1` and absent from default builds.
Every ARROW carries two unsigned fixed-point local magnitudes:

```text
participation
plastic_support
```

The sole candidate dynamics are:

```text
actual traversal:
    participation += 2^32

each elapsed physical tick:
    participation = floor(participation * 15 / 16)

Modulatory activity at an ordinary CELL compartment:
    for each live outgoing ARROW at that compartment:
        plastic_support += participation
```

Arithmetic saturates only at machine representational capacity. Zero
participation naturally adds zero support. There is no branch, threshold,
deadline, timeout, remaining-ticks state, accepted delay, or boolean
interpretation in the plastic response.

The constants are universal candidate physics and are not selected using ARC
or a desired task horizon. CPC1 records the curve rather than optimizing it.

`plastic_support` is a local candidate plastic state, not yet durable
resistance. CPC1 makes no persistence or pressure claim.

## Retained pressure boundary

The old `eligible_until` field remains temporarily present and continues to
serve the unchanged pressure law. It may not gate, scale, or select CPC1
plastic support. Modulation consumes the retained pressure bookkeeping at the
same local contact, but candidate support is computed solely by arithmetic on
participation.

Existing LR-C resistance/coupling strengthening is disabled only in `cpc1`
feature builds so the old rectangular plastic response cannot contaminate the
candidate measurement. Default builds retain LR-C byte-behaviorally.

No candidate state is added to durable or live checkpoints. CPC1 requires
fresh-run replay and does not claim restart support.

## Forbidden substitutions

- `expires_at`, remaining ticks, age gates, or window comparisons;
- `if participation > 0` as a plasticity or pressure gate;
- a path, return, reward, or credit identifier;
- source-global participation state;
- renewal by source activity without traversal, another path, nearby Drive,
  or unrelated Modulation;
- candidate interaction with pressure or durable resistance;
- evaluator mutation of candidate state;
- duplicated Reference and Production candidate laws;
- new candidate `PhysicalEvent` variants that could repeat TC-DS1's
  mechanics-dependent trace instrumentation.

## Delay curve

For one traversal of A followed by one Modulatory return at Ca, record
participation immediately before return and resulting `plastic_support` at
delays `0..20`, plus late control `1024`.

Across two fresh roots, pressure phases `0..9`, Reference, and Production:

- delay zero is positive;
- samples `0..20` are strictly decreasing and contain at least three distinct
  positive values;
- support equals the remaining local participation for one return;
- delay `1024` produces zero support through ordinary integer relaxation;
- no rectangular `1,1,1,1,1,0` response is accepted;
- relative curves are pressure-phase and identity invariant.

This is `22 * 2 roots * 10 phases = 440` physical curve cases and `880`
mechanics rows.

## Locality and renewal controls

Nine controls run across the same roots, phases, and mechanics:

1. `prompt_a`: A and B traverse; prompt return at Ca supports A only.
2. `return_b`: A and B traverse; prompt return at Cb supports B only.
3. `unrelated_activity`: unrelated nearby physical activity cannot renew A;
   A matches the single-traversal curve at the same elapsed time.
4. `repeat_a`: actual repeat traversal of A renews A above the matched
   single-traversal curve; B is not renewed.
5. `repeat_b_while_a_waits`: repeated B traversal raises B while A follows its
   unrenewed single-traversal curve.
6. `source_without_traversal`: subthreshold Drive reaches P without A/B
   traversal; neither local participation state changes.
7. `same_total_wrong_path`: the same traversal quantity occurs on B; A remains
   baseline.
8. `contact_fanout`: Ca has two actually traversed outgoing ARROWs; Modulation
   at Ca supports both, preserving CPC0 granularity.
9. `late_return`: delay 1024 produces zero A/B support while structure remains
   live and quiescent.

This is `9 * 2 roots * 10 phases = 180` physical control cases and `360`
mechanics rows.

Total frozen matrix: `620` physical cases and `1240` mechanics rows.

## Representation independence

Every world is reconstructed twice under each mechanics. Require:

- exact same-mechanics replay;
- exact ordered retained `PhysicalTransition` equality between Reference and
  Production, with no sorting or normalization;
- exact candidate participation/support state equality;
- exact physical work, durable body, clock, pressure phase, final retained
  behavior, and quiescence.

Candidate state is serialized separately because CPC1 forbids adding new trace
events. `ExecutionCost` and raw cross-mechanics checkpoint bytes are excluded.

The complete artifact set must reproduce byte-for-byte in a fresh E2B worker.
Default core tests and strict Clippy plus feature strict Clippy must pass.

## Decision

- Any curve, locality, renewal, retained-history, or mechanics-equivalence
  failure: CPC1 negative; stop.
- All 620 cases and exact replay pass: CPC1 development positive for graded
  local temporal participation/support only.

CPC2, pressure de-supply, ARC A3-A5, authority, oracle, and `arch.md` remain
unchanged in either outcome.
