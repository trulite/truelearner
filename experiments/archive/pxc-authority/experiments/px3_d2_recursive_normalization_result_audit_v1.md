# PX3-D2 recursive normalization result audit v1

Status: **D2-A POSITIVE; FROZEN HARNESS VERDICT ORACLE OVER-CONSTRAINED**.

## Frozen evidence

- implementation commit: `462c9986a61c16b453d2c1f14b873b36fd3f9392`;
- E2B sandbox: `i6x9gykt9tvp6xfz5z8ra`;
- sole registered `--d2` execution completed once and published atomically;
- CSV SHA-256:
  `8d82057b3a87fed3019633de1d4868080ccc75f2873a87b91ab803d949547dbe`;
- Markdown SHA-256:
  `d137f9adfc5acb363d3ecfa8a929622c7844c1dd9444f8ef7238fd766feea7c6`;
- all 20 rows replayed exactly and naturally quiesced.

## Registered D2 result

All registered physical requirements passed in both normal and mirrored
insertion order:

- A alone, B alone, strong A(4), and repeated A produced no AB execution, X
  outlet, or X trace;
- mature AB coupling 1, 2, and 4 each produced one AB firing, one mature
  crossing carrying the raw mature impulse, one X outlet execution, two unit
  normalization arrivals, and exactly one unit X trace firing;
- overlapping X+C produced one downstream XC firing per seed;
- gapped X then C produced no downstream XC firing;
- primitive D+C produced one matched downstream DC firing per seed.

The mature organization therefore compressed to one ordinary participation
trace independently of its raw coupling, and that trace used the same
threshold-two downstream API as a primitive trace. This is D2-A.

## Why the generated report says `NEGATIVE`

The frozen executable marked 14/20 rows passed because its row oracle imposed
an extra condition absent from the protocol: it required C's unit trace not to
cross into an inactive comparison opportunity.

The topology intentionally connects C to both XC and DC. Consequently:

- in each `x-plus-c-overlap` and `x-then-c-gapped` row, C lawfully crossed into
  dormant DC once, while DC did not fire;
- in each `d-plus-c-primitive-baseline` row, C lawfully crossed into dormant XC
  once, while XC did not fire.

Those are the six false row verdicts. They are symmetric opportunity fan-out,
not extra participant traces or false conjunctions. Every registered firing,
normalization, timing, replay, and quiescence requirement still has its exact
expected value. The write-once Markdown report is retained unchanged as the
literal program output; this audit records the defect in its derived verdict.

## Scope

D2-A establishes only level-blind normalization and the matched downstream
equivalence control. It does not establish AB learning, return attribution,
candidate formation, Y/Z learning, or full PX3 recursion.
