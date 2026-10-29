# C11.4 zoomed `fit-content()` WPT 사전 비교

## 고정 출처와 Agent Browser 관측

- Pinned WPT revision은 `c999c58338ee1d223df5ad62e48be1202ced0d37`이다. `css/css-grid/grid-definition/fit-content-track-sizing.html`의 SHA-256은 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`다. 원문과 live response hash는 [`source-pins.json`](./source-pins.json)에 고정했다.
- Agent Browser 0.31.1의 HeadlessChrome 156.0.0.0에서 live WPT harness는 `OK`, 13 tests / 13 Pass였다. 대상 `zoomedGrid`는 authored/computed `zoom:2`, `height:300px`, used `gridTemplateRows:40px`다. 전체 DOM observation은 [`agent-browser-observation.json`](./chrome/agent-browser-observation.json)에 있다.
- 원문 WPT root의 content width는 viewport 기반 632px이고 post-zoom border box는 1264×600px다. item은 computed 60×40px에서 visual 120×80px, 400px child는 visual 120×800px다.
- 수평축 정규화 probe는 같은 WPT DOM을 복제해 width만 60px로 바꿨다. Original과 normalized probe의 `gridTemplateRows`는 모두 `40px`; 두 경우 item은 visual 120×80px, child는 visual 120×800px다. 이는 제한 runtime fixture에서 inline width를 고정해도 이 block-axis assertion이 유지됨을 보이는 Chrome 관찰이다. 원본과 probe의 전체 DOMRect는 [`normalized-width-probe.json`](./chrome/normalized-width-probe.json)에 있다.
- [Agent Browser WPT 화면](./chrome/wpt-fit-content-track-sizing-13-results.png)은 1280×633 viewport의 upstream harness를 캡처한다. SHA-256 `a7eaaf9fd1d68755272c044c519d277546ec6e070861c0cf663dfa3698c77849`. 화면은 Chrome WPT 기준 자료이며 Spinon 실행 결과가 아니다.

## 독립 기대값

고정 입력은 root `grid-auto-rows:fit-content(40px)`, `height:300px`, `zoom:2`, `min-height:0`인 direct item, 400px block child다. CSS Grid 2025 CRD [§11.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-terms)의 fit-content track formula에서 normalized fixture minimum contribution은 0px, normalized max-content contribution은 400px, argument는 40px이므로 normalized/unzoomed track result는 `max(0, min(40, 400)) = 40px`이다. CSS Viewport Editor's Draft [§4](https://drafts.csswg.org/css-viewport/#zoom-property)는 zoom이 used length를 배율만큼 바꾸며 DOM/CSSOM의 computed style length는 unscaled로 반환한다고 설명한다. WPT assertion은 computed `gridTemplateRows`의 `40px`을 검사한다. Runtime은 normalized track result와 zoomed visual frame을 별도로 대조한다. Agent Browser 관측은 이 고정 입력에서 item/child visual heights 80/800px을 보여준다.

독립 입력·기대값과 WPT가 직접 검사하는 항목의 경계는 [`expected.json`](./expected.json)에 있다. Browser observation은 기준 자료이며 normative derivation을 대체하지 않는다.

## 실행 경계

이 폴더는 사전 비교 자료다. Spinon 구현, app runtime, Android/iOS Simulator, WPT runner 통과 또는 일반 CSS zoom 지원을 뜻하지 않는다. 구현 전에 별도 계획 PR을 병합하며, 계획 검토와 PR diff 검토를 구현 검토와 분리한다. 전용 Agent Browser session은 종료했다.
