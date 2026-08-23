# PX3-R Arm B anonymous shared-CELL PROBE v2 implementation audit

Status: **IMPLEMENTATION READY TO FREEZE; EVIDENCE UNSPENT; PX3 AUTHORITY ABSENT**.

## Frozen candidate

- protocol commit: `56bd44bfedbe7c5171106b1cb8c2f0c363128aa0`, tag
  `px3-r-shared-cell-probe-v2-protocol`;
- protocol SHA-256:
  `59c65606ad792a6b0c78e3f3d4a829eb69d77364f39df26a33edda08959fe844`;
- source:
  `crates/px0-physical-correspondence/examples/px3_r_shared_cell_probe_v2.rs`;
- source SHA-256:
  `1ae7592134db8692a95b2ca38d837306c5021f79df9e2ddf9f5e32b6d660631d`;
- organism-visible physical block SHA-256:
  `4a73fde5ef122d54411b35b260a4a28fc9dac7bb454a2a928a66c0302070269f`;
- exact frozen start: `873094497ff6eb74363191dc5edc479c7d66de72`;
- authoritative PX2 ancestor:
  `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`;
- frozen v1 failure: `74813a268f593340703f1fbe4510de57e3c25276`.

No v2 result/staging artifact exists. No v2 CELL has been entered and no
propagation, simulation, `--probe`, MICRO, GATE, authority matrix, or
definitive evidence has run.

## Exact implementation delta

A byte comparison of the v1 and v2 organism-visible blocks shows exactly two
changed lines:

1. each of the six anonymous local CELL specifications uses threshold `4`
   instead of `2`;
2. each of the 12 pre-existing weak resistance-`1` incoming ARROW
   specifications uses coupling `2` instead of `1`.

All source CELLS, drivers, distractors, outputs, return and outward ARROWs,
positions, regions, delays, phases, resistances, schedules, local proposal,
state reads, and execution calls remain byte-identical. Evaluator changes are
limited to v2 paths, markers, namespace constants, frozen-v1 audits, and
fresh-file isolation.

The changed setting preserves single-incident insufficiency at the retained
coupling ceiling (`2 < 4`) and exact two-incident sufficiency (`2 + 2 = 4`).
It adds no substrate law or allocation.

## Frozen-v1 preservation

- v1 source:
  `2268c4445b438ae8e3d4bd6e1cbdc93d5d217c39b0d8200ac1c0a4d8d7f61e4c`;
- v1 CSV:
  `1222b90fee4ed0ce2a20db7ea751f4c469073cde854b1b8ca0e774f9faf8038a`;
- v1 report:
  `d7e559e4ebe434e236e8af3e08d88843dc6a4761ba879d5984452a78bb658973`.

V2 preflight verifies those hashes and the exact v1 frozen-result tag before
it can enter a CELL.

## Pre-evidence validation

- focused formatting: pass;
- focused compile: pass;
- strict focused Clippy with `-D warnings`: pass;
- v1/v2 physical-block diff: exactly the two preregistered lines;
- organism-visible forbidden-token scan: pass;
- frozen authority/source/negative hashes: exact;
- Git whitespace audit: pass;
- fresh namespaces: `0x9_B110_0000` through `0x9_BA10_0000`;
- authoritative/shared files changed: none;
- broad historical suite: not run because shared code did not change.

After this source and audit are committed and tagged, only refusal cases,
hash/source isolation, artifact absence, compilation, and no-CELL preflight may
run before the single evidence command.
