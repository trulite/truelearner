# PX7 LR-C cumulative arrival authority implementation audit v1

Status: **IMPLEMENTATION CANDIDATE; TARGETED E2B VALIDATION PENDING; EVIDENCE UNSPENT**.

## Serial boundary

- exact PX6 authority parent:
  `1ca6df74eacbb743f23ca5b5810919985036cd64` /
  `px6-lrc-consequence-authority-v1`;
- authority protocol:
  `ea53d4a869b2f556a17d5e9f00f9169a51c13c4c` /
  `px7-lrc-arrival-authority-protocol-v1`;
- isolated active PX7 source hash:
  `d248a8af479872d8148115a405ae7332f7d24ca229378d3fde898ffd3d19e63e`.

No isolated development commit was cherry-picked. Development results,
handoffs, PX-C outputs, commands, roots, workspace edit, and publication logic
are absent. The active source is byte-identical; the authority evaluator uses
fresh roots, cumulative controls, resource accounting, an execution firewall,
one marker, and atomic create-new publication.

## Frozen implementation hashes

| artifact | SHA-256 |
|---|---|
| authority evaluator | `EVALUATOR_HASH_TBD` |
| authority Cargo manifest | `3671a215a8075596b51dad24c6793f3314b9cdfbba16fa130e12a39d9a9902d1` |
| authority protocol | `827a220f12ba2c6713becb4d9f87bd1a21b0d756efbe5c9a8f88cd6dded51c8a` |
| active PX7 source | `d248a8af479872d8148115a405ae7332f7d24ca229378d3fde898ffd3d19e63e` |
| coverage audit | `f371bf1d8c05614f16c02dcdce1d0919a1d9c815182c2a813cc3ffced12424b7` |
| static audit | `STATIC_AUDIT_HASH_TBD` |

## Implementation boundary

The active PX7 crate adds topology and one position-addressed participation
surface over the unchanged LR-C law. The evaluator separately exercises the
authoritative PX4 API and observes the same local proposal, Drive traversal,
eligibility, qualified Modulatory update, pressure retention, and strict local
allocation used by PX5/PX6.

There is no request/start/invocation/query/session object, correctness/reward
condition, route owner, level selector, new mode, stored field, transition,
eligibility rule, plastic update, pressure rule, hidden state, or memory leak.

## Targeted validation

Pending one fresh E2B package-only validation. No Rust, project program, or
project audit has run locally. No authority body, result artifact, or evidence
marker exists.
