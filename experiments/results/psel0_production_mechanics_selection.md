# PSEL0 Production Mechanics Selection Result

## Outcome

PSEL0 selects the following production mechanics composition:

```text
TimingWheel + Adjacency + Frontier + AoS + opportunistic Batched
```

It is named `MechanicalConfig::PRODUCTION`.

The permanent correctness oracle remains:

```text
Vec + GlobalScan + FullScan + AoS + Scalar
```

It remains named `MechanicalConfig::REFERENCE`. PSEL0 neither removes it nor
advances organism authority.

## Physics gates

- Frozen accepted corpus: `16` worlds × `5` sequential prefixes = `80/80`
  differential pairs.
- Accepted behavioral clauses: `536/536`.
- PSEL0 stress corpus: `8` worlds × `4` candidates = `32/32` comparisons.
- Each stress candidate was repeated three times with exact complete
  observation equality.
- Natural quiescence and canonical final durable bodies matched throughout.
- Workspace tests: `17/17`.
- Strict Clippy with warnings denied: passed.

The stress families covered many cells, dense layered activity, long delays,
many same-tick arrivals, high fan-out, heavy modulation, a mostly dormant
graph, and live zero-delay topology.

## Aggregate mechanical costs

| Candidate | Median elapsed sum (ns) | Queue ops | Comparisons | Scans | Logical allocations | Logical bytes touched | Maximum resident bytes | Zero-delay fallbacks |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| A — AoS scalar | 323,855,089 | 17,852 | 10,222,434 | 22,918,772 | 37,019 | 3,271,351,360 | 1,774,936 | 0 |
| B — AoS batched | 236,838,425 | 8,926 | 261,632 | 22,886,516 | 27,593 | 1,834,673,440 | 1,774,936 | 1,024 |
| C — SoA scalar | 889,194,113 | 17,852 | 10,222,434 | 22,918,772 | 37,019 | 3,271,351,360 | 1,740,120 | 0 |
| D — SoA batched | 631,764,168 | 8,926 | 261,632 | 22,886,516 | 27,593 | 1,834,673,440 | 1,740,120 | 1,024 |

CPU time is diagnostic, not deterministic evidence. All other counters use a
frozen logical accounting contract. `bytes_touched` counts physical runtime
records read or written; `allocations` counts runtime vector/bucket allocation
events. They are no longer placeholder zeros.

## Selection reasoning

TimingWheel, Adjacency, and Frontier remain selected from R1–R3.

AoS wins the resident-layout decision. SoA saved about two percent of maximum
resident capacity but was about 2.7 times slower in aggregate. The result does
not invalidate SoA; it proves layout replaceability and declines to pay its
current accessor cost.

Exact batching is enabled opportunistically. Relative to AoS scalar it reduced:

```text
queue operations       50.0%
logical allocations    25.5%
logical bytes touched  43.9%
```

When any live zero-delay ARROW can create new current-tick work, batching falls
back to scalar order. The zero-delay workload recorded 1,024 such fallbacks,
remained exact, and incurred no material elapsed regression.

SIMD was neither implemented nor selected.

## Measurement-boundary correction

The first consolidated validation exposed three restart/compaction predicates
that compared the whole `RunResult`, including the newly measured peak resident
capacity. Different valid packing/checkpoint histories can have different
mechanical capacity while producing identical organism history.

Those predicates were repaired to compare crossings, physical work, clock,
canonical durable body, and quiescence. The affected tests then passed, and the
full differential returned to `80/80`, `536/536`. No physical law or workload
was changed.

## E2B provenance

- Instrumentation compile check: `im690yr5d0p8cqr0cm09z`
- Stress corpus: `itep0sv22zd4wtcq3ls21`
- Strict lint/workspace tests: `inewifieyi6881y3yqdrs` (later comparator stage
  stopped on the measurement-boundary issue)
- Repaired targeted restart/compaction tests: `i80st8wlt9mu0tzdkwp8f`
- Repaired full differential: `igw2jbcoxzplgj28zqj7m`

No Rust command ran locally.

## Frozen hashes

```text
core lib.rs       838b6e350709650afb5b292f4c6706c6b5a1e5c064ccf0c63c7aec2e2b0748af
mechanics.rs      6be7c4a4f74929123c66ae0780f65181ae6be7362a8259184c83b89e0f694f18
cost CSV          65bbd8b161399c223edf2f75a73be49df957bcd793c9b4f4c6d522386d6319d7
stress report     d931278a71afc6f9a21c33cb4522456e4c965f37be12022e238f7bafe07d1757
```

## Boundary

This is an engineering selection, not a new physical-law authority claim. R6
must begin on a separate branch and first prove one arena versus an equivalent
zero-added-latency partitioned body before introducing residence latency.
