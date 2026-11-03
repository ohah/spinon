# C11.1 · parsed Root HTML Grid explicit `fit-content(px)` tracks

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [Root HTML Grid 기본 경로](./c11-root-html-grid-runtime.md) · [`minmax()` explicit tracks](./c11-root-html-grid-minmax.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**구현 전 비교:** [Chrome 156 사전 비교](../spec/internal/evidence/c11-root-html-grid-fit-content-precomparison-2026-11-03/README.md) · [고정 HTML](../spec/internal/evidence/c11-root-html-grid-fit-content-precomparison-2026-11-03/root-html-grid-fit-content-precomparison.html) · [독립 기대값](../spec/internal/evidence/c11-root-html-grid-fit-content-precomparison-2026-11-03/expected.json) · [Chrome 관찰](../spec/internal/evidence/c11-root-html-grid-fit-content-precomparison-2026-11-03/chrome-observation.json)

**검증 경계 근거:** [기본 Root HTML font 경로와 Ahem test engine 구분](../spec/internal/evidence/c11-root-html-grid-fit-content-precomparison-2026-11-03/runtime-validation-boundary.md)

**named-area admission 경계 근거:** [현재 Root HTML area gate와 fit-content 조합 제안](../spec/internal/evidence/c11-root-html-grid-fit-content-precomparison-2026-11-03/named-area-admission-boundary.md)

**계획 실패 관점 검토:** [20개](./c11-root-html-grid-fit-content-plan-review-20.md) · **계획 PR 변경 검토:** [20개](../spec/internal/evidence/c11-root-html-grid-fit-content-precomparison-2026-11-03/plan-pr-diff-review-20.md) · **font 경계 계획 정정 검토:** [20개](./c11-root-html-grid-fit-content-plan-correction-review-20.md) · [정정 PR 변경 검토 20개](../spec/internal/evidence/c11-root-html-grid-fit-content-precomparison-2026-11-03/plan-correction-pr-diff-review-20.md) · **projection admission 계획 정정 검토:** [20개](./c11-root-html-grid-fit-content-area-admission-review-20.md) · [정정 PR 변경 검토 20개](../spec/internal/evidence/c11-root-html-grid-fit-content-precomparison-2026-11-03/area-admission-plan-pr-diff-review-20.md) · **default-route semantic gate 계획 정정 검토:** [20개](./c11-root-html-grid-fit-content-runtime-gate-review-20.md) · [정정 PR 변경 검토 20개](../spec/internal/evidence/c11-root-html-grid-fit-content-precomparison-2026-11-03/runtime-gate-plan-pr-diff-review-20.md). **구현/runtime 검토:** 별도 20개 · **구현 PR 변경 검토:** 별도 30개.

**상태:** PR [#1093](https://github.com/ohah/spinon/pull/1093) 검토 대기. cascade/projection admission, named-area semantic gate, layout sizing, Root HTML runtime fixture를 추가했다. focused Rust 검증 223개, `cargo fmt`, `git diff --check`, 변경 crate scoped Clippy가 통과했다. 구현/runtime 실패 관점 20개는 [구현 검토](../spec/internal/evidence/c11-root-html-grid-fit-content-runtime-2026-11-03/implementation-review-20.md), PR 실제 diff 30개는 [PR 검토](../spec/internal/evidence/c11-root-html-grid-fit-content-runtime-2026-11-03/pr-diff-review-30.md)에 기록했다. Android API 37 AVD PixelCopy와 iPhone 17 Pro/iOS 26.2 Simulator Metal 표면 표본은 각각 7/7 exact다. Agent Browser로 PR 본문과 embedded image 2장이 모두 로드됨을 확인했다. GitHub checks·required status와 병합은 남아 있다. 현재 기본 Root HTML `RuntimeRootHtmlTextPaintV1` 경로의 `validate_root_html_grid_tracks()`는 direct px/%/fr와 bounded `minmax()`만 허용했다. 일반 typed converter와 `spinon-layout` Taffy adapter에는 이미 `FitContent` 변환이 있으므로, 이번 slice는 안전한 제한값 admission과 parsed Root HTML 통합을 다룬다.

검증 경계: 기본 Root HTML renderer는 `TextLayoutEngine::new_for_runtime()`를 사용해 Android/Apple OS 폰트를 찾으며, Root HTML profile에는 CSS `@font-face` font loading 경로가 없다. 따라서 Chrome의 고정 Ahem intrinsic matrix는 `spinon-layout`의 Ahem-등록 test engine으로 검증하고, parsed Root HTML default route 및 simulator smoke는 system font에 기대지 않는 고정-size box fixture로 별도 검증한다. 이 둘을 같은 실행 결과로 표현하지 않는다.

## 목표와 범위

기본 parsed Root HTML route에서 explicit `grid-template-columns`와 `grid-template-rows`의 `fit-content(<finite non-negative px>)`를 cascade computed value부터 typed Grid sizing, Taffy layout, 실제 Root HTML scene까지 연결한다. CSS parser, 새 runtime profile, host ID/class/child-index 성공 분기, FFI ABI 또는 public API를 추가하지 않는다.

허용하는 typed 값은 `ComputedCssGridTrackSize::FitContent(LengthPx(value))`이며 `value.is_finite() && value >= 0.0`이어야 한다. 기존 integer `repeat()`가 만든 explicit track list에서도 같은 검증을 적용한다. Browser baseline은 직접 track과 fixed integer repeat에서 px cap 동작을 고정한다.

`fit-content()`의 percentage 및 `calc()` 인자, auto sizing, auto-repeat, implicit `grid-auto-*`, named line 신규 admission, area-created unsized tracks는 범위 밖이다. 현재 Root HTML named-area projection gate는 일부 2×2 track/map 패턴만 허용하고, 별도 default-route semantic gate는 `120×70px`, `header/side/content`, 각 영역 하나씩의 빈 직계 div 세 개를 요구한다. Chrome 기준의 `"fixed fit"` 1×2 map은 두 gate 모두에서 아직 지원되지 않는다. 이 slice는 파서가 만든 유효한 직사각형 area map이 explicit row/column track 수와 정확히 일치하고, 두 축의 모든 track이 finite non-negative direct px 또는 `fit-content(px)`이며 적어도 하나가 `fit-content(px)`인 경우에만 추가 named-area 경로를 허용한다. 이 경로의 container width/height는 finite positive px이고 min/max는 기본값이어야 한다. 모든 named area에는 빈 visible HTML `div` 직계 item 하나씩을 둔다. 아이템 수는 area 수와 정확히 같고 placement는 각 area 이름을 한 번씩 가리켜야 한다. Area 이름, DOM id/class, 노드 번호, child index와 scene 이름은 성공 조건으로 사용하지 않는다. 새 named-area 경로는 direct percentage, `fit-content()` percentage/calc 인자, auto, `fr`, `minmax()`를 허용하지 않는다. 기존 fixed/fraction/percentage named-area profiles와 그 runtime 경로는 바꾸지 않는다. 기존 line-name·auto-repeat·column-flow·implicit-track·area 검증을 유지하고 지원 밖 값은 property-bound diagnostic으로 fail-closed하며 부분 frame을 발행하지 않는다.

## 규범 해석과 고정 기준

[CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-functions)은 `fit-content()`를 `max(minimum, min(limit, max-content))`로 정의한다. 여기서 `minimum`은 auto minimum이며, 흔히 min-content와 같지만 항상 같지는 않다. 이번 비-scrollable Ahem text 장면은 min-content contribution과 auto minimum이 각각 20px(열), 40px(행)인 입력으로 고정했다. 별도 intrinsic probe와 Chrome cross-check를 사용하며 overflow·aspect-ratio에 따른 다른 automatic minimum은 범위 밖이다.

Chrome 156.0.8078.80 Agent Browser observation과 fixture/font/screenshot hash는 [precomparison evidence](../spec/internal/evidence/c11-root-html-grid-fit-content-precomparison-2026-11-03/README.md)에 고정돼 있다. Ahem `AA AA AA AA`의 min/max-content width는 20px/110px이다. 독립 기대와 Chrome은 다음 여섯 동작을 확인했다.

| 장면 | 고정 입력 | 규범상 used size |
| --- | --- | --- |
| Column cap below min-content | `fit-content(10px)` | 20px, minimum floor 유지 |
| Column cap between contributions | `fit-content(40px)` | 40px |
| Column cap above max-content | `fit-content(120px)` | 110px |
| Fixed integer repeat | `repeat(2, fit-content(40px))` | `[40px, 40px]` |
| Named-area baseline | `"fixed fit"`, tracks `20px fit-content(40px)` | `[20px, 40px]` (Chrome 기준이며 현재 Root HTML admission은 아님) |
| Row cap below wrapped minimum | width 20px, four Ahem lines, `fit-content(20px)` | 40px |

Chrome은 computed track list와 item frame을 전부 독립 oracle과 일치시켰다. 그 결과는 Spinon 동작이나 WPT 적합성을 증명하지 않는다.

## 구현 수락 기준

1. 기본 parsed Root HTML profile에서 explicit row/column `fit-content(px)`만 정확히 허용한다. 음수·비유한 px, `fit-content(%)`, `fit-content(calc())`, auto sizing, 그리고 implicit `grid-auto-*`의 `fit-content()`는 거부하며 diagnostic은 해당 Grid property와 node를 보존한다.
2. 기존 `project_track()` typed 변환을 재사용한다. 기존 integer repeat 확장 결과도 동일 admission을 받으며 auto-repeat과 named lines는 계속 거부한다. parser-validated dimension-matched area 조합은 area 이름이나 scene ID에 종속되지 않고, 신규 profile 또는 fixture 전용 product branch를 만들지 않는다.
3. Projection 검사는 column/row 값과 fixed-repeat expansion, `0px`, 양수, 음수, NaN, ±infinity, `fit-content(%)`, `fit-content(calc())`를 acceptance/rejection matrix로 고정한다. Unsupported 값이 다른 valid tracks나 CSS declaration의 부분 성공으로 바뀌지 않는다.
4. `spinon-layout`의 fixed Ahem test engine은 Chrome 기준 중 세 cap column, fixed repeat, row minimum floor의 다섯 Ahem text sizing cases를 independent expected JSON과 비교한다. Named-area scene은 projection 및 default runtime layer에서 검사한다. 이는 Grid/Taffy sizing 근거이며 Root HTML parsing이나 앱 runtime 근거로 부르지 않는다.
5. `spinon-style-to-layout`은 parsed computed Root HTML values의 explicit columns/rows, fixed repeat 및 dimension-matched arbitrary-name rectangular area 조합을 typed `GridTrackSize::FitContent`으로 투영하는지 별도 검사한다. named-area fit-content 경로는 두 축이 finite non-negative px/fit-content(px) track으로만 구성되고 적어도 한 fit-content track을 가져야 한다. `fit-content()`의 percentage/calc argument, invalid lengths, auto-repeat, line names와 implicit `grid-auto-*` fail-closed를 검사한다. 기존 2×2 fixed/fraction/percentage area profiles의 동작은 유지한다.
6. default Root HTML semantic admission도 기존 `120×70px`/`header-side-content`/세 item 전용 gate와 분리해 검사한다. 새 fit-content named-area 경로에서는 definite positive px container 크기, 기본 min/max, 기존 gap 제한, dimension-matched explicit tracks, 임의 이름의 parsed areas, 모든 area에 하나씩 연결된 visible 빈 HTML `div` direct child만 허용한다. Area마다 한 item, 중복 placement 및 unused/missing area는 거부한다. id/class, node ID, child index와 scene identity를 조회하지 않는다. 기존 fixed/fraction/percentage named-area runtime profile은 그대로 회귀한다.
7. `RuntimeSession::new_runtime_gpu_with_root_html()`의 default Root HTML route는 font 독립적인 fixed-size child로 세 scene을 실행한다. 직접 track scene은 30×20px grid, `fit-content(40px) 10px` columns, 20px row, 40×20px first item과 8×20px second marker를 두고 layout tracks `[40px, 10px]`, marker frame `(x=40,y=0,w=8,h=20)`을 확인한다. Fixed-area scene은 30×20px grid, `grid-template-areas: "fixed fit"`, `20px fit-content(40px)` columns, 20px row, `grid-area: fixed`인 20×20px child와 `grid-area: fit`인 40×20px child를 두고 tracks `[20px, 40px]`, child frames `(x=0,y=0,w=20,h=20)` 및 `(x=20,y=0,w=40,h=20)`를 확인한다. Fixed-row scene은 20×20px grid, 20px column, `fit-content(20px)` row, 20×40px child로 row track과 child height 40px를 확인한다. 이는 default route admission/layout evidence이며 Ahem cap 계산 검증이 아니다.
8. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 세 scene을 동일 source fixture 및 기본 앱 entry로 실행한다. 각 scene의 `layout=ready`, boxes, draw/present와 화면 PNG를 남긴다. 두 column scene은 grid origin 기준 CSS 좌표 `(25,10)`은 root 안, `(35,10)`은 root 바깥이지만 40px fit child 안이며, 직접 track scene의 `(42,10)`은 x=40px marker 안임을 확인한다. Fixed-row scene은 `(10,10)`이 20px root 안, `(10,25)`가 root 바깥이지만 40px child 안임을 확인한다. 각 root-relative CSS sample을 recorded DPR/표면 크기로 AVD PixelCopy와 iOS Metal drawable 표본 좌표로 변환한다. AVD software renderer, Metal compositor 제출 시점, simulator와 실기기 차이를 분리 기록한다.
9. 기존 direct px/%/fr, `minmax()`, repeat, named-area, implicit row, column-flow, overflow, allocation failure 및 fallback/recovery 검증을 회귀 검사한다. 불변 fixture/font/source hash와 simulator source commit을 evidence에 기록한다.
10. `cargo fmt --all -- --check`, focused layout/cascade/projection/runtime suites, 변경 crate scoped Clippy, Android/iOS Simulator builds 및 실행을 수행한다. 실행하지 않은 항목은 통과라고 보고하지 않는다. 전체 suite의 비관련 baseline failure와 PR check rollup 부재는 별도 기록한다.
11. 새 구현/runtime 변경을 계획 검토와 겹치지 않는 실패 관점 20개로 검토한다. 구현 PR의 실제 변경은 프로젝트 규칙에 따라 앞선 검토와 겹치지 않는 별도 30개 adversarial review로 검토한다. 발견한 결함은 수정 후 관련 테스트·simulator evidence·review 항목을 다시 확인한다.
12. Rebase-and-merge 후 내부 계약 0059를 `0.1.0`으로 유지하고 plan 및 `spec/STATUS.md`에 제한 slice·증거·한계를 기록한다. C11/C11.1 공식 체크박스와 전체 완료 수는 올리지 않는다. 승인된 Tailnet roadmap preview는 merge SHA 기준으로 갱신해 served hash, 공식 counter, desktop/mobile overflow를 확인한다.

## 진행 순서

1. Chrome precomparison, 독립 expected, 계획 실패 관점 20개와 각 계획 정정 PR 실제 변경 20개 검토를 끝내고 계획 정정 PR을 Rebase-and-merge한다.
2. 최신 `main` 한 체크아웃에서 cascade/projection admission, default-route semantic gate, projection/cascade tests, Ahem layout matrix, system-font-independent runtime box fixture를 순차 구현한다.
3. 수정된 최종 후보에서 구현/runtime review 20개를 별도로 수행한다. 앱 runtime·Android/iOS simulator gates와 format, focused tests, scoped Clippy를 실행한다.
4. 구현 PR의 실제 diff를 별도 30개 관점으로 검토하고 필요한 수정 후 PR 화면과 embedded screenshots를 Agent Browser로 확인한다. GitHub checks/required status checks를 각각 확인한 뒤 승인된 rebase merge를 진행한다.
5. Merge SHA 기준 internal contract, plan, official status, Tailnet preview와 evidence를 순차 동기화한다. Agent Browser session과 process를 종료하고 process list에서 남은 browser를 확인한다.

이 bounded slice는 전체 `fit-content()`, implicit track sizing, CSS Grid 또는 WPT conformance 완료를 뜻하지 않는다. Android simulator는 실기기가 아니고 AVD는 software renderer이며 iOS Metal 표본은 compositor 제출 전이다.
