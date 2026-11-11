# Six-fit implementation/runtime review: 20 failure perspectives

**Target:** local implementation diff on `feat/c11-4-six-fit-auto-minimum-20261111`, after the corrected test runs. This review is separate from the plan review and plan-PR diff review.

| # | Failure perspective | Inspection and result |
| ---: | --- | --- |
| 1 | Six entries accidentally admit a shorter or longer list | Both projection and layout predicates require exactly six members; style/layout unit tests reject wrong lengths. |
| 2 | A non-fit track slips into the profile | Every member is matched as `FitContent`; the unit test replaces one member with `Auto` and rejects it. |
| 3 | An order-specific implementation misses a valid authored order | Both predicates map each typed cap to a unique seen slot; the style projection test enumerates all 720 permutations. |
| 4 | Duplicate caps satisfy the multiset check | `mem::replace` rejects a second occurrence; duplicate px and percentage controls fail. |
| 5 | A percentage is confused with an equal numeric length | The 20% slot matches only `Percentage(0.2)`; computed-style matrix checks typed cap order and serialized values. |
| 6 | An unlisted or non-finite cap bypasses admission | Exact values are matched; unit and RuntimeSession rejection cases cover unlisted, negative, NaN, infinity, alternate percentage, and unresolved calculation. |
| 7 | Absolute `calc()` normalization changes the contract | A RuntimeSession control confirms `calc(10px + 20px)` computes as the admitted 30px member and preserves track order. |
| 8 | Six-fit accidentally broadens existing mixed one-to-five-fit admission | Six-fit uses a separate predicate and explicit route; a unit assertion confirms the mixed-profile predicate remains false for all-six-fit. Existing one-to-five regressions pass. |
| 9 | Layout validation admits a profile projection rejected by style validation, or vice versa | Both layers use the same six typed caps and six-entry requirement; direct style/layout unit tests pass, and 5,760 runtime scenes agree. |
| 10 | A second Grid or wrong owner receives the exception | Root HTML validators require one visible Grid owner; rejection test injects a second owner and publishes no layout. |
| 11 | The candidate accepts the wrong child count or item nesting | The profile requires nine items, a bare first control, and eight direct single-text payload items; missing item and nested payload are rejected. |
| 12 | The explicit control item obtains automatic minimum | Candidate profile requires control `min-height:0`; a control-auto mutation is rejected. |
| 13 | Mixed item minima are partially accepted | The repeated profile requires all eight text items `auto`; a mixed `0/auto` case fails closed. |
| 14 | Scrollable item or Grid incorrectly uses automatic minimum | Validators preserve visible overflow requirements; Grid and item overflow mutations are rejection cases. |
| 15 | Track sizing loses the content floor on a zero cap | Feature trace checks eight automatic-minimum contributions and each final row base size against the line-count floor; candidate matrix has zero floor violations. |
| 16 | Wrong rows/items are paired under reverse DOM order | Matrix validates all nine item frames and row sizes per scene, plus forward/reverse DOM orders and same-revision computed styles. |
| 17 | Alignment stretch alters the candidate row/item geometry | Matrix pairs `start` and `stretch` and asserts zero row-geometry differences; the dark stretch control also passes. |
| 18 | Baseline under-floor or candidate overflow is masked by aggregate totals | Every item/payload frame, glyph count, line count and Grid end is checked scene by scene before fixed totals are asserted. |
| 19 | Rejection publishes stale frames or prevents same-session recovery | Dark media mutation to an unlisted cap fails without completed layout; computed styles carry the invalid revision; switching back recovers with new key and expected frames. |
| 20 | Platform smoke is overstated as six-fit verification | Evidence explicitly says iOS tested the default app route only, Android runtime was unavailable, and six-fit geometry evidence comes from RuntimeSession/Chrome comparison. |

**Result:** no implementation defect found in these perspectives. This is bounded local evidence; PR/CI and final-merge review are still pending.
