# CPC1 local continuous temporal participation implementation audit v1

Status: candidate and evaluator frozen before matrix execution.

## Candidate boundary

- Feature: `cpc1`, default off.
- New ARROW-local state: fixed-point `participation_level` and
  `plastic_support` only.
- Traversal adds one universal impulse.
- Ordinary elapsed time applies one universal proportional relaxation.
- Modulatory arrival performs unconditional saturating arithmetic at each live
  outgoing contact; candidate support never reads retained eligibility.
- No new `PhysicalEvent`, contact ID, return ID, reward ID, path ID, timeout,
  countdown, threshold, or pressure interaction was added.
- Existing LR-C resistance/coupling update is compiled unchanged by default and
  omitted only in the candidate feature build.
- Existing eligibility remains solely for unchanged pressure bookkeeping.

Candidate state is represented in both AoS and SoA resident layouts through
the same shared physical transition code. It is intentionally absent from
durable/live checkpoint formats at this development stage.

## Frozen hashes

```text
core Cargo.toml
5415bb6bed57814e8aa0cab6750a0ca138c8ad77bcfd88a91ddbb177d5bc301e

core lib.rs
027ec827afbf998df07749e428468196f82eb33824401b78aa15a6b48680a6cb

core mechanics.rs
5093e259a324b72a2fd661e1d402030fed356ac19d3b948549d7eea37f8b7295

evaluator Cargo.toml
6eea078e773f018673204f74974cfa7caeca48d6886527defaae0bf42d93d0df

evaluator main.rs
3a230d704174d8110bc6e6e981bb031af4f56341e1fc3a8b457577e46e8d93d0

static audit
4e8c02924b98b9bdf67b16bbc0a693bd35a02656202822bb509d4ed5f27b0453
```

## Targeted E2B validation

Reusable sandbox `i45f1g5a6ob5ww5x6ngke` ran only:

- core and evaluator rustfmt;
- candidate feature strict release Clippy with `-D warnings`;
- default core release tests: `15/15`;
- default core strict release Clippy with `-D warnings`.

All passed at commit
`b62ab99eebb624877ae17634531bf8387ab36dd2`. No CPC1 physical world or result
artifact has run.

## Comparator

The candidate adds no physical-trace event. The evaluator separately compares
the complete ordered retained trace and the complete candidate local state.
Each world is reconstructed twice per mechanics, and Reference/Production
equality is asserted before serialization. No normalization or rescue path is
present.
