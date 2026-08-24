# Boundary Buffers V1 authority result audit

Status: **AUTHORITY ESTABLISHED**.

Parent authority: `physical-body-v1-authority-v1` at
`c9daa0af96e87bc8b0e6ef0f30bea137e0cfc33b`.

Frozen successor source: `10e24f6682b121bf83ebe27867df410f44c9a7a8`.

Immutable evidence: `a712850`.

## Execution

- Formatting worker: `i2082eyezchg8y5tfe2bk`.
- Targeted tests, strict Clippy, and preflight: `izccpptqr3b87caqb7oey`.
- Sole cumulative authority execution: `iay987rwwjy2cttpc8ob6`.
- Evidence marker: `BOUNDARY_BUFFERS_V1_AUTHORITY_EVIDENCE_SPENT` exactly once.
- Established marker: `BOUNDARY_BUFFERS_V1_AUTHORITY_ESTABLISHED rows=16/16 clauses=548/548`.
- Every worker self-terminated; no Rust command ran locally.

## Result

- Cumulative PX0-PX8 rows: `16/16`.
- Cumulative row clauses: `512/512`.
- Cumulative globals: `12/12`.
- Physical Body V1 clauses: `16/16`.
- Boundary-buffer clauses: `8/8`.
- Total: `548/548`.
- Exact replay: true.
- Natural quiescence: true.
- Outward-only observation: true.
- Maximum work: `104331/200000`.
- Maximum resident body bytes: `44328/65536`.

The cumulative evaluator constructed the same physical worlds as the accepted
Physical Body V1 matrix but sent every batch through bounded enqueue, run, and
drain operations. Direct and buffered execution agreed exactly. Input
backpressure, output backpressure, oversize output rejection, FIFO partial
draining, canonical buffered live checkpointing, exact continuation, and
invalid configuration controls all passed.

Output backpressure is transactional: a run that cannot fit its crossings
leaves both substrate and queued inputs unchanged. No physical event is lost,
duplicated, or silently reordered.

## Frozen hashes

```text
protocol
55f04626f0bbaf6f7e5c673256e0ab7bb978ff2df3bd99c1862470ba6f8b2c58

evaluator
b8c6a5e91b8bedc8667125cf5bdbc922fb436b0278344e4a6d545110fb37c007

buffer controls
adb9337885e63e3c59d2ecdb080ef20c916afecfbbb0e31d1dc1fc5f46120291

canonical core
8a0f0c862a9aa6bfaf74a3a09ca5ee0eb6b3dc95e75ce76e5a136c9a8890ff0a

CSV
4a51aa85c115d0ea8f37c750252a8f1cb6461a91d3776817dfe1c0d2112e0abc

report
3f84c2494186471706bd0f8e8760595e020f68db6db4fc272a898b44f3f93c51
```

The capacity panics printed during evaluation are the preregistered and caught
Physical Body V1 bounded-allocation negative controls. They did not escape the
evaluator and are not buffer failures.
