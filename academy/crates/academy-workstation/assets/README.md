# Monitor asset

`coastal-monitor.png` is a project-bound raster generated with the built-in
image generation tool and then frozen for deterministic workstation rendering.

- Dimensions: 1536×1024 RGB PNG
- SHA-256: `a25049d016cd28d70b464040c57997fe9aa69db1502a1ff25e071b95b6768a47`
- Use: ordinary photographic content displayed inside the external monitor

Final prompt:

```text
Use case: photorealistic-natural
Asset type: fixed monitor photograph for a headless workstation simulation
Primary request: a realistic editorial landscape photograph of a rugged green
coastline beside a deep blue sea, with layered cliffs, natural vegetation, and
distant atmospheric haze
Scene/backdrop: outdoor coastal landscape only
Style/medium: natural high-resolution photography, realistic textures and optics
Composition/framing: landscape 3:2 composition, strong foreground-to-distance
depth, no people, no devices
Lighting/mood: soft late-afternoon daylight, calm and clear
Constraints: no text, no logos, no watermark, no UI, no frame, no border, no
surreal elements; image must remain legible when shown small on a monitor
```

The learner receives only grayscale raster samples derived from the rendered
scene. It never receives this filename, prompt, digest, or asset identity.
