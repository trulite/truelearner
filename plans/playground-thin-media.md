# Make Playground load only what a person is viewing

```text
catalog.json ──load once──> episode cards
                                  |
                                  | select
                                  v
                         one video + one poster
                                  |
                                  | explicit download
                                  v
                             record.json

playground -> academy-review <- academy-episodes -> academy-core
```

## Outcome

Playground opens from the small episode catalog without reading or encoding every
media file. Posters are browser-lazy, only the selected video is requested, and
records are requested only when a person follows the download link. A read-only
custom URL protocol serves files below the configured episode root and honors a
single HTTP byte range so native webviews can seek without loading a whole video
when they provide range headers.

Playground remains a Dioxus Desktop review surface. This change does not alter
episode generation, evidence, physical runs, the harness, or TrueLearner.

## Authority

- Path: `academy.md`; `arch.md` Boundaries; `academy/README.md`;
  `academy/docs/episode_review_v1.md`; `/Users/satya/work/br/AGENTS.md`
- Revision: commit `7bf0fbcd878f28ee2a8d69d953c25736767e9341`;
  `academy.md` SHA-256
  `1eacce8ff35f7a8e3e8a660ec49d91f4639d19e7d81d6548494bb8d8990860b0`;
  `academy/docs/episode_review_v1.md` SHA-256
  `5d8e9a54bfa1d3cb27ac8b92b1521a8b8a1360c4e69dbfe8ac216a4c305e33cf`

## Model

- `academy-review` owns the serialized review objects: episode class, outcome,
  frame, episode, catalog, and catalog-loading failure. These are causally inert
  Academy observations with no dependency on the organism.
- `academy-episodes` transforms headless Academy evidence into review objects and
  derived files. It depends on both `academy-core` and `academy-review` and
  re-exports the review vocabulary for existing generator consumers.
- Playground transforms a loaded catalog into selection and filter state. Its
  startup effect reads only `catalog.json`; it does not transform or validate the
  physical evidence.
- A relative catalog media path maps to an `academy-media` URL. The protocol maps
  that URL to a canonical file below one configured root and then to a complete
  or partial byte response. Missing, malformed, escaping, unsupported, and
  unsatisfiable requests become typed HTTP responses rather than path access.
- File I/O is confined to catalog loading and protocol requests. Browser media
  demand controls when posters, videos, and records are read.
- Workspace membership and default build membership remain separate: Playground
  stays a workspace member but leaves `default-members` so headless development
  does not compile the desktop stack.

## Invariants

- The episode catalog and record JSON shape, field names, schema version, ordering,
  labels, and generated media bytes remain unchanged.
- Review interactions and media requests never enter the harness or organism.
- Playground has no dependency path to `academy-core` or `truelearner-core`.
- The media protocol serves only existing regular files whose canonical path is
  below the canonical episode root; absolute paths and parent traversal fail.
- A valid range response preserves the requested file bytes and reports exact
  `Content-Range` and `Content-Length` values. Invalid or unsatisfiable ranges
  return 416 with no file bytes.
- The player requests only the selected video and uses metadata preload; gallery
  images use native lazy loading; a record remains an ordinary on-demand link.
- Normal Academy checks do not build Dioxus through workspace default members.
- Catalog or media failure changes only the review UI and cannot change evidence.

## Scope

- Add `academy/crates/academy-review` for the portable review schema and catalog
  loader.
- Update `academy-episodes` to consume and re-export that schema without changing
  generation.
- Replace Playground data URLs and eager media reads with catalog-derived custom
  URLs, asynchronous catalog state, lazy images, metadata-only video preload, and
  a confined byte-range protocol handler.
- Update Academy workspace dependencies/default members and relevant human-facing
  dependency and loading documentation.
- Add focused schema and media-handler tests and factory receipts.

Exclude visual redesign, gallery virtualization, adjacent-item prefetch, episode
format changes, video generation changes, remote serving, persistence, Academy
semantics, harness behavior, organism physics, and replacement of Dioxus/Wry.

## Development style

TDD. Add portable catalog tests and protocol tests for full reads, ranges,
traversal, and unsatisfiable ranges; then move the schema and replace eager media
loading until those tests and the existing generator checks pass.

## Focused tests

- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-review --lib`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-playground --bin academy`
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-episodes --lib`
- `cargo check --locked --manifest-path academy/Cargo.toml -p academy-playground`
- `cargo check --locked --manifest-path academy/Cargo.toml`
- `cargo fmt --all --manifest-path academy/Cargo.toml -- --check`
- `cargo clippy --workspace --all-targets --locked --manifest-path academy/Cargo.toml -- -D warnings`

These establish portable catalog loading, exact bounded media reads, path
confinement, unchanged generation, desktop compilation, default-member
compilation, formatting, and strict lint cleanliness.

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path academy/Cargo.toml -p academy-review -p academy-episodes --lib`

The pre-change nearest equivalent,
`cargo test --locked --manifest-path academy/Cargo.toml -p academy-episodes --lib`,
completed in `1.87 seconds`, strictly under 10 seconds. Record the first
Playground-bearing compilation separately as cold bootstrap evidence.

## Controls and evidence

- Held-out cases: catalog loading from an absent root and a complete non-range
  media request.
- Negative controls: encoded parent traversal, absolute/path-prefix escape, an
  unsatisfiable range, and a non-GET/HEAD method yield no evidence bytes.
- Laws: review serialization round-trips without changing values; URL resolution
  preserves a valid relative path; range slicing preserves exactly the same bytes
  as the corresponding slice of a full read.
- Falsifiers: any changed catalog JSON, generated episode assertion, physical
  dependency from Playground, path escape, eager base64 media conversion, wrong
  byte range, Dioxus in default-member builds, or warm regression at or above 10
  seconds rejects the candidate.
- Evidence: validated plan, factory-generated candidate receipt, and independent
  verification receipt bound to the candidate digest.
- Not applicable because this changes causally inert review infrastructure: do
  not create a research arm or run a frozen authority evaluator.

## Risks and rollback

- Custom-scheme range behavior differs by native webview; unit-test response
  semantics and retain correct complete responses when a webview sends no range.
- Catalog paths are serialized strings; decode and normalize them before opening,
  then require the canonical result to remain below the configured root.
- Moving types could silently change serialization; retain derives and names,
  re-export them, and test representative round trips and existing generation.
- Async catalog state could reset selection during rendering; load once from a
  stable root and keep selection/filter signals independent of media requests.
- Roll back the new schema crate, protocol module, workspace edges, and docs to
  commit `7bf0fbcd878f28ee2a8d69d953c25736767e9341`; no data migration is needed.

## Open decisions

None.
