# CORE1-E25 — Motor Integration Solve Tournament Implementation Audit v1

## Freeze

- frozen E24 result: `b790725`;
- E25 protocol: `4dd2bb7` plus pre-runtime autonomy clarification
  `b4e4fb6`;
- E25 implementation: `958bf19`;
- runtime evidence: not yet spent at the time of this audit.

## Candidate surface

The implementation adds two independent, default-off cell-local properties:

- signed gating defers participation of subdivision-contact Drive routes until
  local target admission. Unequal signed material wins by local magnitude.
  An exact tie requires already-present positive subthreshold activation and
  resolves toward the cell's positive threshold surface. Suppressed routes do
  not receive participation or E22 return topology;
- the integration window prevents ordinary activation decay on marked cells
  only while the current propagation wave is open. Synchronous algebraic
  incidence and the existing threshold transition are unchanged. Any
  unresolved held activation clears when the pending physical wave becomes
  quiescent.

Academy exposes two enable methods that mark its already-existing motor cells.
The E25 evaluator enables E24 atomic route closure in every arm and then G, W,
or both. It does not supply a route, sign, context choice, consequence choice,
or action outside the frozen E14 fixture.

Both new properties initialize `false` for every cell, remain `false` after
body/checkpoint reconstruction, and are copied only as ordinary runtime
configuration when an experimental organism is cloned. No frozen evaluator
was edited.

## Static rejection audit

- no ARC action, context, usefulness, reward, consequence, or evaluator order
  is visible to either core transition;
- G uses only live topology, local target marking, signed material strength,
  current target activation, and the target's already-positive threshold
  direction;
- W stores no payload and cannot cross natural quiescence;
- G acts before participation/E22 capture, so rejected routes cannot receive
  delayed credit;
- W does not rectify, reorder, split, or amplify incidence;
- neither law changes formation, delays, material values, thresholds, PQLC,
  E22 return construction, consequence timing, or the action map;
- E14, E16, E22, and E24 evaluator files are byte-identical to `b790725`.

## Focused controls

The three new core controls pass:

```text
G only      no crossing; no gated participation; no return
W only      no crossing; both signed routes participate
G+W         crossing; positive participates; negative suppressed; one return
strong -    stronger negative wins; positive suppressed
W stagger   two staggered +1 incidences reach threshold
W cancel    simultaneous +1/-1 remains zero
W close     unresolved activation is zero at quiescence
```

E18 in-flight protection and the Academy blocked-return control also pass.
Live-checkpoint exactness tests affected by the new default-off vectors pass
after reconstruction was made shape-exact.

The broad Academy `four_context_pressure_regimen...` test remains negative
with `[none,none,none,none]`. The identical test was run at detached frozen
baseline `b790725` and failed identically, establishing that it is the known
E14 frontier rather than an E25 default-off regression.

Strict release Clippy passes for the E25 binary with dependencies excluded;
its dependency tree still contains pre-existing broad core Clippy warnings in
untouched functions. Release construction and non-executing `--check` pass.

## Evidence program

The evaluator cites the already-spent E24 baseline and runs eight roots for
each new arm under Reference, exact Reference replay, and Production. Only an
arm reaching the intended first motor crossing at `8/8` advances to the frozen
five-turn consequence/PQLC regimen and autonomous probes. A Gate-1-negative
arm is not allowed to manufacture downstream evidence.

## SHA-256

- protocol:
  `c569ed5a03656eab02106d0a3e5797ad5e1d5df56f723bc4835e3da7a124243a`;
- core candidate:
  `5cd09e35a3db31100798cac50f7164df7eb40461117b613bfaca6c45df89aee6`;
- Academy enable surface:
  `281e1f699883e5bdac751911555e3c9e19ed33b91733a008e59afcf5c2f06589`;
- evaluator:
  `401793d16c49449da4af3bd73740b5eed6bf9b9ddf19d7e792514cf47e3b5cf0`.
