# CJ0-F matched discriminator terminal development handoff v2

Status: **DEVELOPMENT GATE COMPLETE; BOTH FAIL; SHARED BOUNDARY FROZEN**.

## Frozen identities and scope

- authoritative start: clean
  `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`, tag
  `px2-physical-causal-direction-authoritative`;
- CJ-B source commit: `336d2252c10ba1324fc51b0de266ee5d36a94225`,
  exact imported law SHA-256
  `ef0de37a9ac54b632b991f0d4647a5ee78c23810084d61497c88d6f757ec2188`;
- CJ-E source commit: `4f0b14f7266455ac2a7cbb47bb6627349882caa2`,
  exact imported generated law SHA-256
  `e64c8c915c2fbc4679d1e34ee69ecfe36e2c5ff05bdff5d7feeb5a55578bf1c1`;
- primary protocol SHA-256:
  `18babd7614aadf0e2a0aa4ea9bc622a4bbff9c3d7ebc0b43c1ba301c088c959a`;
- mandatory-coverage correction protocol SHA-256:
  `205ab40298517188e87e3620dfd9974ba67472dc3144ffb77b5a5d6d7af9eef0`;
- final comparator SHA-256:
  `37a09f7928be5b47f72f6d4817e9818a665f3add5b63ff2a299d3fc3c5a513e8`.

Both candidates are isolated crates. `cmp` and SHA-256 verify byte identity.
No authoritative PX0--PX2 byte changed; every lane path since the start is an
addition. No PX3/CJ0 negative is reinterpreted. PX3 remains absent and PX4--PX8
remain parked. This handoff ends at development GATE and defines no later
protocol, command, artifact, marker or authority claim.

## Exact matched protocol

For each row, B and E are reconstructed independently from the same serialized
physical spec: CELL identity/position/region/threshold/resistance; ARROW
endpoints/delay/phase/coupling/resistance/insertion order; external SPIKE
origin/target/impulse/tick/phase; mirror, load, allocation, return topology and
pressure horizon. The evaluator retains physical source/path provenance only
for scoring and never supplies contributor, pair, member or semantic labels.
Candidate outputs never choose later schedules.

The valid terminal matrix combines unaffected GATE v1 rows with the fresh
mandatory-coverage correction:

- seeds: original 307/311/313/317 and correction 1307/1311/1313/1317;
- normal/mirror and forward/reverse insertion;
- thresholds 2/3/4, coupling 1/2, distractor loads 0/4/12;
- sparse/dense allocation 2/6;
- spacing -1,0,1,2,3,4,5 and train spacing 0/2/4;
- weak/mature routes, crossed return, late blocked return, stale generation,
  exact replay, full deallocation/bootstrap, contemporary reversal, ambiguity,
  and level-free `A+B->X, X+C->Y, Y+D->Z` recurrence.

The first GATE's core 8,640 rows remain immutable, but 3,312 fixture-defective
rows (burst-4, blocked-return, serialized-only timing transfer) are excluded
from the terminal join. They are replaced by 3,456 fresh rows containing real
four-emission bursts, actual post-window return, actual train/held-out timing,
and 144 added contemporary reversals. Terminal valid dimension: **8,784 paired
rows**. GATE v1's coverage-defect tag remains visible; no byte was overwritten.

The reversal finiteness amendment uses identical ordinary coupling-0 reciprocal
paths to occupy only D->A/B->C and prevent an irrelevant autonomous reciprocal
proposal loop. Those blockers emit no impulse, eligibility, return or effect
under either law. A->D/C->B must still arise by generic bootstrap. A shared
zero-impulse clock arrival equalizes the candidate queue horizon without
supplying evidence.

## Terminal paired results

| family | valid rows | CJ-B pass | CJ-E pass | prediction differences | B false conjunction | E false conjunction |
|---|---:|---:|---:|---:|---:|---:|
| same-source bursts | 2,448 | 1,968 | 2,016 | 240 | 336 | 408 |
| amplitude vs contributor multiplicity | 576 | 288 | 432 | 144 | 288 | 144 |
| dense return topology | 1,008 | 1,008 | 1,008 | 0 | 0 | 0 |
| timing transfer | 3,024 | 3,024 | 3,024 | 360 | 0 | 0 |
| shared controls | 1,728 | 1,512 | 1,584 | 192 | 72 | 0 |
| **total** | **8,784** | **7,800** | **8,064** | **936** | **696** | **552** |

### Same-source and amplitude

- A genuine same-tick weak contributor plus trigger reaches the effect in both
  laws.
- A matched single strong arrival reaches the CJ-B effect and is rejected by
  CJ-E. This is the clearest B/E prediction difference: ARROW coupling does not
  by itself distinguish amplitude from multiplicity.
- Two matched weak arrivals from one physical source/path reach the effect in
  both. Neither law therefore guarantees genuine contributor multiplicity.
- In the corrected four-emission one-path burst, B false-conjoins for all 24
  threshold-2/coupling-2 strata; E false-conjoins for all 72 coupling-2 strata
  across thresholds 2/3/4. No candidate receives the source/path label.

