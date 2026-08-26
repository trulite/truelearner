# CORE1-E27 — Full E14 Candidate-Adoption Implementation Audit v1

## Freeze

- positive E27 candidate: `5045084`;
- adoption protocol: `d7ccd1f`;
- integration implementation: `dbd1fd4`;
- full-context runtime evidence: not yet spent at the time of this audit.

## Actual frozen E14 artifact

The real original evaluator is:

`experiments/arms/core0-blank-physical-organism/src/bin/core1-e14-arc-a2.rs`

It is byte-identical to its original freeze commit `264c26e` and retains
SHA-256:

`1c2f144a3bd3b660bb3f213ce6d13bcc44aeaee13ff3de81378f95b9f2b32858`

### Audit erratum

Earlier E26/E27 audit shell checks referenced the nonexistent filename
`core1-e14-arc-action.rs`. `git diff --quiet` on that absent path returned
success, making those particular automated E14 checks vacuous. Their candidate
results and other hashes are unaffected, but their “unchanged E14 evaluator”
check was not evidence.

This audit corrects the path and verifies the actual E14 blob directly against
its original commit and published hash. The file has not changed.

## Contract equivalence

The integration evaluator copies the E14 contract exactly:

```text
root                     93000000
body                     full 1,024 spatial contexts
frame generation         first five distinct frozen contexts
action map               1|2|3|4
teaching curriculum      1|4|2|3
support_previous         false|true|true|true, then true closing
closing babble           none
acceptance actions       1|4|2|3
acceptance updates       0|1|1|1|1
acceptance quiescence    true throughout
mechanics                Reference|replay|Production
autonomous probes        none
```

The additional fields—Modulatory delivery, E22 return, E26 re-entry, E27
executable-edge, pending, work, tick, and fingerprint—are observers only and
do not enter acceptance.

## Candidate seam

The full-context organism is constructed with the same
`new_spatial_with_profile(..., GenericExternal)` call. Before turn one, the
integration evaluator enables only:

- E24 atomic route closure;
- E25 local signed gating on existing motors;
- E25 motor integration windows;
- E26 consolidation re-entry;
- E27 consolidation executability.

E22 atomic return remains the already-existing ordinary CORE1 Academy
integration. Core candidate physics is byte-identical to `5045084`.

No candidate is enabled globally by this test. Academy's only new runtime
surface is a six-line mechanical reconfiguration delegate used while the base
organism is still quiescent.

## Harness audit

- one full 1,024-context Reference body is constructed;
- three independent clones execute all five E14 observations;
- the Production clone is mechanically reconfigured before activity;
- Reference, replay, and Production execute concurrently;
- no physical state is shared after cloning;
- every result is joined before equality or acceptance is evaluated;
- no context compression, skipped observation, cached result, probe, or
  acceptance shortcut exists.

## Controls

- strict release Clippy for the integration binary: pass;
- release build: pass;
- non-executing frozen-frame observer: contexts `779|34|980|430|702`;
- focused E27 ordinary-source execution: pass;
- unchanged E26 creation: pass;
- Academy blocked-return control: pass;
- formatting and `git diff --check`: pass.

## SHA-256

- protocol:
  `72bffc9ce46ae4ff9dba59a5c67c620da647c034bc211bbc61714043ad19c1c3`;
- unchanged original E14 evaluator:
  `1c2f144a3bd3b660bb3f213ce6d13bcc44aeaee13ff3de81378f95b9f2b32858`;
- unchanged E27 core candidate:
  `778782ea97b3dd0c65e5a55a72e0fec8e97329c814e742f88991fdf3c8cb949b`;
- Academy integration plus harness delegate:
  `9ee285a9055c9db712747e50436b65e9dd18a36980f55162b9c2b30db36bf420`;
- full-context adoption evaluator:
  `a76544305e83e0b35cecdf3c3c3e2bfc1f5d3cbedc958f18a4faa56e9fd89fbe`.
