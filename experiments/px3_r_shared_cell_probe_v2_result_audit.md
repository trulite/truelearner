# PX3-R Arm B anonymous shared-CELL PROBE v2 result audit

Status: **FROZEN NEGATIVE; V2 ARTIFACT FIRST-CLAUSE FAILURE; PX3 AUTHORITY ABSENT**.

## Frozen execution

The sole v2 command ran once from implementation commit
`ccea20467736ad36faafaa9d5415612b2b81f80e`, tag
`px3-r-shared-cell-probe-v2-implementation`, and emitted exactly one
`PX3_R_SHARED_CELL_PROBE_V2_EVIDENCE_SPENT` marker.

- artifact classification: `FIRST_CLAUSE_FAILURE`;
- terminal Arm B interpretation: `FROZEN_NEGATIVE_SHARED_CELL_RECRUITMENT`;
- process exit: `1`;
- source SHA-256:
  `1ae7592134db8692a95b2ca38d837306c5021f79df9e2ddf9f5e32b6d660631d`;
- CSV: `results/px3_r_shared_cell_probe_v2.csv`, SHA-256
  `bb95ff9c05b9105ae2116c4efc6ab8c4f228907eb5feaa42e8a8f7836cae8298`;
- report: `results/px3_r_shared_cell_probe_v2.md`, SHA-256
  `e4e4b2c598c15fab0a03e259c22eec075a5d18a0ec5e2f6319b1c222277354d1`;
- CSV shape: `11` data rows and `31` columns;
- CSV/report storage: `6,584` and `1,471` bytes;
- total ledgered work: `1,700,400` operations.

The artifacts are final and may not be regenerated or rerun.

## Matched discriminator: pass before swap

The conservative physical opportunity cleanly passed the initial mandatory
discriminator:

- route `A`, `B`, `C`, and `D` strength/live multisets were exact equals:
  resistance `0|0|33`, live `false|false|true`;
- all 12 source ports fired exactly `16` times;
- anonymous-CELL training and outward counts were
  `16|0|0|0|0|16`;
- sites `A+B` and `C+D` retained resistance `33|33` and were live;
- all other sites retained `0|0` and were dead;
- held-out `A+B` fired only its trained anonymous CELL and crossed outward
  exactly once;
- held-out `C+D` did the same;
- held-out `A+D`, `C+B`, and the other crossed combinations fired no anonymous
  CELL and produced no outward crossing.

Thus trained versus crossed differed physically under exact individual-route,
frequency, traversal, total external input, early/late timing, return,
pressure, and outward-training marginals. No individually stronger route can
explain the difference.

The mirrored/reversed-allocation/reversed-identity replica was exact in
behavior, and the fresh stable alternative correctly retained only `A+D` and
`C+B` at resistance `33|33`.

## Mandatory swap: decisive failure

After `20` matched contemporary swap rounds:

- old `A+B` and `C+D` incoming ARROWs weakened from `33|33` to `0|0` under
  ordinary pressure and unsupported use;
- new `A+D` and `C+B` also remained `0|0`;
- all four route strength multisets were again exact equals, `0|0|0`;
- every swap training anonymous-CELL firing and outward count was zero;
- every post-swap held-out anonymous-CELL firing and outward count was zero.

The physical cause is fixed by the authoritative PX0-PX2 law. Initial
coupling-`2`, resistance-`1` opportunity ARROWs that are unused are physically
deallocated. A later external source firing invokes the retained local
proposal law, which recreates a local ARROW only at coupling `1`, resistance
`1`. Two simultaneous recreated inputs deliver `1 + 1 = 2`, below anonymous
CELL threshold `4`. The CELL cannot fire, cannot return a SPIKE, and cannot
stabilize those ARROWs; ordinary pressure removes them again.

Contemporary swap/relearning therefore requires a changed ARROW allocation or
proposal law capable of allocating conjunctive-compatible physical coupling,
or another new substrate mechanism. That is outside Arm B. Changing the law,
retaining evaluator-selected dormant ARROWs, creating a CELL, or adding a
semantic lifecycle would be a forbidden rescue.

## Hard controls

- complete-state duplicate replay: exact for every case;
- natural quiescence and zero autonomous source refiring: pass;
- correlated-without-participation: absent;
- participation with blocked return: no reusable held-out structure;
- absent opportunity: absent;
- stale opportunity: physically deallocated and absent;
- ambiguous three-route occurrence: all three actually activated structures
  persisted;
- genuine four-route co-participation: all six structures persisted;
- stable fresh alternative: pass;
- spacing replica (`gap 6`, spacing `14`): fail; a pressure edge deallocated
  the second combination's weak ARROWs before their first returned SPIKE, and
  the coupling-`1` proposal law could not recreate a threshold-`4` structure.

The spacing failure is preserved as part of the literal v2
`FIRST_CLAUSE_FAILURE`. No schedule retry is authorized. Independently, the
mandatory swap failure is sufficient to freeze Arm B negative under the
preregistered no-further-opportunity rule.

## Integrity and stop

- frozen start, authoritative parent, authority source, PX3 negative, v1
  source, and v1 result hashes: exact;
- organism-visible forbidden-information scan: pass;
- shared/authoritative files changed: none;
- source, result, and staging paths were fresh and atomic;
- authority/definitive matrix: not run or simulated;
- MICRO/GATE: not entered because PROBE did not establish a candidate;
- broad historical suite: not run because shared code did not change.

Arm B stops. Advancing requires explicit authority for an additional substrate
allocation/proposal law outside this development arm.
