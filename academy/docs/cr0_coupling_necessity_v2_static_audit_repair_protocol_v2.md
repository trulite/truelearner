# CR0 v2 static-audit repair protocol v2

Status: frozen after audit-repair v1's invalid tool execution and before the
v2 audit edit.

Parent: `cr0-coupling-necessity-v2-static-audit-repair-v1-invalid`
(`8916c73`).

## Sole repair

Replace the two forbidden-surface searches in
`experiments/tools/audit_cr0_coupling_necessity_v1.sh`:

```text
rg -n PATTERN FILE
```

with:

```text
grep -En PATTERN FILE
```

The patterns, files, branch behavior, hashes, anchor counts, and PASS marker
remain byte-identical. `grep` exit 0 still means a forbidden match and causes
failure; exit 1 means no match and permits continuation. Any other audit error
must fail under `set -euo pipefail`.

Run only the shell audit in a fresh E2B worker. No Rust compilation or physical
matrix may run.
