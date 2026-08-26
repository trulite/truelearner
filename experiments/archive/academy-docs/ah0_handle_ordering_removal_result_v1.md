# AH0 handle-ordering removal result v1

Status: development positive; non-authoritative.

Frozen candidate: `ah0-handle-ordering-removal-frozen-v5` (`abbd68f`).

## Result

Numeric `CellId` and `ArrowId` values no longer decide active runtime
transitions. Handles remain usable for equality, hashing, generation-safe
lookup, serialization, and deliberate resident packing.

The static audit passed:

- scheduler keys contain no CELL/ARROW handle or physical identity;
- active and required membership is hash-based;
- firing, Modulation, QLP, adjacency, and proposals contain no handle sort;
- eight surviving handle-order sites are confined to checkpoint, durable
  packing, resident compaction, or normalization;
- one active runtime source file: `truelearner/crates/core/src/lib.rs`;
- former `mechanics.rs`: deleted;
- runtime dependencies added: zero.

Canonical runtime SHA-256:
`9ec9f4fb5ae9c66b8353ef17307715782f0fee2b044928809cc7f24a9fd041db`.

## Retained evidence

- R1-R5 targeted differential: pass;
- R6 partition invariance successor: `38/38`, first legal quiescent checkpoint
  tick `320`;
- SI0 v2: `120/120`, no divergence;
- CPC0 current-parent differential: `440/440` rows, every normalized physical
  observation exact; 80 raw trace hashes differed only by inert recording
  order;
- CPC1: `620/620` physical cases;
- PQLC0: `200/200`;
- PQLC1: `780/780`;
- FD0: `100/100`;
- FD1 v3: `140/140`;
- J0: `160/160` cases and `1880/1880` clauses;
- CV0/J0 + SV1: `240/240` cases and `5480/5480` clauses.

Every retained evaluator reports exact Reference/Production equality, exact
same-mechanics replay, and natural quiescence under its frozen contract.

The generated manifest reverified every evidence file. Manifest SHA-256:
`1f50c7cab0b8041b1a1b80067398f73443e4d6747f8d8569faa4a4ef0b7b8cfe`.

## Compatibility findings

The original R6 checkpoint fixture was already invalid on exact parent
`3f889bc`: continuous participation makes tick 10 non-quiescent. The successor
waits for the first public-contract quiescent state and changes no runtime law.

The historical CPC0 evaluator is also not a cumulative oracle for the current
body: it observes deleted eligibility and asserts unresolved historical
coupling/timing outcomes. AH0 therefore compares the complete current-parent
history. All physical content is identical; only ordering of same-time/phase
observer records changes. J0 and CV0/SV1 independently retain the current
contact-local attribution claim.

No RS2, CE1, FD2, ARC, authority, oracle, or `arch.md` state changed.
