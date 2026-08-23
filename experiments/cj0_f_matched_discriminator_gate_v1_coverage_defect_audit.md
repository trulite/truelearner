# CJ0-F matched discriminator GATE v1 coverage-defect audit

Status: **FROZEN GATE V1; CORE DISCRIMINATOR REPRODUCED; MANDATORY COVERAGE DEFECTIVE**.

The unchanged comparator executed its first terminal development GATE once:
8,640 paired rows, 768 differing predictions, CJ-B `7,368/8,640` row pass,
CJ-E `8,064/8,640`, CJ-B 696 false-conjunction rows, and CJ-E 480.
Both fail the same-source multiplicity requirement. All 864 dense-return rows
pass for both. Every row naturally quiesces and none runs away.

| artifact | rows / bytes | SHA-256 |
|---|---:|---|
| CJ-B CSV | 8,640 / 2,469,770 | `8091be1c121aedd5d93ec0f11a897a946ec4c38f0096565eaa534c361fe9d01d` |
| CJ-E CSV | 8,640 / 2,478,721 | `2f6d30383eab82e3821b1f60d61152a90f6c71dba69ec5eeaff53161140dea93` |
| paired CSV | 8,640 / 1,244,782 | `0379fa306488a7cb6171bf210bc2b7375db56a2122731abf07c0555ed2cb2660` |
| report | 963 bytes | `82bd6e01bd0b3357bdd9dfeaaaef83f741e3b0bd85eadf6108517f1053c324e4` |

The family totals are: same-source `2,448` rows (`1,968/2,088` B/E pass),
amplitude `576` (`288/432`), dense return `864` (`864/864`), timing
`3,024` (`2,592/2,952`), and controls `1,728` (`1,656/1,728`). Native
work is 361,296 B versus 345,552 E; summed per-row persistent storage is
3,393,792 bytes each; summed temporary lower bound is 931,200 bytes each.

Post-execution mandatory-control inspection found four evaluator-only defects:

1. `f1-a-burst-4` reused the two-entry one-path fixture and therefore emitted
   only two events separated by four ticks;
2. `ctl-blocked-return` reused the no-return fixture and did not deliver a
   physical return after eligibility closure;
3. timing-transfer rows serialized `train_spacing` but executed only the
   held-out spacing, so no train-window establishment occurred;
4. crossed and deallocation controls existed separately, but no single row
   executed contemporary A+B/C+D to A+D/C+B reversal and serialized its cost.

These defects affect both candidates identically and do not alter the already
observed amplitude/same-source discriminator, but they violate mandatory
coverage and prevent final CJ0-F completion. No result byte is changed or
reinterpreted. A fresh symmetric correction protocol is required; both frozen
candidate hashes must remain exact. GATE v1 remains development-only and
creates no definitive/authority evidence.
