# AH0 CPC0 parent-differential negative v2

The candidate and exact-parent matrices contain 440 rows. Eighty rows differ,
and every difference is confined to the ordered `trace_hash` column.

All other serialized columns are identical, including deliveries, fires,
resistance updates, proposals, deallocations, crossings, PhysicalWork, clock,
pressure phase, durable body hash, quiescence, same-mechanics replay, and
Reference/Production equality.

V2 nevertheless fails its byte-identical ordered-trace requirement. No runtime
change is justified by this observer result.
