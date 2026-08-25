# CR0 v2 static-audit repair v1 invalid execution

Status: invalid static-audit execution; no CR0 result handoff established.

Frozen audit repair: `c9fca27`.

E2B sandbox: `io8xxjgjrziv44t7r1ukj`.

The repaired FD1 age/tick predicate passed, as did all protected hashes. The
worker then reported `rg: command not found` for both forbidden-surface
searches. Because each command appeared as an `if` condition, Bash treated
exit 127 as a false condition and the script incorrectly printed its PASS
marker.

The execution is invalid rather than positive. It ran no Rust and no physical
world. The frozen CR0 v2 physical matrix remains unchanged and must not rerun.

A second audit-only repair is eligible to replace only those two `rg -n`
invocations with equivalent `grep -En` invocations available in the worker.
