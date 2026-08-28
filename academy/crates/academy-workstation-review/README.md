# Academy workstation review

This optional crate converts a frozen, exactly replayed workstation recording
into observer PNG frames and an MP4. Each frame shows the organism's left and
right eye fields together with external movement, contact, device-event, work,
quiescence, and fingerprint annotations.

Review never receives a live harness. The command writes the canonical record,
decodes that exact file, verifies its ordinary replay, and only then renders:

```bash
cargo run --release --manifest-path academy/Cargo.toml \
  -p academy-workstation-review --bin academy-workstation-record -- \
  output/workstation-run --steps 48 --seed 82001
```

The destination contains `recording.tlwr`, `manifest.json`, source PNG frames,
`timeline.ffconcat`, and `episode.mp4`. Generated media remains under ignored
`output/` and is not organism authority or capability evidence by itself.
