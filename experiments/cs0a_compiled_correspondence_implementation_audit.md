# CS0a compiled correspondence implementation audit

Protocol: `identity-desupply-ladder-v1/cs0a`

Status: implementation and development gates frozen; definitive scientific
outcome unspent.

## Frozen ancestry

- FFS-SAME0 positive tag: `ffs-same0-learned-correspondence-positive`;
- FFS-SAME0 outcome commit:
  `2d96d8d603834ddb67e1b883d6a656b16d58a549`;
- FFS-SAME0 definitive CSV SHA-256:
  `d136492e5ddaa70194f155657e7e86eee97da57e8dedcd9d1f52bcf81395a812`;
- FFS-SAME0 definitive Markdown SHA-256:
  `2bf80d0ed37e4d57aa3fe9ba2fb9ec2efc3663c2eb7ee0b701796731520c3c1f`;
- FFS-SAME0 outcome audit SHA-256:
  `79d98b7571d72556610a9a7dc5a33cb6c28df138eef237bed5fdbe5300e4f06b`;
- identity de-supply ladder tag: `identity-desupply-ladder-protocol`;
- identity de-supply ladder SHA-256:
  `493cc50ac67e7c3985b61e7fbe249f19388fed45b026d29dc81ad70f20c9ccbe`.

The CS0a runner checks all four artifact hashes before any development or
definitive report can pass.

## Implementation freeze

- clean CS0a source commit:
  `4c8147e5d45a5b49183ee2a1daa842be7fddbeb3`;
- compiled-correspondence kernel/harness SHA-256:
  `d1513b7dbbc5c8fb7d9453f27b6d3172d5a769337228ee2e967e7eb3aeb48c9b`;
- CS0a runner source SHA-256:
  `6929b945e937cca8a0eed44bfd7d0efb33c1b169dde1e783c049bc5f0add490a`.

Files added:

```text
src/ffs_same0/cs0a.rs
src/bin/cs0a_compiled_correspondence.rs
```

The only change to the frozen FFS-SAME0 module is the declaration of its CS0a
child module. Generic correspondence acquisition, resolution, binding,
execution, accounting, and definitive artifacts are unchanged.

## Narrow mechanism

CS0a adds no correspondence learner or identity representation. It observes
successful uses of the frozen FFS-SAME0 learned correspondence and applies the
ordinary frozen evidence rule:

```text
successful use       +2
failed use           -1
consolidate at        6
prune at             -2
```

Three separate successful uses per relational motif are therefore required
before a compiled route exists. Subthreshold and shuffled evidence produce no
route.

The persistent route contains only:

```text
learned correspondence-asset identity
anonymous source role
anonymous target role
local context
two relational support atoms
parent dependency fingerprint
ordinary strength
local route identity
```

It contains no `OccurrenceId`, `TruthFillerId`, concrete destination, episode,
future identity, answer, level, or economic field. The source audit extracts
the persistent type region and rejects those channels directly.

Each invocation creates a temporary route containing only the current source
and target occurrences plus the learned asset identity. That route has local
call lifetime and is never written into persistent state. Persistent
fingerprints remain identical before and after every arm.

## Event-driven compiled resolution

The mature path is:

```text
current anonymous relation activity
        -> matching learned compiled route activates locally
        -> validate current support and frozen parent fingerprint
        -> instantiate current temporary source/target route
        -> write the temporary binding
        -> ordinary frozen execution
```

A nonmatching novel context, absent compiled route, or stale dependency reopens
the unchanged generic `RuleStore::resolve` path. A stale compiled dependency
does not supply a fallback target. Returning to the compatible frozen parent
reactivates the historical compiled route with zero reacquisition.

## Work attribution

The frozen FFS-SAME0 mature correspondence tax is reproduced exactly:

```text
anonymous observations       4
temporal relations           4
causal relations             4
generic motif lookups        2
generic comparisons          2
ambiguity check              1
temporary binding write      1
------------------------------
generic correspondence      18
```

The development compiled path is:

```text
compiled local activation    1
context/support validation   1
parent dependency check      1
temporary route install      1
ambiguity check              1
temporary binding write      1
------------------------------
compiled correspondence      6
```

