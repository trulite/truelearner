# PXR0 single-file physical-runtime result audit v1

Status: **FROZEN DEVELOPMENT NEGATIVE; SCIENTIFIC FORK; NO AUTHORITY CLAIM**.

Candidate commit/tag is
`f379767a0d7380fd0728b457538e6d6ab5ba4829` /
`pxr0-single-file-physical-runtime-candidate-v1`. It descends directly from
PX8 authority `9ba11aeb4f88d6707cfba1afdb5f6dce3e380b9f`. The canonical runtime was not
changed after candidate freeze.

## Surface and page gates

The active manifest contains one 474-line Rust file, 13 types and 15
functions/methods, 28 entries total. The exhaustive inventory has zero omitted
or extra source/spec/PDF entries. Runtime dependencies are empty; retained
PX0--PX8 hashes are exact; forbidden modules/includes/macros/cfg/tests are
absent. Manifest-v6 movement is 101 entries/four files to 28 entries/one file:
28 physical entries retained or canonically compacted and 73 entries moved to
research/audit tooling.

Taxonomy v2 is primary `0`, semantic guard `0`, evaluator-input guard `0`, new
kinds `0`, and new guarded surfaces `0`. The A4 portrait specification is one
page, minimum 10-point text/12-point leading, contains all 28 entries, and was
rendered to PNG and visually inspected without clipping, overlap, missing text,
or illegible compression.

## Targeted E2B validation

- formatting-preparation sandbox: `i0jx9po9go2g591iooxxu` (Rustfmt only; no
  compilation or organism execution);
- frozen-candidate validation sandbox: `iopr5tq8dkzkbmuq2vo4q`, snapshot
  `f379767a0d7380fd0728b457538e6d6ab5ba4829`;
- runtime and evaluator format checks passed;
- runtime and evaluator Clippy checks passed;
- extraction map and taxonomy gates passed;
- PDF marker ran exactly once before the only render;
- static/page/dependency gate emitted `PXR0_STATIC_GATE_V1_OK`;
- readiness matrix executed exactly once and published artifacts before its
  aggregate assertion.

The initial validation command stopped after compilation/extraction because
the archive-mode taxonomy invocation omitted `PXC_AUDITED_COMMIT`. The same
sandbox and snapshot continued from that boundary with the exact frozen commit
supplied. Compilation/extraction were not repeated; the PDF marker/render and
readiness matrix each ran once.

## Frozen behavioral result

The result is 8/16 rows, 296/384 row clauses, 8/8 global clauses, and 304/392
total clauses. Exact complete-trial replay is true for all rows. Every advance
is naturally quiescent. Maximum work is 14,762 under the 20,000 bound; maximum
resident memory is 5,936 bytes under the 8,192 bound. All incomplete, blocked,
open, branch, cycle, aged, changed-experience, and memory-delta safety clauses
remain positive.

Rows at schedule shifts `0` and `411` pass. Every row at shifts `137` and `274`
fails. The first failed registered clause is clause 3, returned paired traversal
and strengthening. At shift `137`, pair updates are 0, nested formation updates
are 3, and outward crossing is 0. At shift `274`, pair updates are 1, nested
formation updates are 7, and outward crossing is 0.

## Scientific fork and stop

Authoritative pressure uses absolute ten-tick epochs. Advancing an empty world
to a non-multiple-of-ten schedule offset leaves the pressure epoch at the prior
multiple of ten. Relative pressure therefore lands at tick 3 for offset 137,
tick 6 for offset 274, tick 9 for offset 411, and tick 10 for offset 0. The
registered expectation that all four translated schedules preserve behavior is
not a property of the retained law.

Making every translated row invariant requires a new decision: preserve the
pressure phase when translating a world, choose phase-equivalent fresh
schedules, or change the pressure law. The frozen protocol authorizes none of
these after observation. PXR0 development readiness is therefore not
established. No successor-authority or PX-C evidence was spent.
