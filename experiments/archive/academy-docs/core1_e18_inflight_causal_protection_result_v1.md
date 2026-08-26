# CORE1-E18 — In-Flight Causal Protection Result v1

## Status

**STOPPED NEGATIVE.** The exact candidate law is insufficient. No authority,
ARC, or downstream CORE1 result advances.

## Candidate tested

An emitted SPIKE increments transient in-flight state on its exact
`ArrowId + Generation`. While at least one such arrival remains unresolved,
ordinary local decay cannot erase that ARROW. Delivery or invalidation removes
the corresponding protection without resetting local age.

The law is disabled by default and was tested beside the frozen E17-R control.

## Decisive screen

| Seed | Useful | Routes that actually participated | Updates | Learned useful |
|---:|---:|---|---:|---:|
| 0 | 1 | 1 | 15 | yes |
| 1 | 2 | 2 | 15 | yes |
| 2 | 4 | 3, 4 | 7 | no |
| 3 | 1 | 4, 1 | 7 | no |
| 4 | 2 | 4, 3 | 0 | no |
| 5 | 3 | 1, 4 | 0 | no |
| 6 | 3 | 2, 1 | 0 | no |
| 7 | 4 | 3, 2 | 0 | no |

Result: `2/8`, equal to the frozen E17-R learned-policy boundary rather than
the preregistered `8/8` target.

All eight candidate rows naturally quiesced. The decisive seed 7 was exactly
replayed and matched `MechanicalConfig::PRODUCTION` byte-for-observation.

Raw screen: `experiments/results/core1_e18_inflight_causal_protection_v1/b_screen.csv`.

## What changed physically

The candidate did solve its literal local problem:

```text
E17-R seed 7
selected 3
route 3 acts
selected 2
route 2 expires in transit

E18-B seed 7
selected 3
route 3 acts
selected 2
route 2 also acts
```

So an ARROW no longer dies while its own emitted arrival is pending.

## Why the capability still fails

There are two independent remaining boundaries.

First, an alternative that has not begun is idle structure. Protecting the
currently traversed route does not protect later alternatives. In seeds 4–7,
the useful third/fourth alternative disappears while earlier alternatives are
being evaluated.

Second, a chain finishing its forward delivery is not the same thing as the
causal interaction closing. In seeds 2–3 the useful route acts and consequence
produces seven updates, but the complete source/contact/motor route is not
retained for fresh execution. Protection tied only to pending forward arrivals
ends before the later physical return has consolidated the entire chain.

## Why the candidate was not widened

Making every sibling opportunity live while one route is active would be a
local opportunity reservoir. Holding a whole path until an evaluator declares
`RESULT`/`NO_RESULT` would require representing a causal episode or supplying a
resolution boundary. Protecting all weak structure would simply be the
overlong-lifetime negative control.

Those are materially different laws. None was introduced inside E18.

## Retained conclusion

> Age can be suspended safely for a link carrying a real pending arrival, but
> pending-arrival lifetime is too local and too short to solve unresolved
> alternative evaluation or end-to-end causal closure.

The implementation proves the local rule is deterministic, generation-safe,
checkpoint-reconstructible, and representation-independent on the decisive
case. It does not establish the broader design principle proposed for E18.

## Validation note

`truelearner-core --features core1` compiled successfully. Its suite reported
`14/16`; the E18-specific checkpoint/generation test passed and the two
failures are the pre-existing J0-era stale unit
fixtures `reused_identity_rejects_stale_generation` and
`quiescent_checkpoint_preserves_clock_phase_and_future_behavior`. The new live
checkpoint test passed, including reconstruction of pending in-flight counts.
