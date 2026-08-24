# CJ1-PA participation/amplitude geometry result audit v1

Status: **VALID POSITIVE DEVELOPMENT GEOMETRY**.

## Frozen execution

- executed implementation commit:
  `62982dce657839bd843d53f0fb620918e09b110d`;
- E2B persistent sandbox: `i6x9gykt9tvp6xfz5z8ra`;
- authoritative PX0 SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- authoritative PX1 definitive implementation SHA-256:
  `74716c87d146cb697b37ddf802c12e67a5cb93daf82ec20f8b982e54922bd696`;
- protocol SHA-256:
  `c5827a66f693a2bd6a3558a2fbcece5b38ae1f1c5a66eedcc25bb97d37853abd`;
- executed runner SHA-256:
  `80505822b6ba4a53b6b617c6026c13d1c0735a8dac89cae762a292726cca4a5e`;
- result CSV SHA-256:
  `9dd57a16e285c274ff8a10203c48e282ed688e930016cc561e51693955558986`;
- result report SHA-256:
  `ea1245930891369434503e85ccb4691e6177dabba30c76c7ecbcd72355af524c`.

The exact preregistered command executed once in E2B. The result contains six
ordered unique rows plus one header, no staging remnants, exact duplicate
replay and natural quiescence in every row. Independent checks pass for all 26
serialized fields. PX0 and every frozen PX1 authority input remained
byte-exact. No local Rust process ran.

## Decisive matrix

| world | raw carried impulse | actual raw paths | outlet firings | trace firings | conjunction firings |
|---|---:|---:|---:|---:|---:|
| A(1) | 1 | 1 | 1 | 1 | 0 |
| A(2) | 2 | 1 | 1 | 1 | 0 |
| A(4) | 4 | 1 | 1 | 1 | 0 |
| A(1)+B(1) | 2 | 2 | 2 | 2 | 1 |
| A(2)+B(1) | 3 | 2 | 2 | 2 | 1 |
| A(4)+B(4) | 8 | 2 | 2 | 2 | 1 |

Raw coupling changes are present in the native source->outlet crossing ledger:
the strong single-path cases carry exactly `2` and `4`, and the mixed/two-strong
cases carry exactly `2|1` and `4|4`. The positive result is therefore not an
accidental failure to instantiate amplitude.

Every participating outlet fires exactly once. Its ordinary coupling-one
outlet->trace ARROW traverses exactly once, regardless of the raw impulse that
made the outlet fire. The shared hub fires once and sends the same unit return
to both trace CELLs. In a single-path world the participating trace receives
two unit arrivals and fires once; the nonparticipating trace receives only the
shared return and does not fire. In every two-path world both traces fire once.

Each trace firing sends one ordinary unit traversal into the threshold-two
conjunction CELL. One strong physical path therefore supplies one conjunction
unit; two genuinely participating paths supply two and fire the conjunction
exactly once.

## Interpretation

The existing PX1 physical participation layer removes the raw-amplitude alias
without a new substrate law:

```text
raw path coupling 1/2/4
        -> one actual outlet execution
        -> one side-local PX1 trace firing
        -> one ordinary conjunction unit
```

The relevant multiplicity is realized by separately firing physical trace
CELLs, not by evaluator-side contributor identity or unique counting. Coupling
strength remains physically real upstream but does not multiply participation
events downstream.

Together with CJ1-T, this resolves both narrow CJ0 aliases at the continuous
physical layer:

- scheduled same-source repetition does not become two simultaneous physical
  participants;
- one mature high-amplitude path does not become two PX1 participation-trace
  firings.

This is development geometry, not a retroactive change to CJ1's frozen raw-law
PROBE and not yet a complete event/reversal/dense-world ladder. It adds no new
mechanism and authorizes no definitive evidence, authority claim, PX3 restart
or PX-C by itself.
