# RP0a definitive outcome audit

Status: **positive**. The single definitive E2B run passed all 12 frozen
conjunctive gates.

## Result

- Integrated learners: `8/8` competent.
- Held-out chain execution: `512/512` correct with explicit answers, natural
  quiescence, zero activity-limit hits, and zero fallbacks.
- Fresh provenance-role transfer: `5120/5120`.
- Learned topology: exactly four consolidated arrows per seed and all four
  evaluator-correct.
- Mean competence episode: `12644.875`; seed range `2388..20842`, within the
  frozen `50000`-episode budget.
- Activity-only, shuffled-provenance, shuffled-feedback, random-feedback, and
  symmetric-impossible controls: `0/8` competent each and `0/512` held-out.
- Symmetric-impossible role population: exactly `9`, rather than inventing a
  distinction between the two provenance-identical lower locations.
- Oracle: `512/512` through the same lower runtime.
- Workspace lifecycle: `19236621/19236621` destroyed; maximum live `2`.

The CSV has `646` rows and `42` columns with a consistent row shape. Every
integrated per-seed row independently satisfies the competence, ten-role,
four-arrow, held-out, transfer, opacity, fingerprint, and determinism
invariants.

Artifact hashes:

- CSV: `215f8d18e611585f6d7416ab57ce5072450fad18d4d03750c6199ced2fbf5235`
- generated report: `8318fec3e5144083ec79051e4556d179f795cbfb2a9537ac0275c7e18858b8f2`

## Boundary audit

Persistent role patterns store only anonymous provenance signatures, learned
role IDs, and support. Every episode creates fresh opaque lower-location,
activity-source, and temporary-occurrence identities. Program arrows connect
learned role IDs. The evaluator-side lower-role vocabulary remains confined to
the lower runtime and scoring; it does not enter persistent role or program
learning.

The first excluded E2B smoke was run before this last boundary was corrected.
It is preserved as
`rp0a_invalid_pre_opaque_location_smoke.{md,csv}` and explicitly excluded from
the evidence. The corrected implementation was frozen before the definitive
run; no second smoke or definitive rerun occurred.

## Frozen claim and limits

Supported:

> The same local program-learning physics that constructs procedures over
> external experience constructed an executable recurrent procedure over
> anonymous roles learned from internal computation.

This is a one-level reflected program-discovery result over substrate-native
chain traversal. It is not arbitrary compiler discovery, a d2.6 retrieval
shortcut, reflection economics, or recursive reflection. RP0b economics, real
d2.6 substitution, and recursive F1 require separate preregistration and remain
unimplemented here.
