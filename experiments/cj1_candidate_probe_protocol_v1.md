# CJ1 path-local saturation candidate PROBE protocol v1

Status: **PREREGISTERED; EVIDENCE UNSPENT**.

## Frozen inputs and scope

- branch: `research/cj1-distinct-path-coincidence`;
- parent result: positive shared-path correction at `165bd73bcfbcf274ebca319d426e7752817b2c86`;
- authoritative PX0 law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- parent development protocol SHA-256:
  `8c31387f3337c3ad38d83e030dd6a43d4fce8f2e146d93596c9c7231e8b8a6ad`;
- seeds: `2101, 2111`;
- layout: normal;
- local threshold: `2`;
- incoming couplings: `1, 2` as fixed by the named scenario;
- background load: `0`.

This is development PROBE only. It cannot create definitive, authority,
PX3-restart, MICRO, GATE or PX-C evidence. The authoritative PX0 crate and all
earlier protocols/results remain byte-exact and read-only.

## Sole mechanically frozen candidate

The isolated candidate is generated from the exact authoritative PX0 source.
Exactly one traversal block is changed and no stored field, object or identity
surface is added:

1. immediately before an actually traversed live ARROW rewrites its existing
   `eligible_until`, read whether that same value is live at the traversal tick;
2. for a destination CELL whose threshold is greater than one, enqueue local
   contribution `1` when the path trace was not live and `0` when it was live;
3. for a threshold-one destination, enqueue the ordinary ARROW coupling;
4. rewrite `eligible_until` through the existing path exactly as PX0 does;
5. leave ordinary return, expiry, pressure, strengthening, proposal,
   deallocation and all other CELL/ARROW/SPIKE law byte-for-byte unchanged.

The carried ARROW crossing remains serialized with its physical coupling; the
receiving CELL trace serializes the bounded or suppressed local contribution.
An implementation-time source replacement count other than exactly one, a
frozen hash mismatch, or any second law edit refuses execution.

## Fixed matched rows

For each seed, execute in this order:

1. two unit firings through one physical path;
2. one impulse-2 firing through one physical path;
3. two unit firings through two different traversed paths;
4. two different paths with the second outside the physical window;
5. two different paths together inside the window;
6. four immediate firings through one path;
7. one-path burst plus one different path;
8. one path plus a burst on the other path;
9. three different paths;
10. one mature coupling-2 path;
11. fresh coupling-1 paths together;
12. one external source through two physical paths;
13. two external sources through one shared physical path.

Each row starts from fresh identities. Each row is executed twice from the
same fresh construction and requires exact equality. The result independently
records entered impulse, source firing, actual crossings into the locus,
eligibility writes and closes, local arrival count and contribution sum, local
firing/effect, held-out effect, work, storage, deallocation, fingerprints,
quiescence and replay.

## Conjunctive acceptance and stop rule

The expected effect is one only for genuine distinct-path participation inside
the window: rows 3, 5, 7, 8, 9, 11 and 12. Every one-path count/amplitude or
late/shared-path substitution must yield zero. Every row must be naturally
quiescent and exact-replay equal.

PROBE passes only if all `26` seed/scenario rows pass. Any scientific row
failure freezes the earliest ordered failure and stops before MICRO. In
particular, if ordinary returned-activity handling clears the sole existing
path trace before an immediate repeated traversal, that observed interaction
is a scientific failure; closure semantics may not be changed or supplemented.

## Command and write-once artifacts

The sole evidence command is:

`cargo run --manifest-path arms/cj1-distinct-path-coincidence/Cargo.toml --bin cj1_candidate_probe --release -- --candidate-probe`

It may create only:

- `results/cj1_candidate_probe_v1.csv`;
- `results/cj1_candidate_probe_v1.md`;
- matching hidden `.staging` files during atomic publication.

The runner refuses existing destination/staging paths and refuses a dirty or
hash-drifted source snapshot before evaluation. After publication, schema,
row count/order/uniqueness, exact result hashes, focused tests, strict Clippy,
staging-remnant absence and remote parity are audited before the result is
frozen. Execution occurs in the established E2B persistent sandbox only.
