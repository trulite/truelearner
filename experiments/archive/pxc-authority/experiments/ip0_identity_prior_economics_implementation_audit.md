# IP0 identity-prior economics implementation audit

Protocol: `identity-desupply-ladder-v1/ip0`

Status: accounting implementation frozen after MICRO/GATE validation and
before the single definitive write.

## Capability boundary

IP0 adds no organism capability and contains no learner, correspondence rule,
compiler, grounding route, executor, maintenance behavior, invalidation path,
or environment interaction. It is evaluator-only exact-integer accounting
over frozen write-once artifacts.

The implementation source does not import or call an organism kernel. Parent
CSV files are embedded read-only at compilation and cross-checked against one
another before any classification can pass.

## Frozen ancestry

IP0 consumes:

- supplied-SAME FFS0 positive;
- generic learned FFS-SAME0 positive;
- CS0a compiled correspondence positive;
- the frozen CS0b trigger attribution and `SKIPPED` decision;
- FFS-SAME1 compiled correspondence positive;
- the umbrella identity ladder protocol.

The runner checks fifteen parent artifacts by SHA-256 before any mode passes.
The source ledger additionally verifies:

- FFS0 functional, computational, economic, and adaptive claims;
- FFS-SAME0 correspondence reconstruction, functional recovery, and fractal
  recovery;
- FFS-SAME1 A1-E1;
- eight CS0a acquisition rows with A=860, C=988, two compiled routes, and 80
  compiled bytes;
- all 48 SAME1 scale references against matching FFS-SAME0 rows;
- generic tax 18, compiled tax 6, generic bytes 26, compiled bytes 80, and
  premium versus supplied SAME +6;
- the preregistered CS0b skip condition: grounding slope 2 is positive but is
  less than half of residual slope 6.

No frozen experiment or definitive control is rerun.

## Source freeze

- source commit:
  `4eddf0809dcdcab2738ed4559dd1367a7d52b3c8`;
- accounting kernel SHA-256:
  `9123cbbbfc3468c04bbb287ecef2c4a6ae2bae4bc5042818f2f5d56cc77dfdce`;
- write-once runner SHA-256:
  `d9c6df3e0e4927b1e7c95125142f89d4f2a884441577ceef1bc2bd4833a959a0`.

## Frozen physical quantities

```text
A       generic correspondence acquisition       860 work
C       compiled correspondence acquisition      988 work
G       CS0b grounding acquisition                  0 work
I       installation                                0 work
S       generic + compiled persistent state       106 bytes
M_use   maintenance per use                         0 work
M_time  horizon maintenance/carrying physics        0 work

W_generic  - W_supplied                           +18 work/use
W_compiled - W_supplied                            +6 work/use
W_generic  - W_compiled                           +12 work/use
```

Compilation therefore has physical zero-price break-even against the generic
learned correspondence path at:

```text
ceil(988 / 12) = 83 uses
```

This internal maturation result is separate from comparison against supplied
SAME.

## Ownership views

Every workload is reported twice:

1. `blank-start`: charge A+C once and carry 106 marginal bytes;
2. `exact-asset-already-owned`: charge zero marginal acquisition and zero
   marginal bytes for reuse of that exact existing asset instance.

The second view does not assert that separately learned lookalikes are one
asset. It is the frozen incremental ownership counterfactual. Blank-start
cells remain independently charged because their experiment fixtures learned
separate instances.

## Exact economics

The primary zero-price blank-start comparison is:

```text
Delta(H,d) = 860 + 988 + 6H = 1848 + 6H
```

For an exact already-owned asset:

```text
Delta(H,d) = 6H
```

Both are positive for every positive reuse horizon. The secondary storage
prices are exactly `0, 1, 10, 100, 1000` millionths of work per byte-use.
Accounting is performed in integer millionths:

```text
fixed_micros = (A + C + G + I) * 1,000,000

per_use_delta_micros =
    (W_compiled - W_supplied + M_use) * 1,000,000
  + price_micros * marginal_bytes
```

No tolerance or floating-point comparison exists. A positive slope has no
finite break-even.

## Frozen classifier

Each workload selects exactly one protocol case:

```text
W_compiled > W_supplied  -> EXECUTION_PRIOR_ADVANTAGE
W_compiled = W_supplied  -> DEVELOPMENTAL_PRIOR_ADVANTAGE
W_compiled < W_supplied  -> LEARNED_SPECIALIZATION_ADVANTAGE
```

The scaffold ledger reports independently:

| Dimension | Frozen expected classification |
|---|---|
| Architectural necessity | NO |
| Reconstructability | YES |
| Compilability | YES: 18 -> 6 work/use |
| Recursive compatibility | YES: 0 -> 3 -> 5 -> >=6 |
| Developmental accelerator | YES |
| Mature execution accelerator | YES: 6 work/use |
| Scaling-enabling prerequisite | NO |
| Recursion-enabling prerequisite | NO |
| Value of supplying | development plus fixed 6 work/use |

These are derived from frozen claim rows and physical measurements; no
organism receives them.

## Development modes

- MICRO: seed 0, S1, both ownership views, all five prices; 10 accounting
  rows;
- GATE: seed 0, all six scale/probe cells, both ownership views, all five
  prices; 60 accounting rows;
- DEFINITIVE: all eight seeds and six scale/probe cells, both ownership views,
  all five prices; 480 write-once accounting rows.

MICRO and GATE both produced:

```text
classification                       EXECUTION_PRIOR_ADVANTAGE
generic -> compiled saving           12 work/use
compiled premium vs supplied          6 work/use
zero-price compilation break-even    83 uses
break-even vs supplied               none
parent artifacts consistent          true
CS0b skipped                         true
```

Development output is not claim-eligible and writes no result artifact.

## Verification

The exact source commit was validated locally and in persistent E2B sandbox
`iv7qfq154p7ffq4xpxw0o` with:

- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- three accounting-kernel tests;
- two runner/schema tests;
- release MICRO;
- release GATE.

All passed. The E2B sandbox was left running.

The tabular workflow keeps frozen parent measurements read-only and emits a
separate flat derived ledger with one typed quantity per column. No parent CSV
is cleaned, rewritten, or normalized.

## Definitive lock

The definitive command remains unexecuted:

```text
cargo run --release --bin ip0_identity_prior_economics -- --definitive
```

The write-once files `results/ip0_identity_prior_economics.csv` and
`results/ip0_identity_prior_economics.md` do not exist. DS1 boundary-role code
has not been opened. No rescue or post-classification capability gate is
allowed inside the identity branch.
