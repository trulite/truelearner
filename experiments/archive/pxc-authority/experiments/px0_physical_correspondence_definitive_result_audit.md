# PX0 physical correspondence definitive result audit

Outcome: **DEFINITIVE NEGATIVE; PX0 NON-AUTHORITATIVE; PX1 BLOCKED**.

## Sole evidence spend

The definitive command executed exactly once from frozen implementation commit
`98309fcbd067f22018b02e64bef19cddc64b2a81`, tag
`px0-physical-correspondence-definitive-implementation-v2`, in fresh E2B
authority sandbox `int6ggl9c4s9fr7vyqh1e`.

The runner emitted exactly one `PX0_DEFINITIVE_EVIDENCE_SPENT` marker, completed
all 16 cells, atomically published both write-once artifacts, and exited `1`.
No rerun, rescue, tuning, matrix amendment, or result regeneration occurred.

The earlier v1 sandbox validation stopped before the marker and before cell
zero because Git archive snapshots lack `.git` metadata. That no-cell refusal
is preserved in the v2 implementation audit and is not an evidence spend.

## Conjunctive outcome

- cells: `14/16` passed;
- controls: `190/192` passed;
- generic local proposals: `997`;
- physical deallocations: `885`;
- ledgered work: `231,216`;
- every preflight/hash/isolation control: passed;
- PX0 authority: absent;
- PX1 development eligibility: absent.

All 16 cells passed:

- initial anonymous physical acquisition;
- exact held-out outward execution;
- bounded-absence same-identity reuse;
- resistance-zero full deallocation of original arrows;
- no time-only proposal or resurrection;
- fresh-identity generic reproposal;
- exact B reacquisition and outward execution under opposite contemporary
  return;
- absent-return disappearance;
- ambiguous-return non-privilege;
- duplicate-exact replay;
- quiescence and complete physical accounting.

## Preregistered failure

Cells 0 and 10 failed only `P7`:

| cell | A→B | spacing | active opportunities | allocation | layout | B held-out | later A crossing |
|---:|---|---:|---:|---|---|---:|---:|
| 0 | 0→1 | 8 | 2 | forward | direct | 1 | 1 |
| 10 | 1→2 | 10 | 3 | forward | mirror | 1 | 1 |

In both cells:

- the two original A arrows had already reached resistance zero;
- the original A identities remained non-live;
- B was reacquired through two fresh live arrow identities;
- B executed exactly once;
- subsequent A activity nevertheless produced one outward crossing.

The result is therefore not historical resurrection. Renewed experience made
the same broad generic opportunity law propose fresh structure for all active
local alternatives. In these two schedule/layout combinations, fresh
unsupported A structure remained physically executable at the immediate
historical-route probe instead of having disappeared under pressure. The
preregistered control required silence, so the matrix is definitively
negative.

This audit does not reinterpret that crossing as acceptable transient
variation, weaken `P7`, or promote the 14 positive cells. Any future question
about transient alternative lifetime requires a separately named program and
fresh evidence; this matrix cannot be rescued.

## Immutable artifacts

- [definitive CSV](../results/px0_physical_correspondence_definitive.csv),
  SHA-256
  `da356bc46a9d83d0cd749bcaa697cba66393b7d694de500e2208565806d680d1`;
- [definitive report](../results/px0_physical_correspondence_definitive.md),
  SHA-256
  `7e2c06d63332a680d46031c49d5dc245c6a4f381d7c646a2b0474580469a09b7`;
- result commit `244ac03cd75dc634f2f1d7216173b5bbcfad5052`, tag
  `px0-physical-correspondence-definitive-v1-negative`.

Executed source remained exact at active-law SHA-256
`3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`
and authority-runner SHA-256
`6f1ee16a57ad60f8806914e868b865ebe252a7540a5a77f69c61cad9e1332dfe`.
The result commit adds only the two downloaded write-once artifacts.
