# DS4 cumulative request/start definitive protocol

Protocol identifier: `ds4-cumulative-request-start-definitive-v1`

Status: **PREREGISTERED; NO DEFINITIVE DS4 EXECUTION HAS OCCURRED**.

This protocol spends one authoritative matrix on one question:

> From blank M3/P4 state in every cell, does the byte-frozen cumulative DS4
> development mechanism learn a request role and initiate the frozen recurrent
> computation only from learned M3 event-completion activity, with neither a
> supplied request/query marker nor supplied `START` meaning?

The matrix is conjunctive. A request role without learned initiation is
negative. Learned initiation with a separately supplied target identity is
negative. Development evidence cannot substitute for this matrix.

## Frozen development ancestor

- exact readiness commit/tag:
  `3a82adc23fd179058f01d5004e894833f1cad0f4` /
  `ds4-cumulative-request-start-development-readiness`;
- authoritative M3:
  `ffcdfe8b36fc62348b7ebcb09aaf4797f6146ba8` /
  `m3-cumulative-event-boundary-authoritative`;
- DS4 cumulative port SHA-256:
  `b65b28256d58c184b41bf2ff8d383c99593e6d812480751684209dce1d82f99a`;
- DS4 development runner SHA-256:
  `4f287d66486514dea70cca9fb701e730a8c9e603731fd8159af6ffa7ddfa6846`;
- build composition/hash plumbing SHA-256:
  `2cf2eba6d7ca16f7f18076f0999571bb207f341fa2f2a9a824966bd2ddd8415b`;
- readiness handoff SHA-256:
  `9cbc989c8544d1e94ec359197d085dbd1679ba28ed43b9d158b9655c11093235`;
- amended development protocol SHA-256:
  `a1460c0d30f55edb16888ef4c93d119586cf24fe206cb3a7362c08cee5187e95`;
- DS4--DS8 target freeze SHA-256:
  `f10f9d7b16106b6014767ff6188a6d556145ba3e5b4335e28de245c7622a7595`.

The definitive wrapper may compose-copy and call the exact frozen development
port. It may enumerate cells, verify hashes, duplicate replay, aggregate
results, print diagnostics, and serialize create-new artifacts. It may not
alter any organism observation, state, threshold, selection, execution,
credit, recurrence, event, role, or linker path.

## Frozen matrix

Run exactly sixteen sequential, deterministic, single-threaded blank-start
cells:

```text
cell ids          0..15
base seed         4_000_000 + cell_id * 100_000
M3 acquisition    2 fresh cumulative streams
P4 acquisition    depths 1,2,3,4 rotating until role competence
competence cap    4,000 request episodes
held-out          32 fresh episodes per cell
held-out depths   5,8,16,32 rotating
request positions all 6 observable positions per cell
duplicate replay exact blank-start rerun of the complete cell
```

Total held-out population is `512` executions. All definitive namespaces are
disjoint from DS4 PROBE/MICRO/GATE, every prior definitive matrix, and every
future downstream matrix. Every M3 occurrence, P4 receptor, opaque identity,
serialization order, and request episode is fresh within the deterministic
cell namespace.

Each cell starts with:

- a fresh M3 boundary learner with no support or chunks;
- a fresh P4 request-role learner with no pattern, strength, or role;
- no retained occurrence, request, target, receptor, route, or answer handle;
- only the byte-frozen later-stage supplies explicitly left intact by DS4.

M3 first acquires its frozen event organization through the cumulative M0--M2
path. P4 then acquires only when learned M3 completion activity reaches the
frozen linker. Held-out evaluation calls both mechanisms non-plastically.

## Per-cell ordered gate

Every cell reports the first collapse in this order:

1. **P0 exact-source authority** -- readiness ancestor, M3 authority, port,
   runner, build, handoff, target, protocol, and frozen M3/P4 sources/results
   match exactly.
2. **P1 learned physical initiation** -- learned M3 uses, completion activity,
   P4 selection, execution, and training update are all nonzero; the no-event
   path is silent.
