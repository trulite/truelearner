# CPC0 static-audit portability repair protocol v1

Status: frozen after the immutable positive CPC0 matrix and before audit-v2
implementation or execution.

The physical matrix in fresh E2B sandbox `ifkzazwbrnie733ltd5vz` completed
`220/220` cases and wrote checksum-valid artifacts. The frozen v1 audit then
invoked unavailable command `rg` inside an `if` condition. Bash treated the
missing command as a false predicate and continued, so the semantic-source
scan did not execute even though the remaining hash, row, report, and checksum
checks passed.

This workflow may only add a portable audit-v2 script that:

- repeats the exact frozen core hashes;
- repeats artifact row/report/checksum checks;
- replaces `rg` with POSIX-available `grep -E` for the same forbidden source
  patterns;
- executes in E2B without running Rust or reconstructing a physical world.

The v1 script, physical matrix, evaluator, runtime, laws, and result artifacts
remain immutable. Audit-v2 success completes static coverage; failure stops
CPC0 readiness.
