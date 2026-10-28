# C11.4 · `minmax(auto, max-content)` two-track span과 overflow automatic minimum

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**사전 비교:** [Chrome 156 관측과 독립 계산](../spec/internal/evidence/c11-4-auto-minmax-max-content-span-overflow-precomparison-2026-10-28/README.md) · [기대값](../spec/internal/evidence/c11-4-auto-minmax-max-content-span-overflow-precomparison-2026-10-28/expected.json) · [Chrome 화면](../spec/internal/evidence/c11-4-auto-minmax-max-content-span-overflow-precomparison-2026-10-28/chrome/two-track-span-overflow.png)

**계획 검토:** [독립 실패 관점 20개](./c11-4-auto-minmax-max-content-span-overflow-plan-review-20.md) · [계획 PR diff 실패 관점 30개](./c11-4-auto-minmax-max-content-span-overflow-planning-pr-diff-review-30.md)

## 결손과 목표

PR #752는 `minmax(auto, max-content)` 한 track에 item 하나를 놓고 overflow별 automatic minimum을 확인했다. 그 입력은 하나의 item이 두 intrinsic tracks를 span해 minimum contribution과 track 성장 분배가 함께 결정되는 경우를 다루지 않는다. 이 계획은 그 간극만 다섯 개의 고정 `spinon-layout` unit-test 입력으로 추가한다.

변경은 Rust layout unit-test code와 비교 자료에 한정한다. 일반 CSS admission, HTML parser, app route, public API, FFI/ABI, Taffy vendored source는 바꾸지 않는다. 따라서 Android/iOS 앱 또는 Simulator 동작은 이 작업의 검증 대상이 아니다.

## 규범 근거와 재현 입력

