# PX1 recurrent role-stability diagnostic protocol

Status: **PREREGISTERED; DEVELOPMENT EVIDENCE UNSPENT; PX1 NON-AUTHORITATIVE**.

This diagnostic follows the immutable PX1 PROBE v1 reciprocal-loop negative.
It does not amend, rescue, or rerun that experiment.

## Frozen inputs

- authoritative PX0 commit:
  `e884ae133a562d475565a36700d929b51dd2b2d2`;
- PX1 PROBE v1 negative commit:
  `d76ec9364d022be2f614530e4d78db6d08ecb628`;
- active PX0 law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- frozen PROBE implementation SHA-256:
  `1fb0168729e4181a8e778a93f92ebfae7f10576e66d6ef0aa99bc3050a3021a8`;
- frozen negative SHA-256:
  `f45958a07021d0f116a7a77cfdb543d1b08c40ca7b57f675b3f028bbf6f6efaf`;
- collapse handoff SHA-256:
  `7a32288f4f8e6f3c6cde26cb73af1ba4bfdb5256a04a90b587f0278bf7b3a985`.

The PX0 learning law remains byte-identical in every arm. No old M0/M1 type,
learner, serializer, or behavioral adapter may enter an arm.

## Question

> Which purely physical condition, if any, permits endpoint-local role
> structure to learn in a recurrent PX0 graph while preserving useful returned
> activity and natural quiescence?

## Four independent arms

Every arm receives fresh identities and the same acquisition/role-exposure
schedule. Each also runs a fresh mirrored transfer world with the supported
side reversed.

### Margin

Topology and timing remain those of PROBE v1. Source threshold rises from `3`
to `4`, and ordinary external source activity supplies four spikes. The learned
direct and gated endpoint returns total at most three impulses, leaving a
physical refiring margin while remaining observable.

### Inhibition

PROBE v1 thresholds, topology, and timing remain. Each endpoint receives an
identical nearby weak opportunity to recruit a local brake cell. The brake has
a fixed ordinary inhibitory arrow to the source. It becomes active only if
endpoint return strengthens the endpoint-to-brake proposal through the same PX0
law that strengthens role structure. No evaluator activates the brake.

### Distance

PROBE v1 thresholds and timing remain. Endpoint activity propagates through an
identical fixed physical relay to a learning site outside the source/endpoint
local proposal radius. Broad opportunities and returned activity occur at that
site, not at the correspondence endpoint. The PX0 correspondence arrow remains
the only source-to-site causal entrance.

### Timing

PROBE v1 thresholds and local topology remain. Only the role-return path is
delayed beyond the frozen PX0 eligibility window. This tests whether timing can
avoid reciprocal maturation without also destroying role learning. A quiet arm
with no learned role fails.

## Narrow outputs

Each primary and transfer world records only:

- endpoint/local-site role structure formed;
- useful physical return reached the source/learning site;
- source refired beyond externally initiated firings;
- propagation reached natural quiescence;
- supported/unsupported held-out boundary effects;
- post-gap reuse of the same closed physical return path;
- fresh-identity mirrored transfer;
- work, proposals, deallocations, and fingerprints.

The inhibition arm additionally records whether the endpoint-to-brake proposal
matured and the brake physically fired.

## Productive-recurrence control

An arm cannot pass by globally killing recurrence. After training, a fresh
external source arrival must traverse the retained correspondence, reach the
role structure, complete the physical endpoint/site return path, produce the
supported outward effect, become quiescent, and remain reusable after a gap.

This control permits recurrent physical information flow while rejecting
autonomous self-excitation.

## Pass rule per arm

An arm passes only if both primary and fresh mirrored transfer worlds satisfy:

```text
role structure forms
+ useful return is physically observed
+ supported held-out effect = 1
+ unsupported held-out effect = 0
+ no source refiring beyond external initiations
+ productive-recurrence control passes
+ exact duplicate replay
+ natural quiescence
```

Any timeout, failure to learn, missing return, or trivial suppression is a
negative for that arm. No arm gains PX1 authority.

## Bounded diagnostic execution

The diagnostic parent launches all four arm processes independently and
concurrently. An evaluator-only wall-clock bound may terminate a non-quiescent
arm. The bound has no causal path into a successful arm and cannot alter
organism state. Timeout is serialized as non-quiescence and arm failure.

## Interpretation

- one passing arm: freeze it as the unique supported development target for a
  separately named PX1 retry;
- multiple passing arms: freeze the discrimination and report the remaining
  scientific ambiguity; do not choose by convenience;
- no passing arm: freeze the negative physical boundary;
- a quiet non-learning arm never counts as a solution.

This is development-only. Definitive execution, PX1 authority, PX-C, the
continuous organism, and Harness H1 remain forbidden.
