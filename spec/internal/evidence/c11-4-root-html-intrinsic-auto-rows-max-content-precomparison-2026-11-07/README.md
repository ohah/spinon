# Root HTML `grid-auto-rows:max-content` 사전 비교

## 기준

- Max-content fixture: [`root-html-grid-intrinsic-auto-rows-max-content-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-auto-rows-max-content-precomparison.html)
- Max-content fixture SHA-256: `420ed5e05f9055cd5f8c6c9a031394fc7fdc3ff2d5348e18ed4c35b0f37e5502`
- Min-content comparison fixture SHA-256: `f0759d21c51af46497327686a854b52cd5e004134c35fd562e3e0170c8df49e6`
- Browser: Agent Browser HeadlessChrome 157.0.0.0 on macOS arm64
- Normative source: [CSS Grid Layout Level 2 §7.6](https://www.w3.org/TR/css-grid/#implicit-track-sizing) and [§12.5](https://www.w3.org/TR/css-grid/#algo-track-sizing)

## 독립 기대값과 관측

Grid는 definite 40×40px content box와 explicit 40px column·10px row 하나를 사용한다. 다음 implicit rows에는 각각 10px line-height 기준 두 줄 text item과 고정 34px item이 들어간다. 계산 결과는 explicit `[10]`과 implicit `[20,34]`이며 used rows `[10,20,34]px`다. Item frames는 `first=[0,0,40,10]`, `wrapped-text=[0,10,40,20]`, `last=[0,30,40,34]`다. Grid frame은 `[0,0,40,40]`, visible bottom은 64px다.

Agent Browser에서 `grid-auto-rows:min-content`, `max-content`, `auto`를 각각 적용했다. 세 경우 모두 computed value가 지정값으로 유지됐고 used rows·frames가 위 기대값과 일치했다. Max-content static fixture의 computed track value는 정확히 `max-content`이며 text Range는 두 line rect를 반환했다. 이 고정 horizontal-tb 입력에서는 text의 definite 40px inline size와 line-height 때문에 min/max 결과가 같으므로, geometry만으로 두 track function을 구분하지 않는다. 구현은 cascade value와 style projection의 typed `MaxContent`를 별도 확인해야 한다.

기계 판독 관측과 기대값은 [`chrome-observation.json`](./chrome-observation.json), [`expected-values.json`](./expected-values.json)에 있다. 이 결과는 Chrome oracle 전용이며 Spinon 구현, mobile runtime, glyph pixel 또는 전체 CSS Grid 적합성 근거가 아니다.