[CSS Grid Layout Module Level 1, 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto)은 item이 non-scrollable overflow이고, 해당 축에서 `auto` minimum track을 span하며, 복수 track span에 flexible track이 없으면 content-based automatic minimum을 사용한다고 정의한다. 그 외 overflow에서는 automatic minimum이 0이다. §§11.5/11.5.1은 span-two item의 intrinsic contribution을 track에 균등 분배하고, §11.6은 definite positive free space를 growth limits까지 track base size에 균등 분배한다. [CSS Overflow §3.1](https://www.w3.org/TR/css-overflow-3/#overflow-properties)은 `hidden`, `auto`, `scroll`을 scrollable values로, `visible`, `clip`을 non-scrollable values로 구분하며 축 간 computed-value 전환도 명시한다. 독립 계산, Chrome observation, fixture hash는 [사전 비교 기록](../spec/internal/evidence/c11-4-auto-minmax-max-content-span-overflow-precomparison-2026-10-28/README.md)에 고정했다.

- Columns only: definite root `80×20px`; two `minmax(auto, max-content)` columns; one direct item placed at column line `1 / span 2`; row track `20px`.
- Item content: fixed Ahem `10px/10px` unbreakable text `XXXXXXXXXX`. Min-content and max-content width are both `100px`.
- Target-axis overflow: `visible`, `clip`, `hidden`, `auto`, `scroll`. The authored Y overflow stays `visible`; when X is a scrollable value, CSS Overflow computes Y to `auto`. The layout oracle tests X, records the computed browser pair, and does not claim to implement CSS axis parsing.
- Root size, both track definitions, the exact two-track placement, item source, font, gap, border, padding, and one fixed row stay identical across the five cases.

## 독립 기대 결과

| X overflow | Automatic minimum | Final column tracks | Item frame | Max-content contribution |
| --- | ---: | --- | --- | ---: |
| `visible`, `clip` | content-based `100px` | `[50, 50]px` | `100×20px` | `100px` |
| `hidden`, `auto`, `scroll` | `0px` | `[40, 40]px` | `80×20px` | `100px` |

Non-scrollable cases raise the sum of the two auto-minimum base sizes to 100px, distributed equally. In scrollable cases the automatic minimum does not raise those bases; the max-content contributions set 50px growth limits on both tracks, then the definite 80px available width maximizes the tracks to 40px each. The item remains 100px max-content wide internally, so its observed `scrollWidth` is 100px in both classes. These values are derived from the 2025 CRD algorithm before reading Chrome output and match all five Chrome 156 cases exactly.

## 구현과 판정 기준

1. Add a separate submodule under `crates/spinon-layout/src/grid_auto_min_scrollable_overflow_tests/`; keep the existing `minmax(auto,1fr)` and single-track `minmax(auto,max-content)` cases unchanged.
2. Create the same fixed Ahem text layout through the existing pure `Tree`/paragraph path. Use exactly two `minmax(auto,max-content)` tracks, a span of exactly two tracks, and the five X-overflow values. Keep Y overflow `visible` in the Rust input.
3. For every input, assert both final target track base sizes, the root and item frames, the single `Minimum` contribution source/value, and the `MaxContent` contribution source/value. For `visible`/`clip`, require `AutomaticMinimum` with `spans_auto_min_track=true`, `spans_multiple_tracks=true`, `spans_flexible_max_track=false`, `content_based=true`. For `hidden`/`auto`/`scroll`, require `OverflowAutomaticMinimum` with value zero. All five max-content contributions remain 100px.
4. Compare integers exactly with zero CSS-pixel tolerance. A matching aggregate width alone is insufficient: assertions must distinguish `[50,50]` from `[40,60]` and require the multi-track trace flag.
5. Run the focused new test, the `spinon-layout` suite, `cargo fmt --all -- --check`, and `cargo clippy -p spinon-layout --all-targets -- -D warnings`. These Rust layout gates do not imply app CSS parsing, scroll/clip behavior, or simulator coverage.
6. Review the resulting implementation/runtime boundary in 20 new failure perspectives and its PR diff in 30 separate perspectives. Review this plan in 20 perspectives before implementation, and review this planning PR diff independently in 30 perspectives.
7. The PR may show the [Chrome baseline screenshot](../spec/internal/evidence/c11-4-auto-minmax-max-content-span-overflow-precomparison-2026-10-28/chrome/two-track-span-overflow.png), labeled as Chrome only. After browser evidence work, close the Agent Browser session and process.
8. After merge, reconcile the parent plan, `spec/STATUS.md`, and public roadmap with actual results. Keep the C11.4 checkbox incomplete and official complete count `72/213`.

## 구현 결과 · PR #756 병합 (2026-10-28T11:33:07Z)

구현 PR [#756](https://github.com/ohah/spinon/pull/756)은 Rebase and merge됐고 최종 main merge SHA는 `ca7f6e93842a72f6e410588832a9a431820dffcc`다. 별도 `#[cfg(test)]` submodule의 고정 Columns 입력 다섯 개가 두 `minmax(auto, max-content)` track의 최종 base, root/item frame, Minimum source/value 및 MaxContent contribution을 정확 비교한다. `visible`/`clip`은 tracks `[50,50]px`, item `100×20px`; `hidden`/`auto`/`scroll`은 tracks `[40,40]px`, item `80×20px`이며 max-content contribution은 100px로 유지한다. 새 검사는 공유 허용 오차 대신 `assert_eq!`를 사용한다.

Focused test 1/1, `spinon-layout` 298/298, `cargo fmt --all -- --check`, crate all-target Clippy가 통과했다. 이는 순수 Rust layout unit-test 범위이므로 앱 runtime과 Android/iOS Simulator는 실행하지 않았다. GitHub PR check rollup은 비어 있어 CI 통과로 기록하지 않는다. Chrome 화면은 [독립 browser baseline](../spec/internal/evidence/c11-4-auto-minmax-max-content-span-overflow-precomparison-2026-10-28/chrome/two-track-span-overflow.png)이며 Spinon runtime 결과가 아니다. [실행·source hash](../spec/internal/evidence/c11-4-auto-minmax-max-content-span-overflow-implementation-2026-10-28/README.md) · [구현/runtime 실패 관점 20개](../spec/internal/evidence/c11-4-auto-minmax-max-content-span-overflow-implementation-2026-10-28/implementation-runtime-review-20.md) · [구현 PR diff 검토 30개](../spec/internal/evidence/c11-4-auto-minmax-max-content-span-overflow-implementation-2026-10-28/pr-diff-review-30.md) · [병합 후 상태·로드맵 검토 30개](../spec/internal/evidence/c11-4-auto-minmax-max-content-span-overflow-implementation-2026-10-28/post-merge-status-roadmap-review-30.md). C11.4는 미완료이며 공식 완료 수는 `72/213`으로 유지한다.

## 범위 밖

This slice does not implement CSS `overflow` parsing/admission, general DOM or arbitrary item placement, app/runtime fixture routes, clipping pixels, scroll offsets or scrollbars, painting, WPT conformance, Android/iOS Simulator behavior, or all span/track combinations. Chrome is an independent browser baseline, not a Spinon result. It does not close C11.4 or change official completion count `72/213`.
