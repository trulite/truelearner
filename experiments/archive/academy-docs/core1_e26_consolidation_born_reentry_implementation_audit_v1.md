# CORE1-E26 — Consolidation-Born Re-entry Implementation Audit v1

## Freeze

- frozen E25 result: `23cf456`;
- E26 protocol: `e25d5bb`;
- E26 implementation: `c70b8d0`;
- runtime evidence: not yet spent at the time of this audit.

## Candidate surface

The implementation adds one default-off CORE1 law. When actual modulation
produces a PQLC update at a contact, the same transition may create one durable
Drive edge from the participating source to that contact, but only when the
live physical evidence contains both halves of a complete used route:

```text
participating source -> contact
participating contact -> target
successful modulation/PQLC at contact
```

The edge is born from those physical endpoints at consolidation. It has the
ordinary one-tick Drive delay, `SourceFires` coupling, positive unit material,
and durable resistance. It does not terminate at the motor and carries no
action, context, consequence, reward, episode, route identity, or expected
result. Its only effect is to make later activity at the same physical source
capable of reaching the already-consolidated contact.

The pending E22 consequence-return path remains responsible for delayed
credit and is unchanged. E24 route closure and E25 G+W remain frozen in the
evaluator so the teaching interaction can physically occur.

## Static rejection audit

- the candidate cannot appear before successful PQLC consolidation;
- incomplete routes cannot create it: both a participating incoming stem and
  a participating outgoing contact route are required in the same causal
  evidence;
- the source and contact must be co-located, preserving the local formation
  rule;
- the candidate creates no direct source-to-motor shortcut;
- one source/contact pair can create at most one marked re-entry edge;
- repeated consequence therefore cannot manufacture duplicate topology;
- the marked edge stores no payload beyond its physical endpoints and
  ordinary arrow properties;
- no timeout, probe schedule, useful-sign knowledge, or evaluator order is
  visible to the transition;
- disabled organisms retain the prior behavior;
- E14, E16, E22, E24, and E25 evaluator files are byte-identical to `23cf456`.

## Focused controls

The focused core control passes and establishes:

```text
before consequence             re-entry count = 0
candidate disabled + PQLC      re-entry count = 0
complete used route + PQLC     re-entry count = 1
repeated consequence           re-entry count = 1
incomplete used structure      re-entry count = 0
```

It also verifies that the created edge reaches the participating contact,
starts at the co-located participating source, uses Drive/SourceFires, is
durable, and emits the explicit consolidation-re-entry physical event.

The E25 G+W controls, the live-checkpoint pending-return control, and the
Academy blocked-return control pass unchanged. Strict release Clippy passes
for the E26 binary with dependencies excluded. Release construction and its
non-executing `--check` pass.

## Evidence program

The evaluator first runs the two-turn creation gate for all eight frozen roots
under Reference, exact Reference replay, and Production. It requires the
unchanged E25 teaching actions (`1`, then `4`), zero re-entry after the first
action, successful PQLC after the second consequence, and exactly one re-entry
edge afterward.

Only an `8/8` creation gate advances to the frozen five-turn regimen and
autonomous probes. Full success requires teaching actions `1|4|2|3`, nonzero
modulation and PQLC, E22 return counts `1|1|1|1|0`, re-entry counts
`0|1|2|3|4`, quiescence with no pending returns, and autonomous probe actions
`1|4|2|3` in Reference, replay, and Production for all eight roots.

## SHA-256

- protocol:
  `7faeec1a29b87390bc0006fc4a5816593e174b0f2b01c820c8279ee9bb03ef1c`;
- core candidate:
  `05d85f819fc62e550d5a1f981323d4ec00ee166c706b64b8a9a89fd8f802e214`;
- Academy enable surface:
  `89704c50b6f8184ed658f84dc0afeaf46cc61e981eea31cb201d3b12e3e0d498`;
- evaluator:
  `21bea50cf08e5224fbc7a187bd154143d291dc9bd3ccb2b34a504469bf8fe78d`.
