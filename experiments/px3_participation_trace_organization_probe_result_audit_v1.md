# PX3 participation-trace organization PROBE result audit v1

Status: **FROZEN COMPLETE NEGATIVE (CLASSIFICATION E); PROBE SPENT; MICRO/GATE NOT AUTHORIZED**.

## Write-once execution

- frozen execution commit/tag:
  `246506985a468d5be30edfd585aa488bce3f20be` /
  `px3-participation-trace-organization-probe-implementation-v1`;
- persistent E2B sandbox: `i6x9gykt9tvp6xfz5z8ra`;
- sole command: frozen `--probe` command from protocol v2;
- evidence marker observed exactly once:
  `PX3_PARTICIPATION_TRACE_ORGANIZATION_PROBE_EVIDENCE`;
- CSV SHA-256:
  `579dd5deebe9054cf385fe8a23b52b7641b516aa51c0a2c0f5bd864718339427`;
- Markdown SHA-256:
  `6c41f4433ea3d3a58f1befa52b966926f4c2f2df5b5d1eec48e889c2c86748d3`;
- rows: `24` data rows in frozen seed/scenario order;
- replay: `24/24` exact;
- natural quiescence: `24/24`;
- proposals: exactly six symmetric candidates per row;
- staging remnants: absent;
- native work: `27,118` recorded operations.

No implementation change or rerun occurred. PX0--PX2 remained byte-exact.

## Result

The emitted classification was **NEGATIVE PROBE**, with `0/24` frozen row pass
bits. The material scientific failure is narrower and more informative than
that aggregate count.

The authoritative participation distinction itself worked:

| Input | Actual trace firings | Pair-opportunity firings | Held-out reusable consequence |
|---|---:|---:|---:|
| A(1) alone | `1` | `0` | `0` |
| A(4) alone | `1` | `0` | `0` |
| two scheduled A pulses | `1` | `0` | `0` |
| A then B late | `1+1` | `0` | `0` |
| simultaneous A+B once | `1+1` | `1` | **`1`** |
| recurrent A+B, raw `1+1` | `2+2` | `2` | `1` |
| recurrent A+B, raw `2+1` | `2+2` | `2` | `1` |
| recurrent A+B, raw `4+4` | `2+2` | `2` | `1` |

The two seeds were exact duplicates of these physical counts. Recurrent AB at
all amplitudes selected only AB after the pressure gap; trained AB produced one
held-out consequence and crossed AD, gapped A/B and singleton A each produced
zero. Blocked consequence return and proposal-only also produced zero held-out
consequences.

Thus strong A and repeated A did **not** alias two distinct participants. The
PX1 normalization and threshold-two opportunity used that distinction
correctly.

## Exact collapse: trace arrival is already a local return

PX0 invokes `apply_local_return(target, tick)` on every delivered arrival before
integrating the impulse or deciding whether the target CELL fires. At tick `3`,
the first unit PX1 trace arrival at an opportunity therefore acts as return to
that opportunity's still-eligible priming candidate even when the opportunity
does not reach threshold two.

The physical sequence for one A+B exposure was:

```text
uniform proposal at tick 0
  candidate resistance 1, eligibility through tick 4

first trace unit reaches AB at tick 3
  apply_local_return runs before threshold integration
  candidate resistance 1 -> 4, coupling 1 -> 2
  opportunity has only one accumulated trace unit so far

second distinct trace unit reaches AB at tick 3
  opportunity fires
  already-strengthened coupling-2 candidate traverses

compatible consequence and shared return
  candidate resistance 4 -> 7

pressure gap to tick 50
  candidate resistance 7 -> 2, still live
  one untrained presentation is reusable
```

The same marginal effect is visible without conjunction. A alone left AB, AC
and AD candidates temporarily live at resistance `4`; B alone left AB, BC and
BD temporarily live at resistance `4`. None of their pair opportunities fired,
and gap pressure later removed them, but this proves that the reservoir's
candidate plasticity was already being credited by one participant.

Consequently a genuine A+B exposure receives two kinds of support in one
presentation: premature marginal support from trace arrival and compatible
consequence return. Its resistance reaches `7`, so the frozen gap cannot enforce
the required one-presentation refusal. The two returned evidence units are not
physically distinguished by the existing PX0 local-return law.

This is a self-evidence collapse at the candidate opportunity CELL. It is not a
raw-amplitude alias, same-path repetition alias, evaluator-selected pair, or
failure of the convergence reservoir to detect distinct traces.

## Serialization and aggregate-pass caveats

Two implementation caveats do not change the material negative:

1. PX0 exposes candidate liveness/resistance but not coupling. The frozen
   executable inferred coupling `2` only when current resistance was at least
   `4`. After pressure, resistance can fall below `4` without coupling falling;
   therefore low-resistance `coupling_after_*` fields are not authoritative.
   Native firings, crossings, liveness, resistance, held-out consequences,
   fingerprints and work remain authoritative.
2. The conjunctive row predicate expected all unused candidates to be absent at
   the immediate `after_train` observation. In proposal-only, that observation
   occurs before the pressure gap, so its six unsupported resistance-one
   candidates are correctly still live even though all deallocate by tick `50`.
   Several controls therefore have a false row pass bit for a stricter
   intermediate-state expectation. This explains part of `0/24`, but not the
   decisive failure: `ab-one-overlap` survives the gap and produces a held-out
   consequence in both seeds.

No corrected serialization or predicate run is authorized. The write-once
physical evidence is sufficient to classify the arm.

## Classification and handoff

This is **Classification E: an interpretable complete matrix is materially
negative**.

Formation and amplitude/timing discrimination partly work, but the frozen
hypothesis fails because one compatible A+B presentation becomes reusable.
Ordinary target-local return cannot distinguish an incoming participation unit
from consequence return while the proposal candidate is eligible.

PX3 does not advance to MICRO. The obsolete-organization reversal and recursive
GATE questions remain unspent and unauthorized. Any future PX3 reopening must
begin from this frozen collapse and preregister how candidate opportunity is
kept available without allowing its input traces to reinforce it before joint
opportunity firing. It may not reinterpret or rerun this PROBE.
