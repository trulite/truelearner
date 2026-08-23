# CJ0-D local-subunit PROBE v2 timing correction protocol

Status: **PREREGISTERED DEVELOPMENT RETRY; PROBE v2 UNSPENT**.

PROBE v1 is frozen at commit `7a7890e`, tag
`cj0-d-local-subunit-probe-v1-invalid-timing-abort`. It emitted its development
marker and aborted before publication because an evaluator requested an
`advance_time` earlier than the already quiescent substrate tick. It is not
scientific evidence and is never rerun.

This protocol authorizes one mechanical correction only:

1. expose the retained substrate's current physical tick through a read-only
   observer;
2. before evaluator `advance` or external entry, floor a requested tick to the
   current quiescent tick;
3. count and serialize every such evaluator floor;
4. require zero floors in the primary, replica, alternative, ambiguity, and
   genuine-coactivity scientific cells;
5. permit a floor only in a control whose explicitly requested interval is
   mechanically consumed by the immediately preceding finite propagation;
6. change result paths from PROBE v1 to fresh PROBE v2 paths;
7. make no change to the candidate law, physical constants, topology,
   marginals, pass clauses, source identities, or stage eligibility.

Any floor in a primary scientific cell, any changed scientific outcome, or
any further mechanical class is a frozen v2 negative. Format, tests, strict
Clippy, source hashes, refusal, no-CELL preflight, and artifact absence must
pass before the sole `--probe` retry.

This development protocol authorizes PROBE only. It creates no surface beyond
PROBE/MICRO/GATE and cannot advance the lane past GATE.

