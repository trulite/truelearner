# CJ0-F pre-evidence schedule correction audit v1

Status: **CORRECTED EVALUATOR FROZEN; EVIDENCE UNSPENT**.

The correction protocol SHA-256 is
`435cd5fdc085974a5b41ba0aa46aeffe491fdbc50cef59726fe4579bcc419c92`.
The corrected comparator SHA-256 is
`a83468c15e452451b83d402fb88136ad90b15e81878acde266a558babcde0752`.

The complete production change adds a base tick to the common genuine-entry
helper, uses base 0 at every existing fresh-world call, and uses base 2 for the
held-out reuse call after the fixed advance. One focused regression test
executes that control for each isolated candidate and requires held-out reuse.

Validation after correction:

- result and staging paths: absent;
- CJ-B law SHA-256: exact
  `ef0de37a9ac54b632b991f0d4647a5ee78c23810084d61497c88d6f757ec2188`;
- CJ-E law SHA-256: exact
  `e64c8c915c2fbc4679d1e34ee69ecfe36e2c5ff05bdff5d7feeb5a55578bf1c1`;
- original matrix sizes: unchanged at `120/1,920/8,640` paired rows;
- focused formatting and all-target compile: pass;
- focused candidate/comparator tests: `7/7` pass;
- held-out absolute-time regression: CJ-B pass, CJ-E pass;
- strict all-target Clippy: pass;
- no later-stage surface preflight: pass;
- shared source changes: none.

No identity, topology, physical schedule interval, marginal, expected result,
candidate accounting, or selection rule changed. The v1 implementation tag
remains immutable. This corrected implementation may execute PROBE once after
its commit and annotated tag.
