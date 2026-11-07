# C11.4 parsed Root HTML `grid-auto-rows:fit-content(percentage)` 제한 구현 계획

**상태:** Agent Browser Chrome 157 사전 비교와 독립 기대값을 고정하고 계획 실패 관점 20개를 검토했다. 기능 구현은 시작하지 않았다.

**상위:** [C11 CSS Grid](../spec/STATUS.md#c11-css-grid) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**선행 구현:** Root HTML `grid-auto-rows:fit-content(px)` 제한 slice는 [PR #1265](https://github.com/ohah/spinon/pull/1265)로 병합됐다. 해당 구현은 singleton finite non-negative px cap만 허용한다.

**비교 근거:** [Chrome 관측·independent expected values·화면](../spec/internal/evidence/c11-4-root-html-fit-content-percentage-auto-rows-precomparison-2026-11-08/README.md) · [고정 입력](../tests/fixtures/css/c11/root-html-grid-auto-rows-fit-content-percentage-precomparison.html) · [계획 실패 관점 20개](./c11-4-root-html-fit-content-auto-rows-percentage-plan-review-20.md)

**계획 PR diff 검토:** [실제 PR 변경분 실패 관점 30개](./c11-4-root-html-fit-content-auto-rows-percentage-pr-diff-review-30.md)

## 결손과 목표

기본 parsed Root HTML `RuntimeRootHtmlTextPaintV1`은 definite fixed-px explicit grid를 가진 제한 profile에서 singleton `grid-auto-rows:fit-content(px)`를 implicit row에 연결한다. 같은 profile의 direct percentage cap은 typed cascade와 `GridTrackLengthPercentage::Percentage`로 표현할 수 있지만, profile 선택과 layout admission에서 거부한다. 명시적 track의 `fit-content(percentage)` 지원은 별도 `RootHtmlFitContentV1` 경로이며 implicit row로 자동 확장하지 않는다.

이 slice는 기존 fit-content implicit-row profile에서 singleton direct finite non-negative percentage cap을 허용한다. Row percentage basis는 definite Grid content-box block size다. 구현은 현재 고정 40×100px Grid/content box와 기존 단일 fixed-px explicit row·column 및 세 item shape를 유지한다. 이 조건으로 percentage를 CSS viewport나 border box 대신 definite 100px content-box height에 적용한다.

## 규범과 비교 모델

[CSS Grid Level 2 §7.2.1](https://www.w3.org/TR/css-grid-2/#track-sizes)는 행 track의 percentage를 Grid container 내부 block 크기에 상대적으로 정의한다. Grid 크기가 track 크기에 의존하는 경우 intrinsic sizing 중 percentage를 `auto`로 취급한다. [§12 track sizing](https://www.w3.org/TR/css-grid-2/#track-sizing-algorithm)에 따라 `fit-content()`의 최대 크기는 max-content contribution으로 시작하지만 fit-content 인수로 제한된다. 이 계획은 definite 100px content-box block size만 허용해 cyclic/indefinite percentage 계산은 열지 않는다.

고정 장면에서 fit-content 계산은 `max(auto/minimum contribution, min(resolved percentage cap, max-content contribution))`을 따른다. 중간 item 안쪽 block의 max-content contribution은 34px이고 기본 minimum은 0px이다. 다음 6px probe로 같은 singleton auto-row sizing function의 반복 적용도 확인한다.

## 고정 입력과 기대값

Chrome 157 Agent Browser, macOS arm64, viewport 1280×633, DPR 1에서 일곱 장면을 비교했다. 모든 Grid는 content box 40×100px, `grid-template-columns:40px`, `grid-template-rows:10px`, row flow, gap 0, `align-content:start`, visible overflow다. 첫 item은 explicit row 10px, 두 번째 item에는 34px 안쪽 block, 세 번째 item은 6px probe다. 별도 floor 장면의 두 번째 item은 `min-height:28px`다.

| 장면 | `grid-auto-rows` | 계산된 상한 | 사용 행 크기 | Implicit item | 다음 probe |
|---|---|---:|---|---|---|
| zero | `fit-content(0%)` | 0px | `[10,0,6]px` | `[0,10,40,0]` | `[0,10,40,6]` |
| twenty | `fit-content(20%)` | 20px | `[10,20,6]px` | `[0,10,40,20]` | `[0,30,40,6]` |
| twenty-four | `fit-content(24%)` | 24px | `[10,24,6]px` | `[0,10,40,24]` | `[0,34,40,6]` |
| cap boundary | `fit-content(34%)` | 34px | `[10,34,6]px` | `[0,10,40,34]` | `[0,44,40,6]` |
| above max | `fit-content(40%)` | 40px | `[10,34,6]px` | `[0,10,40,34]` | `[0,44,40,6]` |
| over 100% | `fit-content(120%)` | 120px | `[10,34,6]px` | `[0,10,40,34]` | `[0,44,40,6]` |
| minimum floor | `fit-content(20%)` | 20px | `[10,28,6]px` | `[0,10,40,28]` | `[0,38,40,6]` |

같은 explicit auto row sizing function이 다음 probe 행에도 반복되며, 고정 크기 6px은 모든 장면에서 유지된다. Chrome 관측에는 computed `grid-auto-rows`, 사용된 `grid-template-rows`, Grid/item/payload/probe frame을 담았다. CSS Grid 공식과 고정 입력으로 브라우저 결과를 보기 전에 독립 기대값을 계산했고, 일곱 관측 모두 0.02 CSS px 이내로 일치했다.

## 지원·거부 경계

- 기본 parsed Root HTML runtime route에서 singleton direct `fit-content(percentage)`만 허용한다. Percentage는 finite non-negative여야 하며 `0%`와 `100% 초과`도 유효한 입력으로 다룬다.
- Grid content-box 너비와 높이는 각각 40px, 100px로 고정하고 explicit column/row는 양의 fixed-px track 하나씩 유지한다. 행 percentage는 definite 100px content-box 높이를 기준으로 계산한다.
- 기존 singleton px 상한 동작, 고정 자식 geometry, 안쪽 34px payload, auto-placement 순서, gap 0, 실패 원자성을 유지한다.
- `fit-content(calc(...))`, length-percentage math AST, negative/non-finite typed value, 여러 auto-row 항목, 혼합 track list, definite하지 않은 Grid 높이, 다른 track shape, nonzero gap, border/padding, areas, repeats, dense/column flow, implicit columns, nested Grid, 일반 author CSS admission은 범위 밖이다.
- Percentage 상한은 viewport 높이·border-box 높이·item intrinsic size가 아니라 Grid 내부 block 크기를 기준으로 한다. Percentage를 100%로 clamp하지 않으며, fit-content의 max-content contribution과 automatic minimum으로 사용 track을 정한다.
- 기존 explicit row/column의 fit-content percentage profile 및 다른 모든 Root HTML Grid profile은 기존 admission을 유지한다. 이 제한 slice는 모든 implicit percentage sizing이나 공개 CSS 지원을 뜻하지 않는다.

## 구현 수락 기준

1. 기존 style candidate는 singleton direct `Percentage(cap)` 값만 허용하고 typed ratio를 보존한다. `Calc` AST, mixed list, 나머지 track 크기는 거부한다. 기존 singleton `LengthPx(cap)` 경로는 유지한다.
2. `RootHtmlFitContentAutoRowsV1` layout validation은 기존 40×100px profile에서만 finite non-negative percentage를 허용한다. 이미 있는 Taffy `fit-content(LengthPercentage::Percent(..))` adapter로 sizing하고 percentage resolver를 새로 만들지 않는다.
3. parsed Root HTML runtime fixture에서 oracle 장면 일곱 개를 검사한다. Computed track type, 사용 frame, nested-content overflow, 행 순서, grid/style snapshot identity, percentage가 포함된 `Calc` 또는 지원 밖 shape에 대한 같은 세션 거부·복구를 확인한다.
4. `0%`, 20/24/34/40% 상한 경계, 100% 초과, 상한을 넘는 item minimum, probe에 대한 auto-row 반복, px 회귀를 확인한다. Percentage가 px나 viewport basis로 잘못 바뀌지 않는지도 검사한다.
5. Percentage가 포함된 `Calc`, 다중 track list, 다른 explicit track shape, definite하지 않은 Grid block size, areas, nonzero gap, column/dense flow에 대한 negative control을 추가한다. 실패 revision은 partial frame/draw state를 발행하지 않아야 하며 같은 세션에서 지원 입력으로 복구되어야 한다.
6. Style-to-layout, layout, RuntimeSession focused tests, `cargo fmt --all -- --check`, 변경 crate scoped Clippy, Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 기본 Root HTML 빌드·실행을 수행한다. 로그/screenshot을 최종 source와 설치 artifact hash에 결합하고 AVD/software renderer 및 iOS compositor 이전 한계를 구분한다.
7. 새 구현/runtime 실패 관점 20개를 검토하고 구현 PR diff는 서로 다른 30개 실패 관점으로 확인한다. 발견, 수정, 남은 한계를 기록한다. PR 본문에는 가능한 경우 commit-pinned Android/iOS 화면, 필수 라벨, 변경 상태/명세 ID, 실행 근거 및 제외 범위를 기록한다.
8. 구현 PR 검토 뒤 Rebase-and-merge하고, 별도 PR에서 `spec/STATUS.md`, 내부 계약 0059, 공개 roadmap과 승인된 Tailnet preview를 동기화한다. 제공 hash와 Agent Browser desktop/mobile 화면을 확인하고 browser process를 종료한다. C11/C11.4 checkbox 및 공식 `78/214`는 유지한다.

## 검증 순서

1. 제품 코드 구현 전에 계획 PR을 먼저 병합한다.
2. 같은 checkout에서 typed-style admission, Grid layout admission, 기본 Root HTML runtime fixture, negative control을 순서대로 갱신한다. 무관한 local artifact는 보존하고 검토한 경로만 stage한다.
3. 구현 관점 20개 review, Rust focused gate, scoped Clippy, rustfmt, 최종 source의 Android/iOS Simulator 실행을 수행한다.
4. 가능한 경우 commit-pinned 화면을 구현 PR에 올리고 프로젝트 규칙에 따라 별도로 30개 관점에서 검토한다. Agent Browser에서 PR diff와 이미지 렌더링을 확인한 뒤 사용자가 승인한 흐름에 따라 Rebase-and-merge한다.
5. 별도 PR에서 병합 후 status/roadmap/Tailnet 동기화를 반영하고 `origin/main`과 제공본을 대조한다.

이 slice는 C11.4/C11, 일반 CSS Grid author 지원, WPT, hardware GPU·실기기 coverage, iOS compositor 이후 pixel 동등성, implicit track의 모든 fit-content percentage 지원을 완료 처리하지 않는다.
