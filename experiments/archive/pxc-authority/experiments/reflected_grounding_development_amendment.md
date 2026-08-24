# RG0a development-harness amendment

Status: recorded before the definitive RG0a run. This amendment changes only
the permitted non-claim development loop. It does not change the definitive
seeds, depths, arms, controls, measurements, conjunctive gates, or exactly-once
execution boundary in `reflected-grounding-rg0a-v1`.

## Reason

After the protocol was tagged, the research process explicitly adopted a
three-speed harness so mechanism debugging would not repeatedly repay frozen
acquisition or invoke the repository-wide regression suite:

- **MICRO**: a tiny, evaluator-constructed, non-definitive fixture;
- **GATE**: the same non-definitive fixture with every qualitative control;
- **DEFINITIVE**: the unchanged preregistered frozen-seed matrix, exactly once.

This supersedes the protocol's one-smoke wording for MICRO/GATE only. MICRO and
GATE may be repeated while implementation changes because they:

- use development seed index `10000`, never definitive seeds `0..8`;
- construct role/program state directly rather than reconstruct an RP0a
  definitive learner;
- write no result artifact;
- set `claim_eligible=false` and `passed=false` in their reports;
- cannot satisfy, modify, or preview the frozen definitive gate.

This amendment is transparent rather than retroactive preregistration: MICRO
and GATE had already shown qualitative positives when it was written. Those
outputs remain development evidence only. The definitive matrix has not run,
and no RG0a result artifact exists at this commit.

## Regression policy

The full legacy suite is required when shared/frozen substrate code changes or
at a final integration boundary. RG0a-only child-module or harness changes use
targeted RG0a/research-runtime tests, strict formatting and Clippy, and GATE.

One repository-wide pass was run during implementation. All `154` unchanged
legacy tests and five new tests passed; the sole failure was a new RG0a MICRO
debug overflow caused by using `usize::MAX` as a display seed. The fix replaced
that sentinel with development seed `10000`, after which the affected debug
tests, strict checks, and GATE all passed. The unchanged legacy suite was not
rerun solely for that isolated RG0a-only fix.

## Performance boundary

The harness now keeps the scientific work counter unchanged while reducing
host work:

- immutable permanent fixtures are shared through `Arc`;
- each counterfactual owns only a small episode workspace;
- temporary role bindings are fixed dense arrays;
- hot counters are primitive integers;
- learned-arrow selection is allocation-free in the hot path;
- rows aggregate in memory by arm/seed/depth and serialize only after a run;
- independent definitive seed cells execute in parallel while each organism
  remains single-threaded and deterministic;
- one binary parameterizes MICRO, GATE, and DEFINITIVE.

On the persistent E2B worker, cached GATE execution completed in `0.01s` of
binary runtime. Compilation remains separate from experiment execution.
