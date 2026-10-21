# C11.4 · nested Grid 양 축 intrinsic sizing 제한 계획

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [두 단계 nested Grid inline-axis 계획](./c11-4-nested-grid-depth-two.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 계획·사전 비교·계획 검토를 마치고 구현 중이다. 구현 결과 검토와 PR은 아직 완료되지 않았다. C11.4 checkbox와 공식 완료 수 `72/213`은 유지한다.

PR #452가 2026-10-21 Rebase and merge된 뒤 같은 checkout에서 `feat/c11-4-nested-grid-two-axis-depth-two-20261022` 브랜치를 만들었다. 네 개의 새 양 축 scene adapter와 runtime/layout 경로를 구현하고 Rust focused test, 기본·fixture archive 경계, workspace와 fixture Clippy, Android API 37 ARM64 AVD, iPhone 17 Pro/iOS 26.2 Simulator 캡처를 통과했다. 별도의 구현/runtime 실패 관점 20개와 PR diff 검토 30개도 기록했다. 구현 PR의 생성·필수 check 확인·Rebase and merge와 병합 후 Tailnet 갱신이 남아 있다. 세부 수치와 원본 로그는 [실행 근거](../spec/internal/evidence/c11-4-nested-grid-two-axis-runtime-2026-10-22/README.md)에 있다.

## 목적과 기존 범위

기존 [두 단계 nested Grid fixture](./c11-4-nested-grid-depth-two.md)는 root → middle → inner Grid에서 `width`와 `grid-template-columns`만 `min-content` 또는 `max-content`로 맞춘다. `height`는 `auto`, `grid-template-rows`는 `[auto]`다. 그래서 한 Ahem text item의 inline-axis intrinsic contribution이 두 단계 nesting을 통과하는 것은 고정했지만, 같은 계산에서 block-axis 크기와 행 track을 동시에 intrinsic sizing하는 경로는 다루지 않았다.

이번 slice는 기존 opt-in C11.4 fixture-test 경계에서 depth-one control과 depth-two 장면을 함께 측정한다. 각 Grid의 width/height와 단일 column/row track을 같은 intrinsic constraint로 고정한다. **고정 네 장면에 한해** fixture projection/adapter가 `height:min-content|max-content`와 일치하는 row track을 기존 inline-axis intrinsic sizing과 함께 받도록 좁힌다. 일반 CSS admission, 공개 API, 내부 계약 버전 변경 또는 전체 C11.4 완료 표시는 포함하지 않는다. 미등록 scene·추가 item·다른 sizing grammar는 부분 layout 없이 fail-closed로 거부한다.

## 고정 입력과 독립 기대값

입력은 [HTML fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-nested-grid-two-axis-precomparison.html)와 저장소 Ahem font, `10px/10px`의 `XX XXXXXXX` 한 text item이다. 각 Grid는 margin, padding, border, gap 0, `align-content:start`, `justify-content:start`, `align-items:start`를 쓴다. min-content 장면에서는 모든 Grid의 width/height와 column/row track이 각각 `min-content`; max-content 장면에서는 모두 `max-content`다.

CSS Grid Layout Module Level 1 2025 CRD [§5.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-container-sizing)는 Grid container의 intrinsic size를 해당 track sizing 결과로 정하고, [§11.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-overview)은 columns sizing을 rows sizing보다 먼저 수행한다. 고정 Ahem 입력에서 longest unbreakable word는 70px, unwrapped advance는 100px이다. 따라서 70px min-content column에서 text는 2개의 10px line으로 wrap되어 block contribution 20px을 만들고, 100px max-content column에서는 1개의 10px line으로 남는다. 각 단계는 단일 track과 단일 direct child이므로 같은 contribution을 다음 parent Grid에 전달한다.

| 장면 | Grid nesting | 각 Grid의 기대 columns | 각 Grid의 기대 rows | 각 Grid와 text의 기대 크기 |
| --- | --- | --- | --- | --- |
| `nested-depth-one-min-content` | root → inner | `[70px]` | `[20px]` | `70×20px` |
| `nested-depth-two-min-content` | root → middle → inner | `[70px]` | `[20px]` | `70×20px` |
| `nested-depth-one-max-content` | root → inner | `[100px]` | `[10px]` | `100×10px` |
| `nested-depth-two-max-content` | root → middle → inner | `[100px]` | `[10px]` | `100×10px` |

규범 계산은 [독립 기대값 JSON](../spec/internal/evidence/c11-4-nested-grid-two-axis-precomparison-2026-10-22/spec-derived-expectations.json)에 두고 Chrome 수치에서 역산하지 않는다. Agent Browser `HeadlessChrome/156.0.0.0`, macOS ARM, viewport `1280×633`, DPR 1의 관측은 [별도 JSON](../spec/internal/evidence/c11-4-nested-grid-two-axis-precomparison-2026-10-22/chrome156-observation.json)과 [화면](../spec/internal/evidence/c11-4-nested-grid-two-axis-precomparison-2026-10-22/chrome156-nested-grid-two-axis.png)에 보존했다. Chrome의 absolute page y 위치는 fixture 배치일 뿐이다. 판정은 각 root를 기준으로 한 box 크기·track·상대 위치를 사용하고, 크기와 track tolerance는 1/64 CSS px다. asset/input/evidence SHA-256은 [사전 비교 README](../spec/internal/evidence/c11-4-nested-grid-two-axis-precomparison-2026-10-22/README.md)에 기록한다.

## 구현 및 판정 절차

1. 이 계획 PR을 `Rebase and merge`한 뒤 같은 checkout에서 최신 `origin/main`을 기준으로 구현 branch를 만든다. worktree는 새로 만들지 않는다.
2. 기존 `c11_4_nested_grid_depth_two` fixture adapter를 조사해 네 개의 별도 two-axis scene ID를 고정 allowlist로 추가한다. root/middle/inner owner와 정확한 child 수·순서, Ahem text node, style revision을 확인하고, depth-one controls와 depth-two cases를 각각 dispatch한다.
3. fixture projection에서 동시에 intrinsic width/height와 min/max column/row track을 허용할 최소 경계를 찾는다. 이 네 scene의 정확한 조합만 허용하고, 알 수 없는 intrinsic 조합·track 목록·child topology는 오류로 끝내며 partial `LayoutOutput`이나 불완전 trace를 노출하지 않는다. 일반 style projection이나 다른 runtime profile로 admission을 넓히지 않는다.
4. focused Rust/runtime assertions에서 각 Grid node의 Columns·Rows used track, width/height, root-relative frame, text frame과 입력 revision을 모두 확인한다. Chrome 관측과 독립 규범 기대값을 각각 대조하며 depth-one control이 depth-two 결과와 같은지 검사한다.
5. opt-in fixture 앱을 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 실행한다. 네 scene 각각의 node identity/order, 두 축 track·frame, trace completeness, `drawPresented=true`, 앱 산출물/설치 산출물 SHA-256을 기록한다. Android 실기기는 이번 gate에 포함하지 않는다.
6. `cargo fmt --all -- --check`, focused runtime/layout/FFI tests, workspace all-target Clippy `-D warnings`, 양 플랫폼 fixture app builds를 실행한다. 기본 product archive에서 fixture symbol·scene string·고정 Ahem payload가 빠지고, opt-in fixture archive에서는 positive control로 포함되는지 확인한다. 실행하지 않은 검사나 플랫폼은 통과로 표시하지 않는다.
7. 구현 뒤에는 계획 검토와 중복되지 않는 새로운 구현/runtime 실패 관점 20개를 기록한다. 구현 PR diff는 프로젝트 규칙에 따라 별도의 30개 실패 관점으로 검토하고 수정한 문제의 관점을 재실행한다. PR에는 로드가 확인된 Android/iOS Simulator 화면을 넣는다.
8. 현재 PR required checks와 merge state를 확인한 뒤 Rebase and merge한다. merge 후 Tailnet preview를 선택 build·안전 동기화·hash 및 Agent Browser 확인으로 갱신한다. 전용 Agent Browser session과 프로세스는 확인 후 닫는다.

## 범위 밖과 완료 판정

- 임의 HTML/CSS, 임의 nested depth, multiple/spanning children, subgrid, gaps, inline descendants, writing mode, percentage/calc, author min/max constraints, dynamic font/layout revision은 포함하지 않는다.
- glyph paint equality, WPT, Android hardware GPU 또는 physical device, iOS compositor 이후 pixels, 다른 browser와 전체 CSS Grid 적합성은 증명하지 않는다.
- 이 계획의 Chrome oracle와 예상값은 Spinon 구현 근거가 아니다. 완료 여부는 구현 후 Rust/FFI/runtime, 양 플랫폼 simulator, artifact boundary 및 별도 review 결과가 모두 남아야 판단한다.
- 이 slice가 통과해도 [상태 대장](../spec/STATUS.md)의 C11.4 checkbox와 완료 수 `72/213`은 바꾸지 않는다. 내부 계약 0059는 `0.1.0`으로 유지한다.

## 계획 검토와 근거

- [계획 실패 관점 20개](./c11-4-nested-grid-two-axis-depth-two-plan-review-20.md)
- [Chrome 156 사전 비교·독립 계산·SHA-256](../spec/internal/evidence/c11-4-nested-grid-two-axis-precomparison-2026-10-22/README.md)
- [계획 PR diff 실패 관점 30개](../spec/internal/evidence/c11-4-nested-grid-two-axis-precomparison-2026-10-22/plan-pr-diff-review-30.md)
- [CSS Grid Layout Module Level 1, 2025 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)
