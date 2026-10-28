# C11.4 · `minmax(auto, max-content)`와 overflow automatic minimum

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**사전 비교:** [Chrome 156 관측과 독립 계산](../spec/internal/evidence/c11-4-auto-minmax-max-content-overflow-precomparison-2026-10-28/README.md) · [기대값](../spec/internal/evidence/c11-4-auto-minmax-max-content-overflow-precomparison-2026-10-28/expected.json) · [Chrome 화면](../spec/internal/evidence/c11-4-auto-minmax-max-content-overflow-precomparison-2026-10-28/chrome/mixed-auto-min-max-content-overflow.png)

**계획 검토:** [20개 실패 관점](./c11-4-auto-minmax-max-content-overflow-plan-review-20.md) · [이 계획 PR diff 검토 30개](./c11-4-auto-minmax-max-content-overflow-planning-pr-diff-review-30.md)

## 구현 전 결손과 목표

병합된 PR #750은 `minmax(auto, 1fr)` track에서 item overflow에 따른 automatic minimum을 Columns Ahem text와 Rows replaced image 입력으로 확인했다. 그 입력은 max-content maximum의 growth limit과 scrollable overflow automatic minimum이 함께 있는 track을 시험하지 않았다. 이 slice는 `minmax(auto, max-content)` 한 track에서 그 interaction만 10개의 fixed Rust layout unit-test 입력으로 추가한다.

변경은 `spinon-layout`의 unit-test code와 비교 evidence에 한정한다. 일반 CSS admission, app route, public API, ABI, CSS parser 또는 Taffy vendored source는 변경하지 않는다. 새 scene이나 app runtime route가 없으므로 Android/iOS Simulator 검증은 이 계획의 gate가 아니다. Simulator 동작을 확인한 것으로 표현하지 않는다.

## 규범 근거와 사전 비교

[CSS Grid Layout Module Level 1, 2025 CRD [§6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto)에 따르면 Grid item은 computed overflow가 scrollable overflow value가 아니고 해당 축에 `auto` min track을 span할 때 content-based automatic minimum을 사용한다. 그렇지 않으면 automatic minimum은 0이다. [§§11.5–11.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)은 `auto` minimum track base와 `max-content` maximum growth limit/contribution을 정의한다. 고정 수치, 원 Chrome observation, fixture hash는 [사전 비교 기록](../spec/internal/evidence/c11-4-auto-minmax-max-content-overflow-precomparison-2026-10-28/README.md)에 있다.

입력은 [Chrome fixture](../tests/fixtures/css/c11/grid-auto-min-max-content-c114-precomparison.html)와 일치한다.

- Columns: definite root `60×20px`, `minmax(auto, max-content)` column 하나, direct Ahem `XXXXXXXXXX` item. Min-/max-content inline contribution은 `100px`이다.
- Rows: definite root `30×5px`, fixed `15px` column과 `minmax(auto, max-content)` row 하나, natural `24×16px` image. CSS width `15px`가 transferred row size `10px`를 만들며 item은 `align-self:start`로 `15×10px` frame을 유지한다.
- 두 축에 각각 `visible`, `clip`, `hidden`, `auto`, `scroll`을 둔다. Root/item box sizing, gap, padding, border와 placement는 고정한다.

## 기대 결과

| Axis | Overflow | Minimum contribution | Max-content contribution | Final track | Item frame |
| --- | --- | ---: | ---: | ---: | --- |
| Columns | `visible`, `clip` | 100px content-based | 100px | 100px | 100×20px |
| Columns | `hidden`, `auto`, `scroll` | 0px | 100px | 60px | 60×20px |
| Rows | `visible`, `clip` | 10px content-based | 10px | 10px | 15×10px |
| Rows | `hidden`, `auto`, `scroll` | 0px | 10px | 5px | 15×10px |

All ten IDs and root/item frames are fixed in [expected.json](../spec/internal/evidence/c11-4-auto-minmax-max-content-overflow-precomparison-2026-10-28/expected.json). Track, root/item frame, and contribution values must match the normative integers exactly; no browser-derived expectation is fed back into implementation.

## Implementation and review gates

1. Extend `grid_auto_min_scrollable_overflow_tests` with a separate `minmax(auto, max-content)` matrix. Keep its existing `minmax(auto, 1fr)` expectations unchanged.
2. For every case assert the final target-axis used track and root/item frame, the queried `Minimum` contribution source/value, and the `MaxContent` contribution value. `visible`/`clip` must report `AutomaticMinimum` with `spans_auto_min_track=true`, `spans_multiple_tracks=false`, `spans_flexible_max_track=false`, `content_based=true`; scrollable values must report `OverflowAutomaticMinimum`. The max-content contribution remains 100px for Columns and 10px for Rows in both overflow classes.
3. Use the existing `Tree` + Ahem `TextLayoutEngine` path for Columns and `HostDocument` + `LayoutIntrinsicSizeSnapshot` path for Rows. Keep text row layout out of scope because its existing Taffy/text-height agreement gate is unrelated to this track maximum.
4. Run the focused new test, the `spinon-layout` suite, formatting, and targeted Clippy. This is pure layout-unit coverage; no Android/iOS app or simulator result will be claimed.
5. Review implementation/runtime failure paths in 20 distinct perspectives. Review every submitted PR diff separately in 30 distinct perspectives, as required by the project rules. Fix material misses and rerun relevant gates before merge.
6. PR body may include the Chrome reference screenshot, clearly labeled as browser baseline rather than Spinon runtime. Verify any image link with Agent Browser and close the browser process afterward.
7. After merge, synchronize the parent plan, `spec/STATUS.md`, and public roadmap with actual results. Keep C11.4 incomplete and official complete count at `72/213`.

## Scope limits

This test matrix does not implement scroll containers, clipping, scrolling offsets, scrollbar layout, paint, item pixel overflow, general HTML/CSS admission, WPT, app/runtime paths, simulator behavior, or the full automatic-minimum matrix. It does not resolve every `minmax(auto, max-content)` item/track/span combination and does not close C11.4. Official count `72/213` and the C11.4 checkbox remain unchanged.
