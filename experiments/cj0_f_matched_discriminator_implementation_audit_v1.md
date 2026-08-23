# CJ0-F matched discriminator implementation audit v1

Status: **FROZEN EVALUATOR; DEVELOPMENT EVIDENCE UNSPENT**.

## Frozen imports

The comparison workspace is an isolated addition at
`arms/cj0-f-matched-discriminator`. Each candidate is compiled as a separate
crate and the evaluator constructs each `WorldSpec` independently in the two
types before joining observations by the preregistered row identifier.

| artifact | SHA-256 |
|---|---|
| preregistered protocol | `18babd7614aadf0e2a0aa4ea9bc622a4bbff9c3d7ebc0b43c1ba301c088c959a` |
| workspace `Cargo.toml` | `8b549b8ea5ee26db55967b1d5b36650dcbea2378001bb2eb574941b3bb3e5af3` |
| workspace `Cargo.lock` | `3c70cdf3470a8c2570eca0e4559fbe38a1d6186407bbcab5df1df9eb204938ac` |
| CJ-B crate manifest | `1b7468474ee8723ba65d8ec485a36f20fc27ff21f7a186d44f6f49e982e34e79` |
| exact CJ-B law | `ef0de37a9ac54b632b991f0d4647a5ee78c23810084d61497c88d6f757ec2188` |
| CJ-E crate manifest | `e51995207b1f939d75a2a988d4129f43e2ccba5544b18539e0254b58e86049ae` |
| exact CJ-E generated physical law | `e64c8c915c2fbc4679d1e34ee69ecfe36e2c5ff05bdff5d7feeb5a55578bf1c1` |
| comparator manifest | `2f22980013e618bf1ab5ea554e6849570e9597f1800d23073e209d98f4065e01` |
| fixed comparator | `5fa04f4f156920623decd45c6a6336d7830c148c80f57d08289a06ca5fca475a` |

`cmp` verifies the imported law files byte-for-byte against the stated CJ-B
checkout source and the stated CJ-E generated build source. Neither frozen
candidate file is wrapped, generated again, tuned, repaired, or shared with
the other candidate. The only common code is the read-only evaluator-side
construction and normalized accounting adapter.

## Matrix and observability

The deterministic fixed expansion is:

- PROBE: 120 paired rows;
- MICRO: 1,920 paired rows;
- GATE: 8,640 paired rows.

The row serialization contains the stage, family, physical seed, mirror,
insertion order, threshold, coupling, load, spacing, training spacing,
allocation, and evaluator-only genuine/expected classification. Its FNV-1a
fingerprint is identical in the separately written B and E rows. No candidate
output changes a schedule or later input.

The CJ-B native transmission count is retained. CJ-E has no corresponding
read-only counter and writes `NA`, not zero. Neither exact API exposes pending
bytes, so both write the preregistered `LOWER_BOUND` from the maximum entered
SPIKE count times 40 bytes. Candidate-reported persistent bytes remain native.

Artifacts are create-new staged, file-synced, and atomically renamed. Any
existing final or staging path refuses the whole stage before construction.
MICRO requires a frozen PROBE report; GATE requires a frozen MICRO report.

## Pre-evidence validation

- clean frozen start, exact authoritative start commit and tag: pass;
- CJ-B branch/HEAD/clean check: pass;
- CJ-E branch/HEAD/clean check: pass;
- candidate SHA-256 and byte comparison after formatting: pass;
- protocol SHA-256 after evaluator construction: pass;
- focused formatting check: pass;
- focused all-target compile: pass;
- candidate and comparator tests: `6/6` pass;
- strict all-target Clippy: pass;
- preflight: pass, with `definitive=false` and `authority=false`;
- unique row identifiers: pass at all three stages;
- strong-singleton discriminator unit test: CJ-B effect 1, CJ-E effect 0;
- genuine same-tick physical coincidence unit test: both reach effect;
- result and staging paths before PROBE: absent;
- shared authoritative source changes: none;
- broad cargo workspace test: intentionally not run because shared source is
  unchanged.

The implementation exposes only `--preflight`, `probe`, `micro`, and `gate`.
There is no later-stage command or artifact path. PROBE may execute once after
this audit is committed and tagged.
