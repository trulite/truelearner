# PXR0 single-file physical-runtime handoff v1

Status: **CANONICAL CANDIDATE FROZEN; DEVELOPMENT READINESS NEGATIVE; HUMAN REVIEW REQUIRED**.

The exact candidate is the one-file runtime at
`crates/pxr0-physical-runtime/src/lib.rs`, SHA-256
`f6989555f5a43dff91b39a5c7f79038168f39142fdbecca7e5e40938a72785cb`.
It is 474 lines with 13 types and 15 functions/methods. The runtime has no
dependency and all geometry/choreography remains in tooling.

Static extraction, one-page exhaustiveness, dependency direction, taxonomy,
leakage, retained-hash, work, memory, quiescence, negative counterexample, and
exact-replay gates pass. The cumulative behavioral verdict remains negative
because absolute pressure phase makes preregistered shifts 137 and 274 differ
from shifts 0 and 411. See the frozen result audit and unconditional CSV.

The next permitted action is joint human review of the exact canonical Rust
file and the pressure-phase fork. This branch must not be tagged development
ready, promoted to PXR0 authority, or used for PX-C evidence without a new
approved protocol decision.
