# PX4 LR-C physical lifetime MICRO result audit v1

Status: **DEVELOPMENT MICRO POSITIVE; GATE ELIGIBLE; AUTHORITY ABSENT**.

The unchanged implementation executed the preregistered four-row MICRO once
from clean commit `4b8c76513c3b20da2fcb9122ce1be38ddc62e575` in fresh E2B
sandbox `ioxoyxxk0c3w1enockiut`, using unique state file
`px4-lrc-lifetime-micro-20260824.json`. The sandbox was left running.

## Frozen artifacts

| artifact | SHA-256 |
|---|---|
| MICRO CSV | `3086f2545e8cb714933aa40ed233fac9c1f6ce46b8f4d0cc8d1950c0ad1904de` |
| MICRO report | `db3a596ac588518610ac8aae1234675aec9701c1a184407a946a246fb8d39397` |

The CSV contains four data rows; every row and the header have exactly 38
fields.

## Result

All four fresh identity/layout cells passed conjunctively:

```text
identities                         152001, 152002, 152101, 152102
normal / reversed allocation      2 / 2
forward / reflected geometry      2 / 2
resistance after 1/2/4/8 support  4 / 7 / 12 / 22
pressure steps to deallocation    4 / 7 / 12 / 22
penultimate pressure point live   true in 16/16 observations
next pressure point dead          true in 16/16 observations
row gates                         4/4
exact replay                      true
natural quiescence                true
fresh identity/layout invariance  true
PX0--PX3 conformance              true
```

The ordinary ARROW resistance is therefore both the retained physical state
and the measured pressure budget in this matrix. Recurrence and reuse add only
through qualified modulation; ordinary pressure spends that same scalar until
physical deallocation. No second state, class, boundary or removal operation
was required.

The first scientific collapse remains `none`. The unchanged GATE may execute
with its fresh identities. This result is development evidence only.
