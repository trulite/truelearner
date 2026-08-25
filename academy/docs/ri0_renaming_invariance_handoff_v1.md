# RI0 renaming-invariance handoff v1

RI0 is a deterministic, replay-exact real negative.

```text
same-tick +1/-1 from two sources
rename competing physical IDs
→ target firing changes 1 → 0

same-source parallel +1/-1 arrows
reverse only ARROW insertion
→ target firing changes 1 → 0
```

This proves that arbitrary identity ordering currently enters physical history
through same-tick tie resolution. It is more fundamental than RS2's exact-one
contact predicate.

The next work, if authorized, is not RS2. It must independently discover and
preregister a physically meaningful simultaneous-arrival rule. No resolution
is selected by RI0.

