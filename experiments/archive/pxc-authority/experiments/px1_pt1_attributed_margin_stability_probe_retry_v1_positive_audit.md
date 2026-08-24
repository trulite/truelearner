# PX1-PT1 attributed-margin stability PROBE retry v1 positive audit

Outcome: **DEVELOPMENT PROBE POSITIVE; MICRO ELIGIBLE; PX1 NON-AUTHORITATIVE**.

The separately named retry executed once from frozen implementation commit
`007266fee4a9496a2f3355e87363f5d2c56cb22c`. No rerun or rescue occurred.

- PX0 correspondence resistance: `[22,22]`;
- continuation resistance: `[17,0]`;
- training branch firings: `[8,0]`;
- training outlet firings: `[8,0]`;
- trace arrivals: `[16,8]`;
- trace-cell firings: `[8,0]`;
- local branch returns: `[8,0]`;
- training/held-out/post-gap source refirings: all zero;
- held-out effects: `[1,0]`;
- post-gap effects: `[1,0]`;
- training/held-out/post-gap quiescence: all true;
- duplicate replay: exact.

The unsupported trace cell received one global-return impulse per exposure but
never fired. The supported trace received its local effect impulse plus the
same global return, fired once per exposure, and returned activity only to the
participating branch. This is the first physical PX1 composition to preserve:

```text
differential role learning
+ useful recurrent execution
+ natural quiescence
```

without changing PX0 or adding provenance metadata.

## Narrow limitation

The PROBE artifact serialized held-out effects and source refiring, but not
held-out branch/outlet/trace firings as separate columns. Therefore it does not
yet establish the strongest v2 control that both held-out branches physically
fire while only one downstream outlet and trace fire. MICRO must serialize
that chain independently in fresh mirrored/swap worlds; the PROBE will not be
rerun to fill the measurement gap.

- CSV SHA-256:
  `cda4bf6750abb40f7b3798e84c0b6f39527704a02c69b133896ce51c3925420b`;
- report SHA-256:
  `95debc42a74350b1543aa1ee795f8893e90fad93711d28cf17ecaa480b9a9d77`;
- retry implementation SHA-256:
  `1bcfe295fb8989d1c6489e7c255b128912b98afef8b550eea15b5eaf0e06b443`;
- PX0 source SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- definitive evidence: none;
- PX1 authority: absent.