3. **P2 request acquisition** -- exactly one learned request role consolidates
   within 4,000 episodes from pre-answer traces.
4. **P3 held-out function** -- all `32/32` executions are correct, explicit,
   and naturally quiescent across all four depths and all six request
   positions.
5. **P4 controls 1--12** -- the frozen development controls below all pass in
   the cell.
6. **P5 replay/work/lifecycle** -- duplicate cell replay is exact, cumulative
   M3 and P4 work is nonzero, acquisition/held-out namespaces are disjoint,
   and held-out persistent state is unchanged.

Later stages are blocked after the first collapse. Matrix PASS requires all
six stages in all sixteen cells.

## Frozen controls in every cell

1. Learned M3 event activity is required for selection, execution, and update.
2. Generic subthreshold M3 spans with zero learned use do not activate P4.
3. Missing M3 close produces no activation.
4. Invalid transition is silent and a later valid event reenters.
5. Fresh/relabelled M3 identities and reversed allocation transfer.
6. Fresh request serialization covers all six positions.
7. Symmetric indistinguishable requests form no stable role or competence.
8. The selected request trace exists before recurrence, output, and terminal
   credit.
9. The selected opaque identity is recovered only from its anonymous
   occurrence; no separate target channel crosses the linker.
10. Frozen source, persistent-state, hash, and information-flow audits pass.
11. M3 chunk/byte summaries and exact P4 fingerprint remain unchanged through
    held-out evaluation; temporary state is erased.
12. Duplicate replay is exact and acquisition/held-out namespaces are
    disjoint.

Evaluator expected answers and target identities may be used only after a
complete execution to score external behavior and to supply the still-frozen
DS8 terminal polarity during acquisition. They may never enter event
completion, request selection, initiation, execution input, or persistent
state.

## Conjunctive interpretation

PASS:

```text
16/16 cells pass P0..P5
512/512 held-out executions are correct, explicit, and quiescent
192/192 numbered controls pass
16/16 duplicate replays are exact
zero first collapses

M4 becomes the authoritative cumulative ancestor.
DS5 cumulative becomes eligible.
```

FAIL or incomplete:

```text
M3 remains authoritative.
M4 remains absent.
DS5 cumulative remains blocked.
the result is frozen without rescue or rerun.
```

Program-priority decisions are outside this matrix's evidentiary scope.

## Write-once execution discipline

- The pre-implementation `results/` tree digest is
  `97b85f9056a8404fb2caf81e0fa8e3a1cb06398533874a474a9fe2c9696797a4`,
  computed from the sorted per-file SHA-256 records.
- The definitive implementation is a wrapper over exact readiness commit
  `3a82adc`; development source is never edited or reformatted.
- Preflight may run formatting, compilation, strict Clippy, focused tests, and
  one non-claim audit using development base seed `95_000` only. It may not
  enumerate, sample, print, or partially execute a definitive seed.
- All Rust work runs through
  `/Users/satya/work/br/truelearner/scripts/e2b_persistent.py` with
  `/Users/satya/work/br/truelearner/.env.e2b` and a dedicated definitive state
  file separate from development.
- The frozen `--definitive` command executes exactly once. Once cell 0 begins,
  the outcome is spent. No rescue, tuning, reinterpretation, or rerun is
  permitted.
- Infrastructure failure before P0 spends no evidence. Interruption after cell
  0 begins freezes an incomplete negative outcome.
- Serialize only after all cells finish and create, never overwrite:
  `results/ds4_cumulative_request_start_definitive.csv` and
  `results/ds4_cumulative_request_start_definitive.md`.
- Preserve every pre-existing result byte-for-byte. Freeze result hashes and
  the post-run authority handoff in descendants of the write-once artifacts.
- Leave every E2B sandbox running; do not kill the definitive sandbox.

This protocol commit spends no definitive evidence. No definitive
implementation or execution is authorized until it is committed and tagged.
