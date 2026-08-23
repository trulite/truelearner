# PX5 physical plasticity-allocation GATE v2 protocol

Status: **PREREGISTERED FOCUSED RETRY; EVIDENCE UNSPENT; AUTHORITY ABSENT**.

## Frozen basis and immutable negative

This retry starts at frozen GATE v1 negative commit
`2107558b6cb6b13f501bdbc50924cb493dbcf8fe` and retains authoritative PX2 parent
`2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`.

| frozen input | SHA-256 |
|---|---|
| unchanged PX0--PX2 law | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| GATE v1 implementation | `5e267b344d698fd8ef9833754e2d99e9c2813bddfaff9f7d42a967a017b8af71` |
| immutable GATE v1 CSV | `e50e9c88cc26e8b418b8264a3cb7cb71973e661d5001c76a6c05c28f4a79747c` |
| immutable GATE v1 report | `db5e76774cf160d58e7050c4dcafa2ae484b80d531cf673e830972883c871299` |
| GATE v1 negative audit | `c490b14a1525ef38666307f8fec83e73aa22dfa809a1e5a38807f652cbb13148` |

GATE v1 remains an unchanged `132/144` negative and must not be rerun or
reinterpreted as a pass.

## Unique mechanical correction

GATE v2 is byte-for-byte equivalent in physical law, matrix factors, primary
schedule through tick `62`, loads, controls, claims, observations, work/storage
accounting, and duplicate execution except for the following exact changes:

1. the twelve fresh namespaces are
   `0x5_8000_0000 + cell * 0x0100_0000`;
2. while useful source `0` is withheld after tick `62`, every other useful
   source fires and receives ordinary return at
   `70/72, 80/82, 90/92, 100/102, 110/112, 120/122, 130/132, 140/142`;
3. the substrate advances to tick `150`; only then is source `0` fired for
   generic reacquisition and returned at tick `152`.

The longer fixed horizon supplies enough ordinary scalar pressure to deallocate
the frozen hot-edge resistance `10`. It is the sole resolution preregistered by
the GATE v1 negative audit. No mechanism, parameter, threshold, pressure rate,
eligibility window, topology, return, representation, clause, or scientific
claim changes.

## Matrix, controls, and claims

The exact twelve-cell matrix remains:

- loads `32, 64, 128`, four cells each;
- useful route counts `4, 8`;
- local distances `1, 2`;
- normal/mirrored placement;
- normal/reversed CELL allocation;
- normal/reversed SPIKE insertion;
- fresh independent duplicate reconstruction.

The primary hot/warm schedule through tick `62`, repeated return-free
distractors, matched return-free recurrence, late-return, outside-radius,
evaluator-only shuffle, and matched physical-return permutation controls are
exactly those frozen in the GATE v1 protocol. Every environmental physical
opportunity is listed there and above; no hidden fixture schedule is permitted.

The same `P0--P11` conjunction applies. GATE v2 passes only at `12/12` cells and
`144/144` claims. Any failure is a new immutable negative.

## Boundary, validation, and one-shot execution

Only byte-identical retained CELL/ARROW/SPIKE physics executes. All GATE v1
forbidden representations and paths remain forbidden, including typed
adapters, old M reconstruction, semantic labels, evaluator-selected local
paths, and mechanisms/evidence from other parallel lanes.

Pre-evidence validation must prove the source delta against GATE v1 is confined
to protocol/artifact constants, fresh namespace, the fixed recurrence loop,
and ticks `150/152`; then pass formatting, focused tests/build, strict Clippy,
frozen hashes, zero dependencies, no-cell preflight, refusal without `--gate`,
and artifact absence.

Execute once:

```text
cargo run --release -p px0-physical-correspondence \
  --example px5_physical_plasticity_allocation_gate_v2 -- --gate
```

Atomic outputs:

```text
results/px5_physical_plasticity_allocation_gate_v2.csv
results/px5_physical_plasticity_allocation_gate_v2.md
```

A positive result establishes only non-authoritative PX5 development readiness
and classification A. No authority workflow may be started or simulated.
