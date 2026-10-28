# C11.4 · pinned WPT `fit-content()` implicit column 제한 runtime 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 Grid 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 계획/사전 비교 단계. 구현 PR 전용 runtime scene은 아직 없다. 계획 검토 결과는 [20개 독립 관점](./c11-4-fit-content-implicit-column-plan-review-20.md)에 기록한다. C11.4 checkbox와 완료 수 `72/213`은 그대로 둔다.

**기준:** pinned WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`, source SHA-256 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`. [Chrome·독립 기대값](../spec/internal/evidence/c11-4-fit-content-implicit-column-precomparison-2026-10-29/README.md).

## 결손

`fit-content-track-sizing.html`의 열 축 assertion은 width 300px, explicit `60px` column, `grid-auto-columns:fit-content(40px)`, line 2에 놓인 `min-width:0` item, 400×10px child에서 computed columns `60px 40px`를 요구한다. C11.4에는 이미 `implicit-fit-content-column` runtime scene이 있지만 root 160px, `grid-auto-flow:column`, 두 Ahem text item, 30px cap을 쓴다. 그 장면은 WPT의 definite 300px container, fixed explicit prefix, zero item minimum, child max-content 400px, fit-content cap 40px 조합을 고정하지 않는다.

## 규범 기대값과 고정 입력

Chrome 156 Agent Browser의 upstream WPT harness는 13/13 pass했고, live `wpt.live` source hash는 pinned WPT source와 일치한다. 상세 metadata와 geometry는 [사전 비교 evidence](../spec/internal/evidence/c11-4-fit-content-implicit-column-precomparison-2026-10-29/README.md)에 둔다. Machine-readable 입력과 기대 단계 배열은 해당 evidence의 `expected.json`이며 source metadata는 `source-pins.json`이다. 이는 Chrome oracle이며 Spinon의 WPT 실행이 아니다.

CSS Grid Layout Module Level 1, 2025 CRD [§7.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#implicit-track-sizing), [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content), [§11.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-maximize-tracks), [§11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch)를 적용한다. Positive implicit column의 minimum contribution은 item `min-width:0` 때문에 0px이고, 400px child가 max-content contribution을 제공한다. `fit-content(40px)` 성장 상한은 40px이므로 final column sizes는 `[60,40]px`이다. 남은 200px은 `fit-content()` max sizing function이 `auto`가 아니므로 §11.8에서 column에 stretch되지 않는다. Root columns string은 `60px 40px`, row는 10px이다. Item은 second column에 40px로 놓이고 400px child는 item 바깥 360px 및 root 바깥 160px overflow한다.

고정 scene `fit-content-implicit-column-wpt`는 RootDocument/HostDocument에서 정확히 세 HTML `div`를 사용한다. ID는 `c114-fit-content-implicit-column-grid`, `c114-fit-content-implicit-column-item`, `c114-fit-content-implicit-column-child`다. Root는 `display:grid;width:300px;height:300px;grid-template-columns:60px;grid-auto-columns:fit-content(40px);grid-auto-rows:fit-content(40px);gap:0`이다. Direct item은 `grid-column:2;min-width:0`; child는 `width:400px;height:10px`이다. App-only diagnostic backgrounds는 root `#17212b`, item `#e5484d`, child `#ffd166`으로 두되 sizing과 tree를 바꾸지 않는다. Grid root-relative geometry를 CSS px로 보존한다.

## 구현 범위

1. `RuntimeBlockFlexGridTrackSizingV1`을 일반적으로 넓히지 않고, C11.4 fixture-test opt-in route에만 scene source와 matcher를 추가한다. CSS parser, public API, internal contract version `0.1.0`, production scene admission은 변경하지 않는다.
2. Layout oracle는 computed root/track input, same-revision Columns/Rows trace, explicit/implicit track distinction, item minimum contribution, `MaxContent` contribution, final tracks와 node frames를 검사한다. 기대 final columns `[60,40]px`, rows `[10]px`, item `40×10px`, child `400×10px`다. Track stage arrays는 규범 단계와 existing trace vocabulary를 대조해 별도로 고정한다.
3. FFI scene gate는 unknown scene, 다른 track argument, root width/height, explicit column count, placement, item minimum, child dimension, extra/missing node를 fail-closed하고 partial LayoutOutput을 내지 않아야 한다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 source hash와 revision-bound layout, draw/present를 실행한다. Scene viewport는 최소 480×320 CSS px로 두어 300px root와 400px child overflow가 모두 viewport 안에 보이게 한다. CSS sample centers는 root-relative `(30.5,5.5)` root background, `(80.5,5.5)` child inside item, `(350.5,5.5)` child outside root, `(470.5,5.5)` surface outside child를 구분한다. 예상 색은 순서대로 `#17212b`, `#ffd166`, `#ffd166`, host surface clear다. Simulator 결과는 실기기나 hardware GPU 결과로 확대하지 않는다.
5. Default/product static FFI archive에는 scene ID, marker, symbol, payload가 없어야 한다. Opt-in fixture-test archive는 positive control로 포함해야 한다.

## 수용 기준

- Android/iOS matrix에서 same-revision final tracks, frames, trace, draw/present가 각각 exact assertion을 통과한다. Geometry tolerance는 `0.0001 CSS px`다.
- Row trace는 single implicit row 10px이며, column trace는 fixed explicit 60px과 capped implicit fit-content 40px을 각각 구분한다. `min-width:0` item minimum 0px와 400px max-content contribution이 혼동되지 않아야 한다.
- Negative controls는 잘못된 scene/style/tree, stale revision, malformed or incomplete trace를 거부하고 partial output이나 draw/present를 만들지 않는다.
- Root 바깥 child sample이 Android PixelCopy 및 iOS Metal drawable에서 scene diagnostic color와 일치한다. iOS 표본은 compositor 제출 전이다. Screen capture는 geometry 보조 자료이며 WPT pixel oracle가 아니다.
- Fixture ABI/payload는 opt-in artifact에만 존재한다. 일반 CSS Grid support, WPT runner 통과, C11.4/C11 완료는 주장하지 않는다.
- 구현/runtime은 계획 검토와 다른 새 실패 관점 20개로 검토한다. Implementation PR diff는 30개 서로 다른 관점을 사용하며, 그중 최소 20개를 사용자 요청의 PR 적대 검증으로 충족한다. 발견을 수정하면 해당 관점과 관련 검증을 다시 실행한다.

## 범위 밖

다른 WPT assertions, 일반 `grid-auto-columns` author admission, arbitrary intrinsic child, margins/padding/border, gap, item span, multiple implicit columns, auto-placement matrix, full WPT harness, physical Android device, Android hardware GPU, iOS compositor 이후 pixels와 전체 C11.4는 이 slice에서 판정하지 않는다.
