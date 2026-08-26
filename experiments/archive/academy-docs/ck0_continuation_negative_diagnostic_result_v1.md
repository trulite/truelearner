# CK0 continuation negative diagnostic result v1

Status: complete positive classification.

Protocol: `49ccd611a6fe9c03a5c34ee1e37ca605f86f1e56`.
Frozen evaluator: `f5c0e54fe84aec3d0a23d90d843d8f376a54e1e3`.
Fresh E2B worker: `iizlqm7xoigolcjnqtrg8`.

## Result

- cases: `8/8`;
- classification: `evaluator_observer_defect`;
- observer-enabled physical continuation exact: true;
- observer-enabled ordered traces exact: true;
- PhysicalWork exact: true;
- legacy `Work::total()` exact: true;
- tick exact: true;
- durable body exact: true;
- raw checkpoint hash exact within each uninterrupted/restored pair: true;
- quiescence exact: true.

Every default-restored continuation omitted its trace because checkpoint
restore correctly does not persist or enable the causally inert
`trace_physics` observer flag. Explicitly enabling that observer after restore
made the ordered trace exact without changing any physical state or work.

Therefore all eight CK0 v1 composite continuation failures were caused solely
by the evaluator forgetting to re-enable physical tracing on the restored
body.

The diagnostic also disproves the provisional suspicion that legacy
`Work::total()` caused these eight failures: both legacy total and every
PhysicalWork component were exact in all cases.

The other 24 CK0 v1 failures remain the already-serialized cross-mechanics raw
checkpoint-hash comparison. Their reported physical fields, direct predicates,
body, work, tick, and quiescence were exact; raw checkpoint bytes were not a
preregistered physics predicate.

## Evidence hashes

- matrix:
  `ff68dab02dd6b50ae5c220684dafc6357fb40adbcdd0722c3cda5233867a5417`;
- report:
  `32678436b1977e2ca1ada80a1eeae5f47da5b67e7dd99e80da47f91831170cca`;
- runtime remained:
  `078cf11b3082cade5640b42abfcf52496faf3b36e0c0af10abefa7a9d75992de`.

## Authorized successor

A fresh CK0 v2 may change only the evaluator measurement boundary:

1. enable physical tracing on restored bodies before comparing traces;
2. retain raw checkpoint hashes as diagnostic columns but exclude them from
   Reference/Production physics equality;
3. compare PhysicalWork as the causal work contract.

The runtime, worlds, inputs, direct predicates, roots, mechanics, and
checkpoint implementation must remain unchanged.
