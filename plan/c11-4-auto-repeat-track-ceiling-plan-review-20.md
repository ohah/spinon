# Auto-repeat track ceiling plan review · 20 failure angles

Each row checks a different way this bounded plan could produce misleading evidence or expand beyond its contract.

| # | Failure angle | Required guard |
| --- | --- | --- |
| 1 | The test observes authored descriptors instead of Taffy-expanded tracks. | Read non-gutter entries from the captured `Initialize` stage. |
| 2 | The 9,999 boundary is accidentally tested with a 10,000px basis. | Bind each case's width/height and expected count from one table row. |
| 3 | The 10,001 case silently treats the clamp as an error. | Assert the pinned 10,000-track result and successful bounded computation. |
| 4 | Columns receive a definite width but rows accidentally use an indefinite block basis. | Set and assert the selected axis's fixed root size for every case. |
| 5 | A nonzero gap consumes extra basis and changes the repetition count. | Use zero gap and reject any fixture helper default that adds one. |
| 6 | `auto-fill` and `auto-fit` are conflated because both have the same track count. | Assert each mode's collapsed flag for every repeated track. |
| 7 | Auto-fit collapse is observed before placement when the fixture unexpectedly has items. | Keep the input item map and root children empty and assert that precondition. |
| 8 | The orthogonal axis also expands to 10,000, exhausting the shared trace budget. | Keep the orthogonal axis to one fixed track and run axes independently. |
| 9 | A partial trace is mistaken for a complete observation. | Require no trace failure, one target-axis Initialize stage, and exact count. |
| 10 | Gutter entries are counted as CSS tracks. | Filter `is_gutter` and assert the raw trace still has the expected track/gutter shape. |
| 11 | The source-order test cannot distinguish prefix, repeat, and suffix. | Give prefix, repeated track, and suffix distinct fixed sizes. |
| 12 | The source-order test checks only the count and misses suffix truncation. | Assert first/last track sizes at the 10,003 fitting control and 10,004 cap control. |
| 13 | A prefix is accidentally included in the repeat-start index incorrectly. | Assert the expected remaining repeat count after the one-track prefix. |
| 14 | The test depends on an undocumented system Taffy version. | Pin expectations to the vendored 0.15.0 source and cite its clamp logic. |
| 15 | The trace-state budget is treated as an unbounded 10,000-track guarantee. | State that only this fixed-size trace matrix is covered; retain the separate budget limit. |
| 16 | A helper requests allocation proportional to 10,001 tracks before Taffy preflight. | Construct only the one-element authored repeat pattern and inspect bounded trace output. |
| 17 | Numeric tolerance hides an off-by-one track count. | Compare integer trace counts exactly; use exact fixed `f32` sizes 1, 2, and 3. |
| 18 | New tests alter production feature behavior or fixture ABI. | Add only `#[cfg(test)]` code under the existing layout tests; inspect feature boundaries. |
| 19 | Passing this matrix is used to claim general CSS Grid or WPT support. | Keep C11.4/C11 unchecked and list all excluded grammar and runtime paths. |
| 20 | A visual screenshot is fabricated for a non-visual unit-test result. | Mark screenshots not applicable; preserve raw test output and source revision instead. |

**검토 결과:** 계획 관점 20개를 모두 반영했다. 검토 중 prefix/suffix 크기 5px가 repeat basis에서 차감되지 않아 최초 경계값이 한 track 낮게 잡힌 점을 발견해 source-order control을 10,003px/10,004px로 수정했다.
