# TC-DS1 trace-order negative diagnostic result v1

Status: immutable diagnostic negative. The classification is a
mechanics/candidate trace counterexample under the frozen protocol.

## Execution

The frozen diagnostic at commit
`901f9d3758b8c163ff7abb111585a4a42e771c83` executed once in fresh E2B sandbox
`iurje3quswglzgz4mqjum`.

It reproduced only the identified Gate B geometry:

```text
root              1100000
pressure phase          0
return delay             0
```

Reference and Production agreed on:

- A and B identities and participation magnitudes;
- A and B contact magnitudes;
- one plastic update to A and one to B;
- causal physical work;
- durable body hash;
- physical tick and pressure phase;
- natural quiescence.

The diagnostic then failed its preregistered complete-transition-multiset
equality assertion:

```text
Reference multiset hash
e2e87acea23c9e483c63e7d6ed05d754acd5b96009c7e294d7cac3886aee3a3e

Production multiset hash
3b7ee2ec000a7dab6d70b6ea0ec769bfa4cac91b03fba920ed27405812c66cc2
```

Because the assertion preceded artifact publication, no diagnostic CSV,
report, or checksum file exists. The diagnostic was not rerun.

## Classification

The frozen protocol required a stop when the complete physical-transition
multisets differ. Therefore the v1 matrix difference cannot be repaired as a
mere ordering comparator defect in this workflow.

This does not show a different final durable body. It shows that the
feature-gated participation/contact trace candidate is not yet
representation-independent across the accepted Reference and Production
mechanics. That is sufficient to reject TC-DS1 v1 as implemented.

The first Gate B case also independently contacted and credited both recently
traversed outgoing paths. Thus the existing modulation site remains
source-local and cannot attribute one downstream consequence to only A.

No rescue, TC-DS2 implementation, ARC run, pressure change, authority update,
oracle update, or `arch.md` change is permitted or performed.

