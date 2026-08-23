# CJ0-E transient-coincidence MICRO v2 accounting-correction protocol

Status: **PREREGISTERED; V2 MICRO UNSPENT; DEVELOPMENT-ONLY**.

## Frozen v1 preservation

This protocol begins at clean, remote-exact commit
`5e596d85abda83737e30e68103b2eac1a1a2686a`. The v1 protocol,
implementation, PROBE, MICRO negative, handoff, and all five annotated tags are
immutable premises. Their bytes and tag targets are recorded below and may not
change.

| frozen v1 artifact | SHA-256 / target |
|---|---|
| development protocol | `b10a7454ae4da2f71e53dcc9662078acc5b7a09c7c506a7d77e5159b33774a8e` |
| implementation audit | `0e67c8140f1e1da6d350aadc2b28a30496b9dcd0eafc809a8500e5863d94e226` |
| frozen-negative handoff | `740a20d85e56f4236c365d613a8db86bcd1b1cea09c7558adecfc38e9024bc0d` |
| physical-law build transform | `de69f4913e7c29379bdd13fddf8b70042d92159e59842ed0a9c2f5f9b21201b9` |
| physical-law crate root | `2da7f40828878bdd408cda2674d297f2c8eb65c63740baa9caa3d57b4f6c568a` |
| v1 evaluator | `df8d5c3cb1bcf5d35b345268deed7f40166b2c7307250723020f5907d699c963` |
| PROBE CSV/report | `d2b1bc6da9f865add6eaf35a8f06adb027c491b0f20557d58c034532ed6b6a6f` / `c1b9f6c833093d4f95ad6e1748b2f59da38f7978b278090d162ccedd0a618617` |
| MICRO v1 CSV/report | `340db47ffc46b9f07d79fc7df1069ffe3c53390eb2466b3801bc030c9753dc04` / `f5f01ebc069c2fdc4292d101a53931f6af556a9c737b853dea5329f336093143` |
| protocol tag target | `c179fd0e40ba32eb9f59e6ab67b3c162c65be09e` |
| implementation tag target | `2596ce2b9c561fd20bb4480e3258d26c0117d6eb` |
| PROBE tag target | `15ae29b042835a4baea43db6ec69bd9da1017d28` |
| MICRO negative tag target | `3f3163bb9d401fd469c87a8bc80bf526918ac369` |
| handoff tag target | `5e596d85abda83737e30e68103b2eac1a1a2686a` |

## Sole correction

The v1 physical candidate law and organism-visible generated source remain
byte-identical. V2 uses the same compiled library and exact generated physical
source SHA-256
`e64c8c915c2fbc4679d1e34ee69ecfe36e2c5ff05bdff5d7feeb5a55578bf1c1`.

V1's repeated-A control compared the substrate-wide
`local_return_updates` counter with zero. That counter correctly included
`33` returns to A's three ordinary fanout ARROWs at the four-tick eligibility
boundary even though:

- learned conjunction traversal was zero;
- learned output firing was zero;
- learned conjunction support weakened `15 -> 10`.

V2 changes only the external measurement scope. Aggregate local returns remain
serialized and visible. The actual self-evidence claim is evaluated from the
specific ordinary physical locus-to-output ARROW that constitutes the learned
conjunction organization:

1. count physical traversals whose numeric endpoints are that locus and its
   output CELL;
2. count firings of that output CELL;
3. record its generation, live state, coupling, and resistance before and
   after every repeated-A occurrence;
4. count any occurrence on which its resistance increases;
5. require zero learned traversals, zero learned output firings, zero learned
   support increases, unchanged generation/coupling, and final resistance less
   than initial resistance through ordinary pressure.

No contributor-route return is hidden, subtracted, relabeled, or prevented.
The aggregate count must remain nonzero in the fresh self-evidence cell and is
serialized independently beside learned-specific evidence.

## Fresh v2 matrix

V2 uses no v1 namespace or result path:

- reversal primary: `0xd200_0000`;
- reversal exact duplicate: `0xd200_0000` from byte-identical blank matter;
- reversal mirror/allocation/arrival permutation: `0xd210_0000`;
- full-deallocation bootstrap: `0xd220_0000`;
- repeated-A self-evidence: `0xd230_0000`.

The v1 schedules remain fixed: initial A+B/C+D acquisition, mandatory
A+D/C+B reversal, `12` initial rounds, `40` reversal rounds, spacing `20`,
within-round gap `4`, and `12` repeated-A occurrences at gap `4`.

V2 must independently pass:

- old organization physically deallocates;
- changed organization forms and executes;
- old held-out use and one-use historical replay remain silent;
- individual route marginals remain equal;
- mirror/permutation and exact duplicate replay hold;
- full-deallocation bootstrap forms and executes without mature output first;
- repeated A alone satisfies every learned-specific self-evidence clause above;
- aggregate contributor-route returns remain recorded and nonzero;
- every finite propagation is naturally quiescent.

Every row serializes route/locus/output activity, learned traversal and support
evidence, aggregate returns, topology, coupling/resistance/live/generation,
complete/permanent fingerprints, storage, work, and quiescence.

## Execution discipline

The separately named v2 binary provides only `--preflight` and `--micro`.
Missing/unknown arguments refuse with exit `2`; preflight enters no CELL.
Existing result or staging paths refuse. Atomic create-new staging files are
synced and renamed once.

Before execution: exact v1 hash/tag audit, generated-law hash audit, formatting,
focused tests, strict Clippy, argument refusal, preflight, fresh-identity audit,
forbidden-representation scan of the generated physical source, and artifact
absence.

A v2 failure is frozen and stops development. A v2 pass permits a separately
preregistered development GATE with fresh identities. Development ends at
GATE; no later-stage surface may be created or entered.
