# CL0 ordinary CELL lifetime and reuse result audit v1

Status: stopped negative at Gate 9; Gates 1–8 positive.

## Gates 1–8 result

The frozen CL0 matrix executed exactly once in fresh E2B worker
`iw23q00zu5dwzs6wicbrf` from candidate commit `4cc8fe4`.

```text
physical cases                         100 / 100
mechanics rows                         200 / 200
clauses                               2120 / 2120
Reference / Production exact           PASS
same-mechanics exact replay             PASS
natural quiescence                      PASS
maximum PhysicalWork                       3
```

The matrix covered two fresh identity roots, absolute CELL birth phases 0–9,
five physical families, and both permanent Reference and selected Production
mechanics.

### Local lifetime and phase invariance

Otherwise identical ordinary CELLs produced:

```text
resistance 1  -> death age 10
resistance 2  -> death age 20
resistance 4  -> death age 40
```

Every phase 0–9 produced those same ages. At physical age 9 all three remained
live with local decay load 9. No global phase, birth phase, expiry timestamp,
CELL kind, traversal, or participation influenced persistence.

### Fresh identity and resident-slot reuse

Across every root, phase, and mechanics implementation:

- the old CELL became non-live and advanced generation `1 -> 2`;
- its old `CellRef` stopped resolving;
- a newly added ordinary CELL received a fresh `CellId`;
- the new identity occupied exactly the old `CellSlot`;
- its generation was 2; and
- the new `CellRef` resolved while the old reference remained stale.

This establishes that CELL residence is reusable while physical identity is
not residence.

### Stale incident topology

Incoming and outgoing ARROW controls used incident ARROW resistance 4 so the
ARROW remained live for thirty ticks after the weak CELL died.

After slot reuse:

- an incoming ARROW naming the dead target identity/generation produced no
  delivery, crossing, or replacement firing;
- an outgoing ARROW naming the dead source identity/generation could not
  execute when the replacement fired;
- CELL death did not cascade-delete either ARROW; and
- each stale ARROW disappeared only at its own ordinary local lifetime.

Queued physical ordering retains the original target physical identity, so an
inert stale target need not occupy a resident slot merely to preserve
deterministic mechanics.

### Orphan topology

The weak contact CELL died at age 10. Its two resistance-2 incident ARROWs
remained live through age 19 and independently died at age 20. The CELL slot
was reused immediately after CELL death; both ARROW slots were reused only
after their own deaths. No orphan detector, degree check, reference count,
cascade deletion, or garbage collector appeared.

## Gate 9 result

After the positive matrix, the separately frozen Gate 9 audit ran in fresh E2B
worker `ine5gylab32gb3rkk70xv` from audit-corrected commit `071a666`.

```text
gate_9=negative
accepted_arrow_consolidation=true
accepted_cell_consolidation=false
cell_resistance_increase_paths=0
new_cell_consolidation_added=false
CL0_ORDINARY_CELL_LIFETIME_GATE_9_STATIC_NEGATIVE_V1
```

Accepted qualified Modulation can increase ordinary ARROW resistance. No
accepted local interaction increases CELL resistance. CL0 therefore cannot
establish that a useful ordinary CELL earns longer persistence without adding
a second, unpreregistered learning law.

The frozen stop rule applies. CL0 is not development-ready, and CV0 was not
resumed.

## Integrity and hashes

```text
matrix       0876ed1c3a4e65f4569e751288fc624997f57c0cb5468a8bd42218c401362b5a
report       d38204401a65acb12c7590f5132eea817a5bf886f8a32e801d0f83ebeb8f46f6
Gate 9 audit 8b5c44cc7da8dc90cc689738f171e3fcf05cb8c78d1c7603fca73d9c96db2b88
```

Reusable Rust worker `ifk44bxtlfjlci644r63m` performed formatting, targeted
default-core and CL0 release checks, strict targeted Clippy, and the frozen
implementation source audit. It remains preserved as the requested Rust
compilation worker.

The first Gate 9 attempt in fresh worker `i5swm36phkm9kca2aut3m` produced no
evidence: its output directory was absent and one audit command treated source
text as a filename. Only the audit plumbing was corrected; no organism or
evaluator file changed, and Gate 9 then ran fresh exactly once.

No Rust command ran locally. No workspace-wide build or unrelated test suite
ran.

## Classification

CL0 establishes:

> Ordinary CELLs can undergo phase-free finite lifetime, generation-safe
> death, fresh-identity resident-slot reuse, and stale incident-reference
> invalidation without a special temporary CELL class or garbage collection.

CL0 does not establish:

> Consequence can make a useful ordinary CELL more durable.

That missing consolidation role must be investigated independently. It cannot
be supplied as a CL0 rescue.
