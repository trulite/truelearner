# CJ0-E transient-coincidence implementation audit v1

Status: **FROZEN IMPLEMENTATION; DEVELOPMENT EVIDENCE UNSPENT**.

## Exact implementation

The arm is a standalone workspace beneath
`arms/cj0-e-transient-coincidence`. Its build script reads the frozen
authoritative law only after verifying SHA-256
`3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`,
applies two exact single-match textual substitutions for the preregistered
law, adds two read-only accounting accessors, and emits the isolated candidate
source into Cargo build output. The authoritative file is never written.

| source | SHA-256 |
|---|---|
| `Cargo.toml` | `ef36a0b8f497b922a9bfdc7a1849989ff92e2af0f53afda4b01650d25168e877` |
| `Cargo.lock` | `0523739d0b4e57952ae2dbeb619a5ad750a4b759e1e5f30b52659120e1acb4ac` |
| `build.rs` | `de69f4913e7c29379bdd13fddf8b70042d92159e59842ed0a9c2f5f9b21201b9` |
| `src/lib.rs` | `2da7f40828878bdd408cda2674d297f2c8eb65c63740baa9caa3d57b4f6c568a` |
| `src/bin/cj0_e.rs` | `df8d5c3cb1bcf5d35b345268deed7f40166b2c7307250723020f5907d699c963` |
| exact generated candidate source | `e64c8c915c2fbc4679d1e34ee69ecfe36e2c5ff05bdff5d7feeb5a55578bf1c1` |

The complete executable law delta is:

1. read `live_before = target.state > 0` after ordinary decay and before the
   arriving impulse;
2. for threshold-greater-than-one CELL matter, require `live_before` as well
   as threshold and refractory satisfaction;
3. allow that same transient-completed firing to invoke the unchanged local
   proposal operation;
4. expose read-only ARROW coupling and current-tick accessors to the discarded
   result serializer.

No field, persistent byte, decay rule, refractory rule, pressure rule,
eligibility rule, return update, proposal construction, ARROW traversal,
generation check, ordering rule, or fingerprint encoding changed.

## Pre-evidence validation

- exact frozen start/tag/clean proof: pass;
- focused formatting: pass;
- all-target focused compile: pass;
- inherited substrate unit test: `1/1` pass;
- strict all-target Clippy: pass;
- no-argument refusal: exit `2`;
- wrong-argument refusal: exit `2`;
- no-CELL `--preflight`: pass, definitive/authority false;
- PROBE/MICRO/GATE artifact absence: pass;
- authoritative-path diff from frozen start: empty;
- generated candidate forbidden whole-token scan for Event, Pair, Group,
  member, member-list, semantic, evaluator, serializer, Episode, and History:
  zero matches;
- generated diff contains only header relocation, the exact firing/proposal
  law, and two read-only accessors: pass;
- crate dependencies: zero;
- staging paths: absent.

The fixed binary serializes route firing, convergence firing, output firing,
local return, support resistance/coupling/live state, complete/permanent
fingerprints, ARROW count, persistent bytes, work, and quiescence for every
row. Stage artifacts are create-new staging files followed by atomic rename.

No scientific result is represented by this audit. PROBE may now execute
once. MICRO remains blocked unless PROBE publishes PASS; GATE remains blocked
unless MICRO also publishes PASS. No definitive or authority execution exists.
