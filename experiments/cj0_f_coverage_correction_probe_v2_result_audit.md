# CJ0-F mandatory-coverage correction PROBE v2 result audit

Status: **INTERPRETABLE; BOTH FAIL; CORRECTED FIXTURES COMPLETE AT PROBE**.

The frozen correction evaluator executed 48 fresh paired rows once. Outputs:

| artifact | bytes | SHA-256 |
|---|---:|---|
| B CSV | 15,785 | `d843593027e821dafe949c6e7a95cad66271b6460b24080a3f346066efd4ac7c` |
| E CSV | 15,822 | `a8b82d79ae83e50bb9513eb16cb9bed12659721eee7e6d8b7b5a4da472de47e6` |
| paired CSV | 8,333 | `07f2f680bc3dd63d392dc9bbd5aa6273af8162c176e262b5230a80a51252ac95` |
| report | 968 | `a408a92d64c551ab4f5ae50d1e2e4b4326f169d7328417b0ecb1f214300b096a` |

Both candidates pass both late-return attribution rows and all 42 executable
timing-transfer rows. Both fail one of two actual four-emission burst rows and
both contemporary reversal rows. The laws differ in 18 predictions. Reversal
is naturally quiescent: CJ-B records crossed held-out reuse but fails the full
old/new clause; CJ-E records duplicate crossed effects and fails exact reuse.

Native work is 4,703 B and 4,701 E. Summed per-row persistent bytes are 15,456
B and 15,328 E; temporary lower bounds are 4,160 each. All 48+48 candidate
rows quiesce and none runs away. CSV shapes, unique pairs, atomic publication
and staging absence pass. Classification remains **both fail; shared boundary
frozen**. Correction MICRO may execute with unchanged laws.
