# PX0-P coupling diagnostic v1 audit

Status: **INVALID DIAGNOSTIC TRACE; FROZEN; NO MECHANISM CLAIM**.

The aggregate fresh-world baseline again reproduced unsupported A crossing in
both arms. However, the evaluator accidentally stored the pre-forgetting
initial held-out execution in the trace under stage label `renewal-0` and
discarded the actual first post-deallocation renewal trace.

The generated summary, report, and trace are preserved byte-for-byte. They may
not be used to identify when fresh A coupling increased. The only permissible
successor changes the evaluator assignment/label, uses fresh namespaces, and
leaves the active PX0 law and physical schedules unchanged.
