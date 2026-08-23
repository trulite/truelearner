# PX2-H1 matched-schedule hysteresis diagnostic v1 result audit

Status: **CLASSIFICATION C FROZEN; PX2 LAW UNCHANGED**.

- sole execution marker: `PX2_H1_MATCHED_SCHEDULE_HYSTERESIS_DIAGNOSTIC_V1_EVIDENCE`;
- rerun/rescue count: `0`;
- cells: `40`; duplicate-exact: `40/40`;
- every cell: 12 forward + 12 reverse experiences, zero source refiring,
  natural quiescence;
- summary CSV SHA-256:
  `4eeef4681fba0a5c41dbe42d8911b9f8f285be0c897cb6a5d33c4a8d7da46005`;
- trajectory CSV SHA-256:
  `78afd070081eb17ce23f7b88da3d0d7d6187b126c8f4f76c111b24651c2f0323`;
- report SHA-256:
  `26d7c950acae5af51d06bb88b3609b8ec454546dbed071a71fad80e4af28aeac`.

Classification C is required because the preregistered simple
protection-first classifier was not sufficient for all rotations.

The first scheduled direction always matured at experience `0`. The other
direction always deallocated at experience `1`, before its first scheduled use
could return evidence. In 28 cells, continued recurrence preserved the initial
winner to resistance `4..5`, producing the corresponding held-out direction.

In all 12 cells using rotations `1,3,5`, the initially protected winner later
deallocated too:

- rotation 1: first direction deallocated at experience `5`;
- rotations 3 and 5: first direction deallocated at experience `3`.

For example, rotation 1 changed the initial winner `3 -> 4` through one return,
then ordinary pressure produced `4 -> 3 -> 4 -> 3 -> 2 -> 0`; its next scheduled
use arrived only after the remaining reserve had been spent. Both final
directions were therefore absent despite exactly matched total counts.

The explanatory physical state is not total evidence. It is the ordered
trajectory of use-dependent return gains and ordinary pressure. First use
selects an initial basin; subsequent recurrence density determines whether that
basin remains physically available. No missing causal representation is
implicated by this diagnostic.

The result does not amend the immutable GATE negative, advance PX2, or unblock
PX3.
