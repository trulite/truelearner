# PX3-R Arm B anonymous shared-CELL PROBE v1 implementation audit

Status: **IMPLEMENTATION READY TO FREEZE; EVIDENCE UNSPENT; PX3 AUTHORITY ABSENT**.

## Frozen candidate

- protocol commit: `3c4df518b5bcd570fc94614dc22ef91d53258ac7`, tag
  `px3-r-shared-cell-probe-v1-protocol`;
- protocol SHA-256:
  `7ac42a90fca11d42175acba597a4ac6fed6df47c94b49a5ac8e726ea95d12204`;
- implementation source:
  `crates/px0-physical-correspondence/examples/px3_r_shared_cell_probe_v1.rs`;
- implementation source SHA-256:
  `2268c4445b438ae8e3d4bd6e1cbdc93d5d217c39b0d8200ac1c0a4d8d7f61e4c`;
- organism-visible physical block SHA-256:
  `bd2b8426d8aa5bf4750444bc6414c63189a3464a93478a17f1898d72a2ffef5d`;
- exact lineage start: `873094497ff6eb74363191dc5edc479c7d66de72`;
- exact authoritative PX2 ancestor:
  `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`.

No result or staging artifact exists. No `--probe` command, substrate
propagation, CELL entry, fixture simulation, MICRO, GATE, authority matrix, or
definitive evidence has run.

## Exact physical construction

The implementation instantiates the preregistered complete six-site field.
All six sites are structurally equivalent and exist before a recurrence
schedule is queued. Each site contains two threshold-2 source CELLS at physical
distance 2 from one threshold-2 anonymous local CELL, two weak resistance-1
incoming ARROWs, two ordinary return ARROWs, and one outward crossing ARROW.
The two source CELLS remain physical distance 4 apart, outside the retained
proposal radius. Remote drivers and distractors have no neighbor inside that
radius.

Training enters two physical SPIKEs directly at each participating source.
The retained local proposal law can therefore replace a physically
deallocated weak incoming ARROW at the same local site. Held-out use enters a
remote driver and reaches the source through a fixed coupling-2 ARROW; because
that source firing is ARROW-carried, held-out use cannot propose a missing
ARROW. Anonymous-CELL return is one unit into a reset threshold-2 source, so it
can apply the retained local-return update but cannot autonomously refire the
source.

There is no new CELL creation, substrate-law change, direct source-to-source
or trace-to-trace coupling, downstream-continuation convergence, typed
adapter, lifecycle state, relation record, hidden reset, evaluator deletion,
or evaluator-selected local target. Only the external physical SPIKE schedule
distinguishes contemporary experience.

## Exact namespaces

```text
main and swap                  0x9_B100_0000
layout/order/identity replica  0x9_B200_0000
spacing replica               0x9_B300_0000
stable alternative            0x9_B400_0000
correlated distractor         0x9_B500_0000
blocked return                0x9_B600_0000
absent opportunity            0x9_B700_0000
stale opportunity             0x9_B800_0000
ambiguous three-way           0x9_B900_0000
multiple simultaneous         0x9_BA00_0000
```

Every case is constructed and executed twice in isolated matter with its exact
namespace for complete-state replay. No prior PX3 namespace is reused.

## Evaluator isolation and measurements

The organism-visible block stores only physical substrate handles, physical
identities, and raw CELL/ARROW/SPIKE state. It receives individual physical
source or driver entries. The six-site incidence table, route names,
recurrence schedules, expected sites, comparison clauses, reporting, and
serialization remain outside the block and cannot select an update.

The evaluator serializes all 12 raw incoming resistance/live values, four
separately sorted three-ARROW route strength values, six held-out CELL firing
arrays, six outward-crossing arrays, complete/permanent fingerprints, topology
and storage size, duplicate equality, and all 13 work-ledger counters.

## Pre-evidence validation

- focused formatting: pass;
- focused compile: pass;
- strict focused Clippy with `-D warnings`: pass;
- frozen PX0-PX2 law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- frozen PX2 execution SHA-256:
  `c47d605371d5787cffc7d456f1d9e38168b4b203063fb9dcdeefcf630fa4aed5`;
- frozen PX3 negative source/CSV/report hashes:
  `39ec595fc1204a29083d271ebcadcdb7950c07d1c44e4ce07c0107fca54730ba`,
  `685dc04db32a5785224c62ba5b589fa8e1e37382a8b613f5f2b5e396aa005f38`,
  and `021be8698c010df1e09dc45f2bf9968f2255b6eb7851c38f36fd93be72260d3b`;
- frozen-negative handoff SHA-256:
  `a029f250ed88f8f2fc164e0d2c9042675bf0a8c9ae51c89cf83ad1aa42e4fa9b`;
- organism-visible forbidden-token scan: pass;
- Git whitespace audit: pass;
- authoritative/shared files changed: none;
- broad historical suite: not run because shared code did not change.

After this audit and implementation are committed and tagged, validation may
run only compilation, refusal cases, hash/source audits, and the no-CELL
`--preflight`. The sole preregistered `--probe` command remains unspent until
those checks pass.
