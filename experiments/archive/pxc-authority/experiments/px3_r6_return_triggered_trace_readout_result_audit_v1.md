# PX3-R6 return-triggered trace readout result audit v1

Status: **FROZEN R6-A POSITIVE; PX3 AUTHORITY REMAINS NEGATIVE**.

- implementation commit/tag: `b533bfdfbe3e64bd1433af89015cb533c4bdb603`,
  `px3-r6-return-triggered-trace-readout-implementation-v1`;
- E2B sandbox: `i6x9gykt9tvp6xfz5z8ra`;
- sole marker: `PX3_R6_RETURN_TRIGGERED_TRACE_READOUT_EVIDENCE_SPENT`;
- CSV SHA-256: `35b68303630f69c326fadad1ccc988e807ae0e1a77703b4e751732e0cdeae4d8`;
- report SHA-256: `32f338c9eae943159ef54015322973f29983fb899eeddd71f4e0fa35a5c6d796`.

The one-shot result passed `12/12` rows and `60/60` validity clauses with
exact replay and natural quiescence.

The decisive stress control executed 100 consecutive physical P/X episodes
without R in each insertion stratum. FP and FX each received 100 ordinary unit
footprints but never fired. M received exactly zero arrivals and fired zero
times. Partial evidence therefore never entered the attribution cell.

With timely physical R, its normalized unit trace supplied the second unit to
each still-live footprint. FP and FX fired once, M received exactly two unit
arrivals and fired once, and the unit echo strengthened the eligible candidate
`1 -> 4` without refiring threshold-two P. P-only+R and X-only+R each produced
one readout and one subthreshold M arrival; late R produced no readout. An R
aligned to the second of two adjacent histories read only the currently-live
footprints and produced one attribution.

R6 imports no CJ-B gated-ARROW rule. It uses only authoritative PX0 cells,
arrows, thresholds, ordinary decay, refractory dynamics and normalized
participation traces. Storage stayed bounded (`3072`/`3136` bytes); no memory
leak or oscillator occurred.

This is a development positive for return-triggered trace readout, not a PX3
authority retry. PX3 authority remains negative until an integrated fresh
workflow incorporates and tests this geometry.
