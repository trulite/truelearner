# PX8 closure negative-v1 diagnostic v1

Outcome: **DIAGNOSTIC COMPLETE; NOT AUTHORITY**.

- roots serialized: `16/16`;
- clause records serialized: `224/224`;
- failing roots: `16`;
- failing clauses: `16`;
- maximum work: `14788`;
- maximum persistent bytes: `5488`;
- exact replay roots: `16/16`;
- naturally quiescent roots: `16/16`;

## Failed clauses

- root `862001` layout `reverse=false reflect=false twist=0` clause `12` `bounded_stable_memory`: expected `maximum_bytes<=8192 && memory_stable==true`; actual `maximum_bytes=5488~memory_stable=false`; first divergent state `none`.
- root `862002` layout `reverse=false reflect=false twist=137` clause `12` `bounded_stable_memory`: expected `maximum_bytes<=8192 && memory_stable==true`; actual `maximum_bytes=5488~memory_stable=false`; first divergent state `none`.
- root `862003` layout `reverse=false reflect=false twist=274` clause `12` `bounded_stable_memory`: expected `maximum_bytes<=8192 && memory_stable==true`; actual `maximum_bytes=5488~memory_stable=false`; first divergent state `none`.
- root `862004` layout `reverse=false reflect=false twist=411` clause `12` `bounded_stable_memory`: expected `maximum_bytes<=8192 && memory_stable==true`; actual `maximum_bytes=5488~memory_stable=false`; first divergent state `none`.
- root `862005` layout `reverse=true reflect=false twist=0` clause `12` `bounded_stable_memory`: expected `maximum_bytes<=8192 && memory_stable==true`; actual `maximum_bytes=5488~memory_stable=false`; first divergent state `none`.
- root `862006` layout `reverse=true reflect=false twist=137` clause `12` `bounded_stable_memory`: expected `maximum_bytes<=8192 && memory_stable==true`; actual `maximum_bytes=5488~memory_stable=false`; first divergent state `none`.
- root `862007` layout `reverse=true reflect=false twist=274` clause `12` `bounded_stable_memory`: expected `maximum_bytes<=8192 && memory_stable==true`; actual `maximum_bytes=5488~memory_stable=false`; first divergent state `none`.
- root `862008` layout `reverse=true reflect=false twist=411` clause `12` `bounded_stable_memory`: expected `maximum_bytes<=8192 && memory_stable==true`; actual `maximum_bytes=5488~memory_stable=false`; first divergent state `none`.
- root `862009` layout `reverse=false reflect=true twist=0` clause `12` `bounded_stable_memory`: expected `maximum_bytes<=8192 && memory_stable==true`; actual `maximum_bytes=5488~memory_stable=false`; first divergent state `none`.
- root `862010` layout `reverse=false reflect=true twist=137` clause `12` `bounded_stable_memory`: expected `maximum_bytes<=8192 && memory_stable==true`; actual `maximum_bytes=5488~memory_stable=false`; first divergent state `none`.
- root `862011` layout `reverse=false reflect=true twist=274` clause `12` `bounded_stable_memory`: expected `maximum_bytes<=8192 && memory_stable==true`; actual `maximum_bytes=5488~memory_stable=false`; first divergent state `none`.
- root `862012` layout `reverse=false reflect=true twist=411` clause `12` `bounded_stable_memory`: expected `maximum_bytes<=8192 && memory_stable==true`; actual `maximum_bytes=5488~memory_stable=false`; first divergent state `none`.
- root `862013` layout `reverse=true reflect=true twist=0` clause `12` `bounded_stable_memory`: expected `maximum_bytes<=8192 && memory_stable==true`; actual `maximum_bytes=5488~memory_stable=false`; first divergent state `none`.
- root `862014` layout `reverse=true reflect=true twist=137` clause `12` `bounded_stable_memory`: expected `maximum_bytes<=8192 && memory_stable==true`; actual `maximum_bytes=5488~memory_stable=false`; first divergent state `none`.
- root `862015` layout `reverse=true reflect=true twist=274` clause `12` `bounded_stable_memory`: expected `maximum_bytes<=8192 && memory_stable==true`; actual `maximum_bytes=5488~memory_stable=false`; first divergent state `none`.
- root `862016` layout `reverse=true reflect=true twist=411` clause `12` `bounded_stable_memory`: expected `maximum_bytes<=8192 && memory_stable==true`; actual `maximum_bytes=5488~memory_stable=false`; first divergent state `none`.

## Firewall

- authority-v1 marker emitted: `false`;
- authority-v1 result path written: `false`;
- PX8 promotion or authority claim: `false`.
