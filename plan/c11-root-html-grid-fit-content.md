# C11.1 · parsed Root HTML Grid explicit `fit-content(px)` tracks

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [Root HTML Grid 기본 경로](./c11-root-html-grid-runtime.md) · [`minmax()` explicit tracks](./c11-root-html-grid-minmax.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**구현 전 비교:** [Chrome 156 사전 비교](../spec/internal/evidence/c11-root-html-grid-fit-content-precomparison-2026-11-03/README.md) · [고정 HTML](../spec/internal/evidence/c11-root-html-grid-fit-content-precomparison-2026-11-03/root-html-grid-fit-content-precomparison.html) · [독립 기대값](../spec/internal/evidence/c11-root-html-grid-fit-content-precomparison-2026-11-03/expected.json) · [Chrome 관찰](../spec/internal/evidence/c11-root-html-grid-fit-content-precomparison-2026-11-03/chrome-observation.json)

**계획 실패 관점 검토:** [20개](./c11-root-html-grid-fit-content-plan-review-20.md) · **계획 PR 변경 검토:** [20개](../spec/internal/evidence/c11-root-html-grid-fit-content-precomparison-2026-11-03/plan-pr-diff-review-20.md). **구현/runtime 검토:** 별도 20개 · **구현 PR 변경 검토:** 별도 20개.

**상태:** 구현 전 계획. 현재 기본 Root HTML `RuntimeRootHtmlTextPaintV1` 경로의 `validate_root_html_grid_tracks()`는 direct px/%/fr와 bounded `minmax()`만 허용한다. 일반 typed converter와 `spinon-layout` Taffy adapter에는 이미 `FitContent` 변환이 있으므로, 이번 slice는 안전한 제한값 admission과 parsed Root HTML 통합을 다룬다.

## 목표와 범위

기본 parsed Root HTML route에서 explicit `grid-template-columns`와 `grid-template-rows`의 `fit-content(<finite non-negative px>)`를 cascade computed value부터 typed Grid sizing, Taffy layout, 실제 Root HTML scene까지 연결한다. CSS parser, 새 runtime profile, host ID/class/child-index 성공 분기, FFI ABI 또는 public API를 추가하지 않는다.

허용하는 typed 값은 `ComputedCssGridTrackSize::FitContent(LengthPx(value))`이며 `value.is_finite() && value >= 0.0`이어야 한다. 기존 integer `repeat()`가 만든 explicit track list에서도 같은 검증을 적용한다. Browser baseline은 직접 track과 fixed integer repeat에서 px cap 동작을 고정한다.

Percentage 및 `calc()` 인자, auto sizing, auto-repeat, implicit `grid-auto-*`, named line 신규 admission, area-created unsized tracks는 범위 밖이다. 이미 지원되는 track 수와 일치하는 고정 named-area map 하나와 explicit `fit-content(px)`의 조합만 기준 입력에 포함한다. 기존 line-name·auto-repeat·column-flow·implicit-track·area 검증을 그대로 유지한다. 지원 밖 값은 기존 parsed Root HTML profile에서 property-bound diagnostic으로 fail-closed하며 부분 frame을 발행하지 않는다.

## 규범 해석과 고정 기준

[CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-functions)은 `fit-content()`를 `max(minimum, min(limit, max-content))`로 정의한다. 여기서 `minimum`은 auto minimum이며, 흔히 min-content와 같지만 항상 같지는 않다. 이번 비-scrollable Ahem text 장면은 min-content contribution과 auto minimum이 각각 20px(열), 40px(행)인 입력으로 고정했다. 별도 intrinsic probe와 Chrome cross-check를 사용하며 overflow·aspect-ratio에 따른 다른 automatic minimum은 범위 밖이다.

Chrome 156.0.8078.80 Agent Browser observation과 fixture/font/screenshot hash는 [precomparison evidence](../spec/internal/evidence/c11-root-html-grid-fit-content-precomparison-2026-11-03/README.md)에 고정돼 있다. Ahem `AA AA AA AA`의 min/max-content width는 20px/110px이다. 독립 기대와 Chrome은 다음 여섯 동작을 확인했다.

| 장면 | 고정 입력 | 규범상 used size |
| --- | --- | --- |
| Column cap below min-content | `fit-content(10px)` | 20px, minimum floor 유지 |
| Column cap between contributions | `fit-content(40px)` | 40px |
| Column cap above max-content | `fit-content(120px)` | 110px |
| Fixed integer repeat | `repeat(2, fit-content(40px))` | `[40px, 40px]` |
| Existing fixed named-area map | `"fixed fit"`, tracks `20px fit-content(40px)` | `[20px, 40px]` |
| Row cap below wrapped minimum | width 20px, four Ahem lines, `fit-content(20px)` | 40px |

Chrome은 computed track list와 item frame을 전부 독립 oracle과 일치시켰다. 그 결과는 Spinon 동작이나 WPT 적합성을 증명하지 않는다.

## 구현 수락 기준

1. 기본 parsed Root HTML profile에서 explicit row/column `fit-content(px)`만 정확히 허용한다. 음수·비유한 px, percentage, calc, auto, 그리고 implicit `grid-auto-*`의 `fit-content()`는 거부하며 diagnostic은 해당 Grid property와 node를 보존한다.
2. 기존 `project_track()` typed 변환을 재사용한다. 기존 integer repeat 확장 결과도 동일 admission을 받으며 auto-repeat과 named lines는 계속 거부한다. 신규 profile, scene allowlist 또는 fixture 전용 product branch를 만들지 않는다.
3. Projection 검사는 column/row 값과 fixed-repeat expansion, `0px`, 양수, 음수, NaN, ±infinity, percentage, calc 인자를 acceptance/rejection matrix로 고정한다. Unsupported 값이 다른 valid tracks나 CSS declaration의 부분 성공으로 바뀌지 않는다.
4. Root HTML runtime matrix는 Chrome의 여섯 장면을 같은 DOM/text source와 style/layout revision으로 계산한다. Track size·item frame을 expected JSON과 비교하고 min/max-content intrinsic probes를 분리 기록한다. 중앙 cap 장면의 root 밖 marker와 hit/frame 경계가 track size와 맞는지 확인한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 최신 source fixture를 기존 앱 entry로 실행한다. `layout=ready`, expected boxes, draw/present, 화면 PNG를 남기며 cap 안/밖의 AVD PixelCopy와 iOS Metal drawable 표본을 검사한다. AVD software renderer, Metal compositor 제출 시점, simulator와 실기기 차이를 분리 기록한다.
6. 기존 direct px/%/fr, `minmax()`, repeat, named-area, implicit row, column-flow, overflow, allocation failure 및 fallback/recovery 검증을 회귀 검사한다. 불변 fixture/font/source hash와 simulator source commit을 evidence에 기록한다.
7. `cargo fmt --all -- --check`, focused style/cascade/projection/runtime suites, 변경 crate scoped Clippy, Android/iOS Simulator builds 및 실행을 수행한다. 실행하지 않은 항목은 통과라고 보고하지 않는다. 전체 suite의 비관련 baseline failure와 PR check rollup 부재는 별도 기록한다.
8. 새 구현/runtime 변경을 계획 검토와 겹치지 않는 실패 관점 20개로 검토한다. 구현 PR의 실제 변경은 별도 20개 adversarial review로 검토한다. 발견한 결함은 수정 후 관련 테스트·simulator evidence·review 항목을 다시 확인한다.
9. Rebase-and-merge 후 내부 계약 0059를 `0.1.0`으로 유지하고 plan 및 `spec/STATUS.md`에 제한 slice·증거·한계를 기록한다. C11/C11.1 공식 체크박스와 전체 완료 수는 올리지 않는다. 승인된 Tailnet roadmap preview는 merge SHA 기준으로 갱신해 served hash, 공식 counter, desktop/mobile overflow를 확인한다.

## 진행 순서

1. Chrome precomparison, 독립 expected, 계획 실패 관점 20개와 계획 PR 실제 변경 20개 검토를 끝내고 계획 PR을 Rebase-and-merge한다.
2. 최신 `main` 한 체크아웃에서 validator admission, projection/cascade tests, runtime matrix와 simulator fixture를 순차 구현한다.
3. 수정된 최종 후보에서 구현/runtime review 20개를 별도로 수행한다. 앱 runtime·Android/iOS simulator gates와 format, focused tests, scoped Clippy를 실행한다.
4. 구현 PR의 실제 diff를 별도 20개 관점으로 검토하고 필요한 수정 후 PR 화면과 embedded screenshots를 Agent Browser로 확인한다. GitHub checks/required status checks를 각각 확인한 뒤 승인된 rebase merge를 진행한다.
5. Merge SHA 기준 internal contract, plan, official status, Tailnet preview와 evidence를 순차 동기화한다. Agent Browser session과 process를 종료하고 process list에서 남은 browser를 확인한다.

이 bounded slice는 전체 `fit-content()`, implicit track sizing, CSS Grid 또는 WPT conformance 완료를 뜻하지 않는다. Android simulator는 실기기가 아니고 AVD는 software renderer이며 iOS Metal 표본은 compositor 제출 전이다.
