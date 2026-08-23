# CJ0 ARM CJ-B locally gated ARROW MICRO implementation audit

Status: **IMPLEMENTATION FROZEN; MICRO UNSPENT**.

| frozen source | SHA-256 |
|---|---|
| physical module `src/lib.rs` | `ef0de37a9ac54b632b991f0d4647a5ee78c23810084d61497c88d6f757ec2188` |
| frozen PROBE evaluator `src/bin/probe.rs` | `f8df9f19e76ff800ddcbd24a4eb7be7743c208dc30bfc6be962d6a5de9ce57ca` |
| MICRO evaluator `src/bin/micro.rs` | `d9d9c747fa655b2cfc76fa9b7e06329faee67542612bff3ce2f575ab80fa1a6c` |

Only the fresh MICRO evaluator was added. The physical law, frozen PROBE
source/results, authoritative PX0--PX2 files, and PX3/PX3-R negatives remain
byte-identical.

Pre-evidence formatting, two physical unit tests, all-target strict Clippy,
zero-dependency audit, frozen hashes, forbidden physical-source scan,
no-invalidation/source-selection scan, neutral wrong-argument and no-argument
refusal (`2`/`2`), no-CELL preflight, and final/staging artifact absence pass.

The implementation uses only ordinary global pressure and the generic local
proposal law. No endpoint-specific deletion, organization-change input,
historical state lookup, or evaluator feedback call exists. No MICRO cell or
later-stage surface has executed.
