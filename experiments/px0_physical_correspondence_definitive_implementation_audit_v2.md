# PX0 physical correspondence definitive implementation audit v2

Status: **PRE-EVIDENCE ARCHIVE-PORTABILITY CORRECTION; DEFINITIVE EVIDENCE UNSPENT; PX0 AUTHORITY ABSENT**.

The v1 implementation remains immutable at tag
`px0-physical-correspondence-definitive-implementation-v1`.

## Frozen no-cell refusal

Fresh E2B authority sandbox `int6ggl9c4s9fr7vyqh1e` received the exact v1
Git archive and ran formatting, release compilation, and strict Clippy. Its
no-cell preflight returned false and stopped the command chain before the
matrix because a Git archive intentionally contains no `.git` directory, so
`git rev-parse px0-r-generic-physical-reproposal-development-readiness^{}`
could not succeed remotely.

The runner emitted no `PX0_DEFINITIVE_EVIDENCE_SPENT` marker, invoked no cell,
and created no final or staging result. The definitive outcome remains
unspent.

## Sole correction

The v2 runner keeps the exact Git tag check whenever Git metadata is present.
When it is absent, it accepts the already-required exact readiness commit
constant together with the exact frozen PX0-R readiness bytes. The local
pre-upload audit still resolves the actual tag to
`745a5c3dc6d929faa2908359c5eb0462e8eac663`; the E2B helper archives the clean
tag-descended implementation commit.

No matrix constant, namespace, schedule, fixture, control, claim, active-law
byte, result schema, marker placement, or write-once operation changed.

## v2 executable hash

| artifact | SHA-256 |
|---|---|
| authority-only runner v2 | `6f1ee16a57ad60f8806914e868b865ebe252a7540a5a77f69c61cad9e1332dfe` |
| frozen PX0/PX0-R active law | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| definitive protocol | `a5c918cb15868506a333b06a6f9c70f7cf23f09707a08385d03ab164513d0739` |

Local formatting, strict Clippy, no-cell preflight, refusal exit `2`, exact
active-law hash, and absent final/staging paths pass after the correction.
The fresh E2B sandbox will receive this separately frozen snapshot and repeat
all no-cell validation before the sole authority execution.
