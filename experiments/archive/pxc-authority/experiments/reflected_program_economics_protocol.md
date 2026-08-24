# Frozen protocol: RP0b reflected-program economics

Status: preregistered before RP0b implementation, smoke, or execution.
Protocol version: `reflected-program-economics-rp0b-v1`.

Disposable branch: `research/reflected-program-economics-rp0b`. Frozen parent:
`rp0a-reflected-program-discovery-positive`
(`8faefa0c3e92965c61513ce802bc11cc2654ec63`). The parent tag, TrueLearner
trunk, tau/d2 work, and every F0/RP0a artifact remain unchanged.

Frozen RP0a references:

- implementation tag: `rp0a-reflected-program-discovery-implementation`
  (`0e63511143e537ff80164299664ee3f36f45243a`);
- `src/reflected_program_discovery.rs`:
  `51422c9239842dfc6da04250454f6bcf005ef75c80f9bda78f2055ea9580f530`;
- RP0a protocol:
  `87bfc89d031c1c1ccefa48160581fa931ba333b329fe3be7d8590b973c378eaf`;
- RP0a CSV:
  `215f8d18e611585f6d7416ab57ce5072450fad18d4d03750c6199ced2fbf5235`;
- RP0a generated report:
  `8318fec3e5144083ec79051e4556d179f795cbfb2a9537ac0275c7e18858b8f2`;
- RP0a audit:
  `81b6676eca0e3325fb0f0c51c5eab6a4d0cccf4489e0f7ce33eb37b5288ba1a7`.

The preregistration commit contains no RP0b implementation, runtime result,
price result, or conclusion.

## Question and ordered gates

Given a frozen RP0a learner and frozen learned reflected program, does invoking
that reflected program reduce fully counted future execution work relative to
the lower-level direct program, and, only if it does, after how much reuse does
its acquisition and carrying cost repay?

The gates are ordered:

1. **RP0b.1 technical runtime.** Fully counted reflected runtime must be lower
   than direct lower-program runtime. Prices and reuse horizon are absent.
2. **RP0b.2 amortization.** Run only if RP0b.1 passes. Derive lifetime cost and
   break-even from the frozen physical measurements.

No price, storage assumption, or reuse horizon may rescue an RP0b.1 failure.

## No new capability

RP0b changes accounting and evaluation only. It does not change:

- anonymous provenance or role learning;
- P3 recurrence threshold;
- P0 proposal, probation, eligibility, credit, consolidation, or pruning;
- learned topology, route choice, lower runtime, queue behavior, or finish;
- invocation recognition, temporary binding, or target resolution;
- the task distribution or correctness criterion.

No index, cached context-to-entry shortcut, evaluator jump, new route, compiled
opcode, semantic role, semantic adapter, attention, gate, or second reflection
level is allowed.

## Reconstruct and freeze the RP0a states

The RP0a result did not serialize private learner objects. RP0b therefore
replays only the eight frozen integrated RP0a seeds through the unchanged
learner and immediately freezes the resulting role patterns and four-arrow
programs. Replay work is acquisition work, not free setup.

Before any runtime comparison, each replay must reproduce these immutable RP0a
endpoints exactly:

| Seed | First roles | First success | Competence | Acquisition work | Roles | Correct arrows | Bytes |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 4 | 581 | 9654 | 10740059 | 10 | 4 | 6608 |
| 1 | 4 | 197 | 3874 | 4452016 | 10 | 4 | 6608 |
| 2 | 4 | 117 | 19500 | 21480745 | 10 | 4 | 6608 |
| 3 | 4 | 2481 | 14838 | 16321757 | 10 | 4 | 6608 |
| 4 | 4 | 145 | 13260 | 15321640 | 10 | 4 | 6608 |
| 5 | 4 | 117 | 20842 | 23786489 | 10 | 4 | 6608 |
| 6 | 4 | 57 | 16803 | 18869627 | 10 | 4 | 6608 |
| 7 | 4 | 329 | 2388 | 2676006 | 10 | 4 | 6608 |

Every state must also retain exactly four consolidated evaluator-correct
arrows, reproduce `64/64` frozen held-out answers and `640/640` role transfers,
retain zero concrete source identities, and pass duplicate read-only fingerprint
checks. Any mismatch invalidates RP0b rather than creating a new RP0a result.

Consolidation already installs the persistent structures and is included in
acquisition work. RP0b therefore records installation work as the observed
additional work after acquisition; under the frozen implementation it is
expected to be zero, not fabricated. Maintenance is measured from actual
read-only use and is likewise not assigned a synthetic nonzero label.

## Frozen evaluation workload

For every frozen seed, evaluate 16 fresh read-only chains at each depth:

`5, 8, 16, 32, 64, 128`.

Each chain includes eight irrelevant relations. The three arms receive the
identical serialized chain, opaque identities, and starting state. No arm may
learn, consolidate, update support, change topology, or retain temporary state.

### A. CONCRETE

The frozen direct lower program executes the chain through the same lower
cells, arrows, spikes, temporary state, queue, and finish behavior. Its four
functional lower routes are already present. Count the external invocation,
every dynamic direct-route firing, lower cells/spikes/queue work, and identity
comparisons. It performs no reflected provenance recognition.

