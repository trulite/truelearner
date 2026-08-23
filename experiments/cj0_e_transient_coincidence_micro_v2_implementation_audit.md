# CJ0-E transient-coincidence MICRO v2 implementation audit

Status: **FROZEN V2 EVALUATOR; MICRO V2 UNSPENT; DEVELOPMENT-ONLY**.

## Isolation and exact delta

V2 is a fresh standalone evaluator crate at
`arms/cj0-e-transient-coincidence-v2`. It depends directly on the frozen v1
physical-law library. The dependency build produced generated physical source
SHA-256
`e64c8c915c2fbc4679d1e34ee69ecfe36e2c5ff05bdff5d7feeb5a55578bf1c1`,
exactly matching v1.

No v1 path changed. The only new executable logic reconstructs the already
frozen v1 MICRO physical worlds under fresh namespaces and serializes the
specific locus-to-output ARROW evidence alongside the unchanged aggregate
work counter.

| v2 source | SHA-256 |
|---|---|
| accounting-correction protocol | `53c58316620f4bf528a1e29da08118e70897adc2661cf844be53b68bd43e4998` |
| `Cargo.toml` | `18893fa234fb40b6c29694e5ae1b72a6092a65126605f11e050dedb974a9e4cb` |
| `Cargo.lock` | `9ffd46b9e621d21ab4de1cd891f35a7895bbcaf0a1dadd8c6f221296fcb192e4` |
| v2 evaluator source | `f72f481bfaf0462bcadf7a65179a37e2d91b4bfda32c8aa03501f62a93e82695` |
| exact generated physical source | `e64c8c915c2fbc4679d1e34ee69ecfe36e2c5ff05bdff5d7feeb5a55578bf1c1` |

The self-evidence row observes numeric physical endpoints already exposed by
ordinary crossing records. It has no update path into the substrate. Before
and after each repeated-A occurrence it reads the learned ARROW's resistance,
coupling, generation, and live state. It counts physical traversal and output
firing separately. It also records the full aggregate
`local_return_updates` value without filtering or subtraction.

## Pre-execution validation

- clean, remote-exact v1 handoff start: pass;
- all ten frozen v1 file hashes: exact before and after v2 compilation;
- all five v1 tag targets: exact;
- v1 generated physical source and v2 dependency-generated physical source:
  byte-identical;
- formatting: pass;
- all-target focused compile: pass;
- focused tests: pass;
- strict all-target Clippy: pass;
- missing-argument refusal: exit `2`;
- wrong-argument refusal: exit `2`;
- no-CELL preflight: pass;
- fresh v2 namespaces and artifact paths: pass;
- result and staging paths: absent;
- generated physical-source forbidden-token scan: zero matches;
- newly created later-stage surface: none.

The v2 MICRO binary may now execute once. It atomically publishes its result
regardless of pass or failure. Failure stops development. Pass permits only a
separately frozen development GATE protocol and implementation.
