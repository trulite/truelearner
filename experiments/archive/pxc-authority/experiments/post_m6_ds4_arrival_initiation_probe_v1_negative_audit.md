# Post-M6 DS4 arrival-initiation PROBE v1 negative audit

Status: **FROZEN EXPECTED DEVELOPMENT NEGATIVE; MECHANICAL RETRY ELIGIBLE**.

The first execution under
`post-m6-ds4-arrival-initiation-v2` used clean implementation commit
`16ff6ffe9ddbc72b4d610277e4760df2f7e730d6` and development seed
`140_000_000`. It ran in dedicated persistent E2B sandbox
`icmxrqcsf8br7shgus934`, which was left running.

## Exact implementation

- successor source SHA-256:
  `efbde88ae76eca26f96fb59f2cba1ac6e2b23c4ba136a921ee835a32f6690933`;
- runner SHA-256:
  `cd288a4ca0cbb58c86ca5085d4149cfae5366c9688fbe8488446cc00549bf092`;
- build/hash plumbing SHA-256:
  `01db0fd77430180ceac0aa1b1c7d8a6a88dba41d08fe0ce2756c785d87064dee`;
- frozen protocol SHA-256:
  `01c47af6fe1be9dc1e48a4b81a94e194df36d1631e1d650c8f2e94284bd42d6b`;
- result SHA-256:
  `46dac216ed3977ec8d12821af1b5b69d93f6932e20364c4d4c1b5809b3fba9c1`.

## Observed first collapse

The physical prefix was nonzero:

```text
learned M3 completion activity  2
occurrence selections           1
physical arrival path           true
```

The frozen pre-M6 linker nevertheless contained exactly one evaluator-derived
`functional` feedback call and zero M6 differential links. With that semantic
primitive removed, lawful active-trace updates were exactly zero. The first
collapse is therefore:

```text
downstream physical recurrence
  X
frozen M6 differential -> active occurrence trace
```

This is not the old DS4 definitive negative and does not reinterpret it. The
old CSV/report and M6 CSV/report/handoff matched their frozen digests.

## Focused validation

Remote validation passed:

- `cargo fmt --all -- --check`;
- focused binary compilation;
- release Clippy with all warnings denied except the Rust 1.97
  `derivable_impls`, `manual_is_multiple_of`, and `manual_div_ceil` style
  classes emitted only by byte-frozen M5/M6 source;
- exact v1 collapse test: `1 passed`, `353 filtered out`;
- exact definitive-inert test: `1 passed`, `353 filtered out`;
- definitive CLI refusal before any learner or seed construction.

No definitive seed, cell, command, or artifact was touched. The downloaded
remote result matched the locally frozen result byte for byte.

## Mechanical continuation

Exactly one missing physical relation was preregistered and observed. The
unchanged target may therefore install only:

```text
frozen M6 delayed physical-consequence differential
  -> already-active occurrence-pattern trace
```

No other mechanism change is authorized. A positive retry permits MICRO; a
failure or second plausible relation triggers the protocol stopping rule.
