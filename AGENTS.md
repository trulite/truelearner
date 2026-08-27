# Agent Routing

```text
Request
  |-- Academy work --------> $academy
  |-- Rust model ----------> $categorical-rust
  |-- Rust change ---------> $rust-plan -> $rust-implement -> $rust-verify
  |-- Research program ----> $research-program
  |-- Research forecast ---> $research-forecast
  |-- Experiment batch ----> $research-campaign -> $research-converge
  |-- Benchmark frontier --> $benchmark-climb
  |-- Frozen evidence -----> $research-adjudicate
  `-- Skill work ----------> $skill-author
```

## Locations

- Skills: `.agents/skills/`
- Factory templates, runners, validators, and tests: `factory/`
- Research programs, lessons, runtime, validators, and tests: `research/`
- Human-facing overview: `README.md`

## Boundaries

- Use `uv run` for repository Python, `uv run --with` for ephemeral libraries,
  and `uvx` for packaged Python CLIs; do not use bare `python` or `pip`.
- Keep category theory in the modeling discipline; emit simple, idiomatic Rust.
- Keep the representative warm regression suite strictly under 10 seconds; record cold bootstrap separately.
- Keep research and the implementation factory loosely coupled through neutral, content-addressed protocol and evidence envelopes only.
- Parallelize discovery in isolated E2B workers, stop falsified arms early, preserve failures, and converge every declared round.
- Freeze authority protocols and run them once in a fresh sandbox; never infer authority from discovery or software success.
- Preserve references, negative controls, replay, Production equality, natural quiescence, and evaluator isolation where applicable.
- Let benchmarks expose missing physics; never let them specify what the organism may know.
- Keep routing and boundaries here, judgment in `SKILL.md`, and deterministic mechanics in `factory/` or `research/`.