### Dense return, stale matter and reversal

- All 1,008 dense rows pass. Only the physically traversed eligible route
  strengthens; crossed, nontraversed and late-return routes do not. In the 144
  late-return rows, resistance moves from 3 to 2 under unsupported use and
  return updates remain zero for both. Physical return is never called wrong.
- All 144 stale-generation, 144 exact-replay, 144 full-deallocation/bootstrap,
  and 144 ambiguity rows pass for both.
- All 144 contemporary reversals fail for both and naturally quiesce. CJ-B
  achieves exact crossed held-out reuse in 96 rows, performs 480 physical
  deallocations and 480 proposals, but fails the full old/new clause. CJ-E
  achieves exact reuse in 72 rows; old/self-generated activity prevents the
  required old-route deallocation (`0` deallocations) and produces duplicate
  crossed effects. This is a genuine shared boundary, not a speed result.

### Timing transfer, recurrence, replay and closure

- Actual timing training and held-out execution occurs in all 3,024 rows.
  Both establish all 144 train-spacing-0 strata. At train spacing 2/4, CJ-B
  establishes 24/144 per spacing while CJ-E establishes 0/144.
- At held-out spacing 0 both fire 144/144 for every training window. Outside
  same-time completion, CJ-B fires 24/144 at each before/1..5 spacing; CJ-E
  fires 24/144 only in the before (`-1`) case and 0/144 at spacings 1..5.
  Thus B has a mature-coupling source-alone tail; E has a residual-strong-state
  before-order tail. Every timing row closes and quiesces.
- Level-free three-stage recurrence passes `144/144` for both with aggregate
  depth 432 each. Exact replay passes `144/144`; all 8,784 terminal candidate
  rows naturally quiesce and none runs away.

## Scientific classification

**No winner. Both frozen laws fail; freeze the shared boundary.**

CJ-E explains strong-singleton rejection more narrowly than CJ-B, but it still
accepts same-source weak multiplicity and fails contemporary reversal. CJ-B
also accepts the strong singleton, has mature-coupling singleton/crossed tails,
and fails reversal. Since neither remains scientifically sufficient, fewer
work operations or any isolated advantage cannot select it. This is not the
multiple-valid-substrate fork: both fail a genuine physical requirement.

## Work, storage and accounting

Terminal valid-matrix accounting:

| measure | CJ-B | CJ-E |
|---|---:|---:|
| native learning+execution work | 551,136 | 597,216 |
| queue work | 71,544 | 90,744 |
| proposals | 624 | 432 |
| physical deallocations | 768 | 288 |
| local return updates | 3,840 | 12,024 |
| crossings | 7,536 | 7,176 |
| summed per-row persistent bytes | 3,528,192 | 3,515,904 |
| maximum per-row persistent bytes | 1,408 | 1,408 |
| summed temporary-byte lower bound | 965,760 | 965,760 |
| maximum temporary-byte lower bound | 560 | 560 |
| quiescent / runaway | 8,784 / 0 | 8,784 / 0 |

The lane executed 775,393 native CJ-B operations and 827,359 CJ-E operations
across original PROBE/MICRO/GATE plus correction PROBE/MICRO/GATE, including
the immutable coverage-defective result. Persistent sums compare independently
reconstructed rows, not concurrent storage. Candidate-native transmission is
available only for B; E writes `NA`. Neither exact API exposes pending bytes,
so temporary storage is the preregistered equal `LOWER_BOUND`, never reported
as exact or zero.

## Terminal correction artifacts

| artifact | bytes | SHA-256 |
|---|---:|---|
| correction GATE B CSV | 1,076,208 | `69855e0721d9d0c9d80fce1b0d4de7a44ca58955615cd87ff57a814433e9ca45` |
| correction GATE E CSV | 1,080,415 | `6a26e0941b48586aae8b46defa80bae322c57516f1d6e60cb1038d7311b4b855` |
| correction GATE paired CSV | 582,678 | `5de59fffb1461f49dbc52a63a8679d5512c9a0c51038c5bcefdfe43bfcfe8c5b` |
| correction GATE report | 984 | `61b7ed3d0c0685e25a266391c0f0242c1fedf6b70e6b77ed3a62e95bc4ee5415` |

Original PROBE/MICRO/GATE and both correction-stage hashes are frozen in their
stage audits. There are 24 atomic result files totaling 11,056,428 bytes before
this handoff; no staging path remains.

## Validation and hard stop

- focused formatting, all-target check, 8 focused tests and strict Clippy:
  pass;
- candidate hash/cmp, protocol hashes, unique IDs, CSV 54-column shapes,
  paired joins, artifact hashes and work/storage reconciliation: pass;
- no-argument/wrong-argument refusal and no-later-surface preflight: pass;
- changed-path audit: additions only; shared authoritative diff: empty;
- broad root cargo workspace test: intentionally not run because shared code
  remains unchanged;
- branch/tag push, clean worktree and remote-exact verification: required by
  the terminal commit below.

CJ0 remains Classification D. This terminal development handoff does not
restart PX3 and does not authorize any definitive or authority work.
