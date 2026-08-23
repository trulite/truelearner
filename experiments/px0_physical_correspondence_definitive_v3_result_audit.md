# PX0 definitive v3 result audit

Outcome: **PASS**.

The sole preregistered command executed exactly once from frozen implementation
commit `e49d1bfeeccf517ca5a70c74a9de92a4c6c4f8e7`. It emitted exactly one
`PX0_V3_DEFINITIVE_EVIDENCE_SPENT` marker and exited `0`. No rerun, rescue,
regeneration, or tuning occurred.

## Conjunctive result

- cells: `24/24` passed;
- independently serialized claims: `576/576` passed;
- initial held-out execution: `24/24`;
- survival-window reuse: `24/24`;
- stale post-forgetting execution: `0/24`;
- final contemporary B execution: `24/24`;
- final sparse A execution: `0/24`;
- exact duplicate replay: `24/24`;
- natural quiescence: `24/24`.

The frozen result commit is
`d4013f2ea2b22d677f8be4e3a5ca8c8d2e4bc335`, tagged
`px0-physical-correspondence-definitive-v3-positive`.

## Dense-return observables

- stable opportunities/completions: `840/838`;
- sparse opportunities/completions: `192/160`;
- maturation context range: `0..1`;
- final stable resistance range: `41..72`;
- final sparse resistance range: `0..1`;
- stable no-use lifetime range: `400..710`;
- sparse no-use lifetime: `10` in every cell.

The two completion gaps were cells `7` and `17`, both at spacing `17`:
`34/35` stable returns completed. In each, B matured by context `1`, final
resistance was `46/0`, lifetime was `450/10`, final B/A effects were `1/0`,
and all `24/24` cell claims passed. This is the exact accounting boundary v3
was preregistered to expose rather than misclassify.

## Independent controls

- return-free fixtures: `576` proposals, `0` returns, `0` effects, and `24/24`
  eventual deallocations;
- recurrent dense fixtures: both genuine return paths remained learnable and
  each produced `24/24` held-out effects; simultaneous competition produced
  zero crossing in all cells;
- stable-source swap: stable/sparse returns `840/164`, held-out effects `24/0`;
- absent-return effects: `0/24`;
- equal-ambiguity effects: `0/24`, with `48/48` alternatives physically live;
- original arrows remained stale in `24/24` cells while contemporary arrows
  had fresh provenance.

## Accounting and integrity

- generic local proposals: `30,819`;
- physical deallocations: `30,568`;
- causal work: `10,570,972`;
- evaluator-only cloned diagnostic work: `1,787,991`;
- CSV SHA-256:
  `b750792123de1c0aa7d3104d2d1bcd3fdc6e26a70e54b10f5eedf320fe7d95c9`;
- report SHA-256:
  `6bf27bb98cf3f2ca821918daa966722c3be9a31c1de6b589565d25539b3c702d`;
- post-result results-tree digest:
  `dd889f61b89f7383c00d209fb7ca920869845645918cb42244045ef205074f76`.

All frozen v1/v2 negatives, PX0-P1/PX0-S evidence, D2 evidence, the active PX0
law, retained physics, protocol, implementation, dependency surface, and source
isolation passed the pre-execution audit. Staging paths are absent.

