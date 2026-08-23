# PX3-R direct physical trace coupling PROBE v1 implementation audit

Status: **IMPLEMENTATION READY TO FREEZE; EVIDENCE UNSPENT; PX3 ABSENT**.

## Frozen implementation candidate

- isolated manifest: `arms/px3-r-trace-coupling/Cargo.toml`;
- inherited-plus-opportunity law: `arms/px3-r-trace-coupling/src/lib.rs`,
  SHA-256
  `caac29fd6748725e08323b6426f53011ffa16e639165c05c3517519202037845`;
- physical four-route construction: `arms/px3-r-trace-coupling/src/arm.rs`,
  SHA-256
  `544d1585af2efa004f7b9d0248df1be4d1f35ced14ecc0a329101d727e2dc875`;
- PROBE evaluator: `arms/px3-r-trace-coupling/src/bin/probe.rs`,
  SHA-256
  `e0192b6012ff4b77d55aaffede9e8857988db718479fea162ac68d44b19821d1`;
- lockfile SHA-256:
  `dda7b312488e8fc11b23a4d29823715f6bded8804ac4e40017dfbb5c04eb92ac`;
- executable v3 protocol SHA-256:
  `453e62864327d1a022ef346bd573547120c72f344d821cb066126bce1957e089`.

V1/v2/v3 protocol commits and tags remain separate. V3 is frozen at commit
`8adedf881de0c36642fb11fecabd4abfbb7eda96`, tag
`px3-r-direct-trace-coupling-probe-v3-protocol`.

## Unchanged-port and law-diff audit

The authoritative law remains at
`crates/px0-physical-correspondence/src/lib.rs`, SHA-256
`3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.
It was copied byte-for-byte into the isolated arm before changes. A direct
unified diff shows only:

- export of the isolated arm construction;
- three numeric `LocalActivityOpportunity` fields;
- optional opportunity and recent-firing physical state;
- one call after any CELL firing;
- the preregistered symmetric local proposal/traversal routine;
- opportunity/recent-tick fingerprint and storage accounting.

Every inherited pressure, return, coupling, decay, refractory, ordering,
generation, propagation, and generic-proposal line is unchanged. `new()` keeps
the opportunity absent. No authoritative file or workspace manifest changed.

## Physical construction and isolation

The construction uses four copies of the retained PX2 motif. Trace thresholds
are `4`; ordinary consequence and retained-hub arrivals each carry impulse `2`.
Their conjunction fires an actually participating trace, while candidate
impulse `1` or matured impulse `2` alone stays subthreshold. Only trace loci
occupy one radius-eight neighborhood. All other potentially firing loci are
physically separated or fire outside the one-tick overlap.

The evaluator supplies only external SPIKE schedules. It serializes every
route marginal and the full physical inter-trace matrix after propagation. It
does not name or select an endpoint inside the organism-visible library.
Expected combinations occur only in post-run comparisons.

## Forbidden-information audit

Executable organism source contains no exact token for Event, Episode,
History, Pair, Group, member, boundary, semantic, evaluator, serializer,
old-M3, or DS3. Documentation comments are excluded from executable-token
scanning; the inherited comment stating that evaluator types are absent is not
organism state. There is no typed adapter, member list, pair key, relation
serializer, hidden reset, recruited shared CELL, or evaluator-selected local
update.

Source scenarios and expected masks exist only in `src/bin/probe.rs`. The
library has no dependency on that binary and cannot read it.

## Pre-evidence validation and accounting

- isolated formatting: pass;
- focused compile: pass;
- strict focused Clippy: pass;
- no-argument and wrong-argument refusal: exit `2` before any CELL;
- no-CELL `--preflight`: pass with one marker;
- source, lineage, protocol, frozen generic-result, and vocabulary audits:
  pass;
- final/staging result paths: absent;
- isolated source plus manifest/lock storage: `64,213` bytes;
- shared source changes: none;
- broad historical suite and authority matrix: not run.

No candidate cell, duplicate, control, result, or evidence marker has run. The
sole PROBE command remains unspent.
