# CR0 coupling-necessity result audit v2

Status: development positive; non-authoritative.

## Decision

Under the frozen CR0 decision rule, coupling plasticity is necessary as an
independent physical affordance.

The result is deliberately narrower than “every learned structure must change
coupling.” It establishes:

> At equal durable resistance and under identical future physical input,
> coupling can make a single previously subthreshold route causally effective
> where persistence alone cannot.

## Evidence

The sole valid v2 matrix executed in E2B sandbox `igqql0x2kvj39alq8w063`:

| Gate | Result |
|---|---:|
| Physical cases | 400/400 |
| Mechanics rows | 800/800 |
| Retained-behavior cases | 240/240 |
| Efficacy-control cases | 160/160 |
| Same-mechanics replay | exact |
| Reference/Production physical equality | exact |
| Functional predicates | all pass |
| Natural quiescence | all pass |
| Maximum PhysicalWork | 66 |

Evidence hashes:

```text
matrix a44eb399095609e5f2fa9cd3b4a0250f15f341d2139056a0aed75168053af07f
report 0987ef90b02dd277ae378a5411ecb938e4dfe82bad5a66098b9024947eed0ee1
```

The report retains the evaluator's v1 internal heading/marker because v2 was
restricted to the preregistered comparator and packaging repair. Its location,
commit, roots, hashes, and this audit identify it as v2 evidence.

## Retained behavior

Both equal-resistance arms preserved:

- CPC0 contact-compartment attribution;
- CPC1 path-local continuous participation and renewal;
- PQLC0 one-hop local closure;
- PQLC1 unchanged depth-16 composition with fifteen relay traversals;
- FD0 equal-resistance death age;
- FD1 consequence consolidation and the use-without-consequence negative.

Coupling 2 therefore did not buy efficacy by breaking the retained local credit,
closure, forgetting, or consolidation behavior in the tested worlds.

## Efficacy distinction

At threshold 1, coupling 1 and 2 both fired the target and produced one outward
crossing. At threshold 3, neither did. Two distinct coupling-1 inputs could
jointly fire threshold 2.

The decisive threshold-2 case held resistance constant:

```text
persistence only
    post-consequence state  R4 / coupling 1
    baseline state          R1 / coupling 1
    target fires            0 / 0
    outward crossings       0

efficacy + persistence
    post-consequence state  R4 / coupling 2
    baseline state          R1 / coupling 1
    target fires            1 / 0
    outward crossings       1
```

Thus resistance determines survival but cannot substitute for transmission
efficacy in this fixed single-route geometry.

## Audit accounting

CR0 v1 remains an immutable measurement negative. Its only
Reference/Production mismatch was accidental equality of raw live-checkpoint
hashes containing mechanics-specific causally inert state.

The v2 physical matrix passed, after which two static-audit issues were handled
without rerunning Rust or the matrix:

1. the FD1 evidence regex incorrectly fixed absolute tick 5 instead of age 5;
2. a fresh worker lacked `rg`, and Bash initially treated command-not-found as
   no forbidden match.

The final audit ran alone in fresh E2B sandbox `iavb0vfd9zjh6q9y7660p` and
emitted `CR0_COUPLING_NECESSITY_V1_STATIC_AUDIT_PASS`. It verified protected
core/evidence hashes, both frozen-state anchors, absence of coupling mutation in
the FD1 core, and absence of semantic routing surfaces in the evaluator.

No CR0 source byte under `truelearner/` changed.

## Scientific boundary

CR0 does not establish the coupling-update law for continuous participation.
It does not edit CPC0's historical contract, resume FD2, run ARC, or advance
authority/oracle status.

The next eligible gate must deliberately integrate consequence-supported
efficacy change into the continuous-participation law, with controls for graded
participation, repeated support, saturation/stability, wrong-path Modulation,
PQLC cycles, and retained resistance consolidation. Only after that integration
passes can a fresh FD2 cumulative attempt be preregistered.
