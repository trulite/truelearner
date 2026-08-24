# DS1 after R0 fingerprint and path audit

## Exact fingerprints

| Object | SHA-256 |
|---|---|
| Frozen R0 mechanism | `f17afa482bf345eb680463f7418b6b6c2553cd78eab9b4dbfce74f7ca1483d51` |
| Frozen R0 readiness | `0888ddb8187f606ec7fac72369d4e8b397b624226ddd709540c882925dae82e5` |
| Frozen R0 E2B amendment | `729948af6d49b9428f04091529d087127b78102f063c87d3593a412713c6e7b3` |
| Frozen marked DS1 | `adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e` |
| Retry mechanism | `36c33cb3595001416b4763c29cdba88b5c9567caadc61d8d002177e972ffacce` |
| Retry runner | `7888fac0460a8bb017592dce98986e2466ebff64a0b5db021ca92c02751962d0` |
| Build fingerprint wiring | `74a32d71e98bfa077c1f23788f4ddb7acc7bc6692c1451311ad442f8632f48d4` |
| Frozen results tree | `491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4` |

The retry invokes `frozen_r0::run` exactly once from its `run` body. It does
not include, copy, redefine, or mutate the R0 mechanism or the marked DS1
learner.

## Derived source inventory

```text
frozen DS1 apply_consequence definitions          1
R0 evidence bridge definitions                    1
returned evidence → apply_consequence call edges  0
boundary-strength observation edges               0
held-out reconstruction edges                     0
semantic update edges                             0
```

## Derived runtime inventory per seed

```text
anonymous evidence surfaces                       1
DS1 updates                                       0
boundary-strength observations                    0
held-out reconstructions                          0
```

The three asserted zero path classes are independently mutation-sensitive:

- inserting `learner.apply_consequence(...)` raises the update-edge count;
- inserting `observe_boundary_strengths()` raises the strength-observation
  count;
- inserting `held_out_reconstruction()` raises the held-out count.

Stage 8 requires both a positive source coupling and a positive runtime update.
Both are absent. Stages 9 and 10 are therefore blocked by ordered freeze rather
than evaluated or assigned outcomes.
