# Agent Routing

```text
Request
  |-- Academy work --------> $academy
  |-- Debug or trace ------> $causal-debug
  |-- Benchmark frontier --> $benchmark-climb
  |-- Development ---------> $dev
  `-- Skill work ----------> $skill-author
```

## Locations

- Skills: `.agents/skills/`
- Human-facing overview: `README.md`

## Boundaries

- Use `uv run` for repository Python, `uv run --with` for ephemeral libraries,
  and `uvx` for packaged Python CLIs; do not use bare `python` or `pip`.
- Apply category-theory and TAME lenses to development, but emit simple,
  idiomatic Rust.
- Keep representative warm wave time strictly under 25 ns unless the user
  expressly approves otherwise.
- Keep the representative warm regression suite strictly under 10 seconds; record cold bootstrap separately.
- Preserve references, negative controls, replay, Production equality, natural quiescence, and evaluator isolation where applicable.
- Let benchmarks expose missing physics; never let them specify what the organism may know.
- Keep routing and boundaries here and development judgment in `$dev`.
