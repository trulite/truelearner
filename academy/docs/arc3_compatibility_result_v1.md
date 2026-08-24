# ARC-AGI-3 compatibility result v1

The first compatibility spike is positive for the external world boundary.
It is not a TrueLearner learning result.

The official anonymous ARC-AGI toolkit exposed 25 environments. Academy opened
`ls20`, downloaded official game version `9607627b`, reset it, and executed 12
mechanical interface actions. The run produced 13 observations. Every
observation contained a valid 64×64 frame using only the official sixteen-color
palette and exposed actions `[1, 2, 3, 4]`.

Two fresh captures used seed `205` and the same action sequence. Their
normalized JSONL records were byte-identical. The Rust adapter accepted the
record, rendered its final frame, and the review renderer produced an 8.37
second MP4.

Observed final state:

```text
game              ls20
observations      13
actions           12
state             NOT_FINISHED
levels completed  0 / 7
exact replay      yes
organism admitted no
```

The final line is essential. This proves that Academy can host, record,
validate, and render an official ARC-AGI-3 world. It does not prove that
TrueLearner can perceive the frames, choose actions, infer a goal, or learn the
environment. Those are subsequent developmental gates.

Evidence:

- E2B sandbox: `i0sx3yhxt5wtfvhxb0j0b`
- toolkit revision: `arcprize/ARC-AGI@f12822c4d550121c35a275008d964afbbed47d2f`
- normalized recording SHA-256: `81e1bfa308bc89ab4ce5e7d578c1a3a47cd2c0f6d057720bc04c81784cf81722`
- final raster SHA-256: `e55cd765e44064cf61f8455d5a92b7500914f29720814c5b22c94107a2a13647`
- review video SHA-256: `76455c98c174494469f46b9277848344c5f6469f9de0af016cb062e7805630c1`
- review poster SHA-256: `482baa10e05ad94d535d03543fe27ce47b59700923005c539fd63e1b1bc96688`

Focused Rust formatting, strict Clippy, tests, live capture, exact replay,
ingestion, and raster rendering all passed. No dependency or code was added to
`truelearner-core`.
