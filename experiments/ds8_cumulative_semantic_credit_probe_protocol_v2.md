# Cumulative DS8 non-semantic-credit PROBE v2 mechanical retry

Protocol identifier: `ds8-cumulative-semantic-credit-probe-v2`.

Status: **PREREGISTERED DEVELOPMENT RETRY; OUTCOME UNSPENT**.

Exact negative parent:
`cbb8af415766111a7e1b4d305f27435bd58e26d2` /
`ds8-cumulative-semantic-credit-probe-v1-negative`.

Frozen v1 evidence:

- result SHA-256:
  `31ae6bcabcca510496a586f27fa1b9cda42d69142f64f442513dd888321569ca`;
- audit SHA-256:
  `7921c4ddaecf1fd06a0b73d56ff5373fc2b8bbb9211d4a520ff2e8ff88a574a0`;
- mechanism source SHA-256:
  `9b8284a74381afeae6d12179e187eb069d8c7bf377ca371f141c28436a2f3c44`.

## Mechanically demonstrated mismatch

V1 required all 24 scheduled encounter presentations to form and execute a
variation. The frozen M5 allocator learned during that same history and
lawfully suppressed later presentations of one encounter. A suppressed
variation cannot cause a downstream physical consequence.

V2 therefore changes only the wrapper expectations:

1. both histories must form and execute during their first blank-start
   presentation;
2. every variation that actually executes must be followed by exactly one
   delayed physical consequence with three spike firings and two arrow
   traversals;
3. suppressed encounters must produce zero fabricated consequence or update;
4. the total consequence count is history-dependent rather than fixed at 24.

The M5 mechanism, consequence learner, differential linker, seed `40_000_000`,
episode order, raw consequence histories, thresholds, and all functional
controls remain byte-identical.

## Source-audit preflight

V1 also reported a source-audit failure although a post-result static
diagnostic found:

```text
all frozen file hashes                         exact
forbidden semantic tokens in organism path    0
differential -> delayed-experience linker      1
physical normalizer                            1
occurrence identity in organism path           0
```

V2 may expose these component predicates through a no-cell `--audit` command.
That command must pass before the seed runs. This is audit observability only;
it may not enter or execute the probe cell.

## Execution and interpretation

Commit and tag the exact retry before validation. In the dedicated E2B
development sandbox, run formatting, focused library compilation, and the
no-cell audit. If they pass, execute the v2 PROBE once in release mode. Do not
rerun or rescue it.

All original v1 conjunctive controls remain, with checks 1 and 2 interpreted
only through the explicit mechanical amendments above. PASS warrants MICRO;
M5 remains authoritative. FAIL freezes the new first collapse. Any mechanism
change, new representation, semantic choice, or non-mechanical control change
requires a scientific stop.
