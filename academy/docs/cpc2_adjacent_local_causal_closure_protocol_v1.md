# CPC2 adjacent local causal closure protocol v1

Status: frozen before evaluator implementation or execution.

Parent: CPC1 development positive at
`f77fbc81d9d8e437299bf769735e0b49bc6866c9`.

## Narrow question

Can consequence influence cross an arbitrary-depth forward causal structure by
repeating only adjacent ordinary CELL/ARROW/SPIKE interactions, while every
local plastic effect remains the CPC0+CPC1 intersection of contact-local
Modulation and remaining path-local participation?

```text
P -> C1 -> X -> C2 -> Y -> C3 -> Z
                                  |
                             consequence
```

CPC2 does not ask whether an addressed credit signal can traverse this path.
No candidate may inspect or retain the forward path as a path.

## Frozen unchanged-law arms

CPC2 first tests the two cheapest constructions available under the accepted
physics. Neither changes core Rust.

### Arm M: local Modulation only

The consequence produces ordinary Modulatory arrival at the last contact.
Existing Modulation changes local plastic support but cannot fire a CELL.

This arm asks whether the already-earned local closure itself produces any
ordinary activity capable of reaching the preceding contact.

### Arm R: adjacent ordinary relay

The consequence fires an ordinary local relay beside the last contact. Each
relay uses ordinary outgoing topology to:

1. send Modulation to its adjacent forward contact; and
2. send Drive to the next adjacent upstream relay.

The relay carries no path ID, arrow ID, depth, parent pointer, or semantic
credit state. It is deliberately tested because it supplies physical return
continuity cheaply. It is accepted only if forward participation itself gates
that continuity; a return chain that runs through unused forward contacts is a
negative, not a solution.

No third rescue arm is authorized after observation. In particular CPC2 may
not add a participation-gated relay law, backward traversal mode, or synthetic
coincidence state.

## Worlds

Each arm runs the same nine worlds.

1. `one_contact`: one forward contact participates; consequence must support it.
2. `two_contacts`: both forward contacts participate; consequence must support
   both and no other contact.
3. `three_contacts`: all three participate; consequence must support all three.
4. `broken_chain`: the upstream contact participates, but the forward causal
   connection to the downstream consequence is absent; no influence may jump
   the gap.
5. `unused_intermediate`: topology exists but the intermediate forward contact
   did not traverse; consequence must not pass it to support upstream structure.
6. `parallel_distractor`: an equally active noncontributing chain remains
   unsupported.
7. `branch_both`: two branches genuinely participate in the shared downstream
   consequence; both may be supported.
8. `branch_one`: identical topology but only one branch participates; only that
   branch may be supported.
9. `temporal_break`: the upstream local participation has relaxed to zero
   before downstream closure reaches it; support and further upstream closure
   must stop naturally.

The one-contact world is the CPC1 control. Branching worlds explicitly reject
one-winner semantics.

## Matrix

Run both arms across:

- two fresh identity roots;
- pressure phases `0..9`;
- Reference and Production mechanics;
- one exact same-mechanics fresh replay.

This yields `2 arms * 9 worlds * 2 roots * 10 phases = 360` physical cases and
`720` mechanics rows before replay.

For every case serialize:

- ordered retained `PhysicalTransition` history and hash;
- per-contact participation and plastic support;
- Drive and Modulatory deliveries;
- local-return updates;
- physical work, clock, pressure phase, liveness, and quiescence;
- arm/world predicates independently rather than one opaque assertion.

Reference and Production must match exactly on the ordered retained history,
candidate states, physical work, durable body, clock, pressure phase, and
quiescence. Stop on the first representation mismatch; no comparator repair is
authorized inside CPC2.

## Forbidden mechanisms

- backward ARROW traversal mode;
- credit/reward/return packet or identifier;
- causal-chain identifier;
- parent/predecessor pointer;
- stored path or path stack;
- propagation depth, maximum back steps, or a depth loop;
- evaluator mutation of participation/support;
- direct iteration over a remembered forward causal path;
- new core law or post-observation rescue arm;
- pressure changes, durable strengthening, or deletion of `eligible_until`.

Ordinary explicitly constructed topology is allowed and must remain visible in
the serialized physical history.

## Decision

- CPC2 positive only if one unchanged-law arm passes all nine worlds at depths
  one through three, all phases/identities, and exact mechanics/replay gates.
- If Arm M stops locally and Arm R propagates without respecting unused or
  broken forward participation, freeze CPC2 negative. That result means local
  attribution exists but the substrate lacks a participation-qualified
  adjacent closure affordance.
- Any need for a forbidden mechanism is negative; stop without rescue.

Pressure de-supply, ARC A3-A5, authority, oracle, and `arch.md` remain
unchanged in either outcome.
