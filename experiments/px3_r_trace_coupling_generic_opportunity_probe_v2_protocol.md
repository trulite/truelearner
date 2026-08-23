# PX3-R direct trace-coupling generic-opportunity PROBE v2 protocol

Status: **PREREGISTERED AMENDMENT; PROBE EVIDENCE UNSPENT; PX3 ABSENT**.

This protocol preserves the v1 protocol, implementation, spent marker, and
first-clause failure frozen at commit
`a36442a14a723ae8c9191f18552179d8937ebd14`, tag
`px3-r-trace-coupling-generic-opportunity-probe-v1-first-clause-failure`.
No v1 source or artifact may be changed or executed again.

## Mechanically unique amendment

V1 failed because the direct-external reference allowed newly proposed
positive ARROWs to connect threshold-1 firing loci, producing recurrent
autonomous activity. That reference was intended only to establish that the
generic spatial proposal law is reachable; matched activity belongs to the
actual-participation cell, not to the noncausal reference.

V2 leaves the actual-participation construction and schedule exact. It replaces
only the reference with one externally fired threshold-1 CELL at position `0`
and two passive local CELLs at positions `1` and `2`, threshold `100`, all
resistance `1000`. One external arrival fires the source once. The reference
passes only if it creates exactly two local proposals, neither passive CELL
fires, the source does not refire, and the queue drains naturally.

The actual cell still requires:

- four nearby trace-bearing CELL loci at positions `0,1,2,3`;
- six internally propagated participation occurrences per route;
- matched route counts and alternating `0+1` / `2+3` cluster order;
- zero generic proposals and zero inter-trace ARROWs;
- exact duplicate replay, zero autonomous source refiring, and natural
  quiescence.

If those clauses and the amended reference pass, the frozen scientific
collapse is `GENERIC_PROPOSAL_REQUIRES_EXTERNAL_FIRING`. Existing generic
physics is then insufficient for ARM A. Any other result is a new first-clause
failure and receives no rescue or rerun.

## Isolation and sole execution

All v1 lineage, frozen-hash, anti-representation, atomic-publication,
work-accounting, and authority restrictions remain binding. V2 uses a fresh
source file and fresh result paths; it does not modify the authoritative
substrate or the v1 source.

After the v2 implementation is committed, tagged, formatted, compiled,
strictly linted, and passed through a no-CELL preflight, exactly one command may
spend v2 evidence:

```text
cargo run --release -p px0-physical-correspondence \
  --example px3_r_trace_coupling_generic_opportunity_probe_v2 -- --probe
```

Atomic final artifacts are:

- `results/px3_r_trace_coupling_generic_opportunity_probe_v2.csv`;
- `results/px3_r_trace_coupling_generic_opportunity_probe_v2.md`.

Staging paths use the same basenames with a leading dot and `.staging` suffix.
This remains DEVELOPMENT discrimination only; it cannot advance PX3 or run an
authority matrix.
