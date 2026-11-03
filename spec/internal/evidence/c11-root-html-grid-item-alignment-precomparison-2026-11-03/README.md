# C11.1 Root HTML Grid item alignment 사전 비교

## 환경과 입력

- 기준 구현: Agent Browser `0.31.1`의 `HeadlessChrome/156.0.0.0`.
- DPR 1, viewport 1280×633 CSS px.
- 고정 fixture: [`root-html-grid-item-alignment.html`](../../../../tests/fixtures/css/c11/root-html-grid-item-alignment.html).
- W3C primary source: [CSS Box Alignment 3 §4.1](https://www.w3.org/TR/css-align-3/#positional-values), [§4.4](https://www.w3.org/TR/css-align-3/#overflow-values), [§7.1](https://www.w3.org/TR/css-align-3/#justify-items-property), [§7.2](https://www.w3.org/TR/css-align-3/#align-items-property), [CSS Grid 1 · 2025-03-26 CRD §11.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-align).
- Fixture에는 38개의 fixed-px 2×2 Grid와 기존 content distribution과 item alignment를 결합한 Grid 하나가 있다. 총 39 Grid scene·42 direct item이다. 각 Grid container는 84×64px, columns 40px 40px, rows 30px 30px, gap 4px다. 일반 item은 16×10px이며 overflow controls는 한 축에서만 cell보다 크게 만든다.
- 대상 값은 `normal`, `stretch`, `start`, `end`, `flex-start`, `flex-end`, `self-start`, `self-end`, `center`, `safe|unsafe center`, `safe|unsafe end`, justify-items `left|right`, `place-items:center`, `place-items:end start`다.

## 독립 기대값과 관찰

- [`expected-values.json`](expected-values.json)은 40×30px cell과 fixed item의 free space에서 계산한 frame을 담는다. Inline center/end offset은 12/24px, block center/end offset은 10/20px다. 50×40px overflow subject의 safe center/end는 start로, unsafe center/end는 각각 -5/-10px로 이동한다.
- [`chrome-observation.json`](chrome-observation.json)은 Browser user agent, viewport, computed `align-items`·`justify-items`·content alignment, root size, 모든 item frame과 CSS.supports positive controls를 보존한다.
- 독립 기대값과 pinned Chrome 관찰은 39/39 scene에서 모든 frame component가 일치했다. Combined control의 CSS Box Alignment track distribution 뒤 item center도 expected와 일치한다.
- [`chrome/full-matrix.png`](chrome/full-matrix.png)은 Agent Browser 전체-page 캡처다. 이 캡처는 browser layout 관찰이며 Spinon 결과가 아니다.

## SHA-256

```text
fixture f558d61ca1ea3c70d72284909ba8b213cddd4ec3a2450a72c268750c9cfa4122
expected values bcce9e642565f80b83a76a905114a4330030032af5828abb954c17388f06cafb
Chrome observation 59d465ee7926f5e9a4859039089315301718fe3f1283eae6c7feb435bfc9bba4
full matrix screenshot ee5a2a6347a46cfb9862aa615f09cb85de16ff3b2dcb43403e227185d09a7f70
```

## 한계

Chrome supports/readout은 Chrome 156에 한정되며 제품 CSS parser 동작, Spinon runtime, WPT execution/conformance, native platform rendering 또는 pixel equality를 증명하지 않는다. CSS Box Alignment 3의 overflow-position values와 `justify-items:legacy`는 현재 draft에서 at-risk로 표시되어 제품 완료 판정에 사용하지 않는다. Agent Browser 세션을 닫고 browser-launched process가 없는 상태를 확인했다.
