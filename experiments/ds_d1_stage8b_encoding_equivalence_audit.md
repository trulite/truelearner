# DS-D1 encoding-equivalence audit

The diagnostic uses three enum variants and zero combination variants. Each
arm's evaluator-only payload is erased immediately after producing the frozen
DS1 `positive: bool` input and contributes zero persistent bytes.

For each seed, equality was required over:

```text
boolean update trace
E0 episode schedule
credit update count
all learned strengths and mature alternatives (via learner fingerprint)
held-out choices
held-out success and abstention counts
```

Every equality passed. Seeds may have different fingerprints because frozen
tie-breaking differs by seed, but all three arms are identical within a seed.

The evaluator-only role relation is used once to construct each supplied
payload. It is outside the marked learner and cannot support a cumulative
claim. `current_substrate_observability_established` is false for every cell:
this experiment does not assert that counterfactual magnitudes, polarity, or a
signed state delta currently exists organism-visibly.

Consequently, field count alone cannot select a developmental target. The
next protocol must audit which precursor relation can actually be acquired
from E0+A0+A1+R0+C0 activity without evaluator semantics.
