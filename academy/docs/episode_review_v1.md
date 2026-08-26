# Episode review contract v1

Academy runs a curriculum without a live UI. For each run it freezes canonical
physical evidence before producing any human-facing media.

Each episode directory contains:

- `record.json`: the complete canonical Academy experience;
- `manifest.json`: catalog metadata and exact physical measurements;
- `frame-*.png`: deterministic review frames derived from the record;
- `episode.mp4`: a disposable viewing derivative;
- `poster.png`: the gallery image.

`catalog.json` orders the review collection. V1 contains one A1 development
episode, one fresh held-out learned-relation test, and four negative controls.

The viewer loads the catalog first, requests posters lazily, requests only the
selected video, and fetches a record only for an explicit download. Local media
requests are confined below the selected episode root and videos support byte
ranges. The viewer may select, filter, play, pause, and download records. None
of those actions are admitted to the organism. Captions, labels, metrics,
posters, and video timing are Academy observations, not TrueLearner state.

The evidence boundary is therefore:

```text
physical run -> canonical record -> frames/video -> human review
```

There is no edge from the viewer or derived media back into the physical run.
