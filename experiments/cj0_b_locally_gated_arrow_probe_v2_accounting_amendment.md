# CJ0 ARM CJ-B PROBE v2 candidate-consumption accounting amendment

Status: **PREREGISTERED MECHANICAL AMENDMENT; EVIDENCE UNSPENT**.

The frozen v1 implementation at commit `d858f9b` has not executed a PROBE
cell and has written no result or staging artifact. Its physical module is
unchanged and remains the exact candidate law.

## Unique defect

The evaluator's `Observation.consumed` field copies the global
`local_state_consumptions` work counter. That counter correctly includes
every successful strong ordinary transmission, including the observation
driver's coupling-3 transmission into a source CELL with prior state zero.

The PROBE protocol's self-evidence clause instead preregisters **candidate
destination-state consumption**: a transmission from either source CELL into
one of its weak local destination sites that actually consumes positive
current state. Using the global work counter would report one consumption for
source-only observation even though both weak candidate transmissions are
suppressed and consume nothing.

## Sole authorized correction

Change only evaluator `observe` serialization so `Observation.consumed`
counts emitted source-to-site transmission records whose serialized
`destination_state` is positive. Do not change the physical module, schedule,
topology, thresholds, coupling, work ledger, clauses, controls, output paths,
or evidence command.

The global physical consumption counter remains serialized through work. The
candidate-only field becomes the exact independent-stage projection specified
by the v1 protocol. Any further defect requires another freeze; no scientific
result may be tuned or rescued.
