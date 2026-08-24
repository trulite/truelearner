# PX4 LR-C physical lifetime development protocol v1

Status: **PREREGISTERED DEVELOPMENT PROTOCOL; EVIDENCE UNSPENT; AUTHORITY ABSENT**.

## Lineage and scope

The immutable serial ancestor is
`f9057fe78a86db9111b0b69310d03accef3bc970`, tagged
`px3-lrc-physical-event-authority-v2`. Commits from that ancestor through
`44d696d` are measurement infrastructure only. This lane may develop and
measure PX4, but it may not execute or claim a definitive authority result.

The retained organism law is byte-identical
`crates/lr1-modulatory-physical-return/src/lib.rs`, SHA-256
`7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10`.
The authoritative PX3 handoff is SHA-256
`98067812bc357949af5653a115b353519bede12499804818cfaf4783c0666cbd`.

The question is:

> Does learned physical lifetime reduce completely to existing resistance,
> ordinary pressure, recurrence/reuse, eligibility, and qualified LR-C
> modulation?

No organism-visible lifetime object, history, episode boundary, cleanup or
delete instruction is permitted. The evaluator may observe physical state and
schedule anonymous arrivals, but it may not mutate a learned candidate.

## Smallest existing-physics geometry

The primary world contains exactly three cells:

```text
source --native local proposal--> effect
return --fixed Modulatory arrow--> source
```

The source and effect are within the frozen local variation radius. They are
in different regions, so reuse is an ordinary outward crossing. The return
cell is outside that radius. The only prebuilt arrow is the fixed Modulatory
return arrow. Actual external firing of the source may therefore propose one
weak `source -> effect` Drive arrow through the retained PX0--PX3/LR-C law.

A second control world contains two spatially separated copies of the
source/effect path and one return cell with fixed Modulatory arrows to both
sources. This is the smallest same-substrate geometry that can expose an old
physical organization to disuse while a changed organization receives actual
participation and qualified return.

Cell allocation order, positions, physical identities and presentation phases
are reflected or reversed by the evaluator. These variations do not add an
organism state field or branch.

## Frozen physical schedules

A supported exposure places ordinary Drive arrivals at the source at tick
`t` and at the return cell at tick `t+2`. The native candidate traverses at
`t+2`; qualified Modulatory transmission reaches its eligible source at
`t+3`. Recurrent schedules use starts separated by five ticks. Disuse advances
ordinary substrate time across the retained ten-tick pressure period only
after pending activity has naturally quiesced.

An unsupported exposure omits the return arrival. A late-return control sends
the return only after the four-tick eligibility window. A Drive-return control
changes only the fixed arrow's physical transmission mode. A return-alone
control has no participating candidate. None of these schedules supplies a
boundary, rest state, useful/useless label, future demand, removal command or
semantic contradiction signal.

Changed experience is physical: three supported exposures first organize the
old path; four later supported exposures organize the spatially distinct new
path. Only ordinary elapsed pressure acts on the unused old path. The old path
must physically deallocate while the new path remains live. A subsequent old
arrival may only create a fresh candidate through the ordinary proposal path.

The stale-generation control queues a weak candidate crossing from a source
arrival at tick `9`, queues a second source arrival at tick `10`, and lets the
ordinary tick-10 pressure update deallocate the first candidate before its
delayed crossing arrives. The second firing must create a fresh arrow. Exactly
one effect firing, one physical deallocation, two historical arrow identities,
one dead old arrow and one live new arrow must be observed.

## PROBE

Development identity root: `151001`. The PROBE is conjunctive and checks:

1. one unsupported exposure deallocates after eligibility expires;
2. one qualified exposure reaches resistance `4` without a lifetime field;
3. four recurrent qualified exposures remain live through matched ordinary
   pressure while the one-exposure path deallocates;
4. reuse crosses with mature coupling `2`, performs no new local proposal and
   receives the ordinary qualified `+3` update;
5. continued disuse physically deallocates the supported path;
6. reacquisition creates a distinct arrow through the same source firing;
7. changed experience removes the old path and retains the recurrent new path;
8. stale-generation activity is blocked as specified above;
9. return without participation, late return and Drive return cannot create or
   strengthen the candidate;
10. all propagation naturally quiesces, an exact fresh replay is byte-equal,
    and PX0, PX1, PX2 and PX3 conformance observations are all true.

Any failure freezes the first failing observation. No mechanism repair is
permitted after PROBE execution under this protocol.

## MICRO

Fresh identity roots: `152001` and `152002`; reflected/reversed duplicates use
`152101` and `152102`. Supported exposure counts are exactly `1, 2, 4, 8`.

For every count, MICRO records resistance after recurrence and the number of
ordinary ten-tick pressure steps until physical deallocation. Both quantities
must be strictly increasing in this matrix, and deallocation steps must equal
the observed resistance. A clone advanced through one fewer pressure step must
remain live; the next step must deallocate it.

MICRO also repeats every PROBE causal control, changed-world observation,
reacquisition, stale-generation check, exact replay, layout reflection,
natural quiescence and PX0--PX3 conformance. The complete matrix is
conjunctive. No fitted curve or post-hoc threshold is allowed.

## GATE

Eight fresh identity roots `153001..153008` cross normal/reversed allocation
with forward/reflected geometry. Each row independently executes the complete
PROBE and MICRO claims from fresh substrate state, then executes a second fresh
copy and requires exact structural equality.

The GATE passes only if all eight rows pass every serialized clause, all
layouts share the same normalized resistance/deallocation sequence, every
propagation naturally quiesces, and the following cumulative conformance
observations hold:

- PX0: weak formation, qualified maturation, pressure loss, stale blocking and
  fresh reacquisition use only retained CELL/ARROW/SPIKE physics;
- PX1: return without actual source participation changes no candidate;
- PX2: only the traversed `source -> effect` direction is learned and crossed;
- PX3: recurrent supported organization persists and reuses as an ordinary
  unit, while changed unsupported organization disappears.

The GATE output is development readiness evidence only.

## Frozen commands and artifacts

The sole executable arm will be `arms/px4-lrc-lifetime`. After implementation
freeze, the registered commands are:

```text
cargo run --release --manifest-path arms/px4-lrc-lifetime/Cargo.toml -- --probe
cargo run --release --manifest-path arms/px4-lrc-lifetime/Cargo.toml -- --micro
cargo run --release --manifest-path arms/px4-lrc-lifetime/Cargo.toml -- --gate
```

Each command atomically creates its own CSV and Markdown pair below
`results/px4_lrc_lifetime_*_v1.*`. Each stage runs once in a fresh E2B sandbox
with a unique state file. Formatting, build, Clippy and tests also run only in
a separate fresh E2B sandbox. Exact GATE replay runs from the unchanged clean
implementation snapshot in another fresh sandbox and must produce byte-equal
artifacts.

## PX-C readiness preregistration

If GATE passes, create `experiments/pxc_active_surface_manifest_v2.csv` by
replacing only the PX4 predecessor row. The authoritative LR-C source and all
PX5--PX8 rows remain byte-identical. The new PX4 active source must cover all
new organism-visible geometry or law; evaluator-only runners and tests must be
listed separately with reasons.

The v2 taxonomy and readiness comparator run in fresh E2B sandboxes against
the immutable v1 baseline. The preregistered ceilings are:

```text
primary seams             < 368
semantic guard           <= 218
evaluator guard          <= 752
new seam kinds                0
new guarded surfaces          0
unclassified active files     0
```

Failure of functional evidence, coverage, exact replay or any comparator
criterion freezes a development negative. No authority tag or authority claim
is allowed in this lane.
