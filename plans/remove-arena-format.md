# Remove the arena format boundary

```text
Academy / tests
      | send inputs
      | read junction/link observations
      | save / restore opaque checkpoint
      v
Harness
      |
      v
private Body -> private resident Arena

deleted: public ArenaBody / BodyVersion / cell-arrow format crate
```

## Outcome

Delete `truelearner-arena-format` and remove arena/body persistence types from
the public contract. The harness exposes only input, output, clock, junction,
link, path-return, work, trace, and opaque checkpoint operations using the
vocabulary in `LANGUAGE.md` and the transitions in `algo.md`.

This is a pre-release reset. Old arena blocks, body manifests, content hashes,
and checkpoint bytes need not decode after the change. Keep an opaque checkpoint
byte envelope only because Academy currently persists checkpoints for exact
replay; it is owned by Core and exposes no body or arena representation.

Arena partitioning, foveation, and a future harness foveation boundary are
separate design work and gain no API or persistence commitments here.

## Authority

- Path: `LANGUAGE.md`; `algo.md`; `arch.md` sections 1–3;
  `academy.md`; `/Users/satya/work/br/AGENTS.md`
- Revision: commit `4b0309ac200f50e3b43ebddd60a1195877c08f07` plus the
  uncommitted validated harness-boundary candidate; `LANGUAGE.md` SHA-256
  `2b1954b161358c4a259198b0b9e4c66a93e47350d749d7c3baf3ddcef7bb8a41`;
  `algo.md` SHA-256
  `62363a087f660caa5ea6418fc0dd1c85195ebf0a0745d19ee58894b3a160224b`;
  `arch.md` SHA-256
  `a2ddfc631bd2a3472503d9a44345f738e573d84685b5cbcf1b8994ae72fb6a6d`;
  `academy.md` SHA-256
  `445b94767b59bbe7f11054f9f77330d7ee4286246d4d1c566a5fe8e38227efdb`

## Model

- `JunctionId` and `LinkId` are Core-owned public identities. Generation is a
  private resident safety value. No public identity contains an arena.
- `HarnessBuilder` constructs a private body from capacities and outward region;
  it does not accept an `ArenaId`.
- `Harness` uniquely owns the live body. Its transformations remain `send`,
  `advance_to`, `read`, `save`, and `restore`.
- `read()` returns an owned `HarnessObservation` without a caller-supplied
  version. It contains clock, protocol, return-path count, resident byte count,
  and language-level link observations. Each link observation contains stable
  identity, endpoints, delay, phase, mode, current strength, current resistance,
  participation, and liveness. Add a junction observation only if an existing
  Academy or test query cannot be expressed from links and known junction IDs.
- Observation lookup and fingerprinting are pure. A deterministic observation
  fingerprint may hash explicit observation fields for Academy evidence, but no
  encoded body value or decoding API exists.
- `save()` produces an opaque `Checkpoint`; `restore` consumes one. Core's
  private checkpoint snapshot directly contains the structural and transient
  state needed for exact continuation. `canonical_bytes` and `decode` remain the
  sole byte boundary needed by Academy replay, reset to the new pre-release
  schema with no backward reader.
- The private resident `Arena` remains an implementation detail until the
  separate arena/foveation design. Its indexes and slots never cross `read`,
  checkpoint inspection, or Academy imports.

## Invariants

- The `algo.md` sequence—fire, form, choose, output, return, strengthen, reuse—
  and all physical outputs, work, traces, pressure timing, and quiescence remain
  unchanged.
- Academy and external tests can neither name `Body`/`Arena` nor obtain an
  `ArenaBody`, resident slot, mutable reference, or persistence DTO.
- Repeated reads are equal and causally inert; inserting a read does not change
  the next run or checkpoint.
- Link observations report current physical values and topology without
  permitting mutation. Academy diagnostics and tests use only those values.
- Save/decode/restore preserves complete continuation state and rejects corrupt,
  truncated, and trailing checkpoint bytes.
- No production or Academy manifest depends on `truelearner-arena-format`; the
  crate and its workspace membership are absent.
- Inputs remain anonymous physical arrivals and evaluator-only meaning remains
  outside the organism.
- No arena partition, foveation, framebuffer, or semantic port behavior is
  introduced by this removal.

## Scope

- Delete `truelearner/crates/arena-format` and remove it from both workspace
  manifests and lockfiles.
- Add Core-owned `JunctionId` and `LinkId`; keep generation and resident
  references private. Remove `ArenaId`, `CellId`, `ArrowId`, `ArenaBody`,
  `DurableCell`, `DurableArrow`, `BodyVersion`, `ArenaVersion`, `ContentHash`,
  and `FormatError` from active production code.
