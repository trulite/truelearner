# Cumulative DS8 non-semantic-credit GATE v3 de-aliased retry

Protocol identifier: `ds8-cumulative-semantic-credit-gate-v3`.

Status: **PREREGISTERED DEVELOPMENT RETRY; OUTCOME UNSPENT**.

Exact negative parent:
`675d10df1639f8480af58cbc47185801cc8a5bbc` /
`ds8-cumulative-semantic-credit-gate-v2-negative`.

Frozen v2 result/audit SHA-256:

```text
8bdedbad412376188c20ebbc887404c9f67e04ce5184162ec4a6a1576e61df9e
7aedfb788d1e5ed502560dbafb95051c20fe67db6a5111fab96142097fe70f9d
```

## Sole change

The evaluator-side varying-consequence topology index changes from:

```text
index mod 4
```

to:

```text
(index + floor(index / 4)) mod 4
```

This schedule remains deterministic, uses each of the same four physical
equal-magnitude topologies exactly twice in every eight consecutive
observations and exactly four times in every sixteen, and carries no semantic
polarity. Unlike the old period-four order, observations separated by the
M5 exploration period of eight alternate topology rather than aliasing to one
topology.

Apply this function everywhere the fixture supplies a varying consequence,
including acquisition, retained exploration, and raw-history reversal. Stable
consequences and all organism-visible mechanics remain byte-identical.

The same 18 cells, seeds, loads, thresholds, M5 exploration, DS8 learner and
linker, lifecycle exposure, instrumentation, source controls, held-out tests,
and conjunctive pass criteria remain unchanged. Run focused E2B preflight and
execute GATE v3 once in release mode. PASS permits definitive readiness; FAIL
freezes the remaining collapse. M5 remains authoritative.
