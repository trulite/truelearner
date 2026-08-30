# TrueLearner production workspace

The production workspace has one organism and one physical-world adapter:

```text
truelearner-workstation -> truelearner-body
```

- `truelearner-body` owns junctions, arrivals, paths, choice, effects, outcome
  return, and link memory.
- `truelearner-workstation` owns the `WorkstationHarness`, physical sensor and
  motor attachment, private junction handles, external state, and opaque
  checkpoint replay.
- `truelearner-behavior-contract` holds the shared black-box scenario format.

The former core, embodiment, harness, and checkpoint crates are not production
dependencies. Research archives may refer to them as historical evidence.

Production crates depend only on crates in the production workspace.
