# DS6 cumulative lifetime diagnostic PROBE result

Protocol: `ds6-cumulative-lifetime-probe-v1`.

Status: **DEVELOPMENT CANDIDATE SELECTED; NOT DS6 READY; M3 REMAINS
AUTHORITATIVE**.

## Execution provenance

- frozen implementation commit/tag:
  `493d56bf44c23bd2d174a3ac35bf247dd54fac38` /
  `ds6-cumulative-lifetime-probe-implementation`;
- E2B sandbox: `iyrkw7af5qpmwwfmq3bwm`;
- command: `cargo run --quiet --bin ds6_cumulative_lifetime_probe`;
- authorized executions: `1`;
- process result: successful;
- implementation SHA-256:
  `4bd6007b0bccfd25b708d2f9f06916eaa44a313d3b273c18cbc93afc3bd7d582`;
- runner SHA-256:
  `2e52fe995be1873b242058e291f6f52094c33323f746eef48b1ee5218e3596d4`;
- protocol SHA-256:
  `dd2fa0cf33acde8592be5c92e31f2aa3a883ebff222eb10af95d7c9dc2ad6ead`.

The E2B command completed successfully, but the host tool transport returned
no captured stdout after the asynchronous command completed. The command was
not rerun. The cell values below are therefore a post-run, read-only evaluation
of the exact deterministic frozen implementation, not a claimed copy of the
lost stdout. The process success independently establishes only its frozen
`diagnostic_complete` exit contract.

## Frozen deterministic outcome audit

| Arm | Physical path | Checks | Records final/raw | Outcome | First collapse |
|---|:---:|:---:|---:|:---:|---|
| A recurrence/use competition | yes | 9/9 | 4/15 | PASS | none |
| B surprise/contradiction timescale | no | 1/2 | 0/0 | FAIL | single lifecycle |
| C dependency/reuse economics | no | 1/2 | 0/0 | FAIL | single lifecycle |

Arm A's nine true checks are:

```text
source/information audit
single scalar lifecycle
useful persistence
one-off removal
contradiction response
relearning
economy/lifecycle
determinism
frozen controls
```

The exact scalar trajectory can be audited without executing learner code:

```text
two useful signatures recur four times
        -> strengths 5, 5 after ordinary pressure

eight one-offs plus two ordinary pressure ticks
        -> useful strengths 3, 3
        -> all eight one-offs physically absent

changed signature plus intervening ordinary activity
        -> useful strengths 1, 1 and still allocated
        -> changed causal signature does not match stale learned record

return + relearning
        -> useful records fire
        -> removed one-off is reacquired through the same path
        -> 4 records remain versus 15 in keep-all
```

Arm B is a physical-path negative because the frozen d2.4 trace changes
`value` and `tau` but contains no organism-side deallocation path. Arm C is a
physical-path negative because frozen IR0/economics exposes invalidation,
reuse, bytes, and work but contains no organism-side erase update. Adding such
paths during this PROBE would have violated the frozen no-new-representation
rule.

Exactly one arm therefore passes the preregistered deterministic checks:

```text
selected development candidate
    A recurrence/use competition
```

## Interpretation boundary

This result selects one mechanism family for MICRO. It does not establish
robustness across seeds, exact cumulative M3 preservation at scale, DS6
development readiness, a definitive DS6 result, or M4. The ordinary-pressure
clock and scalar update remain supplied physics; learned lifetime arises only
because recurrence/use history accumulates differential scalar strength.

The stdout-capture defect remains part of this result provenance. MICRO must
write its development artifact atomically inside the single authorized
command so transport loss cannot erase the result.

