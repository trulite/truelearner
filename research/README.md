# Research program as code

```text
program -> fork arms -> falsify cheaply -> converge -> freeze -> adjudicate
```

Research owns claims, lessons, protocols, evidence, verdicts, and authority.
It communicates with any implementation process only through neutral protocol
and evidence envelopes.

Its agent procedures live in
[research-program](../.agents/skills/research-program/),
[research-campaign](../.agents/skills/research-campaign/),
[research-converge](../.agents/skills/research-converge/), and
[research-adjudicate](../.agents/skills/research-adjudicate/).

## Discovery and authority

- Discovery runs isolated arms in parallel and may reuse E2B workers.
- Authority freezes one protocol and subject, then uses a fresh one-shot environment.
- A surviving discovery arm is only not falsified within its declared budget.

## E2B batches

`runtime/dispatch_e2b.py` concurrently launches commands from an adapter batch.
Each command must invoke an E2B runner and download a neutral
`research-arm-result/v1` JSON file. Scientific falsification is a valid outcome;
missing results, adapter failures, and timeouts fail the batch mechanically.

```bash
uv run research/runtime/dispatch_e2b.py --batch <e2b-batch.toml> --dry-run
uv run research/runtime/dispatch_e2b.py --batch <e2b-batch.toml>
```

Run the contract suite with:

```bash
uv run -m unittest discover -s research/tests -v
```
