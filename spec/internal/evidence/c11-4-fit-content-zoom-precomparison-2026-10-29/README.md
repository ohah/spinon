# C11.4 zoomed `fit-content()` WPT 사전 비교

## 고정 출처와 Agent Browser 관측

- Pinned WPT revision은 `c999c58338ee1d223df5ad62e48be1202ced0d37`이다. `css/css-grid/grid-definition/fit-content-track-sizing.html`의 SHA-256은 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`다. 원문과 live response hash는 [`source-pins.json`](./source-pins.json)에 고정했다.
- Agent Browser 0.31.1의 HeadlessChrome 156.0.0.0에서 live WPT harness는 `OK`, 13 tests / 13 Pass였다. 대상 `zoomedGrid`는 authored/computed `zoom:2`, `height:300px`, CSSOM `gridTemplateRows:40px`다. CSS Viewport Editor's Draft §4.1의 DOM/CSSOM 규칙은 computed length를 unscaled로 돌려준다. 전체 DOM observation은 [`agent-browser-observation.json`](./chrome/agent-browser-observation.json)에 있다.
- 원문 WPT root의 content width는 viewport 기반 632px이고 post-zoom border box는 1264×600px다. item은 computed 60×40px에서 visual 120×80px, 400px child는 visual 120×800px다.
- 수평축 정규화 probe는 같은 WPT DOM을 복제해 width만 60px로 바꿨다. Original과 normalized probe의 `gridTemplateRows`는 모두 `40px`; 두 경우 item은 visual 120×80px, child는 visual 120×800px다. 이는 제한 runtime fixture에서 inline width를 고정해도 이 block-axis assertion이 유지됨을 보이는 Chrome 관찰이다. 원본과 probe의 전체 DOMRect는 [`normalized-width-probe.json`](./chrome/normalized-width-probe.json)에 있다.
- [Agent Browser WPT 화면](./chrome/wpt-fit-content-track-sizing-13-results.png)은 1280×633 viewport의 upstream harness를 캡처한다. SHA-256 `a7eaaf9fd1d68755272c044c519d277546ec6e070861c0cf663dfa3698c77849`. 화면은 Chrome WPT 기준 자료이며 Spinon 실행 결과가 아니다.

## 독립 기대값

고정 입력은 root `grid-auto-rows:fit-content(40px)`, `height:300px`, `zoom:2`, `min-height:0`인 direct item, 400px block child다. CSS Grid 2025 CRD [§11.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-terms)의 fit-content 공식은 `max(minimum,min(limit,max-content))`다. Unzoomed author values에서 minimum contribution 0px, max-content contribution 400px, limit 40px으로 기준 row는 40px이다. CSS Viewport Editor's Draft [§4](https://drafts.csswg.org/css-viewport/#zoom-property)는 effective zoom이 used length에 반영되고 §4.1은 DOM/CSSOM computed length를 unscaled로 반환한다고 설명한다. 따라서 WPT CSSOM `gridTemplateRows` 기대는 40px이지만, Stylo 0.22의 computed absolute lengths와 Spinon adapter 경로를 통한 Taffy Grid input은 `fit-content(80px)`, max-content 800px, used row 80px이어야 한다. 이는 소스 감사에 따른 예정 runtime 기준이며 아직 Spinon에서 실행하지 않았다. Chrome 관측은 root/item/child visual heights 600/80/800px을 제공하지만 Taffy stage trace를 증명하지 않는다.

### Stylo→Taffy source audit

2026-10-29 기준 `stylo = 0.22.0`이다. Stylo의 `specified/length.rs`는 absolute length를 `context.builder.effective_zoom`으로 확대해 computed CSSPixelLength로 만들며, `ComputedValues::effective_zoom()`을 노출한다. Spinon의 `crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs`는 computed grid track의 `length.px()`를 `ComputedCssGridTrackLengthPercentage::LengthPx`에 저장한다. 이에 따라 새 fixture는 root specified zoom 2와 세 노드의 effective zoom 2를 exact 검사하고, 이미 zoomed된 typed input을 다시 확대하지 않아야 한다. 이 source audit는 아직 cascade regression이나 runtime 검증이 아니다.

독립 입력·기대값과 WPT가 직접 검사하는 항목의 경계는 [`expected.json`](./expected.json)에 있다. Browser observation은 기준 자료이며 normative derivation을 대체하지 않는다.

## 실행 경계

이 폴더는 사전 비교와 Stylo projection source audit 자료다. Spinon 구현, app runtime, Android/iOS Simulator, WPT runner 통과 또는 일반 CSS zoom 지원을 뜻하지 않는다. 구현 전에 계획 기대값을 source audit에 맞춰 보정하고, 계획 검토와 PR diff 검토를 구현 검토와 분리한다. 전용 Agent Browser session은 종료했다.
