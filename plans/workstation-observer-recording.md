# Record and review workstation runs

```text
live session -> complete physical observations -> frozen recording
                                                     |
                                                     v
                                             exact replay check
                                                     |
                                                     v
                                      observer frames -> MP4 review
```

## Outcome

Add a portable, checksummed recording of complete `WorkstationSession` steps
and a separate reviewer that turns only a decoded frozen recording into an MP4.
The video shows both eye fields plus causally inert movement, contact, device,
work, quiescence, and fingerprint annotations so a person can inspect what the
organism actually did.

This is observation and review infrastructure. It does not teach, select an
action, add learner state, change physical time, establish digit separation, or
promote a capability claim.

## Authority

- Path: `academy.md` Ownership, Evidence rules, and execution-lane separation;
  `arch.md` Boundaries; `research/programs/learner/lessons.toml` lessons
  `RM-000B`, `LP-004`, `LP-006`, and `RM-017`
- Revision: clean physical-workstation parent commit
  `6cc8411bab505096c466b4b2140398227a265fee`

## Model

- `RecordedStep` is the product of the inert read immediately before a step and
  the complete resulting `SessionObservation`. It preserves the already
  available eye rasters, body before/after state, movement, contact, crossings,
  device events/state, work, physical tick, quiescence, and fingerprints.
- `WorkstationRecording` contains the seed, exact initial opaque session
  checkpoint, and an ordered bounded list of `RecordedStep` values. Capture is
  the fold `session -> (recorded step, next session)`; it does not add a second
  diagnostic path.
- Canonical encode/decode is a versioned, length-delimited, checksummed envelope.
  Decode rejects corruption, unsupported versions, truncation, trailing bytes,
  empty recordings, sequence gaps, read/step disagreement, or invalid embedded
  checkpoints.
- Exact verification restores the embedded initial checkpoint and composes the
  ordinary session transition once per recorded step. Every read and complete
  observation must equal the frozen value. Thus replay is identity on the
  complete recording, not merely on a summary or final pose.
- Add optional `academy-workstation-review`, outside default Academy members.
  Its pure frame projection maps one decoded `RecordedStep` to a 1280x720
  observer raster containing left and right organism-visible eye fields and
  external annotations. File writing and `ffmpeg` invocation remain the final
  I/O boundary.
- The CLI first captures and writes `recording.tlwr`, then decodes that exact
  file and passes the decoded value to the reviewer. Live session state is never
  passed to review code.

## Invariants

- Recording observes existing public read and step values only; it introduces
  no new learner event, outcome, timing, ranking, memory, or physical branch.
- The full decisive before/after slice is preserved. The recorder does not
  replace an existing trace with a reduced diagnostic summary.
- Review consumes only a successfully decoded, exactly replayed frozen record.
- Observer labels, device semantics, fingerprints, wall-time frame duration,
  PNG encoding, and video encoding never enter `WorkstationHarness`.
- Recording on or off produces the same ordered session observations and final
  checkpoint bytes.
- Recording bounds are explicit and invalid values fail without a partial
  canonical artifact.
- The same decoded recording produces byte-identical PNG observer frames.
  MP4 container bytes need not be identical across compatible `ffmpeg`
  versions, but decoded frame count and dimensions must agree.
- The headless `academy-workstation` warm regression remains strictly under ten
  seconds; video generation is optional review work and excluded from physical
  runtime.

## Scope

- Add recording types, canonical envelope, capture, validation, and replay
  verification to `academy-workstation`.
- Add `academy-workstation-review` as a non-default workspace member with frame
  rendering, artifact manifest, MP4 encoding, CLI, and tests.
- Document the recording command and output in `academy/README.md` and the two
  crate READMEs.
- Add focused public tests for exact recording replay, corruption, sequence
  continuity, observational inertness, deterministic frames, semantic
  separation, and a real `ffmpeg` smoke artifact when the executable is
  available.

Exclude learner-physics changes, new diagnostic events, a live UI, video input,
audio, evaluator-selected movements, capability scoring, teaching schedules,
research authority, and automatic retention of generated `output/` media.

## Development style

TDD. Freeze the canonical recording and observational-inertness laws before the
review renderer, then add the CLI and real media smoke test.

## Focused tests

- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation recording`
  establishes bounded capture, canonical decode, exact replay, corruption
  rejection, and equivalence with an unrecorded session.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation-review --lib`
  establishes deterministic observer frames, both-eye preservation, annotations,
  and manifest construction from frozen evidence.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation-review --test video_smoke`
  establishes that the CLI path writes a decodable MP4 with the expected frame
  dimensions when `ffmpeg` and `ffprobe` are installed.
- `cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation --test workstation_world`
  preserves physical world and semantic-firewall behavior.
- `cargo fmt --all --manifest-path academy/Cargo.toml -- --check`
- `cargo check --workspace --all-targets --locked --manifest-path academy/Cargo.toml`
- `cargo clippy --workspace --all-targets --locked --manifest-path academy/Cargo.toml -- -D warnings`

## Development loop

Representative warm regression suite:

`cargo test --locked --manifest-path academy/Cargo.toml -p academy-workstation-review --lib`

Its measured warm budget is strictly under 10 seconds. A real `ffmpeg` encode
is a separately recorded integration check and is not the warm loop.

## Controls and evidence

- Held-out cases: a different seed, one-step and multi-step records, a sequence
  boundary after checkpoint restoration, and a frame containing actual movement
  or device events when naturally produced.
- Negative controls: recording disabled versus enabled yields identical session
  observations and checkpoint bytes; one flipped record byte fails checksum;
  truncation and trailing bytes fail; a shuffled or duplicated step fails
  continuity; observer labels are absent from serialized `WorldSample`.
- Laws: capture is observationally inert; decode after encode is identity;
  replay after capture is identity; rendering is deterministic for a frozen
  step; concatenation preserves step order.
- Falsifiers: any recording branch changes learner history, the reviewer accepts
  unreplayed or corrupt evidence, a semantic annotation crosses the harness,
  an eye field is reconstructed rather than preserved, the video cannot be
  decoded, generated media enters source control, or the warm loop reaches ten
  seconds.
- Evidence: validated plan, candidate receipt, exact replay tests, inertness
  comparison, deterministic frame hashes, `ffprobe` output, generated example
  recording/frames/MP4 under ignored `output/`, and independent verification.
- Not applicable because no capability is being scored: transfer, retention,
  and teaching/probe separation begin with the digit-separation campaign after
  this observer layer is available.

## Risks and rollback

- Complete eye rasters make recordings large. Bound the step count and use a
  compact binary envelope rather than JSON pixel arrays.
- Review could accidentally become a second execution path. Require a written,
  decoded recording and expose no live harness to the review crate.
- `ffmpeg` availability varies. Keep frame production testable without it and
  report a typed external-tool error at the final encode boundary.
- Roll back by removing the recording module, optional review crate, workspace
  member, docs, and receipts. The committed workstation world and organism
  checkpoints require no migration.

## Open decisions

None.
