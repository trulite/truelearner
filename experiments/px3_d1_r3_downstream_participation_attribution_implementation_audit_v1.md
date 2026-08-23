# PX3-D1-R3 downstream participation attribution implementation audit v1

Status: **FROZEN; E2B PREFLIGHT PASSED; EVIDENCE UNSPENT**.

- source commit: `bce4e11070abd35d49c91f126d487c7fe8dcc85e`;
- manifest SHA-256:
  `de827c3b37a8e9c2e47dbbef5edc9429ab4eb6fb215fbd313d2672d87d22320e`;
- source SHA-256:
  `275a94b64da66ef8dc6e03574e5342fae63340ea5751fa1e6623cd170da3be54`;
- protocol SHA-256:
  `fd1d3442a699ef4a1d3c62ff92ad85937b16e6d80f315ae9c94ce44b504b96f7`;
- execution-protocol SHA-256:
  `8282b318cf23b34553b7199c6dab136a154b48b42052f93c05704d8cb32dc9af`.

Persistent E2B sandbox `i6x9gykt9tvp6xfz5z8ra` passed formatting, release
check, 2/2 static tests, strict Clippy and the non-propagating `--preflight`.
Result/staging and later-stage surfaces were absent. The evidence marker was
not emitted and no R3 world was constructed or propagated.

The implementation contains six symmetric trace-pair routes. Every P source
has threshold two; the registered attribution output has coupling one except
in the explicit coupling-two safety-risk control. No effect/relay-to-P edge
exists. The code separately serializes attribution inputs, attribution firing,
credit arrival, P firing, candidate traversal and native resistance.

Only the frozen `--r3` command may spend development evidence once.
