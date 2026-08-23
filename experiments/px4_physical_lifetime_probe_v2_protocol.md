# PX4 physical learned-lifetime PROBE v2 protocol

Status: **PREREGISTERED MECHANICAL RETRY; EVIDENCE UNSPENT; PX4 NON-AUTHORITATIVE**.

## Immutable predecessor

PROBE v1 remains an immutable `0/6` negative at commit
`709d9ba86a961f8560928928b5a0ffeb6001a12a`, tag
`px4-physical-lifetime-probe-v1-negative`.

- v1 CSV SHA-256:
  `b7a16d1b5916c2d168366a718e61f46dbb965a0235e12eec3d4a424c2785263e`;
- v1 Markdown SHA-256:
  `5bae8e3175a94a104d85a6eac48ead135e1e2ccf55c5cb508228e6d099d73c61`;
- v1 negative-audit SHA-256:
  `c1c7516ab872dc303891bc2d5d077c7de30fc8a6e05c04cdddabfecb7bc4c426`.

V1 exposed an audit self-match and a mechanically unique pre-use pressure
collapse. It did not discriminate the zero-new-mechanism hypothesis.

## Sole corrections

V2 preserves the six cases, layout, cells, arrows, thresholds, couplings,
reserve `3`, acquisition counts, use counts, ordinary-return routes, pressure
law, matched later gaps, predicates, duplicate comparison, and accounting
schema.

It changes only:

1. both forbidden-token lists are assembled from split string fragments, so
   the scanned source does not literally contain the completed tokens;
2. before each newly available weak direction opportunity is added, ordinary
   substrate time advances to that opportunity's already-fixed first-use tick;
   the candidate is then physically introduced and may first participate at
   that same tick.

This does not protect a candidate from any pressure after introduction. It
only makes the claimed contemporary opportunity physically contemporary.

## Fresh execution

The six cells use namespaces beginning at `0x6_8400_0000`, case stride
`0x0010_0000`; normalized duplicates use `+0x0008_0000`. Outputs are
write-once:

```text
results/px4_physical_lifetime_probe_v2.csv
results/px4_physical_lifetime_probe_v2.md
```

All original physical clauses remain conjunctive. A failure is frozen without
rerun. A positive result makes the already-preregistered fresh MICRO stage
executable; it does not authorize a definitive matrix or create authority.
