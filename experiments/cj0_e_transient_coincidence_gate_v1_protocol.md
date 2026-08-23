# CJ0-E transient-coincidence development GATE v1 protocol

Status: **PREREGISTERED; GATE UNSPENT; DEVELOPMENT ENDS AT GATE**.

## Eligible frozen inputs

GATE is enabled only by:

- frozen v1 PROBE PASS at commit
  `15ae29b042835a4baea43db6ec69bd9da1017d28`;
- preserved v1 MICRO negative at commit
  `3f3163bb9d401fd469c87a8bc80bf526918ac369`;
- corrected fresh-identity MICRO v2 PASS at commit
  `beb2c8ec291377aefad61edfaa06b52593e5bb04` and annotated tag
  `cj0-e-transient-coincidence-micro-v2-positive`;
- exact physical-law generated-source SHA-256
  `e64c8c915c2fbc4679d1e34ee69ecfe36e2c5ff05bdff5d7feeb5a55578bf1c1`.

All v1 and MICRO v2 protocols, implementations, results, handoffs, and tags
remain immutable. GATE uses the same physical-law library byte-for-byte and a
fresh external evaluator crate. No law parameter, field, threshold, decay,
refractory rule, eligibility rule, return rule, pressure rule, proposal rule,
or persistent encoding may change.

## Flat GATE matrix

The exact flat matrix contains `8 seeds x 4 strata x 2 exact executions = 64`
fresh blank worlds. Namespace for seed `s`, stratum `g`:

```text
0xd300_0000 + s * 0x0010_0000 + g * 0x0001_0000
```

The duplicate is reconstructed from blank matter with the same namespace and
schedule; it is not a clone of final state. Every cell performs:

1. twelve A+B/C+D acquisition rounds;
2. held-out trained and crossed uses;
3. forty A+D/C+B reversal rounds;
4. held-out new and old uses;
5. one-use historical replay;
6. learned-specific self-evidence observation;
7. complete/permanent fingerprint and physical ledger serialization.

Strata vary only existing physical conditions:

| stratum | round spacing | cluster gap | distractor CELLS | mirror | allocation/arrival permutation |
|---:|---:|---:|---:|:---:|:---:|
| 0 | 20 | 3 | 0 | no | seed-rotated |
| 1 | 22 | 4 | 8 | yes | seed-rotated |
| 2 | 24 | 5 | 24 | no | seed-rotated |
| 3 | 26 | 6 | 48 | yes | seed-rotated |

Distractors are isolated ordinary threshold-1 CELLS receiving exactly matched
external activity at every cluster. They have no local neighbor and cannot
select or update a convergence site. Both experience schedules have identical
per-route occurrence count, total activity, traversal opportunity, return
opportunity, timing marginal, pressure exposure, effect opportunity, identity
frequency, and distractor load.

Every flat cell requires:

- route firing marginals exactly equal;
- initial A+B/C+D learned support live at coupling `2`;
- initial held-out A+B/C+D output `1|1` and crossed A+D/C+B output `0|0`;
- old support physically deallocated after reversal;
- new A+D/C+B support live at coupling `2` and held-out output `1|1`;
- old held-out and one-use historical replay output `0|0`;
- no learned conjunction traversal, output, or reinforcement under repeated A
  alone while ordinary contributor returns remain disclosed;
- exact duplicate metrics and complete/permanent fingerprints;
- natural quiescence and no autonomous participant refiring.

## Common controls

Fresh namespaces beginning `0xd400_0000` independently serialize:

- A, B, C, and D singleton use;
- too-late completing activity;
- correlation/priming/return without participant traversal;
- traversal with return blocked;
- genuine contemporary co-participation;
- three-route ambiguity and four-route co-participation;
- absent local output opportunity;
- fully stale/deallocated output path;
- full-deallocation changed-organization bootstrap;
- exact replay, mirror/permutation, fresh layout/allocation/identity;
- repeated singleton recurrence without learned reinforcement;
- natural quiescence and bounded useful recurrence without runaway.

No expected result, schedule label, or site index has a causal update path into
physical matter. All six convergence opportunities are present symmetrically.

## Same-law recursion

Fresh namespace `0xd500_0000` uses the same physical law without a level flag:

```text
A + B -> X
X + C -> Y
Y + D -> Z
```

Primitive A/B/C/D and learned X/Y/Z are ordinary threshold-3 participant
CELLS with the same two-impulse activation contract and fixed outgoing
coupling-1 interface. Each conjunction uses an ordinary threshold-2 locus and
locally proposed output ARROW. Eight repeated ordinary experiences must yield
X on rounds `2..8`, Y on rounds `3..8`, and Z on rounds `4..8`; held-out
execution must reach Z. Every queue must drain naturally. Any recursive
failure stops without architectural repair.

## Ordinary convergent reachability

Fresh namespace `0xd600_0000` contains ordinary A->C and B->C ARROWs into one
threshold-1 CELL. Without another law, isolated A, isolated B, and A+B must
each fire C exactly once. The A+B case may suppress a duplicate same-tick C
firing through the existing refractory rule; one physical reach is required.

## Temporal expressivity

Fresh namespaces beginning `0xd700_0000` serialize separate physical traces
for:

- together at one tick;
- ordered A then B within the same tick by physical phase/order;
- overlap/while residual state is live;
- a one-tick delayed B after ordinary state decay;
- B absent before natural queue closure;
- B later than closure.

Expected signatures are fixed: together, same-tick ordering, and live overlap
fire the convergence locus once; one-tick delay, absent B, and post-closure B
do not. No logical operator or persistent timing variable is introduced.

## Atomic result and stop rule

The separately named GATE binary provides `--preflight` and `--gate` only.
It refuses missing/unknown arguments, a missing MICRO v2 PASS artifact, and any
existing result or staging path. Preflight enters no CELL. The single GATE run
atomically publishes CSV and report regardless of outcome.

Any false clause is frozen and ends the lane. PASS also ends the lane after
handoff; no later-stage protocol, command, artifact, simulation, or marker may
be created or entered.
