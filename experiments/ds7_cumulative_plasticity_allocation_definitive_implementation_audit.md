# DS7 cumulative learned plasticity-allocation definitive implementation audit

Status: **AUTHORITY IMPLEMENTATION FROZEN; DEFINITIVE EVIDENCE UNSPENT**.

Protocol commit/tag:
`be8d6ade88c7a9ace32c5f705b333a60627cc415` /
`ds7-cumulative-plasticity-allocation-definitive-protocol-v1`.

Final no-cell implementation commit:
`e0f381c88b9a6e0c2217494d05f6825510c8f4b9`.

Frozen authority implementation hashes:

- wrapper:
  `f58006bdaf732325221c38b4a1a86aa41a49fc7f474e67ba01a00b11f4b59746`;
- write-once runner:
  `3425a9b0a02f723ba56228fffc6e69a7e2c29734c90c84de43cd734103520ea4`;
- protocol:
  `85887e29737732fd98c7b578560b31cf0874c08210cd6e915c7e6fe06bd67f3f`;
- build-time composition/hash audit:
  `1b6f77dd6f6258440d0c06c15514898a7e18202a736a51d5861384f23d5316ca`.

The wrapper compositionally includes and does not edit the exact frozen
mechanism ancestry:

- DS7 GATE v3 source:
  `abaedd16717543270c5ed0ef2c8a16e3a4c0fed0215764443948c36d4adfa297`;
- M4-linked encounter allocator source:
  `e755a70deada891e5c4db3b55809ca84ea8ad31a8bd3affe564bf08a95f8dff7`;
- M4 lifetime source:
  `3d5659fb26ae804dee6122408f9d703ea1f226349772883075a42686ac3fd110`.

The immutable GATE v2 evidence remains exact:

- negative result:
  `cc0278c7476f50c505d7b8813c326203467b6b8b4e17c07f03188891750fccc9`;
- negative audit:
  `9dbc561d7ec25ad9308df8285454e3b2a0f3c4dfcbb8502b1c1f98cd2cb2e58b`;
- collapse handoff:
  `10d35f4e0c29ead317ab3bd7254a83752877de582cb6f321b2676f187709e477`.

## Wrapper boundary

The only call into the frozen organism is:

```text
fresh explicit seed + explicit load
  -> byte-frozen GATE v3 authority cell
  -> frozen returned measurements
```

The wrapper adds hash, explicit-array, namespace, output-absence, and source
preflight; an independent conjunction over already returned measurements;
matrix aggregation; and create-new serialization. It adds no organism state or
learning input and does not rewrite the encounter snapshot, allocator, route,
eligibility, delayed update, shuffle, repair, M4 lifecycle, or GATE predicate.

The source audit extracts only that call boundary and refuses supplied
endpoint/encounter classes, evaluator candidate/proposal/target sites,
productive/distractor classes, and `LEARN_HERE`. The frozen inner GATE source
also repeats its original information-flow audit in every definitive cell.

## Seed and matrix audit

The source contains one explicit `[u64; 16]` array:

```text
30_000_000  30_500_000  31_000_000  31_500_000
32_000_000  32_500_000  33_000_000  33_500_000
34_000_000  34_500_000  35_000_000  35_500_000
36_000_000  36_500_000  37_000_000  37_500_000
```

and one explicit load array `[8, 32, 128]`, for exactly 48 cells. Bases are
spaced by 500,000. Each owns `[base, base + 400_000)` and the frozen cell's
largest derived identity is `base + 300_005`. The authority region is disjoint
from every DS7 development base `20_000_000..=24_500_000`, every derived
development namespace, and all 18 GATE cells.

Each cell calls a newly constructed frozen `PlasticityPath`; no proposal,
prototype, value, eligibility, identity, or layout state crosses cells. There
is no seed range, alternate seed/load argument, selective-cell mode, replay,
resume, or second matrix evaluation.

## Write-once and refusal audit

The runner has only `--audit` and `--definitive`. `--audit` calls source
preflight only. Both modes refuse unless both final paths are absent.
`--definitive` repeats preflight before the first cell and uses
`OpenOptions::create_new(true)`, complete writes, and `sync_all` for:

```text
results/ds7_cumulative_plasticity_allocation_definitive.csv
results/ds7_cumulative_plasticity_allocation_definitive.md
```

There is no overwrite, append, alternate output, stdout-only result, tuning,
rescue, or development mode. Both paths were absent locally and in the clean
E2B snapshot at this freeze.

## Fresh E2B validation without a cell

Dedicated authority state:

```text
/Users/satya/.cache/truelearner/ds7-cumulative-definitive-authority-e2b.json
```

Fresh dedicated sandbox: `iytocn10evr287oxm307n` using template
`truelearner-rust-1-97-worker`. It is distinct from DS6 authority sandbox
`i4p3zohev8t7uabpip91o` and DS7 development sandbox
`iyrkw7af5qpmwwfmq3bwm` and remains running.

The first validation command stopped at `cargo fmt --check`; formatting was
performed in E2B, downloaded, and committed. A later focused no-cell test
stopped because the wrapper source marker selected its own earlier string
literal rather than the final call marker. The parser was corrected to select
the final marker and committed. Neither stop called a definitive cell or
created a result path.

From clean commit `e0f381c88b9a6e0c2217494d05f6825510c8f4b9` in the same
fresh authority sandbox:

```text
cargo fmt --all -- --check                                      PASS
cargo check --bin ds7_cumulative_plasticity_allocation_definitive PASS
focused no-cell exact source/namespace preflight test           1/1 PASS
focused no-cell existing-output refusal test                    1/1 PASS
release --audit                                                 PASS
```

The final preflight reported every source lineage group true, explicit seeds
true, explicit loads true, namespaces disjoint, development disjoint, outputs
absent, and overall PASS. No definitive cell, report function, result file, or
`--definitive` command executed.
