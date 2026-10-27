# C11.4 · `min-height` 아래 `fit-content()` 행 WPT slice 계획

**상태:** 계획 PR #698은 병합됐다. opt-in 구현 후보가 같은 체크아웃에서 진행 중이며, 2026-10-28 기준 Rust oracle과 Android API 37 AVD·iPhone 17 Pro/iOS 26.2 Simulator 실행은 통과했다([runtime evidence](../spec/internal/evidence/c11-4-fit-content-min-height-runtime-2026-10-28/README.md)). 구현 적대 검토, 최종 PR 검토와 병합이 남았다. 일반 CSS 지원 또는 C11.4 완료를 뜻하지 않으며 공식 완료 수 `72/213`과 C11.4 체크 상태는 유지한다.

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

## 목표와 범위

Pinned WPT `css/css-grid/grid-definition/fit-content-track-sizing.html`의 두 번째 assertion을 기존 opt-in C11.4 CSS→HostDocument/Stylo→typed Grid input→Taffy→native app 경로에서 확인한다. 대상은 `height:auto; min-height:300px`인 Grid의 유일한 implicit row가 `fit-content(40px)`이고, 10px 자식의 used row size가 `10px`인 경우다. 같은 WPT의 마지막 `fit-content(40px) auto` assertion은 이미 Rust oracle PR #662와 Android/iOS runtime PR #664에서 다뤘다. 이번 계획은 그 fixture를 넓혀 한 화면에 여러 WPT 사례를 중복 구현하지 않는다.

이 계획은 test-only oracle와 전용 fixture 경로만 다룬다. 기본 CSS admission, production `ParsedRootHtml`, CSS `zoom`, 전체 WPT suite, 새 Grid 알고리즘, 공개 API, 전체 C11.4는 범위에 넣지 않는다.

## 고정 출처와 기준 관찰

- WPT 저장소 revision: `c999c58338ee1d223df5ad62e48be1202ced0d37`.
- 파일: [`fit-content-track-sizing.html`, SHA-256 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html).
- 대상 DOM은 [27행](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L27), assertion은 [87–88행](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L87-L88)이다.
- Agent Browser에서 `wpt.live`의 upstream testharness를 확인했다. 13개 assertion이 모두 Pass였고 `minHeight` assertion도 Pass였다. 전체 목록과 screenshot, pinned/live source hash 비교는 [기존 Agent Browser 근거](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-precomparison-2026-10-27/README.md)에 있다. 이 결과는 Chrome의 upstream WPT 실행이며 Spinon WPT pass가 아니다.
- 이 case의 별도 사전 비교와 독립 기대값은 [사전 비교 근거](../spec/internal/evidence/c11-4-fit-content-min-height-precomparison-2026-10-28/README.md), [expected.json](../spec/internal/evidence/c11-4-fit-content-min-height-precomparison-2026-10-28/expected.json)에 둔다. 계획 자체의 실패 관점 검토 20개는 [별도 검토 문서](./c11-4-fit-content-min-height-plan-review-20.md), 이 계획 PR diff의 새 실패 관점 검토 20개는 [PR diff 검토](./c11-4-fit-content-min-height-pr-diff-review-20.md)에 있다.

## 독립 기대값

입력은 WPT 구조를 유지한다. Grid는 `width:auto; height:auto; min-height:300px`, `grid-template-columns:60px`, `grid-auto-rows:fit-content(40px)`, gap·padding·border 0이다. 직접 자식 item 하나는 auto height, 기본 `min-height:auto`, 기본 `overflow:visible`이며 그 안의 block child 하나가 `height:10px`이다. root 폭은 deterministic frame을 위해 60px로 고정한다. 축은 block sizing만 비교한다.

CSS Grid 2025 CRD [§11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch)는 free space가 indefinite인 경우에도 Grid의 definite `min-height`로 stretch 단계의 free space를 계산한다고 정한다. 하지만 fit-content track의 max sizing function은 [`auto`가 아니라 max-content](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-terms)이므로 §11.8의 auto-max-track 대상이 아니다. `align-content:normal`이 stretch 단계를 실행해도 fit-content row는 남은 공간을 받지 않는다.

독립 CSS Grid 계산은 다음과 같다.

```text
Grid used border-box             = [60px, 300px]
implicit row input               = fit-content(40px)
single item minimum contribution = 10px
single item max-content size     = 10px
row base after intrinsic sizing  = 10px
row growth limit                 = min(10px, 40px) = 10px
positive free space before stretch = 300px - 10px = 290px
StretchAutoTracks alignment      = enabled by align-content: normal
stretch-eligible rows             = [] (fit-content max is not auto)
final row size                    = 10px
item and child block size         = 10px
```

