# PX3 physical event organization definitive implementation audit v1

Status: **IMPLEMENTATION FROZEN; DEFINITIVE EVIDENCE UNSPENT; PX3 AUTHORITY ABSENT**.

## Frozen implementation

- source commit: `135b3d04ef7e1dd4b1ac95e9e64746861267b89d`;
- manifest SHA-256:
  `3fbc74c3724b259332b02829df27cafe018c38a90aeeb6113fd551ccc309bff5`;
- source SHA-256:
  `288ce23199f66b65e022afac4314629ac133edaea93072486326357f8c58b328`;
- protocol SHA-256:
  `fb58387fc8d6f214683fe3d65b5b1c4261eb910e37135fab60db7a8a357d0151`;
- execution-protocol SHA-256:
  `b697904ee90c9b7e120e5a20a8c9bd84ceb95554257295081bcabd8d3066cc1a`;
- frozen developed GATE source SHA-256:
  `969042a740f92237c577d82c67399447040cb96d2c003c28100034566e30d5aa`.

The dedicated authority sandbox is `iz37tc0lg4rv42oxuawcx`, created fresh from
template `truelearner-rust-1-97-worker`. It is distinct from development
sandbox `i6x9gykt9tvp6xfz5z8ra` and every earlier authority sandbox.

## Byte-identical physical blocks

A brace-balanced source audit compared formatted developed GATE source with the
definitive source. All 23 physical construction, scheduling, normalization,
measurement and substrate-helper functions are exact:

| block | SHA-256 | exact |
|---|---|---|
| `build` | `e892cfcb0892bd44fd7e91b39d480ee527a7b7ac5a2ce83e56d4b9d79ab3dc51` | yes |
| `normalize` | `72e642167105f92d4421db088e47ca0e8a442a9b35953b72bcdbca696425ff74` | yes |
| `expose` | `abec34ea27df524d9679c7b7cbb852809a6a4736204288294a89f7e2f8344276` | yes |
| `primitive` | `1a44941cf1ec3d3996ae3f40f8f61673c3d2befe73b0a1e69b89b16a51665bd5` | yes |
| `background` | `f7c148b08d5d2270f693887c0c84ed51dc943b4d397772b1d28ce0e357d27c04` | yes |
| `on_clone` | `6f101767519811bb672c5a6cb0a6c1510d2e0fc1582a264656e7db879090010b` | yes |
| `metrics` | `3ece724e0f9c3fd543bdff1fac997f601ab9bf897200dc3a95e9b0b99f82d986` | yes |
| `candidate_arrows` | `27a1c71ccb83fcb1502f48cc9b3b0874ef2d57a7a429cdd2d9871ca4ffefb5f8` | yes |
| `candidate_count` | `0320bb7aa058c258ba5f26cf93ee6d8a1cf0dbeb38fab68481c345332f4dbec4` | yes |
| `resistance` | `b5c47ce462bcc0409f5535cb246e5af604cd89af9de1f366ef926a93321c2d19` | yes |
| `resistances` | `45ade30beba63ed9dcbf307653b33257012b23128a708495fb54f0a4d93d054d` | yes |
| `cell` | `af8a397707a77367ade29c39588a1ad107c6578f228aa950a69239ded2e3b3eb` | yes |
| `fixed` | `57fc4898092bd7c30baf7ed5226f5a6bafcd368574e8e5f57b309255dbbf57d9` | yes |
| `pulse` | `38aa5ffdde293a687ca611c31b9c39f2790a10a719be2ec12f82105ab776b48d` | yes |
| `physical` | `fcbae3415aab41da3d4d5137c738333230fe301e1f910ba704d07be1c701592e` | yes |
| `fires` | `17639dbddea8c076856b269ec4a27ca27c46bfcb2171422b6248d42b7edf3be6` | yes |
| `arrivals` | `31979977ccc0b30a69c2b17446989e9e2bbd97e78a9344a217ef212e19242d23` | yes |
| `arrival_impulse` | `be38d108ed842b117a9847c9e7ff33b9407bdcfc78c372e68ae278a66253de87` | yes |
| `crossings` | `fc33816ed437b698cbe1af72436b90b413ca1172522b183612bd2e4eec6d2b07` | yes |
| `crossing_impulse` | `6b5ea1d07111e481535651f2722d1ab218ce7ee21739d324bbd1f26a4172b1ce` | yes |
| `three` | `37a6f082d552ad920d5a87ad555438ccfc686e0b7f9b40352c1e155289c93b33` | yes |
| `four` | `321bf22ad7d61995696826791596a9bbc1647d3ad57a89aa5f1534b00f99cab4` | yes |
| `add_work` | `e2b03868ffe0616c7a7d61c123548ac666beefdc6c63bd0777b9b43fc573797c` | yes |

The authoritative PX0 substrate remains byte-identical at SHA-256
`3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.
The definitive arm depends only on that substrate crate.

## Authority-only additions

The source adds only:

- sixteen fresh namespace indices rotating the four frozen GATE strata;
- four evaluator-only world schedules fixed by the protocol;
- W1/W2/W3 control scheduling through existing `pulse`, `background`,
  `expose` and `propagate` paths;
- evaluator-side P0--P9 clauses and exact serialization;
- duplicate execution, refusal/preflight, source hashes and atomic publication.

`Kind`, world names, expected vectors and claims never enter a CELL, ARROW or
SPIKE. Organism state contains no event, member, contributor, composite, level,
world or correctness field. No candidate is inserted, strengthened, selected,
deleted or suppressed by the evaluator.

The namespace base `0x6_5300_0000_0000` was absent before protocol and source
creation. The matrix creates 64 unique namespaces and no development identity
is reused.

## Pre-evidence validation

In dedicated sandbox `iz37tc0lg4rv42oxuawcx`, exact source commit `135b3d0`
passed:

- formatting;
- 2/2 release static tests;
- strict Clippy;
- no-argument refusal with exit 2 before evidence;
- wrong-argument refusal with exit 2 before evidence.

No world, preflight, definitive cell, artifact or evidence marker executed.
Final and staging paths remain absent.

The next clean commit containing this audit is the exact preflight target.
Preflight must remain no-world and non-propagating. Only after that commit is
tagged may the sole definitive command execute once.
