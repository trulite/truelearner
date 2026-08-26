# RS2 learned inhibitory topology implementation audit v4

Status: frozen before RS2 v4 evidence.

Protocol: `0ec2df4` (`rs2-learned-inhibitory-topology-protocol-v4`).

## Exact delta

Only the evaluator's physical-identity allocation changed.

The stale shared counter was replaced by four non-overlapping root-relative
ranges:

- auxiliary pre-generation fixtures start at `root + 10_000`;
- Modulatory source fixtures start at `root + 30_000`;
- probe relay/dummy fixtures start at `root + 50_000`;
- external physical origins start at `root + 70_000`.

CV0/J0 variation continues to allocate contact identities solely through the
substrate. The evaluator does not predict, reuse, or allocate adjacent to those
generated identities after variation.

Training geometry, consequence timing, probe recurrence, nine families, and
all scientific predicates are unchanged from frozen RS2 v3. `truelearner-core`
and all physical laws are byte-identical to the v3 parent.

## Targeted validation

Reusable E2B worker `ifk44bxtlfjlci644r63m`, committed source `3175a90`:

- targeted rustfmt check: PASS;
- targeted release `cargo check`: PASS;
- targeted strict release Clippy: PASS.

No RS2 v4 physical matrix had executed when this audit was frozen.

