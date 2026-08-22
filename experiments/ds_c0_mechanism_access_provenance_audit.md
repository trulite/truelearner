# DS-C0 mechanism and access-provenance audit

## Frozen fingerprints

| Object | SHA-256 |
|---|---|
| Frozen R0 mechanism | `f17afa482bf345eb680463f7418b6b6c2553cd78eab9b4dbfce74f7ca1483d51` |
| Frozen stage-8 retry | `36c33cb3595001416b4763c29cdba88b5c9567caadc61d8d002177e972ffacce` |
| Frozen stage-8 handoff | `729dd43af12ac5ef35d07f2ddba0609f807344d1e40c4804cf29d478cdd405e6` |
| DS-C0 mechanism | `5c8d00189593ca2f7efb47165efddf85111259f90433a016e5822b5b9578aed2` |
| DS-C0 runner | `3b18fb7ce0a1878f3b6cef6429ef869a02ac65d30b398ee47d08a6ec449e3602` |
| Build fingerprint/access wiring | `fb6ba6906b64b1ca6cc55d070c75569d84a2648527f9ea8ec20cacf3dea867b1` |
| Results tree | `491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4` |

## Read-only access boundary

The build script copies the byte-identical frozen R0 source into Cargo's
generated output. One access macro is appended after the frozen source. It
invokes the existing private E0/A1/R0 path and copies only:

- actual execution pulses: occurrence and local tick;
- actual physical propagation endpoints;
- actual R0 evidence members, relative lag, and hop count;
- evaluator-only accounting/control fields.

The C0 `Workspace` receives only `Activity` and `Evidence`. It cannot receive
the evaluator effect, opaque choice index, expected route, seed, or work
ledger. The evaluator effect remains in the outer report solely to verify that
opaque-handle permutation executed a different real route.

## Organism mechanism

The organism-visible mechanism consists of:

```text
EligibilityCell {
    temporary root occurrence
    local creation tick
    local expiry tick
}

CouplingArrow {
    eligible root occurrence
    returned later occurrence
    relative lag
    physical hop count
}
```

Both structures are episode-local and erased at cleanup. There is no persistent
C0 learner or store.

## Derived zero-path audit

```text
exact R0 accessors                         1
DS1 apply/update call edges                0
semantic direction call edges             0
evaluator-effect edges into Workspace      0
persistent identity fields                 0
```

The update and evaluator-to-workspace zeros are mutation-sensitive. Injecting
either forbidden path changes its derived count and fails the source audit.
