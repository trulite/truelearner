# PD1 local pressure-participation implementation audit v1

Status: evidence-eligible freeze. No PD1 matrix has run.

Protocol parent: `46975fe92ecdcf444aabb44fb770cea6b6873b82`, tagged
`pd1-local-pressure-participation-protocol-v1`.

## Frozen implementation

The candidate adds feature `pd1`, which inherits the accepted PQLC0/CPC1
substrate. It adds one transient ARROW-local quantity, `pressure_load`, to both
AoS and SoA mechanical representations.

At each ordinary physical pressure epoch:

1. existing CPC1 relaxation advances to that epoch;
2. remaining participation absorbs up to one existing participation impulse;
3. residual pressure enters local pressure load;
4. complete load quanta reduce durable resistance;
5. sub-quantum load remains as future-causal local state.

The implementation iterates physical pressure epochs even across a large host
time jump. The candidate pressure function contains no read of
`eligible_until`, `LOCAL_WINDOW`, or unsupported-use pressure and no Boolean
participation protection test.

PD1 Modulation records the existing local support and maps its remaining
participation magnitude to a bounded one-to-three integer resistance gain.
Coupling and PQLC continuation are unchanged. Traversal alone cannot add
durable resistance.

The old eligibility field remains present and may expire as inert bookkeeping;
its removal is not part of this gate.

## Frozen hashes

```text
core Cargo.toml   c919b87fb2628f23e019a59ec59eab3fefb7faffa3a48fa03e6e9ea4d1ebbb4c
core lib.rs       0cab3c5d2a630694a161445a26fb432a794be9fca3126cfe5242a269ebc5d22c
core mechanics.rs 7521549b1e348be07e3b2ee943f6d2cf763201cd54de6d8a576ac6592d6e6bb8
evaluator main.rs b3b3439e088bc96b6831e9c49a5e38b0a25955562e0ec128f12f3dbf39fee7eb
evaluator Cargo    dec9d4c1fdecbfff9b8c4f258e186a26df33002fe9a9ca2626f4d06632de7b7a
protocol           1beee6ee40e071a95d2da5982e744968dd98e7cb239497f9343d7dbccc127783
```

## Pre-evidence validation

In reusable E2B worker `it48kiw54nkrebe6zl821`:

- targeted formatting completed;
- release check passed;
- strict release Clippy with `-D warnings` passed;
- the evidence evaluator was not executed.

No Rust or project program ran locally. No ARC, PD2, authority, oracle, or
`arch.md` work occurred.
