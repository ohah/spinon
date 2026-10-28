# C11.4 `minmax(auto, max-content)` two-track span · overflow automatic minimum runtime 후속 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [순수 Rust test-only 결과 PR #756](./c11-4-auto-minmax-max-content-span-overflow.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**독립 기준:** [Chrome 156 관측과 CSS Grid 계산](../spec/internal/evidence/c11-4-auto-minmax-max-content-span-overflow-precomparison-2026-10-28/README.md) · [기대값](../spec/internal/evidence/c11-4-auto-minmax-max-content-span-overflow-precomparison-2026-10-28/expected.json) · [Chrome 화면](../spec/internal/evidence/c11-4-auto-minmax-max-content-span-overflow-precomparison-2026-10-28/chrome/two-track-span-overflow.png)

**계획 검토:** [서로 다른 실패 관점 20개](./c11-4-auto-minmax-span-overflow-runtime-plan-review-20.md) · [계획 PR 변경분 30개 검토](./c11-4-auto-minmax-span-overflow-runtime-pr-diff-review-30.md)

## 배경과 확인된 경계

PR #756은 definite `80×20px` Grid, `minmax(auto, max-content)` 두 columns, Ahem 10px `XXXXXXXXXX` item의 span과 overflow automatic minimum을 다섯 개 순수 Rust layout unit-test로 고정했다. 이는 앱에서 HTML/CSS cascade부터 layout projection, fixture route, draw/present까지 같은 입력을 처리했다는 증거가 아니다.

현재 `RuntimeBlockFlexGridTrackSizingV1` authored-property allowlist는 `overflow-x`/`overflow-y`를 포함하지 않으며, style-to-layout projection도 이 profile에 대해 `LayoutOverflow::Visible`을 사용한다. 따라서 이 profile에 authored `overflow` 선언을 그냥 추가하면 계산된 overflow와 실제 layout overflow 사이의 경계가 불명확해진다. [기존 Grid allowlist](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs) · [overflow layout projection gate](../crates/spinon-style-to-layout/src/projection/style_values.rs) · [현재 generic mixed-span fixture](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/generic_spanning.rs).

## 목표와 범위

기존 C11.4 opt-in fixture-test 경계에서만 사용할 전용 cascade/profile 및 layout adapter 경로를 계획한다. 이 경로는 고정 fixture 입력에서 `overflow-x`/`overflow-y`의 computed 값과 `LayoutOverflow` projection을 보존하고, Grid item의 automatic minimum 계산을 기록한다. 기본/profile 일반 CSS admission이나 다른 app document에는 overflow 지원을 추가하지 않는다. Profile 이름과 dispatch가 fixture 목적을 드러내고 `c11-4-grid-track-sizing-fixture-test` feature 밖에서는 새 route/adapter가 활성화되지 않아야 한다.

수평 overflow에 대해 CSS Overflow §3.1의 축 간 computed-value 규칙을 검사한다. Authored `overflow-y: visible`은 `overflow-x`가 scrollable value일 때 computed `auto`로 바뀔 수 있으므로, 각 장면의 authored 값과 computed x/y 값을 따로 기록한다. 이 후속은 sizing trace와 frame이 목적이다. Clipping pixels, scrolling, scroll offset, scrollbar, paint overflow는 구현하지도 지원으로 표시하지도 않는다.

## 고정 입력과 기대값

CSS Grid 2025 CRD §§6.6, 11.5, 11.5.1, 11.6의 독립 계산 및 Chrome baseline은 위 사전 비교에 고정되어 있다. 새 입력이나 Chrome observation에서 expected values를 만들지 않는다.

| authored `overflow-x` → computed x/y | item Minimum source/value | 최종 columns | item frame |
|---|---|---|---|
| `visible` → `visible` / `visible` | `AutomaticMinimum { content_based: true, spans_auto_min_track: true, spans_multiple_tracks: true, spans_flexible_max_track: false }` / 100px | `[50,50]px` | `100×20px` |
| `clip` → `clip` / `visible` | `AutomaticMinimum { content_based: true, spans_auto_min_track: true, spans_multiple_tracks: true, spans_flexible_max_track: false }` / 100px | `[50,50]px` | `100×20px` |
| `hidden` → `hidden` / `auto` | `OverflowAutomaticMinimum` / 0px | `[40,40]px` | `80×20px` |
| `auto` → `auto` / `auto` | `OverflowAutomaticMinimum` / 0px | `[40,40]px` | `80×20px` |
| `scroll` → `scroll` / `auto` | `OverflowAutomaticMinimum` / 0px | `[40,40]px` | `80×20px` |

모든 장면에서 Grid frame은 `80×20px`, Ahem text의 min-content와 max-content contribution은 각각 100px이다. Scrollable minimum이 0이어도 max-content contribution은 100px로 남는다. 두 track의 개별 base, root/item 좌표·크기, 정확히 하나의 Minimum source/value, MaxContent contribution 및 Columns/Rows trace를 함께 확인한다. Frame과 trace는 zero CSS-pixel tolerance로 비교한다.

## 실행 계획과 판정 기준

1. 새 test-only computed-style/profile gate가 overflow 값을 CSS cascade에서 읽고 동일 값이 layout adapter까지 도달하는지 검사한다. 기본 C11.4 production profile과 fixture feature-off 빌드에서 새 author admission·fixture symbol이 없어야 한다.
2. 5개 장면을 앱의 기존 opt-in C11.4 fixture runtime 경로로 실행한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 각각 사용하고, 실제 기기 결과로 확대하지 않는다.
3. 두 플랫폼에서 각각 5/5 computed x/y values, root/item frame, two-track bases, Minimum source/value, MaxContent contribution, runtime trace, draw/present를 독립 verifier로 비교한다. 초기화·revision/profile 오류, 잘못된 span, 달라진 text, 다른 computed overflow, 누락 trace는 partial layout 대신 fail-closed해야 한다.
4. `cargo fmt --all -- --check`, 영향받은 focused/runtime tests, feature-off와 fixture-test builds, targeted Clippy를 실행한다. 변경된 Android/iOS fixture feature app builds가 통과해야 한다. 실행하지 않은 플랫폼이나 paint 동작을 통과로 기록하지 않는다.
5. 구현 뒤 이 계획과 겹치지 않는 구현/runtime 실패 관점 20개, PR 변경분 30개, 병합 후 상태/roadmap 동기화 30개를 별도로 기록한다. PR 본문에는 해당 구현 slice의 실제 Android/iOS Simulator 캡처를 첨부하되 browser baseline과 분리한다.

## 완료 후에도 남는 범위

이 제한 fixture는 임의 author HTML/CSS, 일반 앱의 overflow admission, C13.1 clip/scroll painting, scrollbar와 scroll offset, WPT 전체, 다른 track 조합 또는 전체 C11.4 지원을 증명하지 않는다. C11.4 checkbox는 미완료이며 공식 완료 수는 `72/213`으로 유지한다.
