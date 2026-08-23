# CJ0-D implementation storage correction protocol v1

Status: **PREREGISTERED MECHANICAL CORRECTION; NO EVIDENCE SPENT**.

Implementation tag `cj0-d-local-subunit-development-implementation-v1`
accidentally tracked the isolated crate's generated `target/` directory
because the repository root ignore rule does not cover nested Cargo targets.
The defect is mechanically unique and concerns storage/accounting only.

Authorized correction:

1. preserve the v1 implementation commit and annotated tag;
2. remove exactly `arms/cj0-d-local-subunit/target/` from version control;
3. add `arms/cj0-d-local-subunit/.gitignore` containing `/target/`;
4. make no change to the candidate law, evaluator, fixed constants, tests, or
   development protocol;
5. rerun format, focused tests, strict Clippy, refusal, source audit, and
   no-CELL preflight;
6. freeze a v2 implementation commit/tag before executing PROBE.

This protocol does not authorize a scientific amendment, stage execution,
negative rescue, PX0--PX2 modification, definitive evidence, or authority.

