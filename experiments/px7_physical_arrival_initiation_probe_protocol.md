# PX7 anonymous physical arrival initiation PROBE protocol

Status: **PREREGISTERED; DEVELOPMENT EVIDENCE UNSPENT; PX7 AUTHORITY ABSENT**.

## Frozen start and isolation

This independent development lane starts exactly at authoritative PX2 commit
`2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`, tag
`px2-physical-causal-direction-authoritative`.

| frozen input | SHA-256 |
|---|---|
| authoritative PX0 law | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| retained substrate law | `6aa28a76e1362ac8dfb1d33fb68807da40e7604dfdc8cca9efa1e314e3ce4263` |
| PX2 definitive CSV | `921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18` |
| PX2 definitive result audit | `7076aca03014d19040020b6bfb126e92f7d25dcac3df9cdab92de7dd7849c6fe` |
| PX2 authority handoff | `98647ab1563593e18e345cd7e5a71c4991d18b397dfe2dec71a4756106d96509` |
| old M0--M8 behavioral ledger | `9bd352ee3b7d0b729aa27e411fd18601d3cef7a0ac3280442980eb3cd132b146` |

The old M7 entry supplies only the frozen behavioral question. No old M7
source, schema, mechanism, evidence, or post-PX2 lane artifact may be linked,
copied, called, or used to choose organism-visible execution.

This protocol does not advance PX3--PX8, change PX0--PX2, create an
authoritative ancestor, or authorize a definitive matrix.

## First question and mechanically unique hypothesis

> Does anonymous physical arrival already initiate retained execution through
> authoritative PX0--PX2 correspondence, participation, causal direction,
> recurrence, and quiescence, with no new mechanism?

The first candidate edge is already present in the frozen law:

```text
anonymous external SPIKE arrival
-> ordinary CELL firing
-> weak local ARROW proposal
-> actual traversal leaves the existing ARROW-local participation trace
-> ordinary returned SPIKE overlaps that trace
-> the traversed ARROW gains persistence and ordinary coupling
-> held-out anonymous arrival traverses the retained ARROW
-> an already-existing downstream execution path runs
-> the SPIKE queue drains naturally
```

No code may add a new organism-visible field, representation, rule, threshold,
adapter, or physical update. The PROBE may only arrange ordinary cells and
arrows, deliver anonymous spikes, call the frozen propagation/time operations,
and observe cloned execution reports.

## Physical fixture

Each fresh world has one anonymous arrival cell, one nearby execution cell,
and one downstream boundary cell. The downstream execution arrow exists before
learning and is identical in every arm. The arrival-to-execution opportunity
does not exist initially; it can only be proposed by the frozen PX0 law after
an external arrival fires the arrival cell.

The weak proposed arrow has coupling `1`, while the execution cell threshold is
`2`, so an unlearned opportunity cannot run the downstream path. A physically
returned impulse reaching the arrival cell inside the frozen local window
overlaps the proposal's actual traversal trace, raises coupling through the
existing local-return rule, and increases persistence. Repeated physical use
and return are recurrence; no counter or lifecycle representation is added.

All organism-visible state remains CELL, ARROW, SPIKE, local time, cell state,
arrow topology/coupling/resistance/generation, and the already-frozen local
eligibility trace.

## PROBE arms

Exactly four fresh deterministic arms are executed once, each with an exact
duplicate built from the same initial physical description:

1. `learned-return`: four anonymous suprathreshold arrivals, each followed by
   an ordinary in-window returned impulse; a held-out arrival must fire the
   execution and boundary cells.
2. `unreturned`: the same four suprathreshold arrivals without returned
   activity; after ordinary time pressure, held-out arrival must not fire the
   execution or boundary cell.
3. `subthreshold`: the learned physical structure is acquired as in arm 1,
   then a held-out subthreshold arrival must not fire the arrival, execution,
   or boundary cell.
4. `absent`: the learned physical structure is acquired as in arm 1, then no
   held-out activity is supplied; propagation must do no delivery or firing.

Training inputs are fixed physical schedules, not events selected by an
evaluator. Expected behavior is evaluated only after propagation and has no
causal path into the substrate.

## Frozen observations and pass rule

Each row records training arrival/execution/boundary firings, return deliveries,
learned arrow existence/liveness/coupling/resistance, held-out arrival,
execution and boundary firings, boundary crossings, autonomous extra source
firings, natural quiescence, exact duplicate equality, work, persistent bytes,
and permanent/complete fingerprints.

The PROBE passes conjunctively only if:

- every learned arm forms exactly one live arrival-to-execution arrow with
  coupling `2` and uses the existing downstream path on suprathreshold
  held-out arrival;
- unreturned recurrence does not make held-out execution fire;
- learned structure alone does nothing for subthreshold or absent activity;
- no arm autonomously refires the arrival source;
- every finite propagation is naturally quiescent;
- every duplicate is exact; and
- all frozen hashes and the fresh namespace range are exact.

The four primary namespaces begin at `0x7a10_0000_0000`; duplicates recreate
the same world rather than share state. These identities do not occur in any
PX0--PX2 development or definitive artifact.

## Forbidden surfaces and stop rule

Forbidden organism-visible surfaces include `REQUEST`, `START`, respond-now,
task/episode boundaries, an initiation flag or role, event selection, semantic
enums, typed intermediate representations, serializers, old M schemas,
evaluator mutation, hidden clocks encoding expected answers, and renamed
equivalents.

If the frozen law cannot distinguish learned from unreturned, subthreshold, or
absent activity, freeze the negative unchanged. If progress would require a
new representation, substrate law, or PX0--PX2 change, stop. If this exact
no-new-mechanism edge passes, freeze it and preregister a fresh MICRO without
rerunning the PROBE.

## Atomic execution

The implementation must first support `--preflight`, which audits hashes,
source surfaces, namespace isolation, CLI refusal, and absent result paths
without entering any experimental cell.

The evidence command may be executed exactly once:

```text
cargo run --release -p px0-physical-correspondence \
  --example px7_physical_arrival_initiation -- --probe
```

It must emit one evidence marker and atomically create only:

- `results/px7_physical_arrival_initiation_probe_v1.csv`;
- `results/px7_physical_arrival_initiation_probe_v1.md`.

Staging paths are removed only after successful atomic rename. Existing final
or staging paths cause refusal. There is no rescue or rerun.
