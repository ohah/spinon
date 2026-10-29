# C11.3.3 WPT inventory refresh · adversarial diff review 30

- Review target: staged diff against `origin/main` at `5391a0923bea81bb55052a105d90d21bfa9ff271`.
- Scope: WPT subtest classification metadata, classifier checks, regression tests, plan/status notes, and derived evidence. No product implementation or runtime ABI changed.
- Result: 30 distinct hostile views passed after the final classifier and inventory hashes were generated.
- Executed checks: C11.3.3 consumer suite 15/15; classifier suite 8/8; duplicate-WPT-path injected input was rejected with `WPT report test paths must be unique`; `git diff --check` passed.

| # | Hostile review view | Result and evidence |
| ---: | --- | --- |
| 1 | Pinned WPT revision drift | PASS — classifier requires the report revision to equal inventory revision `c999c583…`; the source checkout was pinned to the same revision. |
| 2 | Candidate source file replacement | PASS — each WPT source is SHA-256 checked against inventory before classification. |
| 3 | Wrong browser oracle | PASS — report product, Chrome 154 version, and pinned revision are checked before output. |
| 4 | Missing or extra WPT test path | PASS — normalized report path keys must exactly equal the inventory candidate set. |
| 5 | Duplicate normalized WPT path | PASS — injected duplicate report result was rejected before `Map` conversion. |
| 6 | Failed or skipped WPT subtest | PASS — non-PASS subtests and non-OK/PASS test status are rejected. |
| 7 | Subtest order changes | PASS — classification is keyed by report subtest name; reverse-order unit case preserves the corresponding scene mapping. |
| 8 | Missing pinned `.grid` subtest | PASS — explicit coverage must match the report name set exactly. |
| 9 | Newly added `.grid` subtest | PASS — the injected `.grid 5` is rejected as an inventory mismatch. |
| 10 | Duplicate WPT subtest names | PASS — duplicate report names are rejected before creating a name map. |
| 11 | Duplicate inventory subtest mapping | PASS — duplicate explicit mapping names fail the focused unit case. |
| 12 | Unknown classification enum | PASS — only `currently-unsupported` and `spinon-fixture-candidate` are accepted. |
| 13 | Fixture candidate without evidence | PASS — every fixture-candidate mapping must include both a scene ID and evidence path. |
| 14 | Scene ID removed from fixture | PASS — a mapped ID absent from declared `data-scene` attributes is rejected. |
| 15 | Duplicate scene IDs in fixture HTML | PASS — ambiguous duplicate declarations fail closed. |
| 16 | Malformed/null subtest entry | PASS — malformed report entries are rejected before name extraction. |
| 17 | Existing whole-test unsupported fallback | PASS — candidates without explicit mappings preserve legacy unsupported/candidate classification. |
| 18 | Current report bound to stale inventory | PASS — current report inventory SHA-256 equals the current inventory hash and is read through its evidence pointer. |
| 19 | Helper or classifier drift after report creation | PASS — current regression checks bind both classifier and helper paths/hashes to the generated report. |
| 20 | Historical Chrome report overwritten | PASS — original `wpt-chrome154…-run01.json` remains unchanged and is rehashed by the consumer suite. |
| 21 | Historical precomparison provenance rewritten | PASS — old reference and historical classified output retain their original inventory hash. |
| 22 | Historical runtime matrix relabeled as current | PASS — runtime matrix retains its generation-time inventory digest; it is checked against the historical reference digest. |
| 23 | Separate unknown template-area alias accidentally enabled | PASS — `unknown-area-implicit-tracks` remains the documented expected rejection. |
| 24 | Derived scenes generalized beyond fixed positive trailing cases | PASS — inventory text and README constrain the mapping to four opt-in fixed-px trailing-line/span scenes. |
| 25 | Chrome oracle promoted to Spinon WPT conformance | PASS — report meaning, README, plan, and status all state that no Spinon WPT run is claimed. |
| 26 | Official roadmap count promoted by metadata work | PASS — C11.3.3 remains candidate, C11 remains unchecked, and `72/213` is unchanged. |
| 27 | C11.4 progress accidentally changed | PASS — diff contains no C11.4 implementation, checkbox, or count change. |
| 28 | Product ABI/runtime behavior altered | PASS — diff is limited to classifier tooling, fixtures metadata, tests, and documentation/evidence. |
| 29 | New simulator evidence implied without execution | PASS — README links prior PR #393 runtime evidence and explicitly says this change did not rerun Android/iOS. |
| 30 | Final evidence hash or formatting mismatch | PASS — run06 binds current inventory/classifier/helper and derived fixture hashes; focused checks and `git diff --check` passed. |
