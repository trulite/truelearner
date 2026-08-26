# CPC1 static-audit scope repair protocol v1

Status: frozen after the CPC1 physical matrix and before audit repair.

Parent evidence: CPC1 positive artifacts at commit `1edb4f7`.

## Observed defect

The frozen v1 audit uses this broad alternative:

```text
participation.*pressure
```

It scans the evaluator as well as the candidate physics. The evaluator's CSV
header lawfully contains `a_participation` before `pressure_phase`, so the
audit reports a forbidden temporal scaffold even though it is matching two
independent observation columns in a string literal.

The same false positive occurred after both the sole primary matrix and the
fresh replay. In both runs the physical matrix completed before the static
audit. The replayed four-artifact set is already byte-identical to the frozen
positive evidence.

## Authorized repair

Create audit v2 without changing audit v1. V2 may change only the forbidden-
scaffold scan:

- scan the physical candidate implementation files only;
- anchor an `if ... participation` search to actual Rust `if` statements;
- retain searches for deadline/countdown names, participation/eligibility or
  participation/pressure coupling, candidate event variants, and attribution
  identifiers;
- retain all row counts, report predicates, and artifact checksums unchanged.

No Rust source, evaluator, candidate constant, world, artifact, or matrix may
change. No compilation or physical-world execution is authorized for the
repair. One fresh E2B worker must run audit v2 against the frozen commit.

## Decision

- V2 failure: CPC1 readiness remains blocked on static audit.
- V2 pass: accept the already-frozen positive physical evidence and its exact
  artifact replay; record the audit repair transparently.