- Remove `truelearner/crates/core/src/format.rs`. Rewrite Core snapshots and
  checkpoints to capture the private structural and transient state directly,
  with one opaque pre-release checkpoint schema.
- Change `HarnessBuilder::with_capacity` to omit arena identity; change
  `Harness::read` to omit body version and return enriched link observations,
  pure fingerprint, and resident-byte count without a body field.
- Migrate Core integration tests, `academy-core`, and `academy-arc3` to
  `read().link(s)`, Core-owned IDs, local SHA-256 where non-body byte hashing is
  still required, and the opaque checkpoint API.
- Update current README and architecture descriptions so active documentation
  names junctions/links and the harness boundary. Preserve frozen experiment
  records and archived evidence unchanged.

Exclude physical-law changes, arena partition design, foveation, harness
foveation, buffer changes, storage redesign, Academy curriculum changes, frozen
experiment code, and backward compatibility for pre-release bytes.

## Development style

TDD. First rewrite the external harness tests so they compile only against
language-level observations and add source/manifest negative controls. Then
remove the format crate and implement the smallest Core snapshot, checkpoint,
identity, and Academy migrations needed to restore the unchanged behavior.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --test harness_boundary`
- `cargo test --locked --manifest-path truelearner/Cargo.toml -p truelearner-core --test harness_boundary -- --ignored`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-core --lib`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-arc3 --lib`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-arc3 --lib -- --ignored`
- `cargo test --workspace --locked --manifest-path truelearner/Cargo.toml`
- `cargo test --workspace --locked --manifest-path academy/Cargo.toml`
- `cargo check --workspace --locked --manifest-path truelearner/Cargo.toml`
- `cargo check --workspace --locked --manifest-path academy/Cargo.toml`
- `cargo fmt --all --manifest-path truelearner/Cargo.toml -- --check`
- `cargo fmt --all --manifest-path academy/Cargo.toml -- --check`
- `cargo clippy --workspace --all-targets --locked --manifest-path truelearner/Cargo.toml -- -D warnings`
- `cargo clippy --workspace --all-targets --locked --manifest-path academy/Cargo.toml -- -D warnings`
- `python3 -c "from pathlib import Path; roots=[Path('truelearner'),Path('academy')]; bad=[str(p) for r in roots for p in r.rglob('*') if p.is_file() and ('target' not in p.parts) and ('truelearner_arena_format' in p.read_text(errors='ignore') or 'truelearner-arena-format' in p.read_text(errors='ignore') or 'ArenaBody' in p.read_text(errors='ignore') or '.read(0).body' in p.read_text(errors='ignore'))]; assert not bad, bad"`

These commands establish unchanged Core and Academy behavior, held-out behavior,
checkpoint continuation and rejection, complete dependency removal, formatting,
and strict lint cleanliness.

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path academy/Cargo.toml -p academy-core -p academy-arc3 --lib`

Pre-change warm baseline: `1.14 seconds`, strictly under 10 seconds. Record cold
bootstrap separately and reject a candidate whose warm run reaches 10 seconds.

## Controls and evidence

- Held-out cases: unchanged ignored Core boundary tests and ARC3 boundary tests.
- Negative controls: source and manifest scan rejects the removed crate,
  `ArenaBody`, and observation-body access; corrupt/truncated/trailing checkpoint
  bytes remain rejected; unsupported outcome and distant-output controls remain
  silent.
- Laws: read identity, read-before-send preservation, checkpoint continuation,
  stable junction/link identity, and observation lookup consistency.
- Falsifiers: any changed run output, work, trace, pressure phase, replay verdict,
  natural quiescence, diagnostic value, public arena/body escape, retained format
  dependency, or warm regression at or above 10 seconds rejects the candidate.
- Evidence: validated plan, factory candidate receipt with exact tree digest and
  checks, and independent verification receipt tied to the candidate digest.
- Not applicable because this is a pre-release representation simplification:
  no format migration, successor research arm, or frozen authority execution is
  required.

## Risks and rollback

- Rebuilding snapshots can omit causal state; exact checkpoint continuation and
  read-before-send laws detect omissions.
- Replacing durable-body inspection can accidentally expose stale or differently
  scaled link values; observation lookup tests and existing Academy diagnostics
  detect this.
- Removing arena identity can perturb fingerprints only; pre-release fingerprint
  resets are accepted, while within-version replay equality remains required.
- Deleting a workspace crate can leave lockfile or transitive imports; full
  workspace checks and the negative source scan detect residue.
- Roll back only the removal candidate while retaining the already validated
  harness-boundary work; no released bytes or data migration must be restored.

## Open decisions

None.
