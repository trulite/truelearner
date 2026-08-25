# TC-DS0 checkpoint-negative diagnostic v1

- first differing byte including checksum: `66`
- first differing payload byte: `762`
- decoded differing fields: `7`
- header reference: `DecodedCheckpoint { tick: 15, next_serial: 4, manifest_len: 103, body_len: 549, cell_count: 7, arrow_count: 2, pending_count: 0, load_count: 0, cells: [CellRuntime { id: 0, state: 0, last_update_tick: 15, refractory_until: 1 }, CellRuntime { id: 1, state: 0, last_update_tick: 15, refractory_until: 1 }, CellRuntime { id: 2, state: 0, last_update_tick: 15, refractory_until: 0 }, CellRuntime { id: 3, state: 0, last_update_tick: 15, refractory_until: 0 }, CellRuntime { id: 4, state: 0, last_update_tick: 15, refractory_until: 0 }, CellRuntime { id: 5, state: 0, last_update_tick: 15, refractory_until: 0 }, CellRuntime { id: 6, state: 0, last_update_tick: 15, refractory_until: 0 }], arrows: [ArrowRuntime { id: 0, eligible_until: None }, ArrowRuntime { id: 1, eligible_until: None }] }`
- header production: `DecodedCheckpoint { tick: 15, next_serial: 4, manifest_len: 103, body_len: 549, cell_count: 7, arrow_count: 2, pending_count: 0, load_count: 0, cells: [CellRuntime { id: 0, state: 0, last_update_tick: 0, refractory_until: 1 }, CellRuntime { id: 1, state: 0, last_update_tick: 0, refractory_until: 1 }, CellRuntime { id: 2, state: 0, last_update_tick: 0, refractory_until: 0 }, CellRuntime { id: 3, state: 0, last_update_tick: 0, refractory_until: 0 }, CellRuntime { id: 4, state: 0, last_update_tick: 0, refractory_until: 0 }, CellRuntime { id: 5, state: 0, last_update_tick: 0, refractory_until: 0 }, CellRuntime { id: 6, state: 0, last_update_tick: 0, refractory_until: 0 }], arrows: [ArrowRuntime { id: 0, eligible_until: None }, ArrowRuntime { id: 1, eligible_until: None }] }`
- ARROW runtime equal: `true`
- durable-body hash equal: `true`
- physical-transition hash equal: `true`
- independent replay: `true/true`
- identical future causal continuation: `true`
- continuation hash: `e3dff6fb1c09a647d53c4753da24d31ed6212b78b32a9a7230ebafe22a2c61d3`
- continuation durable-body hash: `248d32a9780d881a98d62ae891c389f05abad4e55b23d71b4f83c54981b1e5b9`

Differing fields:

- CELL 0: state 0|0, last_update 15|0, refractory 1|1
- CELL 1: state 0|0, last_update 15|0, refractory 1|1
- CELL 2: state 0|0, last_update 15|0, refractory 0|0
- CELL 3: state 0|0, last_update 15|0, refractory 0|0
- CELL 4: state 0|0, last_update 15|0, refractory 0|0
- CELL 5: state 0|0, last_update 15|0, refractory 0|0
- CELL 6: state 0|0, last_update 15|0, refractory 0|0
