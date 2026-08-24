# DS4 cumulative request/start PROBE handoff

Status: **DEVELOPMENT PROBE POSITIVE; NOT CLAIM-ELIGIBLE**.

The first statically absent physical edge was closed without changing a frozen
mechanism or adding a persistent representation:

```text
learned M3 event completion activity
        -> frozen P4 request-role selection
```

Frozen probe implementation:

- commit: `d561945ac0f4bd88de91898c9976ffcf976f8793`;
- tag: `ds4-cumulative-request-start-linker-probe`;
- development base seed: `94_000`;
- dedicated E2B sandbox: `iuwbijtin0m9nkght0bbv` (left running).

Observed path:

```text
learned M3 uses                 2
M3 completion activity          2
P4 selection activations        1
P4 execution activations        1
P4 update activations           1
selected identity from occurrence true
pre-answer trace                true
M3 physical work                883
```

Identical P4 evaluation with no M3 completion activity produced:

```text
selection 0
execution 0
update    0
```

All frozen M3, P4, target, protocol, result, persistent-state, and linker
source audits passed. The probe establishes path existence only. It does not
establish DS4, create M4, authorize a definitive run, or alter the frozen
MICRO/GATE target. The next action is control hardening around the unchanged
linker.
