# C11.4 Root HTML `grid-auto-rows: min-content` bounded slice

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [parsed Root HTML intrinsic explicit tracks](./c11-root-html-intrinsic-grid-tracks.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**사전 비교:** [Chrome 156 Agent Browser 결과](../spec/internal/evidence/c11-root-html-intrinsic-auto-rows-precomparison-2026-11-06/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-intrinsic-auto-rows-precomparison.html) · [독립 기대값](../spec/internal/evidence/c11-root-html-intrinsic-auto-rows-precomparison-2026-11-06/expected.json) · [browser observation](../spec/internal/evidence/c11-root-html-intrinsic-auto-rows-precomparison-2026-11-06/chrome156-observation.json)

**계획 실패 관점 검토:** [20개](./c11-root-html-intrinsic-auto-rows-plan-review-20.md). **계획 PR 변경 검토:** PR을 만들기 전 실제 diff에 대해 기록한다. 구현/runtime 검토 20개와 구현 PR 변경 검토 30개는 이 계획 검토와 분리한다.

## 결손과 목표

기본 parsed Root HTML 경로는 explicit `grid-template-*`의 intrinsic track sizing을 `RootHtmlIntrinsicTrackSizingV1`에 연결한다. 이 경로는 현재 `grid-auto-rows:auto`만 허용한다. `grid-auto-rows:min-content`는 유효한 CSS typed track이지만 adapter에서 거부되어 implicit row의 min-content contribution을 Root HTML scene까지 전달하지 못한다.

이번 slice는 같은 기본 `RuntimeRootHtmlTextPaintV1` 경로에서 반복되는 단일 `grid-auto-rows:min-content` track을 허용한다. CSS parser, 새 runtime profile, FFI/scene selector, public API 또는 test-only success path는 추가하지 않는다.

## 지원 경계

- Grid는 기본 parsed Root HTML runtime route의 visible direct `<div>` 자식만 사용한다. Grid owner 하나, `grid-auto-flow:row`, LTR `horizontal-tb`, `overflow:visible`이다.
- Explicit columns와 rows는 각각 유한한 양의 fixed CSS px track 하나다. Explicit row는 첫 item의 auto-placement를 고정한다.
- 각 item의 네 `grid-row-*`·`grid-column-*` placement longhand는 모두 `auto`여야 한다. 명시·부분·named·span 배치는 범위 밖이다.
- 새 intrinsic-auto-row 후보는 `grid-auto-rows:min-content` 단일 track 값만 허용해 모든 implicit row에서 반복한다. Runtime geometry matrix는 implicit row 1개와 2개를 고정하며, larger input은 기존 10,000-track ceiling과 checked counts를 따른다.
- Container는 definite positive fixed CSS-pixel width/height를 사용한다. Min/max, aspect-ratio, margin/padding/border, named area, named line, repeat, dense/column flow와 implicit column은 범위 밖이다.
- Grid gaps는 0이다. Content/item alignment는 현재 intrinsic explicit-track route의 `normal` 또는 `start` subset을 유지한다.
- Visible direct item은 빈 HTML `div`이며 width/height는 finite non-negative fixed CSS px다. Margin/padding/border가 없고 min/max auto, `aspect-ratio` unset, position static이다.
- 이 새 후보 path에서는 `max-content`, `fit-content()`, `minmax()`, `fr`, percentage, math, 음수 또는 다중 track list를 받지 않는다. 기존 explicit-intrinsic route가 이미 허용하는 `grid-auto-rows:auto`는 바꾸지 않는다. CSS-valid unsupported 값은 `grid-auto-rows`·node가 식별되는 진단으로 fail-closed하고 partial frame/paint를 만들지 않는다. CSS-invalid declaration은 Stylo cascade fallback을 따른다.

## 고정 기준과 independent expected

CSS Grid Level 1 §11.5의 implicit track sizing과 §11.8의 min-content track sizing을 따른다. 사전 비교는 Chrome 156에서 container 40×40px, explicit row 10px, implicit `min-content` rows, zero gap인 단일 Grid를 관찰했다. Direct item들의 fixed block-size는 각각 10px, 22px, 34px다. Fixed block-size가 각 item의 min-content contribution이므로 독립 기대 row sizes는 `[10, 22, 34]px`; item origins는 `(0,0)`, `(0,10)`, `(0,32)`px다. 마지막 item은 Grid의 visible bottom 40px를 지나 y=66px까지 26px overflow한다. Fixture의 색상은 `background-color`로 지정한다. 같은 색을 쓰는 `background` shorthand는 현재 default Root HTML CSS profile에서 `background-position-x`까지 확장되어 거부됐고, `border:0` shorthand는 `border-top-color`까지 확장되어 거부됐다. Fixture는 이를 기본 0-width border와 longhand background color로 표현해 product route 안에서 색상과 geometry를 보존한다. 독립 기대값과 Chrome 관측 JSON을 각각 보존한다. Chrome은 비교 oracle이며 WPT나 Spinon 결과가 아니다.

## 구현 요구

1. Computed-style route selection은 explicit intrinsic track이 없더라도 `grid-auto-rows:min-content`가 있는 유효한 candidate owner를 같은 `RootHtmlIntrinsicTrackSizingV1` mode로 분류한다. 기존 direct-text aspect-ratio mode 및 다른 Grid candidate mode의 우선순위를 바꾸지 않는다.
2. Cascade→typed projection은 기존 `ComputedCssGridAutoRows` 값으로 단일 `MinContent` track을 표현한다. CSSOM 문자열 재파싱, node id/class/child index/scene name 분기는 추가하지 않는다.
3. Layout adapter는 intrinsic contract에서 `GridAutoRows::TrackList([MinContent])`만 새로 허용한다. 현재 explicit template track validation, definite axes, zero-gap, owner/topology 및 output identity 조건을 그대로 적용한다.
4. Implicit row preflight는 content-derived intrinsic extent를 fixed estimate로 가장하지 않는다. Taffy track trace와 최종 frame 수집에서 각 implicit min-content track의 finite non-negative base, row line, DOM paint order 및 overflow를 검사한다. 오류가 나면 frame/scene publication 전체를 거부한다.
5. Grid 10,000-track ceiling과 checked counts를 유지한다. `required_rows` 10,001, track/value resource limits, unsupported auto-row patterns, stale revision, wrong owner/property와 candidate scene 혼합은 partial output 없이 거부한다.

## 검증

- Browser: 이 계획의 고정 HTML fixture에서 Chrome 156 computed rows·grid/item frames·visible overflow를 독립 expected와 비교한다. Browser observation, expected arithmetic, screenshot hash를 구분한다.
- Rust layout: explicit row + 1–2 implicit min-content rows의 track trace와 base sizes `[10,22,34]`, item frames 및 26px overflow를 검사한다. Adjacent item source order와 implicit row cap을 대조한다.
- Style projection: computed auto-row token이 단일 `MinContent`로 전달되는 positive test 및 `max-content`, multi-track, math/percentage, missing owner, CSS-invalid fallback/typed invalid 경계를 검사한다.
- Runtime: 같은 fixture hash를 기본 parsed Root HTML route에서 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator로 실행한다. `layout=ready`, exact expected frames, draw/present, root 밖 y=40..66의 visible item을 기록한다. `adb screencap`/`simctl screenshot`의 색상 run을 DPR에 맞춰 표본화하고, OS screenshot이 PixelCopy/Metal drawable readback을 대체하지 않는 한계를 함께 적는다. User-visible simulator screenshots는 PR 본문에 첨부할 수 있도록 보존한다.
- `cargo fmt --all -- --check`, focused Rust tests, 변경 crate scoped Clippy, fixture boundary/default artifact checks 및 simulator builds를 기록한다. 전체 suite의 알려진 실패는 기준 revision과 나란히 두며 green으로 일반화하지 않는다.
- 새로운 구현/runtime 실패 관점 20개와 실제 implementation PR diff 적대 검토 30개를 plan review와 별도로 기록한다.

## 제외와 완료 판정

Text, replaced item, aspect-ratio contribution, auto-sized container axis, intrinsic `grid-auto-columns`, 다른 implicit row track function, named areas, auto-repeat, line names, dense/column flow, nonzero gap, arbitrary CSS admission, WPT conformance 및 전체 C11.4/C11 지원은 포함하지 않는다. PR merge 후에도 C11.4/C11 checkboxes와 공식 roadmap count `78/214`는 유지한다. 새 behavior가 선택되지 않았거나 unsupported input이 CSS parser/cascade error로 위장된 경우 이 slice는 완료가 아니다.

작업은 현재 checkout에서 순차적으로 한다. worktree를 만들지 않는다. plan PR과 implementation PR을 따로 검토·병합하고, merge SHA 기준으로 STATUS/internal contract/Tailnet preview를 동기화한다. 사용한 Agent Browser session과 process는 확인 후 종료한다.
