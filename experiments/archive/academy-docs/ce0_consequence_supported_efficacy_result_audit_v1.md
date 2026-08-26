# CE0 consequence-supported efficacy result audit v1

Status: immutable development negative; fully classified.

## Execution

The frozen CE0 matrix executed exactly once in fresh E2B worker
`i3anz5jq8nxgudrg0dfy1` from evidence-eligible commit `023a023`.

The evaluator serialized its complete CSV and report before its aggregate
assertion stopped negative. No rerun, comparator repair, candidate repair,
coupling ceiling, damping, normalization, cycle rule, FD2 work, ARC work,
authority change, oracle change, or `arch.md` change occurred.

Evidence:

```text
matrix  40e64538a67068d09737c1f123bbdd7d4fcdde674925c2efce0a88807445aa46
report  b19bb22681c288ead186f1ac925685d2d2da03803303465377a3b2e72fc43dae
```

## Complete result

```text
physical cases                         200 / 200 serialized
mechanics rows                         400 / 400 serialized
same-mechanics exact replay            400 / 400 PASS
Reference / Production physical match  400 / 400 PASS
natural quiescence                     400 / 400 true
frozen functional predicates           360 / 400 PASS
maximum PhysicalWork                   78
```

The 360 positive rows comprise every root, phase, and mechanics row for nine
of the ten preregistered families:

- no qualified consequence: coupling unchanged;
- repeated traversal alone: coupling unchanged;
- prompt qualified consequence: coupling `1 -> 2`;
- wrong-contact and fully late Modulation: no coupling change;
- weak/full/strong participation: graded support and quantum-crossing efficacy;
- repeated supported experiences: coupling observations `2|3|4`;
- thresholds 1/2/3 and two-input control: `1|1|1|1` expected firings;
- equal resistance with coupling 1/2: distinct efficacy and equal lifetime;
- fan-out: both participating local contacts matured, unrelated contact did not.

These positives are diagnostic properties of the rejected candidate, not an
accepted CE0 law.

## Exact stopped-negative cause

All 40 recurrent-stability rows failed identically: two identity roots, ten
absolute phases, and both Reference and Production mechanics.

```text
before support
    reciprocal threshold-2 topology settles
    initiating CELL fires exactly once

after genuine support in both directions
    A->B coupling  1 -> 2
    B->A coupling  1 -> 2

fresh probe
    A fires 33 times
    B fires 32 times
    total recorded fires 70
    PhysicalWork 78

eventual end
    local forgetting deletes A->B
    B->A remains live at resistance 1
    pending activity empties naturally
```

The final recurrent ARROW states were identical in all 40 rows:

```text
A->B  live=false resistance=0 coupling=2 participation=0 support=0
B->A  live=true  resistance=1 coupling=2 participation=32773720845
      support=4294967296
```

Final clock was `phase + 73`. Eventual deletion makes the trial bounded but
does not satisfy the frozen stability claim: the initiating CELL re-fired 32
times after its intended probe firing. Consequence-supported efficacy therefore
turned a settling reciprocal body into a transient self-sustaining oscillator.

## Classification

CE0's proposed accumulated-support quantum rule is rejected.

CR0 remains valid: coupling has a distinct physical function. CE0 additionally
shows that unrestricted positive efficacy maturation is insufficient as a
general local law because physical feedback topology makes efficacy growth
dangerous. This is not a representation problem: ordered physical histories,
final bodies, work, and replay agreed exactly between Reference and Production.

No claim is made yet about the required missing constraint. It may concern
local efficacy competition, conservation, homeostasis, bidirectional revision,
or another ordinary physical interaction. CE0 did not test those hypotheses.

