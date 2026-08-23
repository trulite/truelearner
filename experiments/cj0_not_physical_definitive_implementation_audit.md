# CJ0-NOT physical definitive implementation audit

Status: **BOTH IMPLEMENTATIONS FROZEN; BOTH DEFINITIVE EVIDENCE SETS UNSPENT; PX2 REMAINS AUTHORITATIVE**.

## Frozen sources

| item | SHA-256 |
|---|---|
| authoritative PX0 substrate source | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| authoritative PX2 runner | `c47d605371d5787cffc7d456f1d9e38168b4b203063fb9dcdeefcf630fa4aed5` |
| authoritative PX2 definitive CSV | `921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18` |
| NOT-1 definitive protocol | `35b2140b6422a0769581c06c43deb921ec2a8d6d22e127bdabe6638468f329ba` |
| NOT-2 definitive protocol | `376afddf1d307fe1de645bc07782d469cea1b3b4145869a31dd368c7eb879f23` |
| shared definitive evaluator/fixture source | `a05a920ed77d8ae8dab763663675f44bc13d30fd90b12e4dd0046dd2451d06c7` |

The two tracks share only a frozen executable source and generic publication /
hash helpers. They have disjoint physical fixtures, namespaces, commands,
matrices, pass clauses, staging paths, final artifacts, evidence markers, and
classifications. Running either track does not execute or publish the other.

## Source and leak audit

The executable imports only the authoritative public `PlasticSubstrate` API.
It defines evaluator fixture/observation types but no substrate CELL, ARROW, or
SPIKE replacement and no new law. Organism-visible execution is entirely the
existing substrate's cells, arrows, signed coupling, transient CELL state,
time/decay, threshold firing, pressure, generation invalidation, and natural
queue drain.

The evaluator chooses only preregistered input schedules before execution; it
never reads state to select a physical branch. All output expectations and
scenario names are applied after propagation. NOT-2 uses identical role graph
and closure across worlds and has no absence symbol, timeout label, or new
persistent variable. NOT-1 has no logical complement primitive. PX3 source and
results are neither read nor interpreted.

## Freshness, replay, work, and storage

Each track uses `112` identities absent from development evidence and from the
other track. Sixteen seeds rotate mirrors, allocation order, ARROW insertion,
spacing, external phases, and origin identities. Each complete state is cloned
before execution and run twice. Rows separately serialize signed arrivals,
role firings, output timing, pressure/deallocation, quiescence, work, storage,
complete/permanent fingerprints, and duplicate equality.

NOT-2 additionally serializes initial and post-trigger complete fingerprints
after a naturally quiescent trigger-only propagation. No persistent allocation
occurs between those fingerprints.

## Authoritative-byte audit

Direct diff of the PX0 source, PX2 runner, and PX0/PX1/PX2 definitive CSVs
against `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129` is empty. Their hashes remain
exact. All repository changes are new CJ0-NOT fixture/evaluator, protocol,
audit, or result paths.

## Focused validation

Before either evidence run, all passed:

```text
cargo fmt --all -- --check
cargo check -p px0-physical-correspondence --example cj0_not_physical_definitive
cargo clippy -p px0-physical-correspondence --example cj0_not_physical_definitive -- -D warnings
cargo run --release -p px0-physical-correspondence --example cj0_not_physical_definitive -- --preflight-not1
cargo run --release -p px0-physical-correspondence --example cj0_not_physical_definitive -- --preflight-not2
cargo run --release -p px0-physical-correspondence --example cj0_not_physical_definitive
git diff --check
```

Both preflights entered no cell and confirmed their own staging/final paths
absent. The unflagged invocation refused with exit `2`. PROBE artifacts remain
immutable. MICRO/GATE were not warranted after both isolated PROBEs resolved
their specified mechanism and control boundaries without ambiguity; the
definitive matrices instead use wholly fresh identities and expanded timing /
layout strata.