### B. REFLECTED

The frozen learned RP0a state receives fresh anonymous execution provenance,
matches its learned roles, constructs temporary lower-location bindings,
activates/evaluates its learned four-arrow program, resolves targets, and then
executes the same lower task. Count all work from:

- invocation;
- provenance events and relations;
- recognition comparisons;
- learned-role activation;
- temporary binding construction and lookup;
- arrow evaluation and dynamic firing;
- target resolution/routing;
- residual lower cells, spikes, queues, and identity comparisons;
- maintenance attributable to use.

No evaluator-created entry or target may enter this arm.

### C. ORACLE ENTRY

Diagnostic lower bound. Supply the correct reflected role/program entry and
skip recognition and temporary target discovery, but execute the same lower
task and count invocation plus every dynamic route firing. Oracle information
never enters REFLECTED and cannot establish a positive gate by itself.

## Physical accounting

One unit is one actually executed primitive event in the frozen harness.
Runtime rows report separately:

- external invocations;
- provenance events and causal relations;
- persistent-pattern comparisons;
- role activations;
- temporary binding writes and reads;
- arrow candidate evaluations;
- target-location comparisons;
- dynamic route-arrow firings;
- lower cell activations;
- spike enqueue/dequeue;
- queue checks;
- opaque identity comparisons;
- maintenance updates;
- total runtime.

Map/container operations that implement binding or target resolution are
charged explicitly rather than hidden behind a host lookup. Dynamic route
firings are counted on every use, not deduplicated by arrow identity. All
fingerprints and workspace lifecycles are reconciled.

Acquisition, installation, runtime, maintenance, and permanent bytes remain
separate physical quantities. Wall-clock time is diagnostic only.

## RP0b.1 conjunctive technical gate

RP0b.1 passes only if all conditions hold:

1. frozen ancestry and the RP0a hashes above match;
2. all eight reconstructed states match every frozen endpoint;
3. all three arms answer every query identically and correctly with explicit
   answer, natural quiescence, zero fallback, and zero activity-limit hits;
4. REFLECTED uses only its frozen learned roles/arrows and fresh anonymous
   provenance, with no oracle entry or semantic routing;
5. duplicate evaluation is deterministic and every permanent fingerprint is
   unchanged;
6. all work rows reconcile and every temporary workspace is destroyed;
7. total REFLECTED runtime is strictly lower than CONCRETE for every seed/depth
   cell and in aggregate.

Also report whether absolute savings and fractional savings improve
monotonically with depth. Scaling improvement strengthens a positive but is not
allowed to rescue any failed technical cell.

If condition 7 fails, RP0b.1 is a frozen technical negative and RP0b.2 is
recorded `NOT_EVALUATED`. Stop without an economic sweep.

## Conditional RP0b.2 economics

Run only after a full RP0b.1 pass. Let:

- `A` be observed acquisition plus installation work;
- `R` be mean fully counted reflected runtime per use;
- `C` be mean fully counted concrete runtime per use;
- `M` be observed per-use maintenance;
- `B` be retained bytes;
- `p` be carrying price in work units per byte per invocation.

For horizon `H`:

```text
delta_cost(H, p) = A + H * (R + M + B*p - C)
```

Frozen carrying-price sweep:

`0, 0.000001, 0.00001, 0.0001, 0.001`.

Frozen reporting horizons:

`1, 2, 4, 8, 16, 32, 64, 128, 256, 1024, 10000, 100000, 1000000, 10000000`.

A finite break-even exists only when `C > R + M + B*p`; then report the
smallest integer `H` with nonpositive delta. Prices never affect learning or
RP0b.1.

## Outcome taxonomy

- RP0b.1 fails: functional fractality only.
- RP0b.1 passes but no finite/practical RP0b.2 break-even: cheaper learned
  computation with uneconomic development/carrying.
- RP0b.1 and RP0b.2 pass: economically useful one-level fractality.
- finite break-even plus improving depth/repetition advantage: scaling evidence
  for persistent reflection.

Recursive F1 remains blocked unless RP0b.1 passes and RP0b.2 demonstrates a
finite break-even under at least the zero-carrying condition. No RP0b outcome
modifies the frozen RP0a positive.

## Permitted pre-definitive work and execution boundary

After this protocol is committed and tagged, implementation may perform source
edits, non-Rust local audits, E2B formatting/compilation/Clippy/tests, frozen
RP0a parity tests, accounting tests, and one excluded diagnostic smoke using an
oracle-constructed program state. That smoke may validate accounting plumbing
but may not evaluate a frozen definitive learner seed or establish RP0b.1.

All Rust work runs through the persistent E2B runner at
`/Users/satya/work/br/truelearner/scripts/e2b_persistent.py`. The runner must
reuse and leave its sandbox alive. Local work is limited to source/document
edits, read-only audits, and Git packaging. The definitive RP0b command may run
exactly once.

A valid negative is frozen without changing invocation, installing a shortcut,
adding a learner, changing prices, omitting overhead, or proceeding to F1.
