# CJ0 Arm A coincidence-threshold CELL PROBE v1 implementation audit

Status: **IMPLEMENTED; DEVELOPMENT EVIDENCE UNSPENT**.

## Exact lineage and bytes

- authoritative ancestor:
  `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`;
- protocol commit/tag: `0df3dd2`,
  `cj0-a-coincidence-threshold-probe-v1-protocol`;
- protocol SHA-256:
  `aab8b7ed8eb8b96b6dd3b8fef95775d77cf797c1b2329284f93997a6c9db6236`;
- sidecar source:
  `crates/px0-physical-correspondence/examples/cj0_a_coincidence_threshold.rs`;
- sidecar source SHA-256:
  `419bc9fc95d6f4d432988e7ccad4117c4ec3fee30c7a2fa5c2dee312ddac1cac`;
- organism-visible physical block SHA-256:
  `179737cd7dbe8a45257c4d77afe41505ced671bcb653e9494976b4540103ca87`;
- authoritative substrate source SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

The diff from the authoritative ancestor contains only the fresh protocol,
this audit, and the fresh sidecar. No PX0--PX2 byte or frozen negative is
modified. The implementation imports only the authoritative public physical
API and adds no substrate law, state field, dependency, or workspace member.

## Physical construction audit

The sidecar literally constructs the preregistered complete six-site field.
Every site has two threshold-`2` source ports, one threshold-`3` convergence
CELL, one outward effect CELL, coupling-`1` resistance-`1` local input
opportunities, and fixed return/outward ARROWs. Mirroring, allocation order,
physical identity order, and insertion order are fixed by the two registered
variants. The A+B/C+D schedule, held-out clones, repeated-singleton control,
A+D/C+B reversal, and all common controls are fixed in source.

The only organism-visible arm function calls ordinary authoritative
propagation. Its isolated block contains none of the forbidden Event, Episode,
History, Pair, Group, member, semantic, evaluator, trained, or crossed tokens.
The numeric incidence table and route letters are evaluator/environment
fixtures; they select external physical source-port arrivals only and cannot
select a local update or read an expected answer.

## Pre-evidence validation

- focused formatting: pass;
- focused `cargo check`: pass;
- focused strict Clippy (`-D warnings`): pass;
- no-argument refusal: exit `2`, pass;
- wrong-argument refusal: exit `2`, pass;
- no-CELL `--preflight`: pass;
- exact authoritative source/tag and frozen PX3/PX3-R endpoint audit: pass;
- result and staging paths absent: pass;
- diff whitespace check: pass;
- existing authoritative paths changed: none.

No `--probe` execution, evidence marker, result row, MICRO, GATE, recursion,
OR/timing matrix, definitive evidence, or authority execution has occurred.
The implementation commit and annotated tag are created next; the sole PROBE
command may run only from that frozen implementation.
