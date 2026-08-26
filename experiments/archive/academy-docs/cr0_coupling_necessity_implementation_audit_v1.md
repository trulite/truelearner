# CR0 coupling-necessity implementation audit v1

Status: frozen before physical evidence.

Protocol: `cr0-coupling-necessity-protocol-v1` (`b79107c`).

## Source boundary

The CR0 candidate descends directly from FD1 readiness commit `59e85d5`.
Before this audit, its complete tracked delta contains only:

- `academy/docs/cr0_coupling_necessity_protocol_v1.md`;
- `experiments/arms/cr0-coupling-necessity/Cargo.toml`;
- `experiments/arms/cr0-coupling-necessity/src/main.rs`;
- `experiments/tools/audit_cr0_coupling_necessity_v1.sh`;
- this audit.

There is no physical-core, existing-evaluator, ARC, Academy runtime, authority,
oracle, or `arch.md` change.

Frozen source hashes:

```text
FD1 core lib       e7b9d60ce0330d10692b13fe85967e189d734a00177edef98018f9b4499a09ed
core mechanics     297775ee625d55e116adb92c9f6906c8a5da40e8533bce2fa71cf7bf4b002947
core manifest      c919b87fb2628f23e019a59ec59eab3fefb7faffa3a48fa03e6e9ea4d1ebbb4c
arena-format lib   8c35c3c07fe95b2cc76cbe9ceb47d83f250c5e0c7c40481e7371583afa48a812
CR0 evaluator      8618c4074942ad769e417db3e9ce1126911108511a8aa3adaeb6c0153b699a6d
protocol           7839aa5903e5557e1e677ffde132beedf3edfd5178ac739541bd51b950ca5111
```

## Frozen-state anchors

The evaluator does not invent either compared post-consequence state:

- PX6 authority CSV SHA-256
  `9e14b0f065ba37966c2ffc300f6d149b0847d092cb90e666456d3889d889d9c6`
  contains qualified outcomes at resistance 4 / coupling 2.
- FD1 v3 CSV SHA-256
  `2ca3ae797a079387ff7e9f4413ae5030f380ab997bea520c79460ffac9f95709`
  contains forty Reference/Production one-consequence rows whose durable
  resistance changes from 1 to 4. The frozen FD1 core contains no coupling
  mutation, so coupling remains 1.

CR0 reconstructs the two equal-resistance durable states through ordinary
`ArrowSpec` construction and then runs their future physical continuations
under the one unchanged FD1 law.

## Evaluator shape

The matrix contains:

```text
2 roots
* 10 construction ages
* 10 families
* 2 coupling arms
= 400 physical cases
* Reference / Production
= 800 serialized mechanics rows
```

Each case is independently reconstructed twice per mechanics for exact replay.
Reference and Production compare the complete physical observation within an
arm. Cross-arm equality is not required because different coupling produces a
different physical impulse.

The six retained-behavior probes exercise contact attribution, continuous
participation, one-hop closure, depth-16 closure, equal-resistance forgetting,
and consequence consolidation. Four efficacy controls distinguish already
executable, newly threshold-crossing, still-insufficient, and two-input
topologies.

## Pre-evidence validation

Reusable E2B worker `iwlakum29bs73vxsgu8d0` performed only:

- targeted rustfmt check;
- targeted release check;
- targeted strict Clippy.

The first formatting check and first compile attempt were pre-evidence
development diagnostics. The former reported formatting only; the latter
reported one missing CSV placeholder. Both were repaired before this freeze.
No CR0 physical world has executed.

The next eligible event is one fresh E2B execution of the frozen CR0 matrix,
followed by the frozen static audit. Any failure is an immutable CR0 negative;
there is no in-gate repair.
