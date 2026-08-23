# CJ0-E transient-coincidence development GATE handoff v1

Status: **DEVELOPMENT GATE POSITIVE; CJ0-E COMPLETE; HARD STOP AT GATE**.

## Outcome and classification

CJ0-E identifies a single substrate-native development-positive conjunction
law using no new persistent state. PROBE, corrected fresh-identity MICRO v2,
and terminal GATE all pass. The v1 MICRO negative remains immutable and is not
reinterpreted as positive evidence.

Classification: **development-positive at GATE**. This handoff makes no claim
beyond the authorized development boundary. No later-stage protocol, command,
artifact, simulation, or marker was created or entered.

## Exact law

The law is the smallest tested predicate over existing transient CELL state:

```text
decay target CELL to current tick
live_before = target.state > 0
add current physical impulse

threshold == 1:
    ordinary threshold/refractory firing
threshold > 1:
    require live_before plus threshold/refractory satisfaction

on transient-completed firing:
    traverse ordinary outgoing ARROWs
    expose the unchanged local proposal operation
```

`live_before` is execution-local and derived from the already serialized CELL
state. It adds no persistent byte, contributor identity, relation key, member
list, semantic flag, historical record, evaluator signal, or level marker.
Threshold remains ordinary physical matter.

Exact generated physical-source SHA-256:
`e64c8c915c2fbc4679d1e34ee69ecfe36e2c5ff05bdff5d7feeb5a55578bf1c1`.
The frozen authoritative-parent law remains SHA-256
`3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

## Physical consume/produce contract

Consumes:

- decayed scalar CELL state;
- current SPIKE impulse, tick, phase, target, and generation;
- existing threshold and refractory state;
- ordinary ARROW topology, delay, coupling, resistance, eligibility, returned
  activity, and pressure;
- symmetric local opportunity and ordinary external activity.

Produces:

- subthreshold transient state after the first physical contribution;
- firing only when another arrival reaches threshold while that state is live;
- ordinary outgoing traversal and weak local proposal from that firing;
- local returned-activity support only for an ARROW that actually traversed;
- reusable output structure under recurrence;
- ordinary weakening/deallocation without recurrence;
- fresh generic reproposal after full deallocation;
- natural queue drain without reset, cutoff, or autonomous source recurrence.

## Frozen lineage

### Preserved v1

| boundary | commit | tag |
|---|---|---|
| protocol | `c179fd0e40ba32eb9f59e6ab67b3c162c65be09e` | `cj0-e-transient-coincidence-development-protocol-v1` |
| implementation | `2596ce2b9c561fd20bb4480e3258d26c0117d6eb` | `cj0-e-transient-coincidence-development-implementation-v1` |
| PROBE positive | `15ae29b042835a4baea43db6ec69bd9da1017d28` | `cj0-e-transient-coincidence-probe-v1-positive` |
| MICRO v1 negative | `3f3163bb9d401fd469c87a8bc80bf526918ac369` | `cj0-e-transient-coincidence-micro-v1-frozen-negative` |
| v1 negative handoff | `5e596d85abda83737e30e68103b2eac1a1a2686a` | `cj0-e-transient-coincidence-frozen-negative-handoff-v1` |

All listed v1 paths and tag targets remain byte-exact.

### Fresh correction and GATE

| boundary | commit | tag |
|---|---|---|
| MICRO v2 protocol | `8efeacb478c7614b05e39b5f842dc168136e4662` | `cj0-e-transient-coincidence-micro-v2-accounting-protocol` |
| MICRO v2 evaluator | `5d015cc92541b40c23f271b847bea6b0606a493a` | `cj0-e-transient-coincidence-micro-v2-implementation` |
| MICRO v2 positive | `beb2c8ec291377aefad61edfaa06b52593e5bb04` | `cj0-e-transient-coincidence-micro-v2-positive` |
| GATE protocol | `27debdb70c3130e75ec4208f19bce2e235292954` | `cj0-e-transient-coincidence-gate-v1-protocol` |
| GATE evaluator | `33d9be022e8171b0f9d07991be8735ff9fca9340` | `cj0-e-transient-coincidence-gate-v1-implementation` |
| GATE positive | `7c763e549d9a9e801fc79eb9923dce0a8fc163d6` | `cj0-e-transient-coincidence-gate-v1-positive` |

## Artifact hashes

| artifact | SHA-256 |
|---|---|
| MICRO v2 protocol | `53c58316620f4bf528a1e29da08118e70897adc2661cf844be53b68bd43e4998` |
| MICRO v2 implementation audit | `81f0ad27f8c86655dfe0df352aeaeb988fdfbd68bc29b91a3658d586b17f1e2b` |
| MICRO v2 evaluator | `f72f481bfaf0462bcadf7a65179a37e2d91b4bfda32c8aa03501f62a93e82695` |
| MICRO v2 CSV/report | `2aacf3247bed83d23a46d9bd6dba9c0611f72db7e907e91afed4ff2d3378d1e4` / `361252975b6c588a2e2046b83ce9b85fe72ead3718ec2325c304ec784b03cd07` |
| GATE protocol | `3af5a80d2167f9c2009dd858a6d8e8ad47191e8049aa598b517fd68bedfae496` |
| GATE implementation audit | `67b2ae14fc88a65053cb3dc5000bf80426ef19ba5cfa013d0652beaac49fcc9f` |
| GATE evaluator | `2f6d15c6064b43b9e8922dc66c6c9e10f2345f36c6a4070a6d4706858b99122c` |
| GATE CSV/report | `e9bc8868fd8da4623b4798e7932295316d4c6cce3fbc21fe9d9ed6dbb8a5242f` / `c876de189dc22e3bec466e09e9270e187a99903e9465b1edc0e1638b475eee0b` |

## Stage results

### PROBE v1

**PASS**, `17/17` clauses, `8` rows, `36,135` ledgered operations.

- matched route marginals `12|12|12|12`;
- trained A+B/C+D held-out output `1|1`;
- crossed A+D/C+B held-out output `0|0`;
- all singletons silent;
- genuine current crossed participation fired its physical locus but not an
  unlearned output;
- too-late, correlation-only, no-return, absent, stale, ambiguity, mirror,
  replay, and quiescence controls passed.

### MICRO v1

Immutable **FIRST_CLAUSE_FAILURE**, `14/15` clauses, `5` rows, `195,855`
operations. Reversal and bootstrap physics passed. The sole false evaluator
bit compared aggregate local returns with zero and included `33` unrelated A
fanout returns. The row itself preserved learned output `0` and learned support
weakening `15 -> 10`. No v1 byte or tag changed.

### MICRO v2

**PASS**, `20/20` clauses, `5` fresh-identity rows, `195,855` operations.

- old A+B/C+D support `15|15 -> 0|0` and physically deallocated;
- new A+D/C+B support `42|43`, coupling `2|2`, and held-out execution `1|1`;
- old held-out use and one-use historical replay `0|0`;
- full-deallocation bootstrap formed resistance-`4`, coupling-`2` structure
  without mature output first, then executed held-out;
- repeated A alone: learned traversal `0`, learned output `0`, learned support
  increases `0`, same generation/coupling, resistance `15 -> 10`;
- unrelated aggregate fanout returns remained explicitly serialized as `33`.

### Terminal GATE

**PASS**, `8/8` top-level clauses, `83` rows, `18,513,426` serialized
operations.

Flat matrix:

- `8 seeds x 4 strata x 2 blank executions = 64` rows;
- all route firing marginals `52|52|52|52`;
- initial trained support range `8..15`, coupling `2`;
- all initial trained uses active and crossed uses silent;
- every old organization deallocated to zero after reversal;
- changed support range `19..43`, coupling `2`, with new held-out execution;
- old held-out use and one-use replay silent;
- all learned-specific repeated-singleton evidence zero; support weakened;
- distractor firings exactly `0`, `832`, `2,496`, and `4,992` per stratum
  execution for loads `0`, `8`, `24`, and `48`;
- `32/32` exact duplicate fingerprint/metric comparisons passed;
- every finite propagation naturally quiescent.

Common controls: `11/11` rows passed for singletons, too-late completion,
correlation without traversal, traversal without return, absent opportunity,
stale path, genuine current participation, ambiguity/four-way activity,
full-deallocation bootstrap, learned-specific self-evidence, and useful
recurrence without autonomous source firing.

## Recursion, reachability, and time

Same-law recursion passed with no level flag:

```text
A+B -> X: training outputs 7
X+C -> Y: training outputs 6
Y+D -> Z: training outputs 5
held-out full chain -> Z: 1
```

Primitive and learned participants were ordinary threshold-3 CELLS with the
same activation and outgoing interface. The recursive row used `30` ARROWs,
`2,400` persistent bytes, `4,176` operations, and naturally quiesced.

Ordinary convergent reachability required no additional law:

- A->C: `1`;
- B->C: `1`;
- A+B->C: `1`, with existing refractory state suppressing a duplicate
  same-tick firing.

Temporal signatures:

- together: locus `1`;
- physically ordered within the same tick: locus `1`;
- overlap while state remains live: locus `1`;
- B delayed one tick after decay: locus `0`;
- B absent before natural closure: locus `0`;
- B after closure: locus `0`.

No logical operator or persistent timing field was added.

## Validation, leak/source audit, work, and storage

- exact frozen-start ancestry and all v1 tag targets: pass;
- v1 files after MICRO v2/GATE: byte-exact;
- generated physical source across v1, MICRO v2, and GATE: byte-identical;
- formatting, all-target compile, focused tests, strict Clippy for every fresh
  crate: pass;
- missing/wrong-argument refusal and no-CELL preflight: pass;
- atomic create-new/sync/rename outputs: pass;
- CSV shapes: MICRO v2 `5 x 25`; GATE `83 x 26`;
- generated physical-source forbidden-token audit: zero matches;
- changed-path audit from the frozen start: `29` additions, zero modified or
  deleted paths;
- shared authoritative sources changed: none;
- staging paths: absent;
- total lane work across PROBE, MICRO v1, MICRO v2, and GATE:
  `18,941,271` operations;
- all atomic result storage: `42,991` bytes;
- lane-added tracked storage before this handoff: `198,061` bytes;
- broad historical suite: not run because no shared source changed.

## Terminal boundary

CJ0-E development is complete at GATE. The exact candidate is positive only
within the executed development scope. PX3 and the parked later physical
stages are not advanced by this handoff. Any later program decision requires
separate user authorization and must preserve every v1/v2/GATE protocol,
implementation, result, tag, and handoff byte-for-byte.
