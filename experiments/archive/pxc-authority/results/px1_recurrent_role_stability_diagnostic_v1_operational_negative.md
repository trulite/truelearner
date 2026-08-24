# PX1 recurrent role-stability diagnostic v1 operational negative

Status: **FROZEN OPERATIONAL FAILURE; SCIENTIFIC CLASSIFICATION ABSENT**.

The frozen v1 parent command was invoked once after all implementation and
freshness checks passed:

```text
cargo run --release -p px0-physical-correspondence \
  --example px1_recurrent_stability_diagnostic -- --diagnostic
```

The parent emitted the development-evidence marker and started the four frozen
arm child processes. It then panicked while decoding the first completed child
result:

```text
primary role
```

Static inspection established the mechanical cause. `ArmResult::encode`
serialized boolean measurements through a `u64` closure as `0` or `1`, while
`ArmResult::decode` accepted only `true` or `false`. The failure occurred in
the evaluator-side result transport after arm execution, before a complete
four-arm matrix could be recovered or written.

- parent exit: `101`;
- CSV artifact: absent;
- Markdown classification artifact: absent;
- surviving child process after parent exit: none;
- PX0 law modification: none;
- arm physics modification: none;
- scientific arm classification: **not available**.

No result is inferred from process completion order or the panic location. The
v1 attempt is permanently spent and will not be rerun or reconstructed. Any
retry must be separately named and may change only the demonstrated transport
defect unless separately preregistered.

