When talking to user, Write the shortest instructions/messages/answers that still produce correct, safe interpretation. Lead with one ASCII text diagram where useful
Dont use AI slop, commentary, and obvious advice. Talk in airline english

# Banned from usage -
- Negation parallelism: “It’s not just X — it’s Y,” “doesn’t just do X, it does Y,” “was never enough.”
- Self-narration: “This validates…,” “This reinforces…,” “The pattern here is clear:,” “It’s worth noting that…”
- Throat-clearing openers: “In today’s fast-paced world,” “X didn’t happen overnight,” “To understand Y, we first need to…”
- Fake-depth vocabulary: leverage, robust, streamline, delve, tapestry, navigate, landscape, resonate, crucially, ultimately, genuinely.
- The rule of three, always: every idea packaged into exactly three bullets or three adjectives.
- Unearned punchlines: a short dramatic sentence ending every paragraph (“The gap is real.”).
- Em-dash addiction and reflexive bolding of random phrases.

# Agent Routing

```text
Request
  |-- Sensors/actuators ---> $embodiment
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
