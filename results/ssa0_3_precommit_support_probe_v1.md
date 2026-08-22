# SSA0.3 pre-commitment support PROBE v1 result

Outcome: **PASS — EARLY-VERSUS-LATE DELIVERY REACHES REAL PHYSICAL
COMPETITION**.

This is a developmental PROBE, not a final SSA0.3 classification. It authorizes
only the preregistered MICRO. It is not definitive evidence, does not reinterpret
immutable SSA0 Classification C, does not advance M6 or create M7, and uses no
Lane A evidence or mechanism.

## Frozen lineage and artifacts

- frozen SSA0 parent/result commit/tag:
  `34277893201c1a72765b143de4b3da1912b6e3b6` /
  `ssa0-spatiotemporal-affordance-micro-v1-negative`;
- SSA0.3 protocol commit/tag:
  `c6f28ce979f05c358f313ee4fc202a6304fa1b70` /
  `ssa0-3-precommit-support-protocol-v1`;
- SSA0.3 implementation commit/tag:
  `ace5f39e77e83ca0478c283a157d9f7dd2f87429` /
  `ssa0-3-precommit-support-implementation-v1`;
- protocol SHA-256:
  `f9121b0b08867b4189892ab9f46658ebd4a95874c73c5a9182bce17f4f49cef1`;
- implementation source SHA-256:
  `4a4e727f4f8ca6ee03faaae76de1a1091472de20ed9d91388e7a36056326edd7`;
- runner SHA-256:
  `3711f123be3a4efc0494cc01f85fd8bc176ffc765eb1a2183b1e14c76baea435`;
- raw PROBE SHA-256:
  `3b03e47107fa74e2e7c6116ccdfb45100bde7ae261b320f955ac0d1d9185a32f`.

## Ordered outcome

| target physical route | world | total target:competitor | winner | commitment tick | duplicate | result |
|---:|---|---:|---:|---:|:---:|:---:|
| 0 | base | 4:4 | 1 | 7 | exact | PASS |
| 0 | extra early at tick 6 | 5:4 | 0 | 6 | exact | PASS |
| 0 | extra late at tick 10 | 5:4 | 1 | 7 | exact | PASS |
| 1 | base | 4:4 | 0 | 7 | exact | PASS |
| 1 | extra early at tick 6 | 5:4 | 1 | 6 | exact | PASS |
| 1 | extra late at tick 10 | 5:4 | 0 | 7 | exact | PASS |

The same count increase therefore reached the physical race in both arms. An
ordinary fifth supporter delivered before the baseline fourth-arrival boundary
advanced the target contender's threshold crossing and changed the realized
effect. The identical count increase delivered after the competing contender
had fired did not change the committed route. This is only the preregistered
reachability result; MICRO and GATE remain necessary for the causal law.

## Controls and focused validation

All PROBE controls passed: byte-exact copied physics, forbidden-primitive
absence, complete-state duplicate exactness, fresh route identity, fresh
occurrence identity, independent handle/allocation/layout permutations, full
A/B physical mirror, blocked route cannot win, stale route cannot win, and
independent execution of both routes.

Pre-execution focused implementation validation was:

```text
cargo fmt --all -- --check                                      PASS
cargo test --release --bin ssa0_3_precommit_support             PASS (3/3)
cargo clippy --release --bin ssa0_3_precommit_support -- \
  -D warnings                                                    STOPPED
  only on the ten pre-existing frozen/generated warnings already recorded by SSA0
cargo clippy --release --bin ssa0_3_precommit_support -- \
  -D warnings \
  -A clippy::derivable_impls \
  -A clippy::manual_is_multiple_of \
  -A clippy::manual_div_ceil                                    PASS
cargo run --release --quiet --bin ssa0_3_precommit_support -- --definitive
                                                                 REJECTED, exit 2
cargo run --release --quiet --bin ssa0_3_precommit_support -- --probe
                                                                 PASS
```

No broad historical suite ran. No frozen/shared source was changed.

## Isolation and stopping state

The frozen parent protocol, implementation, runner, result Markdown, and result
CSV remained byte-identical. Authoritative M6 remains
`aa4e22efd8a65b7694956a53cfaa970582695215`; M7 does not exist. No definitive
run, SSA1, SSA2, randomness usefulness, creative diversity, adversarial
unpredictability, or generative trajectory work occurred.

PROBE does not assign A/B/C/D. Its ordered state is **PASS / CONTINUE TO
PREREGISTERED MICRO**. There is no blocker.
