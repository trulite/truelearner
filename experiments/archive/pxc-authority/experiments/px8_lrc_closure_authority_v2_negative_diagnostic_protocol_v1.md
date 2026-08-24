# PX8 LR-C authority-v2 negative diagnostic protocol v1

Status: **PREREGISTERED; V2-DIAGNOSTIC EVIDENCE UNSPENT; NOT AUTHORITY**.

## Frozen parent and scope

This protocol is the first child of immutable authority-v2 negative commit
`eee2273ec647f9cfe12a050aeb9ff9ab3109af8a`, tagged
`px8-lrc-closure-authority-v2-negative-v1`.

The workflow may localize the v2 failure only. It may not rerun authority v1
or v2, emit either authority evidence marker, claim PX8 promotion, alter
manifest v5, run taxonomy/comparator, or claim final PX-C authority.

Frozen inputs are:

| artifact | SHA-256 |
|---|---|
| active PX8 mechanism | `8623cae7dc1b14e666140a192c49e59ae72df681bc14f0359f0c4465e0d11e8f` |
| retained LR-C law | `7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10` |
| retained PX4 API | `a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71` |
| retained PX7 source | `d248a8af479872d8148115a405ae7332f7d24ca229378d3fde898ffd3d19e63e` |
| frozen authority-v2 evaluator | `e1a830e15c898b113f295d74e22f6dee1d144bd43ee1aa177d4a7c0ef075043c` |
| authority-v2 protocol | `a47866460ecc4504ee713e0b425d049e7816f48e4aa18bceeb0a1705dcbc5328` |
| authority-v2 negative record | `2a527fecb9906e4bdf4bce703a760e646439f205ec2eee6a8288a38de6cc1620` |

Only a new evaluator-only diagnostic package, its audit prose/script, and its
result artifacts may be added. The active PX8 mechanism and every retained
PX0--PX7+LR-C source must remain byte-identical.

## Fresh identities and unchanged worlds

Diagnostic roots are exactly `864001..864016`. Primary namespaces are
`root << 32`; compact controls use offsets `10000,20000,...,60000`; retained
PX7 controls use `(root + 1_000_000) << 32`. These identities are disjoint
from authority-v1 roots `861001..861016`, negative-v1 diagnostic roots
`862001..862016`, authority-v2 roots `863001..863016`, isolated development,
and all earlier serial evidence.

Construction/reflection and twists `0,137,274,411` remain balanced. All
physical worlds, inputs, and schedules remain equivalent to authority v2:

```text
formation        learn_twice
complete         reuse all four at 61
incomplete       omit side four at 70
blocked          outward resistance 0, reuse at 61
stale            learn_once_then_age, reuse at 111
compact          direct/open/fork/ring at 0; aged at 10
PX7 cumulative   maturation at 0 and 10; held-out boundary at 20
```

No measured value may alter a later physical input.

## Unconditional complete serialization

The new package `arms/px8-lrc-closure-authority-v2-diagnostic` depends directly
and only on active PX8 and retained PX7. It reconstructs every root twice and
must serialize all `16 * 14 = 224` clause records before making any aggregate
completeness assertion. A failed physical predicate must never prevent another
root or clause from being recorded.

Every clause record must include:

- root, namespace, construction/reflection/twist layout, and complete schedule;
- clause index/name, exact expected predicate, exact actual observation, and
  pass/fail;
- formation/completed/incomplete/blocked/stale/compact crossing, return,
  update, work, queue, and natural-quiescence observations;
- all seven same-body memory pairs with explicit before and after values:
  primary, uninterrupted, incomplete, duplicate, blocked, stale, and retained
  PX7 cumulative;
- maximum work and bytes, pause/resume, all-batch queue exhaustion,
  cumulative PX7 controls, and exact replay;
- first failed clause for the root; and
- first field divergent between independent reconstructions, or `none`.

The report must enumerate every failing root/clause with exact expected and
actual text. Diagnostic completeness is independent of predicate success.

## Firewall and execution

The evaluator may accept only `--v2-diagnostic-v1` and emit exactly one
distinct marker:

```text
PX8_LRC_CLOSURE_AUTHORITY_V2_NEGATIVE_DIAGNOSTIC_SPENT
```

It must not contain or emit either authority evidence marker, accept authority
or prior diagnostic modes, or write any authority result path. Create-new
outputs are exactly:

```text
results/px8_lrc_closure_authority_v2_negative_diagnostic.csv
results/px8_lrc_closure_authority_v2_negative_diagnostic.md
```

Instrumentation, Cargo, serialization, hashes, and the static audit must be
batched. No Rust, project program, or project audit may run locally.

One fresh E2B sandbox may perform the minimal package-only validation:

```text
package rustfmt check
package cargo check
static hash/dependency/identity/firewall audit
```

It must not construct a body. One second fresh E2B sandbox then runs the
diagnostic matrix exactly once and a non-executing result serialization audit.
No workspace-wide build, test suite, Clippy, authority command, or diagnostic
rerun is registered.

## Classification and v3 stop

The frozen result must classify the failure as exactly one of:

1. measurement/evaluator/fixture defect;
2. physical/mechanism counterexample; or
3. new-law fork.

For classification 1, freeze the exact defect and propose the smallest
disjoint v3 measurement repair. Authority v3 evidence must remain unspent
until a v3 protocol, evaluator implementation, coverage/static audit, targeted
validation, and implementation audit are all separately frozen.

For classification 2 or 3, stop before changing the active mechanism or any
retained law. This workflow never creates manifest v6 or a PX-C result.
