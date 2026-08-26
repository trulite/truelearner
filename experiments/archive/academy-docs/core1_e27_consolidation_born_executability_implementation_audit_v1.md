# CORE1-E27 — Consolidation-Born Executability Implementation Audit v1

## Freeze

- frozen E26 result: `2d8141a`;
- E27 protocol: `ddaa050`;
- E27 implementation: `8ec7ae7`;
- runtime evidence: not yet spent at the time of this audit.

## Candidate surface

The implementation adds one default-off CORE1 property on arrows. Successful
PQLC at a contact can mark exactly the two physical halves of the complete
used route:

```text
E26 source -> contact re-entry
used contact -> target route
```

For each marked arrow, consolidation preserves the existing sign, raises its
material magnitude only if it is below the local target's threshold surface,
and makes it durable. The ordinary observer coupling is updated from that same
material value.

No execution function is introduced. Marked arrows remain ordinary Drive,
`SourceFires` arrows. When their source fires, the pre-existing propagation
loop emits them, the target integrates their incidence, and its existing
threshold decides whether it fires. A trace-only `ConsolidatedExecution` event
records that ordinary traversal for the preregistered physical observable.

Academy exposes only enable/count methods. Enabling E27 also enables the E26
re-entry construction on which the candidate is explicitly composed.

## Static rejection audit

- candidate state defaults off and its arrow vector defaults false;
- it can arise only inside successful post-PQLC E26 complete-route creation;
- both an actually participating incoming stem and actually participating
  outgoing route are required;
- sign is preserved; negative material is never converted to excitation;
- efficacy is derived only from the existing material and the physical
  target's threshold;
- target threshold evaluation is not bypassed;
- no source, contact, motor, action, context, consequence, reward, episode, or
  probe identifier is stored as payload;
- no evaluator callback, recall operation, direct source-to-motor edge, motor
  injection, timeout, or action choice exists;
- unmarked arrows retain their prior coupling, resistance, decay, and
  propagation behavior;
- E14, E16, E22, E24, E25, and E26 evaluator files are byte-identical to
  `2d8141a`.

## Focused controls

The E27 core control passes and establishes:

```text
candidate disabled + PQLC       executable edges = 0
candidate enabled + PQLC        executable edges = 2
marked triggers                  SourceFires
marked resistance               durable
signed magnitude                target threshold
later ordinary source input     outward crossing
consolidated traversals          >=2
```

The same control supplies no later motor incidence, recall call, or route
lookup.

The unchanged E26 creation control, all E25 G/W/G+W controls, live-checkpoint
pending-activity control, and Academy blocked-return control pass. Strict
release Clippy passes for the E27 binary with dependencies excluded. Release
construction and the non-executing `--check` pass.

## Evidence program

The evaluator first runs all eight frozen roots under Reference, exact
Reference replay, and Production through two teaching turns and one recovered
autonomous probe. Gate 1 requires E26 re-entry `0|1`, executable arrows `0|2`,
ordinary action `1`, at least two consolidated traversal events, and exact
quiescent mechanics.

Only Gate 1 `8/8` advances. The full frozen E26 regimen then requires teaching
actions `1|4|2|3`, nonzero later modulation/PQLC, E26 re-entry counts
`0|1|2|3|4`, executable-edge counts `0|2|4|6|8`, E22 returns
`1|1|1|1|0`, no pending metadata, autonomous actions `1|4|2|3`, at least two
consolidated traversals per probe, and exact Reference replay/Production in all
eight roots.

## SHA-256

- protocol:
  `c041ce5bc08e038a0b00b773c37b929ebb8aac5a137ae7158f7432c265712b9b`;
- core candidate:
  `778782ea97b3dd0c65e5a55a72e0fec8e97329c814e742f88991fdf3c8cb949b`;
- Academy enable surface:
  `274d877a8752408e4a979a0921239ea7de294c2bfb6d466e1e663d323f6165be`;
- evaluator:
  `06fc35dcbb118b70272c1548d65f2feec6a494a6f8282bb48b7df60b5e1e48b4`.
