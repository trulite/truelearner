# Compose physical moments into body learning

```text
physical moment + local topology + link memory
                    |
                  react
                    |
              primitive edits
                    |
                  apply
                    |
             next physical moment
```

## Outcome

Make `Body::step` and `Body::run` compose path formation, choice, actual path
participation, physical return, strengthening, reuse, and recursive boundary
construction directly from retained physical moments. All 28 isolated
`truelearner-new-harness` laws must pass unchanged while that crate depends
only on `truelearner-body`.

Remove `Surface`, `Candidate`, `ReturnedOutcome`, `Closure`, and the semantic
`Event`/`Context` input boundary from the body's causal path. The claim is
self-sufficiency for the 28 retained laws, not equivalence with the old core or
authority for workstation, Academy, or benchmark behavior.

## Authority

- Path: `LANGUAGE.md`, `lessons.md`,
  `truelearner/crates/body/src/{arena,core,engine}.rs`,
  `truelearner/crates/new-harness/tests/body_laws.rs`, and
  `plans/{physical-body-acceptance,body-physical-evidence,body-local-incidence-participation}.md`
- Revision: Git `bedcbe54208bd8dc2df9d1bdde976f6c28c4ea7a`; authority
  digests `2b1954b161358c4a259198b0b9e4c66a93e47350d749d7c3baf3ddcef7bb8a41`,
  `5b50453e4895e5a25c337555af167894cbfd4625d89837976815914aa21e1bb0`,
  `607968f296b1db5d2612a69765b573159c9705171d55be7fee6d04e272623d01`,
  `b2893936b6746a86a8f291c53e6f6cc59e2e1538f123b44d917eb6e59745b6f8`,
  `8162cf58c7ed6907ef5ddd79900449cfe6d82d644b26284da5ab1b67a4ec4366`,
  `a98e6b3790fcef02cdf21ed12190278fb8efd48353f2cd05951888b64b8605fa`,
  `1b8d566dfa29362b355adce13b3ab80feed6e27efda02313abb24c5828e669d0`,
  `1372a5fedf935aa48ae7b1253d54c508ea4e36c8988c2bb42da06c382b1af3de`,
  and `2ffce131883ba1197b252f8f1ed39de7fa129a75d32ae3ed2bcca3102b71bee2`.

## Model

The authoritative state remains junctions, links, link roles, link memory, and
scheduled physical activity. One private transformation consumes the complete
`PhysicalMoment` after junction change and ordinary drive transmission:

```text
react(body, physical_moment, work) -> reaction result
```

`Change` remains an ordered composition of primitive junction, link, send, and
link-memory edits. `Body::step` applies that change before releasing the
moment's participants; `run` remains repeated `step` until natural quiet.

The local laws are:

- A boundary-carried junction change may form two signed paths only through
  outgoing, zero-impulse morphology links whose delay is one or two. Existing
  local path-entry links prevent duplicate formation. Outward nonzero effects,
  learned middle junctions, and distance three do not form paths.
- A path is two physically adjacent links: a path-entry link from the changed
  surface to a middle junction and a live drive link from that middle junction
  to an output. Candidates are assembled only from links incident to changed
  junctions and their local middle junctions.
- Choice groups paths by shared surface, physical outcome-witness source, and
  learner membership. It selects at most one executable path per connected
  component using, in order, a unique exact open return, a unique latest
  available outcome, a unique latest outcome, an eligible fresh organism path,
  then participation, strength, and stable physical order. Consuming an
  available outcome is part of the same change as sending the chosen path.
- An output change credits only a drive link present in that change's retained
  participant chain. Reverse incidence reconstructs its path-entry link, and
  the recorded cause and time must agree. The reaction marks those exact links
  as participated and creates one temporary return link while the path is live.
- A later boundary transition resolves only returns indexed by its exact
  nonzero cause and opened strictly earlier. One matching physical path records
  the outcome, strengthens once, and gains a non-driving outcome-witness link
  from the consequence junction to its middle. Zero matches, repeated samples,
  pre-opened transitions, and ambiguous matches are identity for credit. Every
  sibling return in the resolved cohort then retires.
- A same-moment causal meeting of at least two boundary-carried members may
  create a membership junction. Repeating the parent's complete member set is
  identity. New membership creates a child whose physical members are the
  parent boundary plus only the novel members, so the same rule composes
  recursively. A same-moment sampled transition may witness an outcome only
  for a path belonging to a newly direct member; stale or inherited evidence
  cannot cross the new boundary. Construction moments do not also execute an
  ordinary fresh choice.

Later consequences are not topologically adjacent to earlier outputs. Reverse
incidence and per-link transmission memory therefore cannot locate an open
return without scanning all links. Add a derived `cause -> live return LinkId`
incidence to `Body`. Return links remain authoritative; the index is rebuilt or
remapped during attachment, updated with return creation/retirement, cloned by
checkpoints, and never supplies a learning fact absent from a live return link.
Outcome source identity is retained as a non-driving physical witness link,
not as evaluator metadata.

For `F` retained firings, `C` changes, `V` locally visited links, `P` local
candidate paths, `R` exact-cause live returns, and `M` same-moment members, one
reaction is average `O(F + C + V + P + R + M)`. Derived cause lookup is average
`O(1)`. Storage adds `O(open returns)` incidence plus ordinary learned links.
Neither reaction nor choice may scan all `J` junctions or `L` links, so dormant
body size adds no execution work.

