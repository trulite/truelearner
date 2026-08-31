# Academy formal evidence

`academy-formal` invokes the pinned Lean closure checker on an already-frozen
causal trace. Its projector consumes only the public body-trace events returned
by `WorkstationHarness`; it never owns a live harness or body and cannot affect
organism input, output, state, physical time, or action selection.

For a closed return, the projector requires an intact naturally quiet trace,
valid recorded choices, the exact output transition, an accepted return naming
the resolved physical path, and strengthening of both path links. It refuses to
invent a relation when any one of those arrows is absent.

For an ambiguous return, the projector requires every live competing path,
each path's actual output transition, and the absence of strengthening on all
of them. It then asks Lean to verify that several explicit causal explanations
make the return ambiguous and persist no links. Explicit event ancestry, not a
timestamp guess, orders an output and return that occur in the same body tick.

Build the checker first:

```sh
cd formal/closure
lake build
```

Run the Rust-to-Lean integration checks from the repository root:

```sh
TRUELEARNER_LEAN_CHECKER="$PWD/formal/closure/.lake/build/bin/truelearner-closure-check" \
  cargo test --manifest-path academy/Cargo.toml -p academy-formal
```

An accepted receipt means only that the submitted claim follows from the
submitted causal graph under the proved closure model. Academy remains
responsible for showing that the graph faithfully records a real experiment.
