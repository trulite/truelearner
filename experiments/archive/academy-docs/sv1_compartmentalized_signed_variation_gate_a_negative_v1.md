# SV1 compartmentalized signed variation Gate A negative v1

Status: immutable static negative; runtime selection gates not constructed.

## Result

The accepted ordinary variation operator can create weak signed ARROW
alternatives, but only between CELLs that already exist. Its local proposal
path:

```text
enumerate nearby live CELLs
    ↓
construct ordinary ARROW proposals
    ↓
emit Proposal { arrow, from, to }
```

contains no CELL construction, contact budding, or other ordinary operation
that can produce the required separate contact compartments:

```text
             C+ --(+1)--> X+
            /
P ----------
            \
             C- --(-1)--> X-
```

SV0 established that the signed `+1/-1` ARROW pair can be proposed
symmetrically and boundedly. It also established that alternatives sharing one
contact share participation and mature together. SV1 therefore requires
variation itself to produce distinct ordinary contact CELLs. Preplacing `C+`
and `C-` in an evaluator would supply the attribution resolution under test.

Static Gate A is negative under the frozen stop rule. No selection evaluator,
runtime world, CELL-variation law, sign-specific operation, Rust compilation,
or learning run was created.

## Static evidence

The frozen source audit ran in fresh E2B worker
`ijp33os0mttzgo1s9zrdd` from corrected protocol/audit commit `a16ee47`.

```text
gate_a=negative
variation_adds_arrows=true
variation_adds_cells=false
contact_compartment_creation=false
runtime_gates_constructed=false
SV1_COMPARTMENTALIZED_SIGNED_VARIATION_GATE_A_STATIC_NEGATIVE_V1
```

Frozen hashes:

```text
core lib.rs  b6b7f2a47818d84ac2fd69aab466f5f917e6d3ba7cfc8f8c5db4ce91b97fbae5
core Cargo   4cb6d665d738cdea61f928975fa34ddf89d62aa9150420748d94d574ed731aeb
audit script 424f4e91ad144112b5e1a8f0177d5f2e05cf49373cabcab795aaec98c1441912
audit output 1b90c4bc6e2c88abc47c038437ec89829f36db25cf8443d23319eab1df432ea2
```

The first fresh static-audit attempt stopped before producing evidence because
one source sentinel assumed a single-line rustfmt layout. The sentinel was
corrected and committed before the successful fresh audit. No organism code or
scientific criterion changed.

## Unrun gates

Because Gate A failed, SV1 did not construct or evaluate:

- symmetric creation of separate `C+` and `C-` contacts;
- positive-only or negative-only consequence selection;
- identity, slot, or position permutation;
- neither-useful or both-useful controls;
- the deliberate shared-contact SV0 reproduction;
- bounded nonrecursive contact variation; or
- Reference/Production runtime equivalence, replay, and quiescence.

## Classification

SV1 does not falsify compartmentalized signed selection. It establishes a
prior support failure:

> Current ordinary variation can vary the sign of an ARROW, but cannot vary
> the contact-compartment topology needed to give those alternatives distinct
> local participation.

No new law is selected or authorized. The next independent scientific
question is whether a general, bounded, consequence-agnostic local variation
law can create ordinary contact CELL compartments without knowing which sign,
placement, or candidate will be useful.
