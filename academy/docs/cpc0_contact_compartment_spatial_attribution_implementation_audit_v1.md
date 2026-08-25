# CPC0 contact-compartment spatial attribution implementation audit v1

Status: evaluator frozen before matrix execution.

## Scope audit

- Active runtime or substrate-law files changed: none.
- LR-C, retained eligibility, pressure, proposal, timing, and ordering laws:
  unchanged and hash-locked by the protocol.
- New active type, transmission mode, contact identifier, return identifier,
  or targeted-credit field: none.
- Contact compartments: ordinary experiment-constructed CELLs.
- Contact and return paths: ordinary Drive and Modulatory ARROWs.
- Evaluator mutation of organism state: only public physical construction,
  admitted `SpikeInput`, and ordinary time progression.

## Frozen hashes

```text
evaluator Cargo.toml
9c186e9f7f4d60b0877aa4ae5942aad730b34ebf64e7b95c3278ff525af14d52

evaluator main.rs
70df5e2a7d87d515a154f58de154b64191c3e3b37baef803b80208478ffdbe48

static audit
9a36640d8560ab24071eb2c92c74a36b46687b3cec6337d4891578988565eed9
```

## Targeted E2B validation

Reusable sandbox `inlgs9h8g1992uxva5t86` ran only:

- evaluator rustfmt;
- evaluator strict release Clippy with `-D warnings`.

Both passed at commit
`0a94660fd641700e316f6e320a3bf9866ef320cf`. No CPC0 physical world or result
artifact has run.

The accepted core was not recompiled because its three preregistered hashes
remain exact and CPC0 changes no runtime file.

## Comparator

Every world is reconstructed from scratch for Reference, Reference replay,
Production, and Production replay. Before serialization, the evaluator
requires exact equality of the complete ordered `PhysicalTransition` vector,
physical work, durable body, clock, pressure phase, quiescence, and all
scenario observations.

There is no event sorting, trace normalization, or post-failure comparator
repair path.
