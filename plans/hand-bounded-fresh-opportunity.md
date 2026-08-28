```text
local output group
      |
      +-- executable incumbent -- one live, non-recent unanswered return
      |
      `-- owner-compatible fresh candidate -- equal opposing live paths
                         |
                         v
                 one transient UNIT opportunity
                         |
                         v
             existing sign choice + threshold
                         |
                         v
          existing replacement supersedes return
```

# Supply one bounded local fresh opportunity

## Outcome

An opt-in recursive protocol lets one live unanswered incumbent return supply one
transient physical opportunity to one owner-compatible local fresh candidate before
signed path cancellation. It changes neither path strength nor topology. If the
candidate becomes executable, the existing replacement rule selects it and
supersedes the exact donor return in the same moment.

## Authority

- Path: `research/campaigns/hand-pre-executability-opportunity-v1/convergence.toml`
- Revision: `sha256:72a4d2dcf42964d9ab22608b8fd27d713c3814a57fef67c4cd524d857ea0270f`

## Model

Candidate evaluation has two stages. The first maps every output incidence to its
ownership, local position, raw signed complete-path strength, ordinary opportunity,
executability, consequence recency, and live unanswered return set. The second
examines each existing local competition group.

A donor is executable, has an admitted unanswered return, and has no consequence
inside the existing recent-eligibility window. A recipient is non-executable,
owner-resolved, has equal nonzero positive and negative path strength, has no
ordinary opportunity, and has no admitted return for the existing positive tie
choice. At least one donor return must have the same actual return-memory owner as
the recipient candidate owner. Among compatible recipients, select at most one by
stronger live path magnitude, then older path participation, then stable incidence
order.

The selected recipient receives exactly `UNIT` as transient opportunity. That unit
both opens the existing positive tie choice and participates in projected drive,
just like an ordinary current opportunity. If threshold is crossed, the recipient
joins ordinary competition with its unchanged path drive. Existing fresh-candidate
replacement and return supersession perform consumption; no new persistent state is
introduced. Diagnostics report the transferred unit and exact donor return.

## Invariants

- A return supplies at most one candidate in one local competition group and only
  while it is live, unanswered, owner-compatible, and outside recent preservation.
- An answered, superseded, absent, recent, owner-incompatible, or nonlocal return
  supplies no opportunity.
- The recipient must already have complete opposing live paths; the law creates no
  route, sign, strength, consequence, owner, or semantic motor preference.
- Existing output locality, recent-consequence preservation, deterministic ranking,
  reflection, and fresh-return replacement remain authoritative.
- One successful transfer closes the donor through existing supersession, so the
  same return cannot be consumed repeatedly.
- Old protocols, default behavior, physical-transition admission, replay, natural
  quiescence, and evaluator isolation remain unchanged.

## Scope

- Add one cumulative opt-in protocol and its inherited binding predicates.
- Refactor output evaluation only enough to retain provisional blocked candidates,
  supply one local opportunity, and reuse already resolved return state.
- Extend candidate diagnostics with supplied-opportunity amount and a donor/recipient
  transfer event.
- Add focused core fixtures plus one solve experiment/campaign over controls and the
  unchanged reflected hand.
- Exclude random choice, strength reset, persistent exploration memory, hand-specific
  reversal, semantic output identity, broader owner factorization, default adoption,
  commits, and authority promotion.

## Development style

TDD. Build a two-output fixture in which both complete paths exist, one strengthened
incumbent has a live unanswered return, and the other candidate has balanced paths
but no current opportunity. Require exactly one transfer and alternative output,
then add answered, recent, no-alternative, nonlocal, owner, repeated-return, and
reflection controls before running the hand.

## Focused tests

- `cargo test --locked --manifest-path truelearner/Cargo.toml --test harness_boundary bounded_fresh_opportunity`
  proves pre-executability transfer, exact-return consumption, lifetime, locality,
  ownership, recent preservation, reflection, and old-protocol equality.
- `cargo test --locked --manifest-path research/experiments/hand-bounded-fresh-opportunity/Cargo.toml`
  proves the constructed mechanism, frozen controls, hand release/composition
  predicates, replay, quiescence, and cost.
- `cargo test --locked --manifest-path research/experiments/hand-pre-executability-opportunity/Cargo.toml`
  preserves the localization reference.

## Development loop

`cargo test --locked --manifest-path research/experiments/hand-bounded-fresh-opportunity/Cargo.toml`
is the representative warm regression suite and must remain strictly under 10
seconds. Record cold dependency bootstrap separately.

## Controls and evidence

Held-out cases are reflected positions, unequal drive, already executable recipient,
multiple fresh recipients, and reordered incidence. Negative controls are answered,
recent, missing, owner-incompatible, and nonlocal donor returns; no alternative;
same-return single consumption; old protocol equality; no path/strength mutation;
exact replay; natural quiescence; zero propagation exhaustion; and bounded work.
Falsifiers are any control exposure, repeated consumption, semantic leakage, fixture-
only survival, failure to execute the second hand motor and leave the upper boundary,
loss of replay/quiescence, or a warm loop at least 10 seconds.

## Risks and rollback

Broad transfer could create motor oscillation or cross-owner leakage; exact-link
supersession, owner equality, locality, recent preservation, replay, and quiescence
controls detect it. Two-pass evaluation could resolve returns twice; provisional
state must be reused. Rollback removes the new protocol, transfer stage/event,
experiment, and campaign while retaining the observational diagnostics.

## Open decisions

None.
