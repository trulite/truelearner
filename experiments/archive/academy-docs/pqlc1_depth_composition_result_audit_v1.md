# PQLC1 depth composition result audit v1

Status: development positive. No authority or retained-law promotion.

Frozen evaluator: `226c180`, tagged
`pqlc1-depth-composition-frozen-v1`.

Immutable evidence: `47406a9`, tagged
`pqlc1-depth-composition-positive-v1`.

## Result

The sole frozen matrix completed:

```text
case variants                               39 / 39
physical cases                             780 / 780
mechanics rows                            1560 / 1560
same-mechanics exact reconstructions      3120 / 3120
Reference/Production ordered-history match 780 / 780
predicate-positive cases                    780 / 780
variant-complete count                       39 / 39
natural quiescence                              true
frozen PQLC0 core hashes                        exact
```

Every case passed under two fresh identity roots and all ten pressure phases.
Reference and Production observations were exactly equal before predicates
were evaluated.

## Depth composition

Complete chains produced the exact depth-independent composition expected from
the local rule:

| Contact depth | QLP traversals | PhysicalWork |
|---:|---:|---:|
| 1 | 0 | 6 |
| 2 | 1 | 11 |
| 4 | 3 | 21 |
| 8 | 7 | 41 |
| 16 | 15 | 81 |

QLP traversal count is exactly `depth - 1`. PhysicalWork is exactly linear on
the tested complete chains (`5 * depth + 1`). No runtime field, branch, counter,
recursive parameter, or physical law varies with depth.

This is development evidence for depth-independent local-rule composition
through depth 16. It is not a mathematical proof over unbounded depth and does
not advance authority.

## Causal-boundary controls

All `240` structural-break cases passed. The downstream participating suffix
received closure; the first live but untraversed contact stopped it; the break
and upstream prefix received zero support.

All `240` temporal-break cases passed. The selected contact had genuinely
traversed, but its CPC1 state naturally relaxed across the ordinary `1024`-tick
delay. Closure stopped there while the later downstream suffix remained
available.

All `80` wrong-branch cases passed. The final qualifying compartment emitted
through both explicit QLP outputs, but only the branch with its own local
participation continued. The other stopped at its first contact.

All `20` honest-fan-out cases passed. When both adjacent branches genuinely
participated, both continued and received support. The mechanism therefore
retains the coarse attribution resolution supplied by ordinary topology and
does not hide one-path semantics.

## Recurrent controls

Every recurrent chain naturally quiesced without new damping:

| Contact depth | QLP traversals | PhysicalWork |
|---:|---:|---:|
| 1 | 310 | 1247 |
| 2 | 310 | 1248 |
| 4 | 310 | 1250 |
| 8 | 310 | 1254 |
| 16 | 310 | 1262 |

The QLP traversal count remained constant at `310`; recurrent work increased
only with the initial chain depth and remained far below the frozen `8192`
ceiling. No attenuation, participation consumption, TTL, cycle detector,
special stop, or recurrent-case runtime branch exists.

## Integrity and provenance

Evidence hashes:

```text
matrix.csv
ca4ba6be393fe0c282f47bbabd5bcfbdecdacb6d57752dbe50cf8462a793b4f6

report.md
e2bf9ffdd8051afe1755e4b12446826c252f0a7c1e9ef098c78ce81f50459533
```

E2B:

- targeted formatting/check/Clippy/preflight:
  `i5uijy8319ryducypqppi`;
- sole matrix execution and first static audit:
  `ikfb09v8s2pctcpll0trt`;
- fresh committed-artifact verification:
  `i2j83brspegxmmngvw0by`.

The matrix executable ran exactly once. The later verification executed only
the frozen shell audit and checksum validation.

## Boundary

PQLC1 adds no physical law. The accepted PQLC0 core is byte-identical. Pressure,
eligibility deletion, ARC A3-A5, authority, oracle, and `arch.md` remain
unchanged. CPC2 remains an immutable stopped negative.

The causal-closure prerequisite for reopening pressure de-supply is now
developmentally satisfied. Any pressure integration requires a new, separately
preregistered gate.
