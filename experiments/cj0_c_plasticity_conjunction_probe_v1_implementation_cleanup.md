# CJ0-C PROBE v1 implementation packaging cleanup

Status: **MECHANICAL CLEANUP; SCIENTIFIC IMPLEMENTATION UNCHANGED**.

The first implementation commit `2c5b34c` accidentally tracked the isolated
package's reproducible Cargo `target/` directory because the package is an
independent workspace and the repository-root `/target/` ignore does not
match nested build directories.

This cleanup removes exactly the generated
`arms/cj-c-plasticity-conjunction/target/` paths and adds an arm-local
`/target/` ignore rule. The manifest, lockfile, build script, authoritative
include boundary, organism-visible addition, runner, protocols, hashes, and
all scientific clauses are byte-identical to the frozen implementation tag.
No organism execution or result artifact occurred before this cleanup.

The original implementation commit/tag remain immutable. The cleaned commit
receives a distinct annotated implementation tag and is the only eligible
parent for the one-shot PROBE execution.
