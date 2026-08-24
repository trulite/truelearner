# ARC-AGI-3 compatibility spike v1

The first ARC-AGI-3 slice tests an external world boundary, not intelligence.

The official environment supplies one or more 64×64 frames using sixteen color
values and exposes a subset of seven actions. Academy records those frames and
actions in normalized JSONL. The adapter validates frame dimensions, palette,
turn order, action range, and coordinate-bearing action 6, then renders frames
through Academy's existing `VisualSurface`.

The boundary is deliberately one-way:

```text
official ARC environment
    -> normalized Academy recording
    -> validated raster surface
    -> future physical admission
```

Game identifiers, action identifiers, score, level counts, terminal state, and
goal semantics remain outside TrueLearner. The v1 capture uses a deterministic
mechanical action cycle only to exercise the interface. It is not an organism
policy and cannot support a learning or benchmark claim.

Acceptance requires:

1. one official anonymous environment starts headlessly;
2. reset and successive actions return valid frames;
3. normalized evidence is accepted by the Rust adapter;
4. the final raster renders without transformation ambiguity;
5. the capture is reviewable as an Academy episode;
6. no ARC dependency enters `truelearner-core`.
