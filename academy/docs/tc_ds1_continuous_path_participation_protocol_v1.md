# TC-DS1 continuous path participation protocol v1

Status: frozen before candidate implementation or execution.

Parent characterization: `tc-ds0-old-window-characterization-positive-v2` at
commit `0a14c7e`.

## Narrow question

Can actual traversal create a continuously relaxing, path-local physical
participation state whose evolution and renewal depend on traversal rather
than a supplied credit horizon or source-level activity?

TC-DS1 does not repair pressure, replace plasticity, run ARC, delete
`eligible_until`, or advance authority.

## Candidate state

The candidate is feature-gated experiment state stored on each actual ARROW
alongside the retained law:

```text
participation_level: unsigned fixed-point physical magnitude

actual traversal
    participation_level += one universal impulse

elapsed physical tick
    participation_level = floor(participation_level * 15 / 16)
```

The traversal impulse is `2^32`. Arithmetic saturates only at machine
representational capacity. There is no candidate threshold, deadline, timeout,
expiry tick, remaining-ticks field, or boolean interpretation.

The relaxation ratio and impulse are universal substrate constants for this
candidate. They are not selected by ARC or by choosing a desired lifetime.
TC-DS1 records the resulting curve; it does not optimize these constants.

The state is causally inert with respect to retained pressure, resistance,
coupling, eligibility, firing, and modulation. Default builds do not contain
the candidate state. The existing `eligible_until` mechanism remains active
and byte-behaviorally unchanged.

TC-DS1 does not claim candidate live-checkpoint support. The state is
development instrumentation alongside the old mechanism; exact fresh-run
replay is required. Checkpoint integration is forbidden until a later stage
selects a replacement physical law.

## Forbidden substitutions

- `expires_at`, `eligible_until`, deadline, timeout, or age threshold;
- decrementing a remaining-ticks counter;
- `if participation > 0 { eligible/protected }`;
- any pressure or plasticity branch reading participation;
- per-world, per-delay, per-path, or ARC-selected constants;
- source-global renewal;
- renewal by nearby activity, subthreshold source Drive, another path,
  unrelated Drive, or unrelated Modulation;
- evaluator mutation of participation state;
- duplicated Reference and Production laws.

Static audit and behavioral discriminators must both enforce this boundary.

## Gate A — path-local state

Minimal paths are physically distinct ARROWs. Where selective traversal from
one firing CELL is impossible because retained fan-out traverses every live
outgoing ARROW, selective controls use two physical branch-source CELLs. A
separate shared-source fixture tests simultaneous fan-out explicitly.

Required controls across initial pressure phases `0..9`, fresh identity roots,
and both `MechanicalConfig::REFERENCE` and `PRODUCTION`:

1. Traverse A only: A rises; B remains baseline.
2. Traverse B only: B rises; A remains baseline.
3. Traverse A then B: both retain their own independently evolved magnitude.
4. Subthreshold Drive reaches a source without traversal: neither rises.
5. Unrelated nearby path traverses: neither rises.
6. Repeated traversal of A: A is renewed above the matched single-traversal
   curve; B continues relaxing or remains baseline.
7. Equal traversal quantity on B cannot maintain A.
8. Shared-source fan-out: A and B both rise because both actually traverse;
   no source-global state is inferred.

## Decay characterization

For one traversal of A, record its magnitude at every integer delay `0..20`
under every initial pressure phase and both mechanics.

Required shape:

- delay zero is positive;
- every successive sample is strictly smaller than the previous sample;
- at least three distinct positive magnitudes exist;
- the samples are not a rectangular sequence;
- phase translations produce the same relative curve;
- B remains exactly baseline throughout.

No delay is labeled accepted, rejected, timely, or late.

## Gate B — modulation-attribution discriminator

Use two live outgoing paths from the same source:

```text
       A
P ----------> X

       B
P ----------> Y
```

Both physically traverse. Only X receives the downstream event that emits one
ordinary Modulatory transmission back to P. At delays
`0,1,2,3,4,5,8,12`, record which outgoing path-local participation states the
existing source-local modulation site physically contacts.

A feature-gated causally inert physical-trace observation may serialize each
contact `(ARROW, participation_level)`. It may not select an ARROW, change
plasticity, or add provenance.

Desired discriminator:

```text
A contacted according to remaining A participation
B not contacted
```

If both A and B are contacted, Gate A may be reported positive but TC-DS1
stops as an attribution negative. No rescue mechanism, route label, pressure
change, or TC-DS2 implementation is permitted in this workflow.

## Evidence and equivalence

Every physical world is defined once and run under Reference and Production.
They must match on candidate levels, candidate contact observations, retained
physical transitions, causal work, durable body, clock, pressure phase,
quiescence, and final retained behavior. `ExecutionCost` and raw cross-mechanics
checkpoint hashes are excluded.

The complete artifact set must reproduce byte-for-byte in a fresh E2B worker.
Default-feature retained core tests and strict Clippy must pass. The candidate
feature must also pass formatting, strict Clippy, and its focused matrix.

## Decision

- Gate A failure: TC-DS1 negative; stop.
- Gate A positive and Gate B positive: TC-DS1 development-positive for the
  narrow path-state claim; TC-DS2 becomes eligible.
- Gate A positive and Gate B negative: freeze the split result and stop. This
  identifies physical credit-path continuity as the next missing scaffold.

ARC A2-A5, pressure protection, `Arrow.eligible_until`, `arch.md`, oracle, and
authority remain unchanged in every outcome.
