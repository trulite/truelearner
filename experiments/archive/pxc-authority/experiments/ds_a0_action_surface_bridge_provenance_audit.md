# DS-A0 action-surface and bridge provenance audit

Implementation: `375bf2c3f7da3f2695cc4910347826c3b5b37278`.

## Causal construction order

1. The baseline has zero executable ARROW roots among the three current DS-E0
   members. Raw spikes and propagation observations are activity history, not
   executor adjacency.
2. `observed_paths` traverses ordinary current-episode coactivity. It emits no
   handle and changes no executable substrate state.
3. Persistent anonymous support gates `form_routes`. For each supported current
   coactivity path it allocates three fresh episode CELLs bound to current DS-E0
   members and installs two live episode ARROWs.
4. `RouteRoot` contains only the installed root CELL reference and generation.
   It does not contain a cells array, ARROW array, effect, opcode, or rank.
5. Only after all roots exist, `expose_routes` copies each root once into an
   opaque-handle table. Permutation reverses only copy order.
6. `execute_handle` looks up that root, validates it, injects a SPIKE, and
   discovers propagation solely by scanning live substrate ARROW endpoints and
   generations. It never reads `RawActivity` or a route descriptor.

## One-to-one inventory

| Matrix | Preformation event roots | Installed roots | Handles | Unique root provenance | Physical paths | Positive ARROW steps |
|---|---:|---:|---:|---:|---:|---:|
| MICRO seed 100 | 0 | 16 | 16 | 16 | 16 | 32 |
| GATE each seed | 0 | 32 | 32 | 32 | 32 | 64 |
| GATE seeds 100..104 | 0 | 160 | 160 | 160 | 160 | 320 |

Primary GATE installation creates 480 route CELLs and 320 route ARROWs before
the bridge copies 160 roots. Each handle maps to exactly one unique installed
root; each installed root maps to exactly one handle; each independent
execution traverses exactly two live ARROWs. Availability is the mechanically
derived length of this table, never a hardcoded boolean.

The removed-observation arm preserves the DS-E0 EventFrame fingerprint while
the installed-root, handle, and executable-path inventory falls from two to
one. The no-plasticity arm uses identical raw activity and mature support but
installs and exposes zero. These controls establish that the bridge does not
infer or construct alternatives.

Mechanical call-surface inventory: one installer, one bridge constructor, one
root executor, zero preassembled `RouteInstance` fields, zero semantic opcode
sites, zero evaluator-selection sites, zero hidden executor sites, zero DS1
choose/apply calls, and zero derived post-action consequence paths.
