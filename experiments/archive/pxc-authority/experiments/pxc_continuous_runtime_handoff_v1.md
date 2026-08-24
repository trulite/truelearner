# PX-C continuous-runtime handoff v1

## Canonical production surface

The complete active organism is
`crates/pxr0-physical-runtime/src/lib.rs`, SHA-256
`e34a9442205fa63d4bde3d286fb7c0c6e722ba04b64c403535e1db71cf3fb8aa`:
one Rust file, `485` lines, `13` types and `16` functions/methods (`29` total
entries). The mechanically reconciled exhaustive inventory is
`results/pxc_active_gate_v1/inventory.csv`; the legible one-page rendering is
`output/pdf/pxc_active_runtime_spec_v1.pdf`.

The sole PX-C production addition to accepted PXR0 v2 is:

```rust
pub fn arrive(&mut self, inputs: &[SpikeInput], outward_region: i16) -> RunResult
```

It enters anonymous physical inputs through the frozen `enter` transition,
runs the frozen `propagate` scheduler to natural quiescence, and only then
filters the already-produced crossing vector to the requested outward region.
The filter cannot feed back into state, work, queue order, pressure,
plasticity, or later arrivals.

## Canonical Rust review conclusions

1. `external_arrival` proposal gating is a legitimate retained physical
   boundary law: ordinary external spike firing is the preregistered proposal
   cause in PX0-R and the exact gate was retained by LR1 authority. The new
   `arrive` method does not select or invoke that mechanism.
2. `region` is causally inert except at boundary observation. It is stored on
   cells, compared only to recognize a physical crossing, copied into
   `Crossing`, and used by `arrive` only after propagation. It does not alter
   activation, threshold, refractory state, traversal, eligibility,
   resistance, pressure, proposal, generation, queue order, or decay.
3. `Work` and `resident_bytes` are causally inert observer surfaces. They are
   valid bounded-work/allocation witnesses but are not physical inputs. A
   future optimized kernel could place them behind an observer boundary;
   PXR0/PX-C retain them to preserve the frozen evidence contract.
4. `Cell.live` is causally active. `Cell.generation` is read but remains fixed
   in the present kernel, so it is redundant state. Stored `Cell.resistance`
   is initialized but never read after construction, so it is scaffolding.
   Removing either redundant field would change the frozen runtime hash,
   memory evidence, and public bootstrap contract and therefore belongs only
   in a separately preregistered successor study.
5. Pressure phase is intrinsic substrate time with epoch zero and period ten.
   Equivalent translated constructions first advance an empty substrate to an
   origin congruent modulo ten, then build identical topology and apply
   identical relative timings. Noncongruent origins may lawfully differ.

The exhaustive review found no hidden semantic or test-world logic in the
production file. All named topology, choreography, clause mapping, reporting,
hashing, and inspection live in causally inert evaluator/audit tooling outside
the active closure.

## Oracle-ready completion statement

Given the exact canonical runtime hash above and the authority evidence rooted
at disjoint identities `3_200_001..3_200_016`, the PX-C authority oracle is:
`524/524` conjunctive clauses, exact replay, natural quiescence, outward-only
crossings, work at most `105446`, retained allocation at most `39248` bytes,
one page / `29/29` entries, and primary/semantic/evaluator/new-kind/new-surface
state `0/0/0/0/0`. Any byte change to the canonical runtime, evaluator,
manifest, active gate, firewall, matrix, or one-page specification invalidates
this authority and requires a new preregistered successor workflow.
