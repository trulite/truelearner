# PX7 LR-C cumulative arrival authority implementation audit v1

Status: **FROZEN; TARGETED E2B VALIDATION PASSED; EVIDENCE UNSPENT**.

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
| authority evaluator | `3f94662ea73d663e0fb6eaabd0ea381037bdf2e5a622a47bd1c9f8b872c7fe66` |
| authority Cargo manifest | `3671a215a8075596b51dad24c6793f3314b9cdfbba16fa130e12a39d9a9902d1` |
| authority protocol | `827a220f12ba2c6713becb4d9f87bd1a21b0d756efbe5c9a8f88cd6dded51c8a` |
| active PX7 source | `d248a8af479872d8148115a405ae7332f7d24ca229378d3fde898ffd3d19e63e` |
| coverage audit | `f371bf1d8c05614f16c02dcdce1d0919a1d9c815182c2a813cc3ffced12424b7` |
| static audit | `2a972c5849647ec1e876be549d2ecde1bb666918063913b61d9a8d6b83789f89` |

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

Formatting-only E2B sandbox `iu8zf96pw7n97n5wcyfs2` canonicalized the
authority package and computed frozen hashes without compiling or constructing
a body.

Exact clean snapshot `80f6d5c2562594e574a2f0120220f505e0252a4b`
then passed the sole targeted package-only validation in fresh sandbox
`iw75mdqvewnd2bnszua5c`:

```text
package rustfmt check                               PASS
strict release package Clippy -D warnings           PASS
fully qualified no-world matrix test             1/1 PASS
static hash/dependency/coverage/firewall audit      PASS
release --authority-preflight                       PASS
```

Cargo compiled the package dependency closure once and reused it for the test
and preflight. No workspace-wide build, unrelated suite, repeated validation,
body construction, row replay, evidence marker, or result publication
occurred. Static audit reported `active_sources=3`, `new_active_px7=1`,
`evaluator_sources=1`, and `unclassified=0`.

No Rust, project program, or project audit ran locally. The sole definitive
command may now execute exactly once from the unchanged source plus this
audit-only freeze commit in a new E2B sandbox.
