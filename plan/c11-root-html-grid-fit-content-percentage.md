# C11.1 · parsed Root HTML Grid explicit `fit-content(percentage)` tracks

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [Root HTML Grid 기본 경로](./c11-root-html-grid-runtime.md) · [explicit `fit-content(px)` 제한 구현](./c11-root-html-grid-fit-content.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**구현 전 비교:** [Chrome 156 percentage matrix와 독립 기대값](../spec/internal/evidence/c11-root-html-grid-fit-content-percentage-precomparison-2026-11-03/README.md) · [계산식·기대값](../spec/internal/evidence/c11-root-html-grid-fit-content-percentage-precomparison-2026-11-03/expected.json) · [matrix SHA-256](../spec/internal/evidence/c11-root-html-grid-fit-content-percentage-precomparison-2026-11-03/SHA256SUMS.txt) · [border overflow control](../spec/internal/evidence/c11-root-html-grid-fit-content-percentage-overflow-control-2026-11-03/README.md) · [over-border screenshot](../spec/internal/evidence/c11-root-html-grid-fit-content-percentage-overflow-control-2026-11-03/chrome-156-agent-browser.png) · [overflow SHA-256](../spec/internal/evidence/c11-root-html-grid-fit-content-percentage-overflow-control-2026-11-03/SHA256SUMS.txt)

**계획 실패 관점 검토:** [20개](./c11-root-html-grid-fit-content-percentage-plan-review-20.md) · **계획 PR diff 검토:** [user 요청 20개](../spec/internal/evidence/c11-root-html-grid-fit-content-percentage-overflow-control-2026-11-03/plan-pr-diff-review-20.md) 및 [프로젝트 규칙 30개](../spec/internal/evidence/c11-root-html-grid-fit-content-percentage-overflow-control-2026-11-03/plan-pr-diff-review-30.md) · **구현/runtime 검토:** [20개 완료](../spec/internal/evidence/c11-root-html-grid-fit-content-percentage-runtime-2026-11-03-pass02/implementation-runtime-review-20.md) · **구현 PR diff 검토:** user 요청 20개와 프로젝트 규칙 30개를 별도 진행.

**계획 상태:** 계획 PR [#1105](https://github.com/ohah/spinon/pull/1105)이 `2026-11-03` 기본 브랜치에 병합됐고, 이 후속 구현을 진행 중이다. 기존 [#1093 계획](./c11-root-html-grid-fit-content.md)은 `fit-content(%)`와 `fit-content(calc())`를 명시적으로 제외한다. 따라서 이 작업은 기존 계획에 포함된 것으로 간주하지 않고 별도 계획으로 유지한다. 이번 항목도 C11.1/C11 완료나 공개 CSS 지원을 뜻하지 않는다.

**현재 구현 후보 근거:** percentage layout 12/12 및 전체 `grid_fit_content` 회귀 20/20, cascade 4/4, style-to-layout `fit_content` 7/7(그중 percentage projection 2개), Root HTML runtime 3/3, style-render 기본 71 passed·1 ignored 및 C11.4 fixture mode 74 passed·1 ignored, scoped Clippy/rustfmt를 확인했다. Android API 37 ARM64 SwiftShader AVD PixelCopy 4/4, iPhone 17 Pro/iOS 26.2 Simulator Metal drawable 4/4도 고정 overflow scene에서 통과했다. 양 플랫폼 캡처는 한 fixed overflow scene의 표면 표본이며, iOS 결과는 compositor 제출 전이다. [runtime logs, screenshots, source/artifact hashes](../spec/internal/evidence/c11-root-html-grid-fit-content-percentage-runtime-2026-11-03-pass02/README.md) · [구현/runtime 실패 관점 검토 20개](../spec/internal/evidence/c11-root-html-grid-fit-content-percentage-runtime-2026-11-03-pass02/implementation-runtime-review-20.md). 구현 PR diff 검토(user 20개 및 프로젝트 규칙 30개), PR checks와 merge는 남아 있다. C11.1/C11 checkbox와 공식 `72/213`은 유지한다.

## 목표와 범위

기본 `RuntimeRootHtmlTextPaintV1` parsed Root HTML route에서 explicit `grid-template-columns`와 `grid-template-rows`의 `fit-content(<percentage>)` cap을 definite finite non-negative 해당 축에서만 계산한다. 기존 `GridTrackLengthPercentage::Percentage` typed 경로와 integer `repeat()` expansion을 재사용하고, projection/layout admission은 이 Root HTML profile에 한정한다.

Percentage basis는 해당 축의 definite content-box 크기다. Column은 definite inline/content width, row는 definite block/content height를 쓴다. 이 제한 구현은 기존 `RootHtmlFitContentV1` layout gate를 따르므로 두 Grid content dimension이 모두 definite해야 하며, percentage가 없는 반대 축의 `auto` size는 아직 지원하지 않는다. `0%`와 definite zero basis는 유효한 입력이고, 100%를 넘는 cap은 줄이지 않는다. Fit-content의 auto minimum은 resolved cap보다 클 수 있으며, max-content contribution이 cap보다 작으면 track은 그 contribution에 머문다. Gap은 basis에서 빼지 않는다. 이번 fixture profile은 padding·border·gap 0인 cap 계산 장면과, border를 별도로 검증하는 content-box basis 및 overflow control을 분리한다.

긍정 범위:

- parsed Root HTML 기본 route의 explicit rows/columns, finite non-negative direct percentage argument.
- 이미 지원되는 positive integer `repeat()`가 확장한 direct `fit-content(percentage)` track. 반복 횟수와 확장 결과는 기존 10,000-track quota를 따른다.
- 해당 축 definite size를 요구한다. 각 축의 `0%`, zero basis, 100% 초과 cap을 확인한다.
- overflow-visible 기본 동작에서 used track/item이 Grid border 바깥에 그려질 수 있음을 별도 고정-size scene으로 확인한다.

지원하지 않는 입력은 기존 node/property 진단으로 fail-closed하고, scene/frame 일부를 publication하지 않는다.

## 범위 밖

`fit-content(px)` 이외의 동작은 현재 지원 범위를 그대로 둔다. 특히 `fit-content(calc())`, `minmax()`와 조합, intrinsic/auto sizing, auto-repeat, named lines, `grid-template-areas`와 percentage fit-content 조합, area-created/implicit tracks, `grid-auto-rows`/`grid-auto-columns`, column-flow, dense placement, indefinite/cyclic percentage axis, non-finite/negative raw values, public CSS admission, WPT 적합성 및 전체 C11은 제외한다. 기존 제한 named-area `fit-content(px)` 경로 및 다른 profile은 바꾸지 않는다.

## 규범과 독립 비교 기준

[CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-functions)의 `fit-content()` formula는 `max(auto minimum, min(limit, max-content contribution))`이다. [Chrome 156 사전 비교](../spec/internal/evidence/c11-root-html-grid-fit-content-percentage-precomparison-2026-11-03/README.md)의 independent expected JSON은 Chrome 값을 보기 전에 formula와 고정 Ahem intrinsic probes로 만들었다. Ahem font hash는 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`이다.

고정 입력은 13개다: column cap 0%/5%/20%/60%/120%, `repeat(2, fit-content(20%))`, definite zero-width basis, 100px row basis의 0%/20%/120% cap, definite zero-height row basis, authored 60px fixed-item floor, 그리고 Ahem 20-word text에서 80px cap. 독립 기대값은 각각 `[20]`, `[20]`, `[40]`, `[110]`, `[110]`, `[40,40]`, `[20]`, `[40]`, `[40]`, `[40]`, `[40]`, `[60]`, `[80]`이다. 두 행 축 zero-basis 및 over-100% scene은 cap 자체를 별도 계산하고 40px row minimum floor가 이를 넘는 결과를 기록한다. Ahem intrinsic은 min/max column 20px/110px, 20px inline size의 row minimum 40px, text control의 min/max 10px/390px이다. Chrome 156 Agent Browser 관찰은 13개 장면에서 독립 track/item 값과 일치해야 한다. CSS 0.02px 허용 오차를 적용한다. `fit-content(120%)`의 50px definite content-box discriminator도 별도로 확인했다. Formula상 cap은 60px이지만 definite available grid space가 50px이므로 Chrome 156의 used track은 50px이다. 테두리가 있는 별도 discriminator는 content-box width 30px, border 2px, `fit-content(50%)`, Ahem min/max 10px/30px으로 15px content-box cap과 17px border-box 오해를 구별하며, Chrome 156과 공유 layout 알고리즘이 모두 15px을 사용했다. [Chrome 관찰과 화면](../spec/internal/evidence/c11-root-html-grid-fit-content-percentage-runtime-2026-11-03-pass02/README.md).

추가 [over-border control](../spec/internal/evidence/c11-root-html-grid-fit-content-percentage-overflow-control-2026-11-03/README.md)은 content width 30px, `fit-content(200%)`, fixed child 60px를 사용한다. 독립 기대는 60px track, root border right `x=54`, child right `x=82`, visible overflow 28px이다. Headless Chrome 156 Agent Browser 측정값과 screenshot pixel samples가 이를 확인한다. 이 control은 browser baseline이지 Spinon 결과가 아니다.

## 구현 수락 기준

1. cascade snapshot에서 finite non-negative direct percentage argument를 typed 값으로 보존한다. 음수/invalid CSS 선언은 parser/cascade 진단 경계를 확인하고, percent 외의 calc/intrinsic variant를 percentage로 강제 변환하지 않는다.
2. Root HTML projection은 no-area explicit row/column의 direct `fit-content(percentage)` 및 기존 positive integer repeat expansion만 허용한다. fixed px, direct percentage/fraction, bounded `minmax()`, fixed repeat이 각자 현재 갖는 profile 밖으로 새 조합이 열리지 않는지 profile 경계를 테스트한다.
3. `grid-template-areas`, auto-repeat/line names, implicit auto tracks, column-flow, `fit-content(calc())`, raw negative/non-finite percentage를 거부한다. invalid input은 property-bound diagnostic을 반환하고 partial layout output이 없어야 한다. 기존 `fit-content(px)` area path와 다른 profile 회귀는 유지한다.
4. Layout에서 `RootHtmlFitContentV1`의 percentage limit은 동일 축의 definite finite non-negative size만 사용하고, 현재 profile gate에 따라 두 content dimension 모두 definite해야 한다. 반대 축의 `auto` size는 범위 밖이다. 0%와 definite zero basis를 분리해 검사한다. Indefinite width/height, cyclic/overridden basis, non-finite/negative typed value는 거부한다. Aggregate track/frame overflow와 10,000-track quota는 Taffy/frame publication 전 검사한다.
5. Ahem `spinon-layout` test engine으로 Chrome 156 13-scene percentage matrix를 independent expected JSON과 비교한다. Width와 height axis에서 각각 0% cap, zero basis, >100% cap, below-minimum floor를 관찰하고, column intermediate/above-max cap, fixed repeat, fixed-size floor 및 text contribution도 확인한다. 이 결과는 Root HTML font-loading 또는 mobile text-paint evidence가 아니다.
6. 기본 parsed Root HTML runtime에 font-independent fixed-box positive/rejection fixtures를 추가한다. Direct column percentage, row percentage, fixed integer repeat, definite zero basis, unsupported indefinite axis와 property diagnostic을 확인한다. Border overflow positive scene은 content width 30px, cap 200%, width 60px child로 track/item width 60px과 root border를 넘는 28px을 검증한다.
7. Runtime geometry와 표면 증거를 분리한다. Android API 37 ARM64 AVD에서는 PixelCopy, iPhone 17 Pro/iOS 26.2 Simulator에서는 Metal drawable sample을 고정 source revision에 묶는다. root 안, 표시한 border edge, root 바깥에서 child 안, child 바깥의 sample을 각각 확인한다. AVD software renderer, iOS compositor 제출 전 drawable, simulator, 실기기·hardware GPU 한계를 구분한다.
8. 기존 `fit-content(px)`, percentage tracks/gaps, `minmax()`, integer repeat, named-area, implicit row/column, column-flow, overflow, C11.4 track sizing 회귀를 실행한다. `cargo fmt --all -- --check`, 관련 focused Rust suites, 변경 crate scoped Clippy, Android/iOS Simulator builds와 default Root HTML route 실행을 남긴다. 전체 suite 실패나 미실행 gate는 분리 기록한다.
9. 구현 전 계획 자체 failure mode 20개, 구현 후 새로운 runtime/code 관점 20개, 구현 PR diff 별도 30개를 기록한다. 계획 PR diff도 사용자 요청 별도 20개와 프로젝트 규칙 별도 30개로 검토한다. 리뷰에서 발견한 오류는 수정 후 관련 근거를 다시 확인한다.
10. Rebase-and-merge 뒤 내부 계약 `0059` 버전은 `0.1.0`으로 유지한다. `spec/STATUS.md`에는 bounded slice, 증거와 한계를 연결하되 C11.1/C11 checkbox와 공식 `72/213`은 그대로 둔다. 승인된 Tailnet roadmap preview를 merge SHA에 맞춰 sync하고 Agent Browser desktop/mobile, served hash와 border-overflow evidence를 확인한 뒤 browser/session process를 종료한다.

## 진행 순서

1. 계획 PR을 먼저 병합한다. 계획 PR 본문에 Chrome overflow screenshot을 포함하고, 필수 라벨·변경 범위·잔여 한계·review 결과를 기록한다.
2. 같은 checkout에서 cascade → projection → layout → Ahem sizing → default Root HTML runtime을 순차 구현한다. 새 worktree나 병렬 agent를 만들지 않는다.
3. 변경된 최종 구현에서 구현/runtime 20개 review, focused tests, default artifact/feature 경계, Android API 37 AVD 및 iOS Simulator 실행을 수행한다.
4. 구현 PR의 실제 diff를 프로젝트 요구 별도 30개 관점에서 검토하고, 사용자 요청 별도 20개 PR review도 보존한다. Chrome 및 두 Simulator 화면을 PR 본문에 올리고 Agent Browser에서 로드한다.
5. Checks/required checks와 merge state를 확인한 뒤 사용자가 이미 승인한 Rebase-and-merge 흐름을 진행한다. Merge SHA 기준 문서·상태·Tailnet 미리보기를 순차 갱신하고 Agent Browser 세션과 프로세스를 종료한다.

이 slice는 전체 `fit-content()`, CSS Grid, WPT 또는 공개 CSS 지원을 뜻하지 않는다. Android AVD는 실기기가 아니고 iOS Simulator drawable은 compositor 이후 화면을 검증하지 않는다.