Thus development produces:

```text
generic correspondence       18 work/use
compiled correspondence       6 work/use
reduction                    12 work/use  (-66.67%)
```

The complete depth-32 invocation moves from 162 to 150 work. The evaluator-only
supplied-SAME reference is 144, so development shows a partial reduction, not
parity or specialization.

Compilation itself costs 988 work after the frozen 860-work generic
correspondence acquisition and installs two compiled routes occupying 80
bytes. These quantities are reported separately and do not enter the primary
CS0a compatibility gate.

## Development matrix

MICRO uses development seed `99997` and two fresh held-out episodes. GATE uses
development seed `60000` and eight fresh held-out episodes. Neither overlaps
definitive seeds `0..7`; neither writes an artifact.

Both modes pass all fourteen arms:

```text
generic learned correspondence
compiled correspondence
fresh occurrence identities
permuted occurrence allocation
permuted memory order
changed binding/truth population
changed causal context
invalidated parent dependency
historical compatible return
subthreshold evidence
shuffled evidence
missing correspondence
ambiguous correspondence
supplied-SAME evaluator reference
```

GATE results include:

```text
compiled/fresh/permuted/changed-binding correctness       8/8 each
compiled uses without generic reopening                    8/8
changed-context generic reopening                          8/8
stale-parent invalidation and generic reopening            8/8
historical compiled reuse without reopening                8/8
missing/ambiguous exact observable behavior                8/8 each
subthreshold compiled routes                               0
shuffled-evidence compiled routes                          0
```

The stale-parent arm costs 23 correspondence work because it charges compiled
activation, validation, invalidation, generic reopening, and the full generic
18-work resolution. This is deliberate: recovery is not hidden or free.

## Leak and control audit

Fifteen CS0a-specific controls pass. The harness also executes all fourteen
frozen FFS-SAME0 correspondence controls in the same development cell, for 29
controls total:

```text
occurrence relabeling
allocation-order perturbation
memory-order perturbation
same shape / different continuity rejection
different shape / same continuity acceptance
invocation-local occurrence lifetime
covert reused-token detection
evaluator-truth relabeling
missing and ambiguous correspondence
changed context and historical return
permanent-state stability
threshold acquisition
failed/shuffled evidence behavior
```

No old occurrence enters persistent compiled state. No persistent field can
contain the current or future bound target. The source audit and dynamic
persistent-fingerprint audit both pass.

## E2B validation

Persistent sandbox: `iv7qfq154p7ffq4xpxw0o`.

The exact clean implementation commit was validated with:

```text
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -q --lib ffs_same0::cs0a
cargo test --release -q --bin cs0a_compiled_correspondence
cargo run --release --bin cs0a_compiled_correspondence -- --micro
cargo run --release --bin cs0a_compiled_correspondence -- --gate
```

The remote chain exited zero:

- CS0a kernel tests: 3 passed, 0 failed;
- runner/schema tests: 2 passed, 0 failed;
- formatting: PASS;
- all-target Clippy: PASS;
- MICRO: PASS, development only;
- GATE: PASS, development only;
- frozen ancestry: PASS;
- parent fixture: PASS;
- duplicate determinism: PASS;
- persistent source audit: PASS;
- all 29 development controls: PASS.

Legacy definitive experiments were not rerun. Their source and artifacts were
not changed; the child-module declaration only makes the new CS0a target
available. The focused all-target build and CS0a tests cover that integration.

## Stopping boundary

```text
FFS-SAME0 definitive A/B/C       positive, frozen
CS0a implementation             frozen
CS0a development gates          PASS
CS0a definitive outcome         pending
CS0b trigger attribution        blocked on definitive CS0a
FFS-SAME1                       blocked
IP0                             blocked
```

No CS0a definitive command has run. Neither
`results/cs0a_compiled_correspondence.csv` nor
`results/cs0a_compiled_correspondence.md` exists. Development supports only
freezing this implementation for the single preregistered definitive matrix.
The observed `18 -> 6` reduction is not yet a scientific result.
