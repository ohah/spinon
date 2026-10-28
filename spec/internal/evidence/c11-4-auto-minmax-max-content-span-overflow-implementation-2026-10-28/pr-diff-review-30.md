# Two-track span overflow 구현 PR diff · 30개 독립 실패 관점

검토 대상은 새 Rust test module과 declaration, Chrome fixture-link correction, implementation evidence와 이 PR delta 기록이다. 각 관점은 별도 path/contract condition을 확인한다.

| # | 독립 diff 실패 관점 | 확인 결과 | 판정 |
| ---: | --- | --- | --- |
| 1 | 코드 변경이 production layout path로 새어 들어감 | 신규 source는 기존 `#[cfg(test)]` test module 안에서만 등록된다. | 통과 |
| 2 | product API/ABI가 바뀜 | 공개 타입, FFI, header, API 문서 변경은 없다. | 통과 |
| 3 | Taffy vendored behavior를 무단 수정 | vendored source 변경은 없다. | 통과 |
| 4 | 기존 single-track max-content test를 덮어씀 | 기존 `max_content.rs`는 변경하지 않았다. | 통과 |
| 5 | 기존 auto-min/1fr matrix를 덮어씀 | Existing `grid_auto_min_scrollable_overflow_tests.rs` 변경은 submodule 등록 한 곳뿐이다. | 통과 |
| 6 | module path가 중복되거나 누락 | Parent declaration과 sibling file stem이 일치한다. | 통과 |
| 7 | Rust source가 권장 응집 크기를 초과 | New module은 한 테스트 책임으로 응집된 303줄이며 500줄 권장 상한보다 작다. | 통과 |
| 8 | test IDs가 이전 matrix와 충돌 | `columns-span2-*` 접두어로 다섯 ID를 분리했다. | 통과 |
| 9 | case array 길이와 values가 달라짐 | `[Case; 5]`와 visible/clip/hidden/auto/scroll 다섯 initializer를 대조했다. | 통과 |
| 10 | X/Y overflow를 같이 바꿔 원인을 흐림 | Only X varies; Y is Visible in all Rust cases. | 통과 |
| 11 | overflow values are not copied to the item style | `case.overflow` feeds `overflow_x` directly. | 통과 |
| 12 | `scroll` and `auto` collapse through an adapter | All five enum values reach the test input separately. | 통과 |
| 13 | root size differs from independent fixture | Root dimensions, viewport and fixed row are 80×20px. | 통과 |
| 14 | item has an implicit single-column placement | Explicit columns use line 1 and line 3 endpoints. | 통과 |
| 15 | track function differs between columns | Both vector entries use identical auto-min/max-content track functions. | 통과 |
| 16 | test passes from total width only | The trace must contain exactly two tracks and each base is compared. | 통과 |
| 17 | minimum source can be wrong while values match | Source variant and content-based flags are matched independently from value. | 통과 |
| 18 | max-content stage is not observed | MaxContent contribution kind and exact value are asserted once per item. | 통과 |
| 19 | item/grid frame mismatch goes undetected | Root frame, item frame, origin, and target track bases are checked separately. | 통과 |
| 20 | zero-tolerance CSS pixel expectation is loosened | New local `exact` helper uses `assert_eq!`; the existing shared helper's `1/64px` tolerance is not reused. | 통과 |
| 21 | Ahem font depends on host-installed fonts | The engine disables system fonts and registers repository Ahem bytes. | 통과 |
| 22 | font/text path differs from the baseline input | Paragraph writing mode, size, line height, white-space, and text match the fixture. | 통과 |
| 23 | missing tree item hides a bad owner mapping | `LayoutInput` tree has root and item; the paragraph owner and Grid item use the same item ID. | 통과 |
| 24 | trace is selected from the wrong grid or axis | Trace node matches the root and pass axis is Columns. | 통과 |
| 25 | unrelated columns/gutters pollute the trace | Final track collection filters gutters and collapsed tracks, then requires length two. | 통과 |
| 26 | test omits minimum contribution due query behavior | Every case requires exactly one Minimum trace for the item. | 통과 |
| 27 | `MaxContent` contribution differs only in scrollable cases | All five cases compare the contribution to 100px. | 통과 |
| 28 | evidence overstates visual clipping or scrolling | README says no app, clip pixel, scrolling, scrollbar, WPT, or full CSS claim. | 통과 |
| 29 | local docs link escapes Tailnet `/spinon/` base | Fixture reference is a pinned GitHub blob URL, so its target remains valid from both generated docs and repository view. | 통과 |
| 30 | validation artifacts describe an earlier formatted source | Source SHA table, post-format focused/full tests, formatter, Clippy, and review files are included in the PR. | 통과 |

Review caught that the shared layout test helper permits `1/64px`, wider than the plan's zero tolerance. The affected assertions were switched to exact equality and the focused test, package suite, formatter, and Clippy were rerun on that source. No blocker remains. Final gates are focused 1/1, package 298/298, `cargo fmt --all -- --check`, and crate all-target Clippy with warnings denied. Android/iOS Simulator and general CSS/runtime coverage are not claimed.
