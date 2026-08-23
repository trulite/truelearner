# PX3-D1-R2 implementation audit v1

Status: **FROZEN; E2B PREFLIGHT PASSED; EVIDENCE UNSPENT**.

- source commit: `df0fe5afd0f1dff700551170d3c41246cfe5395b`;
- manifest SHA-256: `22da14d39473643f02e6e39cb390c394c42625c929fb3d30cf101119b5042e20`;
- source SHA-256: `cc2292954a929c0a9434bf5842dc5da0beb84aca619ad43315d24c64682f0fb3`;
- protocol SHA-256: `8273aa6569cafec456f06fb677d07f2854b3b7512f181a2eeef7abd24cf96e20`.

Persistent E2B sandbox `i6x9gykt9tvp6xfz5z8ra` passed format, release
check, 2/2 static tests, strict Clippy and the non-propagating `--preflight`.
Result/staging and later-stage surfaces were absent. The evidence marker was
not emitted. Only the frozen `--r2` command may spend evidence once.

The implementation serializes both weak P->effect candidate resistance and
fixed O->P connector resistance; candidate success cannot hide connector
plasticity.
