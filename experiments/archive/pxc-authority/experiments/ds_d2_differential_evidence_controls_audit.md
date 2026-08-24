# DS-D2 controls and leak audit

The differential mechanism receives exactly two role-relative `Prediction`
values and one returned `Prediction`. A prediction contains only a normalized
local route trace and three relative activation counts. The mechanism does not
receive the selected route index, occurrence identity, route root, opaque
handle, evaluator truth, reward, correctness, or polarity.

The selected index exists only in evaluator-side temporary reporting and is
used after formation to score whether the anonymous relation points to the
route whose physical execution emitted the returned activity.

## Frozen GATE controls

All controls passed for seeds 100 through 104:

| Control | Expected | Result |
|---|---|---|
| equal compatibility | abstain | 5/5 |
| compatible with neither | abstain | 5/5 |
| alternative/handle order swapped | relation reverses | 5/5 |
| fresh occurrence identities | transfer | 5/5 |
| reversed allocation + padded layout | transfer | 5/5 |
| corrupted/shuffled returned activation | abstain | 5/5 |
| equal magnitude, reversed structural relation | relation reverses | 5/5 |
| duplicate effects | abstain | 5/5 |
| matching route removed | invalidate/abstain | 5/5 |
| temporary ARROW execution | one traversal | 5/5 |
| cleanup | no temporary relation or activation remains | 5/5 |

## Source boundary

- exactly one differential formation function is present;
- no frozen DS1 update edge is called;
- no semantic direction field is present in the mechanism;
- the formation signature accepts alternatives and evidence only;
- all DS-D2 learned/formed state is episode-local;
- persistent storage is exactly zero bytes;
- the temporary relation is represented as a live local ARROW and validated by
  ordinary SPIKE propagation, not as an evaluator-interpreted action token.

The source audit is intentionally narrower than later functional claims. It
proves absence of a supplied semantic bit at DS-D2's formation surface; it does
not prove that the resulting route attribution will be useful to DS1.
