# DS1 after C0 fingerprint and path audit

## Exact fingerprints

| Object | SHA-256 |
|---|---|
| Frozen C0 mechanism | `5c8d00189593ca2f7efb47165efddf85111259f90433a016e5822b5b9578aed2` |
| Frozen C0 readiness | `a69e440639bc37eefc0e9f30402cde6c3b5dec945d95a77060513d7a96491572` |
| Frozen marked DS1 | `adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e` |
| Retry mechanism | `dba8ac027ec304a489b99c65e9629fe1537a33f256d9b3992d205de0e40b5c14` |
| Retry runner | `2e3f7cafe3e2a167fe727e03383e3fe6033b71aa626ee79edc80e040586ad035` |
| Build fingerprint wiring | `06acd4bdd232efd44881e44d262cddf7fc52b14433ebed051a83f87fbfd8aa84` |
| Frozen results tree | `491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4` |

The retry invokes `frozen_c0::run` exactly once from its `run` body. It does
not copy, redefine, or mutate C0 or the marked DS1 learner.

## Derived source inventory

```text
frozen DS1 apply_consequence definitions       1
C0 coupling representations                    1
coupling → apply_consequence call edges         0
boundary-strength observation edges             0
held-out reconstruction edges                   0
semantic update edges                           0
```

## Derived runtime inventory per seed

```text
anonymous C0 couplings                          1
coupling polarity fields                        0
DS1 updates                                     0
boundary-strength observations                  0
held-out reconstructions                        0
```

The three absent downstream path classes are independently mutation-sensitive:

- inserting `learner.apply_consequence(...)` raises only the update-edge count;
- inserting `observe_boundary_strengths()` raises the strength-observation count;
- inserting `held_out_reconstruction()` raises the held-out count.

Stage 8a is established by the actual frozen C0 runtime coupling and its zero
polarity fields. Stage 8b separately requires both a reachable update edge and
a runtime DS1 update. Both are absent. Stages 9 and 10 are blocked by ordered
freeze rather than evaluated or assigned outcomes.
