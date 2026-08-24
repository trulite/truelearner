# PX-C continuous-organism targeted validation audit v1

Status: **TARGETED VALIDATION PASS; DEVELOPMENT AND AUTHORITY MATRICES UNSPENT**.

Frozen candidate `0d03a300bfba4edea72d23cee9e03189ec7bb8d7` / tag
`pxc-continuous-organism-preflight-frozen-v1` was validated in fresh E2B
sandbox `i5tzhdf0ar3y2j5kfx7kq` using unique state file
`/Users/satya/.cache/truelearner/pxc-targeted-20260824-a.json`.

The worker ran release Clippy with warnings denied for the one-file runtime and
the PX-C evaluator. It compiled but did not execute either behavioral matrix.
It then rendered and audited the exhaustive active specification, ran the
frozen taxonomy at zero ceilings, ran the frozen harness and active gates, and
exact-replayed all generated static evidence.

## Positive gates

- canonical runtime SHA-256:
  `e34a9442205fa63d4bde3d286fb7c0c6e722ba04b64c403535e1db71cf3fb8aa`;
- evaluator SHA-256:
  `55f6ad153f58b803d587814ed554521689b2586da4cac60805f600c73f06fb6d`;
- one active production file, `485` lines, `13/13` types, `16/16`
  functions/methods, `29/29` exhaustive entries, zero omissions/extras;
- primary seams `0`, semantic guard `0`, evaluator guard `0`, new kinds `0`,
  new guarded surfaces `0`;
- runtime dependencies `0`, banned vocabulary hits `0`, hidden/cfg/module
  surfaces `0`, and retained authority hashes exact;
- exactly one substrate constructor, empty-clock advance before topology,
  `9` source call sites for the sole continuous `arrive` activity interface,
  `3` ordinary empty-queue time advances, and no evaluator call to `enter` or
  `propagate`;
- development and authority roots are disjoint; all registered origins have
  modulus zero under the frozen ten-tick pressure phase;
- row/report publication precedes assertions;
- PDF is one A4 page and contains every inventory name; fresh 150-DPI visual
  inspection found no clipping, overlap, missing glyph, or illegible entry;
- PDF and all static JSON/CSV audit evidence reproduced byte-exactly in the
  same worker.

## Frozen artifact hashes

| artifact | SHA-256 |
|---|---|
| one-page PDF | `01e7827b58deb52a14d12d11ea5a25e313e13013819e9220fd9c14b41ad15958` |
| fresh page PNG | `80145f7459eb9395c70d54652709ec52123775c6bebe92f5c3ab3417d1837615` |
| active gate JSON | `ddde757611b7cb106271d37ec1249be4e79a409af1a3fafab5e910241afe14c8` |
| exhaustive inventory CSV | `37948272b28190792c05cadf477516b17d489539b31d08aa82ff948ae07a63fa` |
| harness gate JSON | `f4b9a465a635508f24779810c4946a7ba5579e1e6c157ac6647cbb71f7317e53` |
| taxonomy summary CSV | `55a318766e289645a0da947f3cdfeeac82d3c3aa39744a2a68ff910746c911db` |
| taxonomy inventory CSV | `69a462ef864cfea79596d3b4547175a0e6cd14e768f4836da344025bc28f870f` |
| taxonomy guard CSV | `b67add85e46265999a606cb81e866f3d87d56a3e55052e0f5f59036647970cb3` |

The next authorized execution is one complete development invocation of the
frozen evaluator in a new E2B sandbox. Failure freezes a development negative;
no rescue run or mechanism, topology, schedule, bound, or predicate repair is
permitted.