Capacity and time failure remain explicit. Automatic reaction validates a
complete change before mutation; capacity exhaustion or a backward scheduled
send returns a typed `RunError` without partially applying that reaction.

## Invariants

- Physical firings, their exact transmitting links, meeting causes, times, and
  participants are the only event inputs to reaction.
- No code constructs semantic surface, candidate, returned-outcome, closure, or
  natural-cycle events before calling `react`.
- Observation is read-only: changing, omitting, or reordering observer work
  cannot alter body state or future physical events.
- Formation, choice, credit, and construction use only current activity, local
  incidence, exact-cause open returns, and physical membership; no dormant-body
  scan or hidden morphology label is allowed.
- Only links that physically transmitted on the used path can participate,
  receive an outcome, strengthen, or be consumed.
- One physical consequence produces at most one learning effect; ambiguity,
  unchanged samples, stale evidence, and pre-opening are identity.
- Connected components choose independently, and equivalent disconnected
  construction order cannot change the observable choice.
- Membership requires physical novelty, nests through ordinary membership
  links, and carries only a same-moment live witness to a newly direct member.
- Quiet remains identity, every run reaches natural quiet within its supplied
  limit, and repeated `step` equals `run`.
- Junction and link propagation slots remain 32 bytes. The public attachment,
  calibration, input, observation, and checkpoint boundaries remain intact.
- The 28 acceptance laws and every negative control remain unchanged; no old
  core dependency may enter `truelearner-new-harness`.

## Scope

Change `truelearner/crates/body/src/core.rs`, `engine.rs`, `arena.rs`, and, only
as required for atomic remapping and typed failure, `attachment.rs` and
`physics.rs`. Add focused private tests under `truelearner/crates/body` and a
candidate receipt under `factory/receipts`. Do not change
`truelearner/crates/new-harness/tests/body_laws.rs`, its fixture morphology, the
old `truelearner-core`, checkpoint semantics, workstation, Academy, research
programs, frozen evidence, workspace membership, or production selection.

Remove body-public semantic reaction inputs that have no callers outside the
new body crate. Retain `LinkRole` and the narrow link setters required by body,
attachment, and checkpoint controls. Do not add sensor, actuator, position,
direction, region, resistance, policy, recognizer, or evaluator types.

## Development style

Use TDD. First add private focused tests for physical formation and reuse,
actual-link output participation, exact-cause return indexing, outcome-witness
connectivity, ambiguity, membership novelty, recursive membership, atomic
failure, observer independence, and dormant-body work. Then implement the
private physical `react` composition without changing the acceptance oracle.

## Focused tests

- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-body`
  checks moment composition, local topology, exact participation, return
  indexing, outcome credit, membership, attachment, calibration, slot size,
  atomicity, and kernel regressions.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-new-harness --features candidate --test body_laws`
  requires all 28 unchanged isolated laws and controls to pass.
- `cargo test --manifest-path truelearner/Cargo.toml -p truelearner-checkpoint`
  checks quiet-cut clone, replay, learning state, return incidence, and time.
- `cargo check --manifest-path truelearner/Cargo.toml -p truelearner-body -p truelearner-new-harness --features truelearner-new-harness/candidate --tests`
  checks the isolated crate boundary.
- `cargo clippy --manifest-path truelearner/Cargo.toml -p truelearner-body -p truelearner-new-harness --features truelearner-new-harness/candidate --all-targets -- -D warnings -A clippy::obfuscated-if-else`
  checks the complete candidate without weakening other warnings.
- `cargo tree --manifest-path truelearner/Cargo.toml -p truelearner-new-harness --edges normal`
  must show `truelearner-body` and no `truelearner-core` dependency.

## Development loop

The representative warm regression suite is
`cargo test --manifest-path truelearner/Cargo.toml -p truelearner-new-harness --features candidate --test body_laws`.
Its measured budget is strictly under 10 seconds; record cold compilation
separately from the warmed test duration.

## Controls and evidence

The unchanged 28-law file is the primary oracle. Negative controls cover no
nearby output, distance three, outward reentry, mixed cause, repeated and
expired samples, pre-opening, duplicate consequence, ambiguous current return,
ambiguous completed cycles, repeated membership, stale construction witness,
disconnected components, reversed construction, observer variation, dormant
growth, and arena growth. Held-out cases are the full body attachment and
calibration suites, checkpoint continuation, slot sizes, dependency tree, and
strict Clippy.

The candidate is falsified by any changed acceptance test, semantic event
adapter, old-core dependency, credit to an untransmitted link, duplicate
learning effect, construction without novelty, stale witness crossing, partial
reaction mutation, observer-dependent result, work dependent on dormant body
size, loss of natural quiet, or a representative warm run at or above 10
seconds. Expected artifacts are the validated plan, candidate receipt, exact
test-output digests, and an independent verification receipt.

## Risks and rollback

The largest risk is turning the derived return incidence into a second source
of truth. Every lookup must revalidate the referenced live return link and
remove stale entries when a return retires or attachment remaps it. Another
risk is accidentally interpreting arbitrary zero-impulse links as morphology;
formation is restricted to a boundary participant, local delay, live drive
role, and exact deduplication. Multi-law moments risk order dependence; compute
one immutable reaction from the retained moment, validate the complete ordered
change, then apply it once.

Rollback removes the physical reaction invocation and cause incidence, restores
the semantic `core.rs` input API, and returns to the accepted 13-pass/15-fail
baseline without changing the acceptance suite or the already validated
physical-evidence and reverse-incidence work.

## Open decisions

None.
