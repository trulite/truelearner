# PX-C PX__ development-readiness seam handoff v1

Status: template. Replace `PX__` and every placeholder before freezing a lane
handoff. A functional positive does not authorize this handoff by itself.

## Lineage

- serial authority ancestor: `____________`
- development implementation commit: `____________`
- before active-surface manifest SHA-256: `____________`
- after active-surface manifest SHA-256: `____________`
- E2B sandbox: `____________`

## Functional result

- protocol/result: `____________`
- verdict: `PASS | FAIL`
- exact replay: `true | false`
- natural quiescence: `true | false`
- first collapse, if any: `____________`

## Manifest coverage

- predecessor entries replaced: `____________`
- new active mechanism sources: `____________`
- evaluator-only sources excluded with reasons: `____________`
- source/dependency audit proving complete candidate coverage: `____________`
- unclassified active source files: `0`

## Mandatory PX-C delta

| metric | before | after | delta | accepted |
|---|---:|---:|---:|:---:|
| primary seams | N | M | M-N | true/false |
| semantic guard | X | Y | Y-X | true/false |
| evaluator guard | P | Q | Q-P | true/false |
| new seam kinds | 0 | 0 | 0 | true/false |
| new semantic surfaces | 0 | 0 | 0 | true/false |

Attach the generated:

- readiness delta CSV and Markdown report;
- primary-kind delta CSV;
- layer delta CSV;
- new-seam-kinds CSV;
- new-guarded-surfaces CSV;
- after taxonomy inventory, guard inventory, and summary;
- exact artifact hashes.

Required E2B comparator invocation:

```text
PXC_BEFORE_MANIFEST_HASH=<hash> \
PXC_AFTER_MANIFEST_HASH=<hash> \
PXC_REQUIRE_PRIMARY_DECREASE=1 \
scripts/compare_pxc_readiness_delta_v1.sh \
  PX__ \
  <before-summary.csv> \
  <before-guard.csv> \
  <after-summary.csv> \
  <after-guard.csv> \
  <output-directory>
```

## Readiness verdict

Set development readiness positive only when:

1. the lane's functional gate passes;
2. primary seams strictly decrease;
3. semantic and evaluator guards do not increase;
4. new seam kinds equal zero;
5. new semantic surfaces equal zero;
6. complete active-surface coverage is proven; and
7. exact E2B replay passes.

Otherwise freeze the lane as a functional or physicalization negative. Never
repair the delta criteria after seeing the result.
