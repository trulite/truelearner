# RC0a pre-definitive implementation audit

Status: implementation and source audit completed before any definitive RC0a
command. No RC0a result CSV or Markdown artifact existed when this audit was
written.

## Frozen boundaries

- Parent outcome tag: `rg0a-reflected-grounding-functional-positive`.
- Parent outcome commit:
  `c690fad5e9defccaae69f3920ee24e74a1f7ee37`.
- RC0a protocol tag: `rc0a-compiled-grounded-recurrence-protocol`.
- RC0a protocol commit:
  `2ff4200261ec9b6da2a5cd0eb082eda7efe82fa7`.
- RC0a protocol SHA-256:
  `e641aa2367e5bf504b28b22fb84268e69092577b9b6efdf5903d924b990dd2a4`.

Implementation hashes before this audit document:

- `src/reflected_program_discovery/grounding.rs`:
  `3ba62a23adc36e58a68836eced7a88c571c75027f318b515ba5f7f9fb3cf6bef`;
- `src/reflected_program_discovery/grounding/compiled_recurrence.rs`:
  `3274fbf2380c0db457b585d03e7ac0476daf393f3910e2ec37f0b65d3513ab55`;
- `src/bin/compiled_grounded_recurrence.rs`:
  `2ed65a9ca3afb9c29942e904a12b84b88a4979a45adeee0b3e48cb2845c05b7a`.

The only shared RG0a changes are:

- declaration of the RC0a child module;
- two zero-default compiled-arrow work counters;
- a `LocalGroundArrow { id, destination }` temporary arrow type;
- a `GroundRouter::Compiled` arm in the existing `route_spike` function;
- collection of compiled-arrow identities from that arm;
- serialization of the two new counters.

Existing direct and reflected router arms and all lower cell physics are
unchanged.

## Persistent-state audit

The only persistent fast-path item is `CompiledRoleArrow`:

```text
id: usize
parent_arrow: usize
from_role: usize
to_role: usize
strength: i32
```

It has no `LowerLocation`, concrete identity, episode identity, answer, depth,
relation, operation label, callback, or fragment boundary. A compiled arrow is
created only after the same parent reflected arrow appears in three separately
successful generic grounded episodes and receives the unchanged RP0a
`+2/+2/+2 -> 6` consolidation trajectory.

Used reflected arrow identities are recorded as a set, so recurrence within one
episode cannot provide multiple compilation credits. Every required transition
therefore needs three distinct successful episodes.

The shuffled-evidence control rotates observed target roles before
consolidation. Its persistent arrows are structurally incompatible with their
claimed parent arrows and are invalidated before firing.

## Episode-relative binding audit

Concrete destinations occur only in `TemporaryCompiledRoutes`, constructed for
one invocation after ordinary RG0a provenance recognition and fresh binding.
Installation performs this join for each earned arrow:

```text
persistent source role -> current bound source CELL
persistent target role -> current bound target CELL
```

The temporary arrow is indexed by the current physical source CELL and stores
the current physical target CELL. It is erased after execution. Persistent
compiled fingerprints are checked before and after fresh and permuted-binding
evaluation.

The changed-binding arm deterministically permutes all physical lower CELL
positions while retaining their current structural observations. The same
persistent compiled state must reinstall and answer correctly. Consequently a
path keyed by `source-role -> fixed cell index` or a cached lower identity fails
this arm.

## Single-executor audit

RC0a does not define a second lower executor. Direct, generic reflected, and
compiled dispatch all enter the unchanged `run_cell_machine` queue loop.
`GroundRouter::Compiled` performs exactly one local arrow evaluation and one
local arrow firing at each preexisting `RouteSource` activation, then enqueues
an ordinary lower spike. Lookup, feedback, continuation, finish, relation
scans, current updates, answer emission, quiescence, and activity limits all use
the same existing code.

The compiled mature arms require zero:

- reflected arrow evaluations or firings;
- per-step binding reads or deliveries;
- direct arrow evaluations or firings;
- direct executor, pre-resolved persistent route, fallback, or oracle calls;
- generic resumptions.

They also require their lower-effect counters to match the concrete and generic
branches for identical episode state. The permuted arm is compared with a
permuted concrete diagnostic because physical search order legitimately changes
CELL-location comparison counts.

## Invalidation audit

The invalidation arm replaces every consolidated learned arrow identity while
preserving its role-relative endpoints. Before temporary route installation,
the compiled set scans current learned structure for its parent identities.
Failure invalidates the entire four-arrow fast path before any compiled firing.
The current generic RG0a path then executes the replacement learned topology
and must remain fully correct.

This condition consumes no answer, evaluator role, target location, or
correctness callback. The invalidation evidence is the local absence of the
compiled arrow's parent structure. A changed concrete binding alone does not
invalidate.

## RC0b exclusion audit

Search of the implementation confirms:

- no persistent item stores more than one parent arrow;
- no fragment start/end, macro length, skipped-step count, depth key, or
  residual-effect program exists;
- expected depth is used only by the harness to generate episodes and audit the
  frozen `3*d+2` route count;
- every lower CELL activation and effect remains present;
- no work-elimination or economic-advantage gate exists;
- RC0b source or result artifacts do not exist.

RC0a compiles dispatch only.

## Development evidence

Development evidence is non-claim and writes no result artifact.

MICRO passed:

- concrete, generic, compiled, changed-binding, invalidation, subthreshold, and
  shuffled-evidence behavior matched their specified paths;
- no-bindings produced `0/4` correct;
- generic per-episode excess slope was exactly `3.0` work per route firing;
- compiled per-episode excess slope was exactly `0.0`;
- every qualitative gate passed.

Full GATE passed:

- CONCRETE `12/12`;
- GENERIC GROUNDED `12/12`;
- COMPILED GROUNDED `12/12`;
- CHANGED BINDINGS `12/12`;
- INVALIDATED TRANSITION `12/12` through generic resumption;
- SUBTHRESHOLD `12/12` with zero compiled arrows;
- SHUFFLED EVIDENCE `12/12` through pre-fire invalidation and generic
  resumption;
- NO BINDINGS `0/12`;
- generic slope `3.0`, compiled slope `0.0`;
- workspaces `271/271` destroyed, maximum live per cell `1`;
- all 13 development gates passed.

The development fixture contains only the four correct outgoing arrows, so its
generic slope is smaller than frozen definitive RG0a's mixed-candidate slope.
This cannot favor the ratio gate: RC0a compares generic and compiled work inside
the same matrix, and the exact compiled slope is independent of the number of
outgoing reflected candidates.

## Verification

- `cargo fmt --all -- --check`: pass.
- `cargo clippy --all-targets --all-features -- -D warnings`: pass.
- targeted RC0a tests: `2/2` pass.
- MICRO: pass, development only.
- GATE: pass, development only.
- One full shared-boundary regression:
  - library: `162` passed, `0` failed, `2` intentionally ignored E2B-only
    tests;
  - main binary: `10/10` passed;
  - reviewer API: `4/4` passed;
  - all remaining binary targets contained zero tests and passed.

The complete regression was run once because the original RG0a router gained
the ordinary compiled-arrow arm. It was not repeated during experiment-local
development.

## Remaining boundary

The definitive runner refuses to overwrite existing artifacts. The next
claim-eligible action is the single `--definitive` command after this audit and
implementation are committed and tagged. That command has not been run.
