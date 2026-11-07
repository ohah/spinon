# C11.4 parsed Root HTML auto-row alignment percentage `row-gap`

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 intrinsic track sizing](./c11-4-grid-track-sizing.md) · [선행 fixed row-gap slice](./c11-root-html-auto-row-content-alignment-row-gap.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 계획 검토 20개, bounded 구현, 기본 RuntimeSession·Android/iOS Simulator 검증, 구현/runtime review 20개와 실제 PR diff review 30개를 마쳤다. 구현 PR [#1243](https://github.com/ohah/spinon/pull/1243)은 2026-11-07T00:01:43Z Rebase and merge로 병합됐고 merge SHA는 `50d171d69b730af821076faf4ffcb02a4a5e44a8`이다. PR check rollup은 비어 있으며 main required status check과 approving review가 없어 CI green으로 기록하지 않는다. main의 기존 `RootHtmlAutoRowStretchV1`은 fixed px `row-gap:0..=7.5px`만 허용했다. 이 bounded 후속 구현은 definite 40px block-size profile에서 finite non-negative percentage를 보존해 기존 Taffy sizing/alignment 경로로 전달한다. 일반 CSS admission이나 C11.4/C11 완료를 선언하지 않는다.

## 규범과 사전 비교

- CSS Gaps Level 1 §2.3은 percentage gap을 해당 축의 content-box 크기에 상대적으로 해석하고, Grid contents layout에서 definite content-box size로 resolve한다. CSS Grid Level 1 §10.1은 row-gap을 Grid rows 사이 gutter로 정의한다.
- [사전 비교 근거](../spec/internal/evidence/c11-root-html-auto-row-percentage-gap-precomparison-2026-11-07/README.md)는 definite 40×40px content box, one 40px explicit column, one 10px explicit row, two 10px auto rows, `column-gap:0`, direct child 3개를 고정했다. 7 percentage values × 기존 `align-content` 10개, 총 70 scene을 Chrome 157 Agent Browser 관측과 독립 CSS Grid 계산으로 대조했다.
- gap values 0%, 6.25%, 12.5%, 18.75%, 20%, 100%, 120%는 각각 0, 2.5, 5, 7.5, 8, 40, 48px다. 120%/center scene은 첫 item y=-43px, 마지막 item y=73px로 40px Grid 밖 overflow를 만든다. 최대 Chrome 관측 좌표 오차는 0.0260416667 CSS px이고 비교 한계는 1/32 CSS px다.

## 구현 범위

1. 기존 `RootHtmlAutoRowStretchV1` shape와 exact `align-content` allowlist를 유지하며 finite non-negative computed percentage `row-gap`을 허용한다. Percentage typed value는 px로 치환하지 않고 `LayoutLengthPercentage::Percentage`로 보존한다.
2. Percentage basis는 검증된 40px content-box block size다. 두 row gutters와 30px base tracks의 합산 크기가 finite f32 범위를 벗어나면 `row-gap` node/property error로 거부한다. CSS `normal`은 Grid에서 used zero gap으로 투영한다. Projection/Layout 경계에 직접 주입한 NaN, infinity, negative 및 overflow-producing percentage, `calc()`/unsupported values는 실패 원자성을 유지한다. 현재 Spinon parser는 HTML의 음수 authored `row-gap`을 fatal cascade diagnostic으로 처리한다. CSS invalid-declaration recovery 차이는 이 slice의 parser 범위 밖에 남기고 실행 근거에 명시한다.
3. 기존 px `0..=7.5px`, `column-gap:0`, direct child shape, tracks, static position, writing mode, overflow 및 style ownership 조건을 변경하지 않는다. 다른 Root HTML/C11 profiles는 이 확장으로 열지 않는다.
4. Taffy의 track sizing 및 content distribution이 gutter를 한 번만 계산하게 한다. 별도 manual gutter 보정은 넣지 않는다.

## 검증 관문

1. 별도 [계획 실패 관점 검토 20개](./c11-root-html-auto-row-percentage-gap-plan-review-20.md)를 마쳤다. Chrome observation과 CSS Gaps/Grid 독립 기대값을 구현 출력과 분리했다.
2. Style projection·Layout·RuntimeSession에서 percentage ratio 보존, `normal`/0%, 120%, fixed px 회귀, 직접 typed-input negative/NaN/infinity 거부, `calc()` Layout 거부, representability 경계, property/node attribution, 실패 atomicity와 recovery를 확인했다. HTML authored negative declaration은 현재 fatal cascade diagnostic으로 재현해 CSS parser recovery 차이로 한정해 기록했다.
3. Layout과 기본 `RuntimeSession::new_runtime_gpu_with_root_html()`의 70-scene matrix가 모두 통과했다. 40px basis의 row-gap, used auto-row sizes, item frames, 모든 10개 alignment와 overflow coordinates를 `expected-values.json`에 대조했다. RuntimeSession은 전용 filter 4/4, 기존 intrinsic-auto-row 회귀 묶음 28/28이다.
4. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 기본 Root HTML route에서 0% positive-space와 120%/center overflow 장면을 확인했다. 같은 HTML hash, layout-ready boxes, draw/present와 화면을 저장했다. 마지막 item이 Grid bottom 밖에서 실제로 그려지는지 simulator PNG pixels로 확인했으며 platform surface readback과 구분한다. [실행·hash·검증 로그](../spec/internal/evidence/c11-root-html-auto-row-percentage-gap-runtime-2026-11-07/README.md)
   - Variant scene은 기존 opt-in `*-fixture-test` route로만 앱 빌드에 넣는다. Fixture scene ID, HTML payload와 FFI adapter는 기본 artifact에서 제외한다. Route를 바꾸면 기본 artifact와 fixture-test artifact를 각각 대조한다.
   - Harness에서 사용할 수 있으면 Android PixelCopy와 iOS drawable readback으로 root 밖 pixel을 확인한다. 두 표본은 Agent Browser 및 simulator PNG screenshot과 구분한다. Screenshot만 관찰한 경우 surface readback이나 iOS compositor 이후 pixel 결과로 주장하지 않는다.
5. Focused layout 6/6, 기존 two-auto-row·single-auto-row Layout regression 각 1/1, projection 1/1, RuntimeSession row-gap 4/4 및 intrinsic-auto-row regression 28/28, 변경 crate all-target Clippy, `cargo fmt --all -- --check`, `git diff --check`를 실행해 통과했다. 원본 출력은 runtime evidence의 `verification/focused-validation.log`에 둔다.
6. [구현/runtime failure review 20개](../spec/internal/evidence/c11-root-html-auto-row-percentage-gap-runtime-2026-11-07/implementation-runtime-review-20.md)와 [실제 PR diff review 30개](./c11-root-html-auto-row-percentage-gap-pr-diff-review-30.md)를 마쳤다. 구현 PR #1243 본문에는 Android/iOS final simulator PNG를 첨부해 Agent Browser에서 네 이미지가 로드되는 것을 확인했다.
7. 병합 결과를 `spec/STATUS.md`, contract 0059, public roadmap source에 기록하고 [병합 후 문서 review 30개](../spec/internal/evidence/c11-root-html-auto-row-percentage-gap-docs-sync-2026-11-07/docs-pr-diff-review-30.md)를 마쳤다. 지정 Tailnet 제공본은 별도 사본이므로 별도 backup·served hash·Agent Browser 확인으로 동기화한다. 공식 `78/214`, C11.4, C11 완료 상태는 승격하지 않는다.

## 제외 범위

Indefinite/cyclic row-gap basis, percentages in `column-gap`, CSS math, nonzero column gap, 다른 Root HTML track/item shapes, nested Grid, arbitrary CSS admission, WPT conformance, Android 실기기/hardware GPU 및 iOS compositor 이후 pixel parity는 이 slice에서 다루지 않는다.
