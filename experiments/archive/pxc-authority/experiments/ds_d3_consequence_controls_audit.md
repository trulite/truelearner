# DS-D3 consequence and leak-control audit

All controls pass in MICRO seed 100 and GATE seeds 100..104.

| Control | Frozen result | Protection |
|---|---:|---|
| distinct immediate A1 effects | PASS | both affordances physically execute |
| downstream stability differs | PASS | recurrent consequence forms direction |
| same downstream consequence | ABSTAIN | route identity alone is insufficient |
| different but stable consequences | ABSTAIN | mere outcome identity is insufficient |
| both consequences unstable | ABSTAIN | no unsupported ranking |
| swapped alternatives | PASS | direction follows consequence history |
| fresh consequence occurrences | PASS | no episodic occurrence cache |
| allocation/layout perturbation | PASS | no memory-layout dependence |
| opaque-handle permutation | PASS | no stable handle meaning |
| shuffled downstream evidence | ABSTAIN | recurrence must survive matching |
| equal magnitude, reversed support | REVERSES | magnitude is not polarity |
| immediate-tick evidence | REJECTED | discriminant must be downstream |
| recurrent route removed | INVALIDATED | no stale direction survives |
| directional SPIKE/ARROW | PASS | result is physically executable |
| temporary cleanup | PASS | episode-local direction is erased |

## Information-flow audit

The physical fixture produces fresh consequence occurrences, delayed ticks,
and live directional propagation. The organism-visible learner receives only:

- a role-relative frozen A1 affordance shape;
- a normalized delayed consequence shape obtained by physical propagation;
- local counts of repeated shape co-occurrence.

The episode-local affordance/consequence association is a mechanical use of the
already-frozen C0/D2 attribution boundary. It does not rank the alternatives.
The evaluator chooses environment schedules for positive and negative controls,
but schedule identity and the recurrent slot never enter `ContrastLearner`.

The direction function reads only recurrence support and within-affordance
shape-count margins. Its source body contains no immediate-effect trace or
activation access. Persistent state contains no occurrence ID, opaque handle,
selected index, concrete destination, boundary-role truth, or polarity field.
Reported retained occurrences and handles are both zero.

