# PQLC0 participation-qualified local closure implementation audit v1

Status: frozen before PQLC0 matrix execution.

Parent protocol: `a328674`, tagged
`pqlc0-participation-qualified-local-closure-protocol-v1`.

## Candidate surface

PQLC0 is feature-gated behind `pqlc0`, which implies `cpc1`. The candidate
adds one independent ARROW trigger property:

```text
SourceFires
QualifiedLocalParticipation
```

Every existing ARROW defaults to `SourceFires`. Only a Modulatory ARROW may be
constructed with `QualifiedLocalParticipation`.

An ordinary Modulatory arrival at a CELL first runs unchanged CPC1 local
plasticity. If any live outgoing Drive contact at that CELL retains nonzero
CPC1 participation, every live outgoing QLP ARROW traverses and emits an
ordinary Modulatory SPIKE. This does not activate or fire the CELL. Ordinary
source firing explicitly skips QLP ARROWs.

QLP traversal receives one ordered physical observer event and one independent
work counter. It does not consume or renew the qualifying Drive contact's
participation. No attenuation, TTL, cycle detector, damping, route history, or
new SPIKE effect was added.

Trigger state is explicitly compared by the development evaluator and remains
outside durable/checkpoint formats exactly as preregistered for this gate.

## Frozen matrix surface

The evaluator contains exactly ten worlds, two identity roots, ten pressure
phases, Reference and Production mechanics, and exact same-mechanics
reconstruction. If execution completes, the unconditional result contains
`200` physical cases and `400` mechanics rows.

The closure-cycle world uses delay-one QLP topology and the frozen work ceiling
of `4096`. A timeout, non-quiescent cycle, failed world, mechanics divergence,
replay divergence, or static-audit failure is an immutable negative.

## Frozen hashes

```text
core Cargo.toml
14d45bc379a5220d33b028b48f38319cb888f732d0b34655fda02b3941a829a8

core lib.rs
c5173e8d43d109465252813fba411288c59e3bfa274f790519747eb34314e894

core mechanics.rs
266b713130be6b221432022c7518cc413a0def30ca00371422af6aceeda900da

evaluator Cargo.toml
c3eb86916ccf6dbd94a49f8aad4f4295474efcf7ce9a2329bd465312ea0524bc

evaluator main.rs
b58517163d26103348ab3e391fb512b92aed2912c4c5814412b47d1164e2b209

static audit
4425aadcda706b2b7c8e8a1f7c01f595ea3287582f6b0a225912f2a23bb17635

protocol
ad80ef997a09f15c074b947501ea177a7d2210fa27aabc22505bc56a33fa31fc
```

## Targeted E2B validation

Reusable sandbox `ileplicyovfi2buycb8zz` ran only:

- evaluator formatting;
- targeted evaluator release check;
- strict evaluator Clippy with `-D warnings`;
- default-feature core release check;
- shell-audit syntax and frozen enum-shape checks;
- the preregistered semantic-routing and damping source scans.

No physical world or matrix ran.

Before this freeze, one evaluator control was corrected because its ordinary
Drive input also fired the downstream effect and therefore produced a genuine
Modulatory consequence. The corrected control contains no consequence path and
tests Drive alone. Two mechanical compile errors (one CSV placeholder and one
moved value) were also corrected. None of these pre-freeze corrections executed
a world or changed the candidate substrate law.

## Boundary

PQLC0 changes no pressure law, durable learning, eligibility deletion, ARC,
authority, oracle, or `arch.md`. It makes no arbitrary-depth claim. No rescue,
parameter tuning, comparator repair, trace consumption, attenuation, or cycle
stop is authorized after evidence begins.
