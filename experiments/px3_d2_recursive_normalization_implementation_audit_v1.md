# PX3-D2 recursive normalization implementation audit v1

Status: **FROZEN; E2B PREFLIGHT PASSED; EVIDENCE UNSPENT**.

- source commit: `23b23a6fc7ff4eb61c1496b31056291b1cb96e1e`;
- manifest SHA-256: `ea543f393516760a5c603a2c964b82cccd025432bbcf84764b8e25b86badaed3`;
- source SHA-256: `cfab51961825e4d46474923c969e8af5a13f7eb13b7abb077ed7f943951c33de`;
- v2 protocol SHA-256: `8f8c092668e355589fdf81e60e644719e81adabcd1db6853fa5eb585b5b0abc5`.

Persistent E2B sandbox `i6x9gykt9tvp6xfz5z8ra` passed format, release
check, 2/2 static tests, strict Clippy and the non-propagating `--preflight`.
Result/staging and Y/Z surfaces were absent. The evidence marker was not
emitted. Only the frozen `--d2` command may spend evidence once.
