# SV0 symmetric sign variation result audit v1

Status: immutable development negative; first failed gate B.

## Execution and integrity

The frozen SV0 matrix executed exactly once in fresh E2B worker
`inihr4kunadbpkcoxa8lk` from evidence-eligible commit `b1dfdfa`.

```text
physical cases                         72 / 72
mechanics rows                         144 / 144
same-mechanics exact replay            PASS
Reference / Production physical match  PASS
Gate A symmetry                        PASS
Gate B positive selection              FAIL
Gate C negative selection              FAIL
Gate D usefulness permutation          FAIL
Gate E neither useful                  PASS
Gate F both useful                     PASS
Gate G bounded variation               PASS
maximum PhysicalWork                   17 / 256
classification                         SV0 negative at Gate B
```

Evidence hashes:

```text
raw matrix       b1e147f2c2f8f29b43a705469e75d72deadf4008b299f08344db40ca5ef4310a
report           8222d55a3c9260a703fd7b53e42765e8b0c14e584eeed1318d9e44e561f28eca
recovered matrix 59ef1ba63fbf344c8e6a5c6e202a4bb91101491b63488a7d6921f1c1db4bbcfc
recovery audit   57cf413335549a3d3e56a5cb9316a91883a5fe53adb3b650398276e11e17bf02
```

The raw matrix is immutable but emitted two diagnostic list fields without CSV
quoting. A deterministic post-run recovery parsed only those bracket-delimited
fields, retained every physical value, and produced 144 rows of exactly 27
fields. The organism was not rerun.

## Gate A — symmetric opportunity

Every local opportunity created exactly two ordinary Drive proposals:

```text
coupling  -1 / +1
resistance 1 / 1
delay      1 / 1
phase      0 / 0
```

Both traversed and crossed the physical region boundary. With no consequence,
both had identical participation, plastic support, decay load, and resistance.
Both deallocated at physical age 10 across every root, start tick, translated
position, and mechanics implementation.

This establishes the narrow variation result: the proposal operator can expose
both sides of signed Drive without favoring either lifetime or opportunity.

## Gates B–D — stopped attribution negative

Positive-useful and negative-useful worlds had identical candidate IDs,
ordering, topology, and complete pre-consequence history. Each world admitted
one ordinary consequence only because its selected crossing sign was present.

However, both signed ARROWs necessarily traversed from the same source contact
at the same physical opportunity. They therefore held identical local
participation when Modulation returned. Existing contact-local learning updated
both:

```text
before consequence
-1  resistance 1
+1  resistance 1

after one selected-sign consequence
-1  resistance 4
+1  resistance 4

retained probe at age 14
-1  resistance 3, live
+1  resistance 3, live
```

The result was identical whether the external world treated `+1` or `-1` as
useful. Thus consequence did not select the useful sign; both alternatives
were co-retained. Gate B is the first failure, with C and D failing for the
same physical reason.

No sign check, ArrowId lookup, candidate filter, contact split, or attribution
repair was added after observation.

## Gates E–G — retained controls

- Repeated traversal without consequence produced zero durable updates; both
  candidates deallocated normally.
- When both signs genuinely received consequences, both matured symmetrically
  from resistance 1 to 7 and remained live.
- One opportunity never held more than two live candidate structures. Repeated
  activation while they were live created no new variants. After reclamation,
  one fresh opportunity created exactly one new pair. Two generations produced
  four proposals, peak live candidates two, four deallocations, and
  PhysicalWork 17.

## Classification

SV0 separates two claims:

1. **Signed variation is symmetric and bounded:** positive evidence.
2. **Existing consequence learning can select between co-located signed
   alternatives:** negative evidence.

The failure is not sign preference in variation. It is physical attribution
granularity: alternatives sharing one source contact share traversal-derived
participation and therefore share consequence. This matches CPC0's earlier
finding that specificity is only as fine as ordinary contact-compartment
topology.

SV0 does not justify targeted credit, sign labels, or winner-take-all. A future
successor must independently ask how physically distinct alternative contacts
can arise without supplying which alternative is useful.
