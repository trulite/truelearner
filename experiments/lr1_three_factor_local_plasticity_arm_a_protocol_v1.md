# LR1 three-factor local plasticity Arm A protocol v1

Status: **PREREGISTERED; DEVELOPMENT EVIDENCE UNSPENT; PX3 REMAINS NEGATIVE**.

Start: frozen LR0-D0 commit `9f6ad1ea8ce3f471595fcb0f3f3e9c32a3b70136`,
tag `lr0-d0-qualified-physical-return-information-v1`.

| frozen input | SHA-256 |
|---|---|
| authoritative PX0 law | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| LR0-D0 audit | `96be2143b13eb42bbfe6fda418b312123824e9f57a8904ff89ca6fd64148e6ff` |
| R6 integration-collapse audit | `52072394fcf9867f23d1ec982f030fac6d5b5601c8f7294f178098743664b033` |

## LR1 question

> Can a recently traversed candidate be strengthened only by later activity
> arriving through a physically completed downstream route, while physically
> matched renewed upstream drive may execute the same source but cannot alter
> that candidate?

LR1 treats execution and plasticity as different local physical effects. It
does not add reward, valence, outcome, RETURN, CREDIT or CAUSE data.

The mechanisms are staged by architectural cost:

1. Arm A -- route-aware use of existing topology and eligibility state;
2. Arm B -- a separate local return compartment plus a small successor rule;
3. Arm C -- distinct modulatory transmission.

Only Arm A is opened here. B and C remain unimplemented and evidence-unspent.

## Frozen Arm A law

The successor propagation layer passes the actual incoming ArrowId to local
plasticity instead of discarding it. It adds no active state.

For each recently eligible candidate `P -> X` and incoming physical traversal
`S -> P`, strengthening is permitted exactly when a directed path exists from
`X` to `S` such that every path arrow:

- is live;
- has actually traversed recently;
- still has its ordinary PX0 `eligible_until` window live at the arrival tick.

Together, the eligible arrows form a currently completed directed cycle:

```text
P -> X -> ... -> S -> P
```

The incoming arrow is not nominated by the harness. The rule is applied
uniformly to every internal arrival and every eligible outgoing candidate.
External arrivals do not qualify because they have no incoming ArrowId.

This arm uses only existing ArrowId, endpoints, liveness, generation,
`eligible_until`, tick and topology. It adds no per-arrow mode, trace, port,
flag or semantic type. The reachability work must be ledgered. A functional
positive does not by itself make the law acceptable: global/unbounded graph
search or excessive work is an explicit architectural cost that may favor Arm
B even if A discriminates correctly.

## Matched physical geometry

Every fresh world has one resistance-one candidate `P -> X`, with `P`
threshold two. Two ordinary upstream arrows `U0 -> P` and `U1 -> P` each carry
one unit; together they may execute P. The real return arrow `S -> P` also
carries one unit and therefore cannot execute P by itself.

Candidate execution makes X participate. The downstream path contains an
ordinary `X -> S` traversal plus one unit of physically external world
activity at S. Only their coincidence makes S fire and traverse `S -> P`.
Thus timely return closes a live physical cycle. Upstream `Ui -> P` is matched
in coupling, delay, generation and liveness but has no live directed path from
X to Ui.

An independent X driver exists only for the no-candidate-participation
control. It can make X and the downstream route participate while `P -> X`
remains ineligible.

No structural proposal, candidate formation, R6 detector, PX3 organization,
reversal or recursion is in scope.

## Frozen matrix

Seeds `4101, 4109, 4111, 4127` cross normal/reversed cell insertion and
forward/reflected positions. Six fresh cases execute per seed, in this order:

1. `renewed-upstream-no-return` -- P executes, the candidate traverses, and
   matched U0/U1 drive reaches P again at the lawful-return tick; P may execute
   again, but candidate resistance remains one and return updates are zero;
2. `completed-downstream-return` -- P and candidate traverse, X and the world
   jointly make S traverse the real `S -> P` route; exactly one update changes
   candidate resistance `1 -> 4`, while the one-unit arrival does not refire P;
3. `independent-return-no-candidate` -- an independent driver makes X and the
   same downstream return route traverse while `P -> X` never traversed;
   candidate resistance remains one and updates are zero;
4. `late-downstream-return` -- the same completed physical route reaches P
   after candidate eligibility expires; resistance remains one and updates are
   zero;
5. `simultaneous-upstream-and-return` -- renewed U0/U1 drive and lawful S
   return reach P at the same tick; P may execute, but exactly one lawful
   update occurs and candidate resistance becomes four, not seven;
6. `upstream-execution-only` -- fresh U0/U1 activity may execute P and traverse
   the candidate once, but cannot immediately alter its resistance.

Exactly 24 rows run twice from fresh state for exact replay.

## Required serialization

Each row records:

- actual U0/U1, P, candidate, X, world, S and S->P participation;
- incoming physical ArrowIds/endpoints, coupling, delay and generation at P;
- candidate ArrowId, traversal ticks, eligibility window, coupling,
  resistance transitions and liveness;
- route-qualification checks, accepted/rejected arrivals, searched path edges
  and exact local-return update count;
- P firing count after the later activity;
- work, storage, complete/permanent fingerprints, exact replay and natural
  quiescence.

Resistance may not stand in for route participation, qualification or source
execution.

## Classification

- **LR1-A functional positive:** every row passes, especially zero update for
  renewed upstream drive, exactly one update for a completed return, and one
  rather than two updates when both coincide.
- **LR1-A negative:** upstream traffic strengthens; a completed route cannot;
  independent/late return strengthens; lawful weak return refires P; replay or
  quiescence fails; or any matched-world clause fails.

Functional classification is separate from architectural selection. The
result report must preserve path-search work and state whether the rule is
bounded/local enough to retain or whether Arm B remains required.

## Evidence discipline

The successor law lives in a new crate; authoritative PX0 is byte-identical.
All Rust formatting, compilation, tests, lint, preflight and runtime occur only
in E2B. After implementation and a separate execution protocol are frozen and
tagged, `--lr1-a` executes once. Failure is frozen without tuning or rerun.

No result changes PX0 authority, opens PX3, runs an authority matrix, or
authorizes PX4. Any future continuous stack must separately replay PX0--PX2
conformance against a selected successor law.
