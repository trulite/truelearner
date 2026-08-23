# CJ0 ARM CJ-B PROBE v2 implementation audit

Status: **ACCOUNTING CORRECTION FROZEN; PROBE UNSPENT**.

- parent correction protocol: tag
  `cj0-b-locally-gated-arrow-probe-v2-accounting-protocol`;
- physical module SHA-256:
  `ef0de37a9ac54b632b991f0d4647a5ee78c23810084d61497c88d6f757ec2188`
  (exactly unchanged from frozen v1);
- evaluator SHA-256:
  `f8df9f19e76ff800ddcbd24a4eb7be7743c208dc30bfc6be962d6a5de9ce57ca`;
- changed executable lines: the candidate-consumption projection in
  `observe` only.

Focused formatting, two unit tests, strict all-target Clippy, and no-CELL
preflight pass. Frozen authoritative hashes, zero dependencies, forbidden
physical-source scan, refusal behavior, added-path isolation, and artifact
absence remain exact. No scientific cell has executed and no result path has
been written.
