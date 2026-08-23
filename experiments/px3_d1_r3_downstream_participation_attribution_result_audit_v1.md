# PX3-D1-R3 downstream participation attribution result audit v1

Status: **R3-A DEVELOPMENT POSITIVE; FROZEN ROW ORACLE OVER-CONSTRAINED**.

## Frozen evidence

- implementation commit: `ee396ceb164dc194481c6dde2cc483a7eff2b24f`;
- E2B sandbox: `i6x9gykt9tvp6xfz5z8ra`;
- sole registered `--r3` execution completed once and published atomically;
- CSV SHA-256:
  `62b34a64396728c28b617bab75cf1141ee2b2db53897ee655809b6180cb2a67b`;
- Markdown SHA-256:
  `4e0fa8ce6863debbbe0453d89ed433a7f6dfb8505448e33a84901b586b017989`;
- all 18 rows replayed exactly and naturally quiesced;
- structural proposals: zero.

## Registered physical result

The unit-attribution route passed the dynamical safety invariant in both normal
and mirrored insertion order:

```text
AB -> P fires once -> candidate traverses once -> effect fires once
P trace + effect trace + ordinary return -> M fires once
M -> P carries impulse 1
candidate resistance/coupling 1/1 -> 4/2
P remains below threshold 2
P refiring 0; extra candidate traversal 0; world quiescent
```

The causal controls remained separate:

- late A after blocked effect produced no M firing or P credit pulse; the
  candidate remained one, although late A lawfully strengthened the still-live
  fixed O->P connector `100 -> 103`;
- P participation with blocked effect produced P trace but no effect trace, no
  M firing and no credit;
- independently firing every effect produced effect traces but no P traces, no
  M firing and no dormant-candidate credit;
- P and effect participation without ordinary return produced both traces but
  no M firing or credit;
- late ordinary return produced no M firing or credit and the expired AB
  candidate deallocated;
- separated completed AB and CD paths each produced their own P/effect traces,
  M firing and unit credit; candidates AB/CD became four while all crossed
  candidates remained one.

The coupling-two risk control changed only attribution impulse `1 -> 2`. Per
seed it made P fire twice, the candidate traverse twice with total impulse
three, and the effect fire twice. No second ordinary return was supplied, so
the control quiesced after exposing exactly one unsafe re-execution. This
establishes that R3's safety is the physical `return impulse < P threshold`
margin, not hidden execution suppression.

## Why the generated report says `12/18 NEGATIVE`

The six false row verdicts are two copies of three scenarios. Their registered
firing, trace, attribution, credit, resistance target, refiring and quiescence
measurements all matched. The executable additionally required unrelated
fixed/dormant resistance values to remain at their construction values across
later physical-time boundaries:

- `ab-real-late-return`: the used AB O->P connector's eligibility expired by
  tick 9, so ordinary unsupported-use pressure changed it `100 -> 99`;
- `ab-then-cd-two-completed`: the first AB connector similarly became 99 before
  the second episode completed;
- `ab-suprathreshold-return-control`: reaching tick 10 applied ordinary global
  pressure. The five unused weak candidates became zero, the credited AB
  candidate became three, all connectors became 99, and the expired used AB
  connector received its additional unsupported-use decrement to 98.

These are authoritative PX0 pressure effects. The protocol required them to be
serialized; it did not require fixed connectors or dormant weak candidates to
ignore elapsed time. They do not create attribution, credit, P firing or a
candidate traversal. The write-once generated Markdown remains unchanged as
the literal executable output; this audit records the defective derived row
verdict.

## Scope

R3-A is development evidence that trace-gated, subthreshold return can credit
an eligible pair candidate without restarting its causal chain. It does not
establish candidate formation, recurrence-based persistence, reproposal,
reversal, D2, MICRO, GATE, authority or complete PX3 recursion.
