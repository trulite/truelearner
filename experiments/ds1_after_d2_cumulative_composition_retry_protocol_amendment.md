# DS1-after-D2 protocol amendment: MICRO acquisition floor

Status: **PREREGISTERED HARNESS CORRECTION BEFORE IMPLEMENTATION FREEZE**

The original protocol at `bd7cdc2f3f6f59b90ae17310799e905baa96e449`
specified eight MICRO acquisition episodes. The exact frozen E0/DS1 fixture
used by the already-frozen DS-D1 functional screen requires sixteen
presentations before its anonymous event representation is available across
the four signature contexts. Eight therefore returns no composition episode
before any DS-D2-to-DS1 scientific stage is evaluated.

MICRO is amended to:

```text
seed                 100
DS1 acquisition       16 episodes
held-out evaluation    8 episodes
frozen D2 support     16 presentations
```

GATE remains exactly unchanged at five seeds, 32 acquisition episodes, and 16
held-out episodes. All ordered stages, controls, frozen hashes, the mechanical
direction bridge, stopping rules, and the prohibition on evaluator semantics
remain unchanged.

This amendment repairs development-harness reachability only. It does not
change or reinterpret any observed GATE result and does not authorize a rescue
after the first scientific collapse.
