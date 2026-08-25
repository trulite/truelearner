# AH0 handle-ordering removal handoff v1

AH0 is development-ready on `research/ah0-handle-ordering-removal`.

The review target is:

`truelearner/crates/core/src/lib.rs`

It is 5483 lines and is the sole active runtime implementation file. The empty
`src/main.rs` remains only as the host composition root. Durable representation
remains in the separate `arena-format` crate.

AH0's architectural result is:

> CELL and ARROW handles are addressable and generation-safe, but their numeric
> names are not properties of nature.

The next lawful step is RS2 on this hardened parent. CE1, FD2 v2, and frozen ARC
A2 remain blocked behind RS2 exactly as before.

Key evidence:

- `academy/docs/ah0_handle_ordering_removal_result_v1.md`;
- `results/ah0_handle_ordering_removal_v1/execution.log`;
- `results/ah0_handle_ordering_removal_v1/SHA256SUMS`;
- `results/ah0_handle_ordering_removal_v1/cpc0_parent_comparison.txt`;
- `results/ah0_handle_ordering_removal_v1/si0/report.md`;
- `results/ah0_handle_ordering_removal_v1/r6/r6_partition_report.md`.
