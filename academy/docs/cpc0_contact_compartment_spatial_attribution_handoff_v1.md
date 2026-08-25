# CPC0 contact-compartment spatial attribution handoff v1

Status: development-ready for human review; non-authoritative.

## Narrow claim

> Given ordinary contact-like CELL/ARROW topology, unchanged LR-C provides
> spatial attribution whose resolution is limited by compartment granularity.

This claim passed `220/220` physical cases and exact Reference/Production
ordered-history comparison with no runtime or substrate-law change.

## Frozen artifacts

- protocol: `cpc0-contact-compartment-spatial-attribution-protocol-v1`;
- evaluator: `cpc0-contact-compartment-spatial-attribution-frozen-v1`;
- positive evidence: `cpc0-contact-compartment-spatial-attribution-evidence-positive-v1`;
- audit-repair protocol: `cpc0-static-audit-portability-repair-protocol-v1`;
- final readiness/result tag: created with this handoff.

## Status ladder

```text
TC-DS0  old eligibility geometry characterized       done
TC-DS1  graded participation candidate                stopped negative
CPC0    contact-compartment spatial attribution       development positive
CPC1    local continuous temporal participation       now eligible, not started
CPC2    chained local causal closure                  blocked on CPC1

pressure de-supply                                    paused
ARC A3-A5                                             paused
authority / oracle / arch.md                          unchanged
```

The CPC0 branch starts directly from TC-DS0 commit `0a14c7e`; it does not
contain or rely on the TC-DS1 feature implementation.
