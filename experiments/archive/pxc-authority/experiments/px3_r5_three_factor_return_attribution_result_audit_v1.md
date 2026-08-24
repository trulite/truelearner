# PX3-R5 three-factor return attribution result audit v1

Status: **FROZEN R5-B CARRYOVER NEGATIVE; PX3 AUTHORITY REMAINS NEGATIVE**.

## Frozen execution

- implementation tag:
  `px3-r5-three-factor-return-attribution-implementation-v1`;
- implementation commit:
  `d79eedd7b5a711933924b51cc919aa0e8d93053e`;
- E2B sandbox: `i6x9gykt9tvp6xfz5z8ra`;
- sole marker:
  `PX3_R5_THREE_FACTOR_RETURN_ATTRIBUTION_EVIDENCE_SPENT`;
- CSV SHA-256:
  `947257da74420ca4d8a1dc4f49402ddd63bb7ae6d1fe758362c6779365bc73cf`;
- report SHA-256:
  `c88dcec19cd18de0723ec927dcaec0829efff1b5c7c4f867b598161b9d61acd1`.

The command executed once in E2B. The atomic artifacts are unmodified. No
rescue, tuning, regeneration or second execution occurred.

## Outcome

The preregistered classification is **R5-B CARRYOVER NEGATIVE**:

- all `16/16` rows were valid;
- all `80/80` validity clauses passed;
- exact replay and natural quiescence held in every row;
- every non-adjacent core control passed in both insertion strata;
- the adjacent A+B recurrence/no-return control failed its scientific claim in
  both strata exactly as R4's carryover warning predicted.

## What three-factor normalization solved

The world-return participant was physically real:

```text
external world input
-> R source fires
-> R outlet traverses
-> direct + local-hub normalization
-> exactly one ordinary R trace
```

M had no raw return input and no semantic return flag. In both strata:

- `P+X+R` fired M once, emitted one unit echo, strengthened the candidate
  `1 -> 4`, and did not refire P;
- `P+X` without R did not fire M;
- `P+R` with X blocked did not fire M;
- `X+R` with P absent did not fire M or create a candidate;
- `P+X+late A` without R did not fire M;
- the real-return/upstream collision contained exactly one R trace and one
  R-backed M firing;
- two separated complete chains produced exactly two R traces, two M firings
  and two unit echoes; the candidate ended at resistance 6 without oscillation.

This establishes a useful narrow positive: an ordinary normalized physical
return path can serve as the third causal participant for isolated completed
chains. No provenance representation is required for those cases.

## What remained broken

In each adjacent recurrence/no-return row, R never fired:

```text
R source firings   0
R outlet firings   0
R trace firings    0
```

Nevertheless M fired at tick 4 and emitted a unit echo:

```text
tick 3  episode-1 P trace + X trace -> M state 2
tick 4  one unit survives ordinary decay
        episode-2 P trace + X trace -> M reaches 3 and fires
tick 5  false unit echo reaches P
```

The renewed upstream arrival had already strengthened the candidate `1 -> 4`;
the false echo then strengthened it again to 7. Thus the R factor is required
for an isolated episode but not logically required by the current dynamical M
cell across adjacent episodes.

The remaining failure is not return provenance and not source-window timing.
It is temporal carryover inside the attribution convergence cell: ordinary
decaying amplitude allows contributions from different candidate episodes to
sum into the three-factor threshold.

## Boundary

R5 validates physical return-path normalization but rejects the stronger claim
that a threshold-three ordinary cell implements continuous three-factor causal
attribution under the current decay law. Every execution quiesced and storage
remained bounded (`2784` or `2848` bytes); this is not a memory leak.

No PX0 law changed, no semantic field was added, and PX3 authority remains
absent.
