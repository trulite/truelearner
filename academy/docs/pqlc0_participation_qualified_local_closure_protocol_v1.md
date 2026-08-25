# PQLC0 participation-qualified local closure protocol v1

Status: frozen before any substrate or evaluator change.

Parent: CPC2 stopped negative at
`ca6818d5dcdb2f34915fc29cf2bbda98cc0059d3`.

## New physical affordance under test

PQLC0 explicitly proposes one new substrate property. ARROW transmission is
factored into two independent physical properties:

```text
effect:   Drive | Modulatory
trigger:  SourceFires | QualifiedLocalParticipation
```

Every existing ARROW defaults to `SourceFires`; its behavior is unchanged.
PQLC0 tests only:

```text
Modulatory + QualifiedLocalParticipation
```

No authority or retained-law claim is implied by the feature-gated candidate.

## Trigger law

When ordinary Modulatory activity arrives at CELL C:

1. unchanged CPC1 arithmetic intersects it with each local outgoing ARROW's
   remaining participation;
2. if at least one live outgoing `Drive` contact at C retains nonzero CPC1
   participation, every live QLP-triggered outgoing ARROW at C traverses;
3. each such traversal emits an ordinary Modulatory SPIKE through its explicit
   topology.

C does not activate or fire. Ordinary Drive arrival, activation, and source
firing never trigger a QLP ARROW. Conversely a QLP ARROW is not traversed by
ordinary source firing. This is the hard premature-closure boundary.

Qualification is compartment-local and deliberately coarse. Multiple QLP
outputs at one qualifying CELL all traverse. Specificity at their destinations
still comes only from each destination contact's own remaining participation.

QLP traversal receives its own physical-work count and ordered observer event.
The emitted SPIKE is otherwise an ordinary Modulatory SPIKE. QLP ARROW
traversal does not consume or renew the qualifying outgoing Drive contact's
participation. No attenuation, TTL, refractory state, or damping is added.

The candidate is feature-gated behind `pqlc0`, which implies `cpc1`. Candidate
trigger state need not enter durable/checkpoint formats in this development
gate; trigger values are compared explicitly across mechanics.

## Frozen worlds

1. `positive_one_hop`: both forward contacts participate; consequence reaches
   C2; C2 QLP traverses; ordinary Modulation reaches C1; C1 local structure
   responds.
2. `never_participated`: required outgoing Drive contact at C2 never traverses;
   Modulation at C2 emits no QLP activity.
3. `expired_participation`: C2 participated but its CPC1 magnitude naturally
   relaxed to zero before Modulation; no QLP traversal.
4. `wrong_path`: another local outgoing Drive path participated while the
   required contact did not; the frozen contact geometry must not bridge the
   required connection.
5. `unrelated_activity`: equal nearby Drive/activity without required contact
   traversal emits no QLP activity.
6. `two_upstream_one_participated`: qualifying C2 may emit toward C1a and C1b;
   only C1a has local participation and can respond/continue; C1b stops.
7. `contact_fanout`: one compartment with multiple genuinely participating
   outgoing Drive contacts remains compartment-granular; no finer attribution
   is claimed.
8. `no_premature_forward`: execute the full forward episode without
   consequence; source firing emits zero QLP traversals and zero QLP-delivered
   Modulation.
9. `drive_not_consequence`: ordinary Drive arrives at participating C2; no QLP
   traversal.
10. `closure_cycle`: two qualifying contacts have delay-one QLP topology in
    both directions. One legitimate closure is introduced, then the unmodified
    queue runs. It must naturally quiesce before the frozen work ceiling. No
    cycle-specific stop law is allowed.

The cycle uses physical delay one so ordinary CPC1 relaxation is experienced
between successive local interactions. PQLC0 does not add a decay or
attenuation law for the cycle.

## Matrix

Run all ten worlds across:

- two fresh identity roots;
- pressure phases `0..9`;
- Reference and Production mechanics;
- exact same-mechanics reconstruction.

This is `10 * 2 * 10 = 200` physical cases and `400` mechanics rows before
reconstruction. The cycle ceiling is `4096` physical operations. Reaching the
ceiling or command timeout is an immutable negative, not authorization for a
stop rule.

Serialize independently:

- ordered physical transition history;
- QLP traversal observer events and work;
- Drive and Modulatory deliveries;
- per-contact participation and plastic support;
- source fires;
- proposals and deallocations;
- clock, pressure phase, durable body, trigger surface, quiescence, and replay.

Reference and Production must match exactly on every physical observation and
final state. No sorting, normalization, or comparator repair is authorized
after evidence.

## Static prohibitions

New substrate logic may not contain or encode equivalents of:

```text
backward previous predecessor credit reward cause
path_id route_id parent depth hop_count
```

Also forbidden:

- source firing or Drive arrival triggering a QLP ARROW;
- lookup of an earlier route or forward-history container;
- trace consumption, attenuation, TTL, cycle detection, or a special stop;
- new SPIKE effect/type for closure;
- pressure changes, durable strengthening, or eligibility deletion;
- post-observation rescue or parameter tuning.

## Decision

- Any world, quiescence, mechanics, replay, or static-audit failure: PQLC0
  stopped negative; freeze and stop.
- All 200 cases pass: PQLC0 development positive for one-hop qualified local
  transduction and cycle safety only.

No multi-depth gate, pressure integration, ARC A3-A5, authority, oracle, or
`arch.md` change occurs in PQLC0.