따라서 CSSOM의 `grid-template-rows`는 `10px`, root used height는 `300px`이어야 한다. Track trace에서 `StretchAutoTracks { applied: true }`는 정렬 단계가 실행됐다는 뜻이며, track이 실제로 늘었다는 뜻이 아니다. 그 단계 전후 row base는 모두 `[10px]`이어야 한다. 이 구분을 유지한다.

## 구현 경계와 입력 고정

계획 병합 후 scene ID `fit-content-min-height-wpt`를 추가한다. 기존 ID와의 중복은 없다. 제안 identity는 root `c114-fit-content-min-height-grid`, item `c114-fit-content-min-height-item`, child `c114-fit-content-min-height-child`다.

- Root style는 `display:grid; width:60px; height:auto; min-height:300px; grid-template-columns:60px; grid-template-rows:none; grid-auto-rows:fit-content(40px); align-content:normal; grid-auto-flow:row`이며 margin·padding·border·gap은 0이다.
- Tree에는 root의 직접 자식 item 하나와 item의 단일 nested block child만 둔다. Item의 기본 `min-height:auto`와 `overflow:visible`은 유지하고 child height는 10px로 둔다. Inline text, image, span, explicit row, second track, nonzero gap 및 size-affecting paint style은 넣지 않는다.
- 전용 fixture adapter는 root의 `height:auto`, `min-height:300px`, computed `grid-auto-rows`, typed `GridAutoRows`, root/node identity와 같은 revision을 각각 확인한다. 입력이 다르거나 node가 빠지거나 추가되면 거부한다.
- fixture는 기존 opt-in `c04-runtime-gpu-fixture-test` 경계 아래에 둔다. 기본·제품 artifact에 scene ID, fixture FFI 심볼 또는 payload를 추가하지 않는다. 새 제품 API나 CSS admission은 만들지 않는다.

## 수용 기준

1. Fixed WPT source hash와 기대값 JSON을 계획/fixture evidence에 연결한다. Testharness의 upstream 13/13 Pass는 Spinon 결과와 분리한다.
2. Rust oracle은 root `min-height:300px`, `height:auto` 입력에서 한 implicit row의 sizing stages와 final `[10px]`을 확인한다. 기대값을 Taffy 결과로 역산하지 않는다.
3. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일 scene을 실제 V8→HostDocument→Stylo→typed Grid→Taffy→WGPU 경로로 실행한다. Computed style, Grid input, revision, root/item/child frames, row trace, draw/present가 한 scene revision으로 결합돼야 한다.
4. 정확한 CSS-px frames는 root `[0,0,60,300]`, item `[0,0,60,10]`, child `[0,0,60,10]`; row track은 `[10]`이다. Columns/Rows pass가 각 하나여야 하며 Rows의 `StretchAutoTracks { applied:true }` 전후 base가 모두 `[10]`이어야 한다. Unknown scene ID는 source를 반환하지 않고, 잘못된 root identity·추가/누락 node·변형된 style은 기대된 거부로 처리한다. 누락·중복·stale trace, partial frame, intended fixture source의 거부 및 빈 draw/present는 실패다.
5. Surface screenshot은 보조 시각 자료다. 표본을 추가하면 item 안 10px 영역과 10px 아래의 root 배경을 CSS-to-device 좌표 변환을 기록해 따로 비교한다. Simulator 결과는 실기기/hardware GPU 결과로 확대하지 않는다.
6. 기본/product FFI archive에는 fixture marker·symbol·payload가 없고 fixture-test positive control에는 있어야 한다. `cargo fmt`, 관련 focused Rust/FFI 검증, artifact boundary, Android/iOS simulator build·실행을 기록한다.
7. 구현 전 plans review 20개, 구현/runtime review의 새 실패 관점 20개, 구현 PR diff는 서로 다른 실패 관점 30개로 검토한다. 결함 수정 뒤 영향을 받은 항목을 다시 확인한다.

## 남은 범위

이 slice는 다른 12개 WPT assertion의 전체 runtime coverage, Spinon WPT harness, 일반 CSS admission, CSS zoom, 임의 Grid tree, Android 실기기/hardware GPU, iOS compositor 이후 pixel, C11.4 전체 완료를 주장하지 않는다. `spec/STATUS.md`와 완료 집계는 변경하지 않는다.
