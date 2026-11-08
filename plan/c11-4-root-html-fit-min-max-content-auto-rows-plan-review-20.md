# C11.4 four-member intrinsic auto-row plan failure review · 20 checks

Plans: [`c11-4-root-html-fit-min-max-content-auto-rows.md`](./c11-4-root-html-fit-min-max-content-auto-rows.md) and its 96-scene Chrome 157 oracle. This review precedes implementation; it is not reused for implementation/runtime or PR-diff review.

| # | Adversarial failure path | Required plan guard | Result |
|---:|---|---|---|
| 1 | Four-item support is described as arbitrary track-list support | Limit admission to the 24 permutations of one fit-content, min-content, max-content, and auto member | PASS |
| 2 | A duplicate member slips through while one required type is absent | Reject duplicate, missing, and extra members; exercise exact permutation count | PASS |
| 3 | `max-content` is projected as `min-content` because fixed payload geometry is equal | Assert the computed and typed enum identity independently from frames | PASS |
| 4 | Equal min/max geometry is presented as proof of both intrinsic algorithms | Record that this fixture has equal min/max contributions and make no such claim | PASS |
| 5 | Track order is sorted or canonicalized during style projection | Compare all 24 authored/computed order strings and the typed list order | PASS |
| 6 | A repeated list starts after the explicit row instead of at the first implicit row | Keep one explicit row and verify the fifth implicit row repeats the first member | PASS |
| 7 | The cycle assertion is invalid because the two occurrences have different content | Use equal 20px payloads in implicit rows one and five; capture the corrected fixture separately | PASS |
| 8 | The initial invalid cycle fixture is mistaken for final evidence | Preserve it as pass01 and identify pass02's fixture hash as the sole accepted oracle | PASS |
| 9 | Percentage cap basis is indefinite or tied to viewport height | Fix a definite 100px grid block-size and compare `20%` with `20px` | PASS |
| 10 | Percentage normalization erases the computed authored type | Compare computed `fit-content(20%)` while separately requiring px/% geometry equality | PASS |
| 11 | `calc()` accidentally joins direct px/percentage cap admission | Keep calc-valued caps outside the accepted computed-style set | PASS |
| 12 | Negative, NaN, infinite, or non-representable caps enter layout | Keep finite non-negative validation and property-attributed fail-closed controls | PASS |
| 13 | Stretch distributes space to max-content, min-content, or fit-content tracks | Compare `start` and `stretch`; only computed auto max-sizing positions may change | PASS |
| 14 | A repeated leading auto track receives the wrong share | Include leading-auto orders; there are two auto tracks and each must receive 11px of the 22px free space | PASS |
| 15 | Positive free space is exhausted twice or not exhausted under stretch | Require total used rows to equal 100px under stretch and 78px under start | PASS |
| 16 | Item source order or explicit-row identity shifts the expected frames | Record explicit frame, all five item frames, payload frames, and row positions | PASS |
| 17 | CSS computed track list is confused with resolved `grid-template-rows` | Store and validate both `computedRows` and used `gridTemplateRows` values | PASS |
| 18 | A different Grid owner or nested/unsupported topology inherits this profile | Preserve single root owner, direct-child topology, fixed explicit tracks, and revision gates | PASS |
| 19 | A rejected revision publishes partial layout or poisons the session | Require no partial layout/draw publication plus same-session recovery in implementation tests | PASS |
| 20 | A Chrome/static fixture result is promoted to product support or C11 completion | Require Android/iOS basic-route evidence, retain all exclusions, and keep C11.4/C11 and `78/214` unchanged | PASS |

## Review correction

Pass01 used a 6px payload in implicit row five while row one had a 20px payload. The first cycle check incorrectly required equal used sizes despite those different contributions. The fixture now gives both repeated positions 20px payloads; pass02 independently verifies equal row sizes for the repeated first track. Pass01 artifacts remain labeled as superseded evidence and are not counted in the 96/96 result.

## Disposition

The plan fixes the support boundary, oracle, independent expected values, cycle invariant, and platform/runtime gates before implementation. No unresolved plan blocker remains.
