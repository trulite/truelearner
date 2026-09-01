---
name: dev
description: Develop, debug, review, formalize, or change TrueLearner code. Apply category-theory and TAME lenses, compose Rust physical evidence with observer-only Lean checks, keep the implementation lagom, preserve black-box behavior, and keep representative warm wave time strictly under 25 ns unless the user expressly approves otherwise.
---

# Development

```text
problem -> compositional physical model -> smallest complete change -> verify
               |                                  |                 |
        category theory + TAME              simple code       warm wave < 25 ns
                                                  |
                               new type or elaborate mechanism
                                                  |
                         concept audit -> propose -> user confirms -> edit
                                                                    |
                                                    slower -> ask user to approve
```

## Model

- Apply a category-theory lens: identify objects, arrows, identity, composition,
  products, and ownership boundaries. Make independent parts compose without
  hidden coupling. Keep these ideas in the modeling; use plain domain names in
  code.
- Apply the TAME lens: explain behavior through local physical incidence,
  retained history, and composable competent parts. Never admit evaluator,
  benchmark, answer, or semantic knowledge into the organism.
- Prefer the smallest complete change that preserves existing boundaries and
  black-box behavior.

## Confirm new mechanisms

- Before editing code to introduce a new struct or an elaborate mechanism,
  stop and propose the design to the user.
- Show why existing types are insufficient, the exact new state and arrows,
  the smaller alternatives considered, and the tests and cost boundary.
- Proceed only after the user explicitly confirms that proposed design.
- Do not treat a request to investigate, fix, or continue as confirmation of a
  design the user has not yet seen.

## Enforce conceptual parsimony

```text
proposed concept
      |
      v
distinct physical fact + one owner + distinct legal transitions?
      | no                         | yes
      v                            v
reuse or compose existing      fit it on the structures page
concepts                       and its law on the laws page
                                    |
                                    v
                           propose exact concept delta
```

- Inventory the affected persistent structs, enums, roles, and laws before
  adding one. State the concept delta: names added, names removed, owner, and
  why a product or composition of existing concepts is insufficient.
- Add a type only when it owns a distinct invariant or lifecycle. Do not create
  aliases, manager/context/info bags, parallel representations, or a new enum
  variant when an existing product, arrow, state, or derived view suffices.
- Give one concept one production name and one authority for changing it.
  Separate persistent evidence, transient computation, observer projection,
  and test fixtures.
- Update `docs/body-structures.md` and `docs/body-laws.md` in the same change as
  persistent state or learner-law edits. Stop and simplify if the new concept
  cannot be defined in one sentence or the two sheets cease to be concise.
- TrueLearner is prerelease: delete compatibility-only vocabulary, duplicate
  wire representations, and migration paths. Bump the current schema version,
  regenerate fixtures, and reject old artifacts. Add compatibility only after
  explicit user approval.
- Before handoff, search for duplicate/dead concepts and obsolete names, run
  clippy with warnings denied, and report the final concept delta.

## Keep the workstation generic

- Run software tasks as applications on the existing workstation: application
  output reaches the organism through the monitor, and application input comes
  only from ordinary workstation `DeviceEvent`s such as keys, pointer motion,
  and clicks.
- Extend a missing generic monitor or device surface. Never replace the
  workstation body with a task-native morphology, map `BodyControl` or
  `MotorEffect` directly to task actions, or let a task's action catalog select
  or inhibit internal movement.
- When a claim depends on the completed body course, restore the checkpoint
  produced by that course. Use a fresh `WorkstationHarness` only as an explicit
  cold-body negative control. Stop if the checkpoint cannot cross this boundary
  without adding benchmark knowledge to the organism.

## Compose causal changes

```text
Rust physical trace -> structure-preserving projection -> Lean claim check
        |                                                        |
        +-------------- learner never receives -----------------+
```

- Draw the commuting square before changing a causal handoff. Name the physical
  objects and the arrows that act, cross a boundary, return, close, and persist.
- Make Rust witness every premise. Retain zero, one, or several causal parents;
  never let an observer projection invent ancestry discarded at runtime.
- Use Lean only on frozen evidence to prove the resolver and choice claim. Keep
  the checker, receipt, formal names, and theorem results out of organism input,
  checkpoints, choice, and the warm path.
- Model temporary witnesses as open, closed, ambiguous, or expired historical
  instances. Closure may preserve witnessed support; ambiguity and no claim
  preserve none.
- Give continuation precedence only to a unique executable, retained,
  boundary-open path with fresh progress carrying its exact physical motor
  parent. Timing after movement is not ancestry. Recompute the condition at
  every choice and remove it after closure, absent progress, ambiguity,
  physical limit, or expiry.
- Treat boundary completion as local inhibition, not a global action command.
  Release only to an uninhibited antagonist in the same ordinary outcome
  component; independent components remain a product, and simultaneous
  components make no arbitrary local claim.
- Distinguish ongoing world progress from terminal boundary closure. Persist a
  current exact parent through the interaction, close with that parent rather
  than a stale predecessor, and treat several simultaneous parents as
  ambiguity.
- Model support as one-sided external reaction: cancel only inward effort along
  the surface normal. Never let contact pull outward or inhibit a tangential
  axis. Keep tactile incidence morphologically local; a thumb surface should
  not reopen every historical hand action.
- Prefer a physically open neighboring surface and a real crossing over
  preloaded contact, timers, counters, or an episode-length continuation rule.
  Restore only the frozen external pose for a fresh probe; retain learned
  topology and discard probe mutation.
- Do not diagnose forgetting from an unchanged retention context that gives the
  body no physical occasion to act. If a probe needs an external perturbation,
  freeze the checkpoint before setup, record and exactly replay the
  perturbation, prove it changes pose without changing learner topology, causal
  time, or history, and give it no organism parent or credit.
- Test identity at quiet, renaming invariance, and product independence in
  addition to the smallest positive and killing negative fixtures.

## Verify

- Add the smallest law or black-box scenario that fails before the change and
  passes after it.
- Run focused tests, then the affected regression suite.
- Measure the representative warm wave before and after the change. Exclude
  build, setup, rendering, serialization, and cold bootstrap unless they are the
  behavior being changed.
- Require wave time to remain strictly below 25 ns. If it reaches 25 ns or more,
  optimize it or stop and obtain the user's express approval before accepting
  the change.
- Report behavior, tests, and wave time in simple English.
