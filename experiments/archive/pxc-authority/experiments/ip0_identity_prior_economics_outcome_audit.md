# IP0 identity-prior economics outcome audit

Protocol: `identity-desupply-ladder-v1/ip0`

Outcome: **EXECUTION PRIOR ADVANTAGE** for every frozen workload, ownership
view, and carrying price. The identity scaffold is completely classified.

## Frozen execution

The single definitive command was executed once from implementation tag
`ip0-identity-prior-economics-implementation` at commit
`7ae287b4c75baaf2ed07727863f20ace88544f1d`:

```text
cargo run --release --bin ip0_identity_prior_economics -- --definitive
```

Execution occurred in persistent E2B sandbox `iv7qfq154p7ffq4xpxw0o`.
The sandbox was left running. IP0 did not execute or alter any parent learner,
kernel, environment, or definitive experiment.

## Write-once artifacts

- definitive CSV: `results/ip0_identity_prior_economics.csv`;
- CSV SHA-256:
  `ce6a09d9511e256edfe875c225ff2d7a76125f0a9fe7d015be755c957f2ff9bd`;
- definitive Markdown: `results/ip0_identity_prior_economics.md`;
- Markdown SHA-256:
  `cfae16dd4300918dc89b97ee0866ba712b635f53127e2048ac86d394979a8658`.

The CSV has a fixed 39-column schema with zero malformed rows:

| Row type | Count |
|---|---:|
| exact accounting | 480 |
| completed scaffold ledger | 1 |

The 480 rows cover the full Cartesian accounting matrix:

```text
8 definitive seeds
* 6 scale/probe workloads
* 2 ownership views
* 5 carrying prices
= 480
```

Each workload contributes 80 rows; each ownership view contributes 240; each
price contributes 96.

## Frozen physical result

All 480 accounting rows reproduce:

```text
generic learned correspondence - compiled learned correspondence
    = 12 work/use

compiled learned correspondence - supplied SAME
    = 6 work/use
```

Compilation acquisition is 988 work. Its zero-price physical break-even
against the generic learned mechanism is therefore:

```text
ceil(988 / 12) = 83 uses
```

This maturation is economically useful within the SAME-less organism.

## Comparison against supplied SAME

### Blank-start ownership

The learner pays:

```text
A = 860 work
C = 988 work
G = 0 work, because CS0b was preregistered SKIPPED
I = 0 work
S = 106 marginal bytes
M_use = M_time = 0 work
```

At zero carrying price:

```text
Delta(H,d) = 860 + 988 + 6H
           = 1848 + 6H
```

This is positive for every `H >= 0`.

### Exact learned asset already owned

The incremental view charges no marginal acquisition or storage to another
use of that exact asset instance:

```text
Delta(H,d) = 6H
```

This is positive for every `H > 0`.

The prices `1, 10, 100, 1000` millionths of work per byte-use only add a
nonnegative carrying term. They cannot reverse the positive six-work mature
slope. No accounting row has finite break-even against supplied SAME.

Every fixed-cost, marginal-byte, per-use slope, runtime delta, compilation
break-even, and blank break-even field was independently recomputed from the
CSV. There were zero formula discrepancies.

## Exact protocol classification

Every row satisfies:

```text
W_compiled(d) > W_supplied(d)
```

Therefore all 480 rows select exactly:

```text
EXECUTION_PRIOR_ADVANTAGE
```

Neither developmental-prior parity nor learned-specialization advantage was
observed at any depth, population, recursive scale, seed, ownership view, or
carrying price.

## Completed scaffold ledger

| Property | Filler correspondence |
|---|---|
| Architecturally necessary | No |
| Reconstructable | Yes |
| Functionally sufficient | Yes |
| Recursively compatible | Yes |
| Compilable | Yes: 18 -> 6 work/use |
| Learned compilation pays versus generic | Yes: 83 uses |
| Learned version pays versus supplied prior | Never |
| Developmental accelerator | Yes |
| Mature execution accelerator | Yes: 6 work/use |
| Scaling-enabling prerequisite | No |
| Recursion-enabling prerequisite | No |
| Classification | Execution prior advantage |

The `Functionally sufficient` row is the frozen FFS-SAME0 B result consumed by
IP0. The flat CSV ledger records this through its reconstruction and recursive
compatibility fields rather than a redundant dedicated column; this audit
makes the completed scaffold view explicit without modifying the write-once
artifact.

## Final identity-scaffold claim

> Supplied filler equality is a nonessential but economically valuable
> substrate prior. Its function can be reconstructed, compiled, and
> recursively reused without changing endogenous fractal organization.
> Supplying it avoids correspondence acquisition and persistent storage and
> removes a fixed six-work mature-use premium.

Equivalently:

> SAME is an optional primitive that permanently lowers the execution cost of
> a function the organism can otherwise learn for itself.

This closes the identity branch. DS1 boundary-role de-supply is now
scientifically unblocked, but no DS1 implementation is part of IP0 and no DS1
code was opened before this outcome.
