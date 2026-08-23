# PX1-PT1 attributed-margin stability implementation audit

Status: **FROZEN IMPLEMENTATION; PROBE EVIDENCE UNSPENT**.

- protocol v2 commit: `f342a0a7fcc55c0abc94ee776af96da8320f2bfb`;
- protocol v2 SHA-256:
  `da8396b1955e393a56be2c770f96b8262bf7fad7e9c71e98a81f5abe9fa38725`;
- implementation:
  `crates/px0-physical-correspondence/examples/px1_pt1_attributed_margin_stability.rs`;
- implementation SHA-256:
  `767b6804faed71913bbbc15794b5a5e2a1c3fd0a1437716ac2aa54e35795236c`;
- PX0 law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

## Physical path audit

Both source-to-endpoint correspondences form through the authoritative PX0
broad local proposal and ordinary returned activity. The source threshold is
the frozen margin value `4`.

Two identical weak branch-to-outlet continuations are installed only after
correspondence acquisition. Physical training activity supports one branch and
outlet. Each outlet sends one impulse to its local trace cell and one to a
global return hub. The hub broadcasts one identical impulse to both trace
cells. Only coincidence between actual outlet firing and global return makes a
trace cell fire and return locally to its branch.

Held-out context activity reaches both branches identically. Both branches may
fire, but only the matured continuation can fire its outlet. The same
coincidence path remains active during held-out and post-gap use; no evaluator
disables plasticity.

## Readout audit

The artifact independently serializes correspondence and continuation
resistance, branch and outlet firings, trace arrivals and firings, local branch
returns, training/held-out/post-gap source refirings, effects, quiescence,
productive recurrence, work, fingerprints, and duplicate exactness.

## Pre-evidence validation

- formatting: pass;
- strict Clippy: pass;
- focused PX0 test: `1/1` pass;
- frozen input hashes: pass;
- semantic/type/provenance source audit: pass;
- fresh result paths: pass;
- non-PROBE/definitive refusal exits `2`: pass.

No PT1 world was executed during implementation validation. Protocol v1 spent
no evidence and remains superseded by v2.

