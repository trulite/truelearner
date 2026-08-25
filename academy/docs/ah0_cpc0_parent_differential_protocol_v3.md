# AH0 CPC0 parent-differential protocol v3

Status: frozen before observer correction.

The parent observer's ordered trace hash assigns meaning to the recording order
of transitions that share physical tick and phase. AH0 intentionally removes
arbitrary handle ordering, so that sequence is not a valid invariant.

Add an observer-only normalized trace hash:

- retain tick, phase, event kind, event payload, and multiplicity;
- sort only the serialized observations used to form the hash;
- retain the raw ordered trace hash as a diagnostic column;
- do not change runtime execution or any world.

Freeze the exact SI0 v2 parent matrix under this observer. Compare the AH0
candidate row by row. Every column except raw ordered trace hash must be exactly
equal; normalized trace hash must be equal. Any event-content, multiplicity,
body, work, clock, replay, mechanics, or quiescence difference stops AH0.

This normalization is confined to the AH0 parent differential. It does not
broaden checkpoint equality or alter any retained evaluator.
