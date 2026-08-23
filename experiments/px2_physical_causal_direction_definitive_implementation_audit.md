# PX2 physical causal direction definitive implementation audit

Status: **IMPLEMENTATION READY TO FREEZE; DEFINITIVE EVIDENCE UNSPENT; PX2 AUTHORITY ABSENT**.

## Frozen implementation

- source:
  `crates/px0-physical-correspondence/examples/px2_physical_causal_direction.rs`;
- definitive source SHA-256:
  `c47d605371d5787cffc7d456f1d9e38168b4b203063fb9dcdeefcf630fa4aed5`;
- definitive protocol SHA-256:
  `d55a33c76b3a2f5f9421e85147e116e8a67ef99e9401f630cecccd065f519cd2`;
- frozen developed source SHA-256:
  `af0c781eb0b53a7e972497ab4e247e8db2c74b5cf61e06e4de82a8da7be74151`.

The authority implementation adds only the fixed fresh matrix, evaluator-side
`P0–P9` serialization, recorded boundary fields, refusal/preflight handling,
and staged write-once publication.

The physical execution and learning blocks are byte-identical to the frozen
developed source:

| physical block | definitive SHA-256 | frozen SHA-256 | exact |
|---|---|---|---|
| `run_world` | `9defb7d4e13749665109356c120e4ea427e3bb1bfd0078d8f2f509a7866f2d83` | `9defb7d4e13749665109356c120e4ea427e3bb1bfd0078d8f2f509a7866f2d83` | yes |
| `run_lifecycle` | `e7746091b8044e27a61811955e6ca7a662b35431ed5d4ebf07257c30e497b1ad` | `e7746091b8044e27a61811955e6ca7a662b35431ed5d4ebf07257c30e497b1ad` | yes |
| `train` | `36fb8a9811055342fffa0e32c82d072593d125b9fc35b230b2a507827e30931f` | `36fb8a9811055342fffa0e32c82d072593d125b9fc35b230b2a507827e30931f` | yes |
| `acquire_correspondence` | `9167653a737c1eb44ab8cd2c02c492c4193f6e202ac296d5dd72e149ae2179cb` | `9167653a737c1eb44ab8cd2c02c492c4193f6e202ac296d5dd72e149ae2179cb` | yes |
| `add_directional_candidates` | `adb604d28c23bcffa01b46dcc1eaafa0454140c18e21b2673fd49bfce815db27` | `adb604d28c23bcffa01b46dcc1eaafa0454140c18e21b2673fd49bfce815db27` | yes |
| `build_world` | `b9b81c7b71225f135207a7d8af28ce1816db0f80e3be433c6811198f34bfe333` | `b9b81c7b71225f135207a7d8af28ce1816db0f80e3be433c6811198f34bfe333` | yes |
| `measure_execution` | `25d76c0f2f4d9adf38b6a7d63de08c1370430752474fc2b93db868e22dc8621a` | `25d76c0f2f4d9adf38b6a7d63de08c1370430752474fc2b93db868e22dc8621a` | yes |

## Corrected measurement semantics

- Seven causal worlds, not the failed GATE's interleaving/lifecycle worlds,
  determine authority.
- Schedule class, arrival order, first-use delay, the O1 dead-opportunity
  boundary, and final resistance are serialized.
- No pass clause requires equal evidence totals to yield equal strength.
- No pass clause requires a weak unused proposal to survive arbitrary delay.
- A read-only audit of the `28` applicable frozen GATE rows found `28/28`
  satisfy the definitive return/coincidence clause; no cell was executed.

## Pre-evidence validation

- formatting: pass;
- focused compilation: pass;
- strict focused Clippy: pass;
- focused dependency-free substrate test: `1/1` pass;
- no-argument refusal: exit `2` before audit/cells;
- wrong-argument refusal: exit `2` before audit/cells;
- no-cell `--preflight`: pass with exactly one preflight marker;
- evidence marker during validation: absent;
- final and staging result paths: absent;
- sorted pre-existing results digest:
  `6235145cbc1593247fed40b3b75410dfea17d22bbbe8d615051edbd07b256c3d`;
- fresh namespace base `0x5_4200_0000`: absent from prior source/evidence;
- authoritative PX0/PX1, immutable GATE, H1, O1, and boundary-handoff hashes:
  exact.

No definitive seed, cell, duplicate, result, or evidence marker executed during
validation. The sole `--definitive` command remains unspent.

