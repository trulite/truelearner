# PX3-R direct physical trace coupling PROBE v3 protocol

Status: **PREREGISTERED LAYOUT AMENDMENT; EVIDENCE UNSPENT; PX3 ABSENT**.

This protocol preserves the unexecuted v1 and v2 protocols. No CELL,
candidate preflight, simulation, or evidence command has run.

## Pre-execution physical-path audit

The frozen PX0 correspondence motif places an arrival CELL two position units
from its correspondence-end CELL and fires the latter two ticks later. The v2
maximum overlap of two ticks would therefore expose the arm opportunity inside
ordinary PX0 correspondence paths as well as at the intended trace-bearing
loci. That is unnecessary scope and would confound the direct-trace question.

## Sole amendment

The local activity opportunity's maximum recent-firing overlap is `1` tick,
not `2`. Candidate trace loci in the PROBE fire simultaneously, so the target
edge remains exposed. The frozen arrival-to-correspondence delay is `2`, so
ordinary PX0 acquisition/traversal cannot expose the arm edge. The maximum
physical distance remains `8`, normal proposed delay remains `1`, and every v2
symmetric edge-exposure clause remains exact.

The temporal-spacing control remains at `4` ticks and is still outside the
opportunity. All other construction, matched marginals, controls,
classifications, artifact paths, atomicity, audits, and authority restrictions
remain exactly as v1 plus v2.

V3 is the sole executable PROBE protocol and must be hash-audited by the
implementation. Earlier protocols remain immutable preregistration artifacts.
