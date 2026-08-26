# Remove unused Academy S3 infrastructure

```text
Academy run -> local episode evidence -> Playground

                         no AWS edge
```

## Outcome

The living Academy workspace contains no S3 crate, AWS SDK dependency, remote
publication worker, S3 smoke binary, configuration, or S3 documentation. Academy
continues to generate and review local evidence exactly as before.

Archived research records keep their historical `academy-storage` statements.
This deletion changes no organism behavior, Academy run, episode layout, or
Playground behavior.

## Authority

- Path: `academy.md`; `arch.md` Boundaries; `academy/README.md`;
  `/Users/satya/work/br/AGENTS.md`
- Revision: commit `7bf0fbcd878f28ee2a8d69d953c25736767e9341` plus the validated
  `plans/playground-thin-media.md` candidate;
  `academy/Cargo.toml` SHA-256
  `fbb0579db9ef942d19610f47634be5af5f303a305ff0827558aae989d98107e9`;
  `academy/crates/academy-storage/src/lib.rs` SHA-256
  `fc3e431506a297677d49c69df0d3ab49971ee257501bb1b17fb3d1920c56581b`

## Model

- The living workspace is the composition of headless Academy, episode review,
  the runner, and the explicitly selected Playground.
- `academy-storage` is an isolated unused object: no living crate has an edge to
  it and it has no effect on local evidence.
- Removal deletes that object and its AWS-only dependency closure. All remaining
  transformations and effects compose as before.
- Local filesystem evidence remains the only current persistence effect. A
  future remote adapter requires a new explicit design and dependency edge.

## Invariants

- Headless runs preserve inputs, outputs, work, checkpoints, replay, body
  fingerprints, and natural quiescence.
- Episode catalogs, records, frames, posters, and videos remain unchanged.
- Playground remains causally inert and depends only on `academy-review`.
- No living source, manifest, lockfile package, or living Academy document names
  `academy-storage`, `academy_storage`, an AWS SDK crate, or `ACADEMY_S3_*`.
- Archived experiment records remain byte-for-byte unchanged.
- The representative warm regression remains strictly under 10 seconds.

## Scope

- Delete `academy/crates/academy-storage`, including its library and S3 smoke
  binary.
- Delete `academy/docs/s3_storage_v1.md`.
- Remove the crate, AWS SDK dependencies, and storage-only dependencies from
  `academy/Cargo.toml`; regenerate `academy/Cargo.lock` through Cargo.
- Remove live storage descriptions and dependency arrows from
  `academy/README.md`.
- Update the broader Academy performance plan to assume no remote-storage lane.
- Delete the generated `academy/target` cache so stale compiled AWS artifacts no
  longer occupy disk. The first verification build is intentionally cold; later
  builds repopulate only the remaining dependency graph.

Exclude local episode evidence, Playground changes, archived research, organism
physics, Academy semantics, and unrelated cleanup.

## Development style

Implementation-first. The crate has no consumers, so delete the isolated files
and let metadata, lockfile, compile, tests, and source scans prove the absence and
preservation boundary.

## Focused tests

- `python3 -c "from pathlib import Path; roots=[Path('academy/Cargo.toml'),Path('academy/Cargo.lock'),Path('academy/README.md'),Path('academy/docs'),Path('academy/crates')]; terms=('academy-storage','academy_storage','aws-config','aws-sdk-s3','ACADEMY_S3_'); bad=[(str(p),term) for root in roots for p in ([root] if root.is_file() else root.rglob('*')) if p.is_file() for term in terms if term in p.read_text(errors='ignore')]; assert not bad,bad"`
- `cargo metadata --locked --manifest-path academy/Cargo.toml --format-version 1 --no-deps`
- `cargo check --locked --manifest-path academy/Cargo.toml`
- `cargo check --locked --manifest-path academy/Cargo.toml -p academy-playground`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-review -p academy-episodes --lib`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-core -p academy-arc3 --lib`
- `cargo fmt --all --manifest-path academy/Cargo.toml -- --check`
- `cargo clippy --workspace --all-targets --locked --manifest-path academy/Cargo.toml -- -D warnings`

These establish complete living-source removal, a valid dependency graph and
lockfile, preserved headless and episode behavior, explicit Playground
compilation, formatting, and lint cleanliness.

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path academy/Cargo.toml -p academy-core -p academy-arc3 --lib`

Pre-change warm baseline: `1.19 seconds`, strictly under 10 seconds. The measured
clean runner check was `8.67 seconds` and `53 MiB`; the current default including
AWS was `26.58 seconds` and `262 MiB`. Record cold bootstrap separately.

## Controls and evidence

- Held-out cases: run the ignored Core and ARC3 adversarial tests without
  changing assertions or ignored reasons.
- Negative controls: the living-source scan must fail if any S3 crate,
  dependency, environment variable, or documentation is restored.
- Laws: removing an unreachable dependency preserves every reachable Academy
  transformation; repeated local evidence generation remains exact.
- Falsifiers: any changed physical result, episode result, archived record,
  remaining live AWS edge, invalid lockfile, or warm regression at or above 10
  seconds rejects the candidate.
- Evidence: validated plan, candidate receipt with exact checks, Cargo metadata,
  dependency/source scan, and independent verification receipt.
- Not applicable because this deletes unused engineering infrastructure: do not
  create a research arm or run a frozen authority evaluator.

## Risks and rollback

- A hidden consumer could rely on the crate outside Cargo metadata; scan all
  living source and manifests before deletion and compile every remaining
  workspace target afterward.
- Lockfile editing can retain orphaned AWS packages; let Cargo regenerate it and
  assert the package names are absent.
- Historical claims can be accidentally rewritten; exclude `experiments/archive`
  and verify its working-tree paths are untouched.
- Roll back by restoring the deleted crate, document, workspace entries, and
  lockfile from the accepted parent; local evidence needs no migration.

## Open decisions

None.
