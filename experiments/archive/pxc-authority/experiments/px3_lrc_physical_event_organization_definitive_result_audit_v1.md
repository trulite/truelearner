# PX3 LR-C physical event organization definitive result audit v1

Status: **DEFINITIVE NEGATIVE; PX3 AUTHORITY ABSENT**.

Frozen implementation commit: `e42da31ceddac96ffa52dfb61f39257d5d32389a`,
tag `px3-lrc-physical-event-definitive-v1`.

## Immutable evidence

| artifact | SHA-256 |
|---|---|
| implementation audit | `ef87c4a663fa2ee8b510e366e913aab3bb72b2148448c1f79d905aa1f96baaae` |
| lifecycle CSV | `bd74bd95855b272d8cc79d7448a72e01f7ace6e73fb3cc12319422592149e3ea` |
| lifecycle report | `414fbf8773c8644b92bcf162a0ce700a9021968b4d0e1b5cba897e1a2be1e240` |
| recursion CSV | `8706ad8a08fde334719f6e1e14c9af755d8faf5fa6629721728f25cab80490cb` |
| recursion report | `daa6864b702911ec57ea616cfd3728ed545739440b7e0c4a8cfd4034e55a766e` |

The two frozen matrices executed once and concurrently in distinct fresh E2B
sandboxes. Each emitted exactly one evidence-spent marker and atomically
published its artifacts. No rerun or rescue occurred.

## Formal verdict

```text
lifecycle rows       0/16
lifecycle clauses  176/192
recursion rows       16/16
recursion clauses  192/192
joint rows           16/32
joint clauses       368/384
```

The preregistered verdict is conjunctive. PX3 therefore remains
non-authoritative and PX4 is not authorized.

## What passed

Every lifecycle row passed eleven of twelve clauses, including all
counterexamples that motivated LR-C:

- coupling-four A, repeated A and gapped A/B formed no joint organization;
- two adjacent unsupported AB episodes produced two real candidate traversals,
  zero plasticity updates and a dead candidate;
- one supported exposure died;
- recurrent completed AB/CD loops matured;
- obsolete AB/CD deallocated under ordinary pressure;
- fresh AD/BC identities formed and controlled held-out execution;
- replay was exact and every propagation quiesced.

Every recursion row passed all twelve clauses. AB, XC and YD matured through
the same physical API; one exposure died; single-sided and gapped controls
failed to form a higher stage; final context-free reuse emitted one X, Y and Z
trace; and execution-only reuse generated zero relay, Modulatory or plasticity
traffic.

## Sole failure: L5 accounting overreach

All sixteen lifecycle rows serialized the identical L5 physical observation:

```text
independent effect path fired        1
world-return relay fired             1
modulatory transmitter fired         1
Modulatory crossing to P             1
P firing                              0
plasticity updates                    0
joint P->effect candidate count       0
quiescent                          true
global structural proposals          1
```

The causal discriminator itself passed: real modulation reached P without
prior candidate participation, P did not execute, no joint candidate existed
and nothing learned.

The harness nevertheless made L5 require `local_structural_proposals == 0`.
That counter is global. The authority world externally drove the effect CELL
to instantiate an independent downstream occurrence. Under the frozen native
law, every external arrival that fires a CELL calls `propose_local_arrows()`.
Because the effect and P are distance-one neighbors, the effect firing lawfully
proposed one weak **effect-to-P** Drive ARROW. The serialized joint
**P-to-effect** candidate count remained zero.

Thus L5 conflated “no joint candidate was proposed” with “no structural
proposal occurred anywhere.” The protocol text required the former; the
evaluator enforced the stronger global condition.

This is an accounting-only false negative for the intended modulation-without-
participation discriminator, not evidence that modulation credited or fired P.
But definitive discipline does not permit rewriting the generated verdict or
promoting the scientific interpretation to authority.

## Boundary and next permissible action

The v1 artifacts and negative verdict are permanent. A successor authority
workflow, if explicitly opened, must use new seeds/namespaces and may change
only the L5 measurement:

- measure the existence/traversal of the joint `P -> effect` candidate
  directly; and
- serialize any independent `effect -> P` native proposal separately.

It may not change LR-C physics, PX3 topology, timing, thresholds, resistance,
pressure, any other clause or any frozen v1 artifact.
