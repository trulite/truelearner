# SV1 compartmentalized signed variation handoff v1

Status: Static Gate A negative; stopped before runtime selection.

## Outcome

```text
SV0
ordinary variation proposes weak Drive +1 and -1 ARROWs
at one shared contact
→ symmetric and bounded
→ consequence cannot select between co-participating alternatives

SV1 Gate A
required separate ordinary C+ and C- contact CELLs
→ accepted variation can add ARROWs only among existing CELLs
→ cannot construct the required topology
→ STOP NEGATIVE
```

Supplying the contact CELLs in the evaluator would encode the spatial
attribution resolution that SV1 was intended to make developmental. Therefore
none of the positive/negative selection, permutation, shared-contact, bounded
variation, or Reference/Production runtime gates ran.

## Frozen lineage

- SV0 parent/result: `9ad6a5f`, tag
  `sv0-symmetric-sign-variation-result-v1`;
- protocol and initial audit: `700f491`, tag
  `sv1-compartmentalized-signed-variation-protocol-v1`;
- multiline source-sentinel correction: `a16ee47`;
- fresh E2B static evidence: `ijp33os0mttzgo1s9zrdd`;
- executable core changes: zero;
- Rust compilation, tests, evaluator, and organism execution: none.

## Boundary

Do not rerun RS2 Gate B by preconstructing `C+` and `C-`. That would supply the
missing topology rather than show ordinary variation discovering it.

A successor may independently investigate ordinary contact-compartment
variation. It must remain sign-symmetric, local, bounded, and ignorant of
usefulness. SV1 authorizes no preferred sign, selected candidate identity,
inhibitory-contact constructor, semantic reward, or evaluator topology
template.

FD2, ARC, authority, the oracle, and `arch.md` remain unchanged.
