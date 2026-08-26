# CORE1-E18 — In-Flight Causal Protection Protocol v1

## Status

Preregistered. No runtime result exists when this protocol is frozen.

## Question

Can ordinary local expiry distinguish idle structure from structure that is
currently carrying an unresolved physical arrival, so a traversal can finish
without granting a longer general lifetime?

## Candidate physical law

For one live ARROW generation:

```text
ARROW emits a SPIKE
        ↓
that exact ARROW generation has one in-flight arrival
        ↓
ordinary local resistance decay is suspended for that ARROW
        ↓
arrival is delivered or invalidated
        ↓
in-flight arrival ends
        ↓
ordinary local decay resumes from the unchanged local age
```

Multiple emitted arrivals compose as an ordinary count. Protection exists
exactly while the count is nonzero.

The state is derived only from physical emission and resolution. It has no
access to success, action meaning, consequence, reward, task state,
uncertainty, or an evaluator timeout.

## Frozen arms

All arms use the E17 refractory candidate and the frozen CORE1-B/E16 world.

```text
A  E17-R unchanged
   expected historical boundary: 2/8 useful policies

B  E17-R + exact in-flight protection
   target: 8/8 useful policies

C  in-flight protection, no self-trigger
   target: zero first actions and zero learning

D  overlong/global weak-structure lifetime
   negative control: it may enable actions, but it must retain unrelated
   weak structure and therefore cannot satisfy the candidate-law contract
```

## Required observations

For every attempted route record:

- emitted ARROW and generation;
- in-flight count before emission, during transit, and after resolution;
- durable resistance/age before and after transit;
- whether the arrival delivered or was invalidated;
- whether the route later consolidated through ordinary consequence;
- whether unrelated weak routes changed lifetime.

## Hard gates

1. A reproduces the frozen E17-R boundary.
2. B produces a first action, encounters the useful action, and learns the
   useful policy for all eight opaque permutations.
3. C proves protection cannot initiate activity.
4. D proves a general lifetime extension is observably broader than B.
5. An emitted route cannot expire before its own pending arrival resolves.
6. Resolution immediately removes that arrival's protection; local age is not
   reset.
7. Failed, unsupported attempts eventually disappear after their arrivals
   resolve.
8. Unrelated weak structure receives no protection.
9. A stale generation receives no protection from an old arrival.
10. Multiple arrivals protect only until the last one resolves.
11. Depth 1, 2, 4, 8, and 16 traversals compose without a path, depth counter,
    or global active-episode flag.
12. A recurrent cycle may remain active only while it keeps producing real
    arrivals. Once activity stops, nothing remains pinned. No cycle detector,
    TTL, or maximum-wave rule may serve as organism physics.
13. Natural quiescence, exact replay, and Reference/Production equality hold.
14. A and the prior CORE1-A/C lifetime boundaries are not repaired by keeping
    idle or merely nearby structure alive.

## Prohibitions

No:

- unresolved/action/episode flag;
- success or terminal-result branch;
- timeout-based release;
- path, predecessor, parent, depth, or route identity;
- global protection epoch;
- age reset on emission or delivery;
- participation-as-protection predicate;
- random choice, curiosity, information gain, or action preference;
- evaluator knowledge in the runtime.

The only lawful protection key is the emitting `ArrowId + Generation` carried
by an actually pending physical SPIKE.

## Success claim

If all gates pass:

> Age controls idle structure. Causal completion controls structure that is
> actively carrying a physical traversal.

This is a development result only. It does not advance authority or alter the
frozen ARC curriculum.
