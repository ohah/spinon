# C11.4 pinned WPT fit-content implicit column 사전 비교

## 출처와 기준

- WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`의 [`fit-content-track-sizing.html`](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L64-L66)은 300px Grid, 고정 60px explicit column, `fit-content(40px)` implicit column, `grid-column:2; min-width:0`, 400×10px child를 사용한다. [WPT assertion](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L122-L124)은 resolved columns가 `60px 40px`인지 검사한다.
- Pinned 파일과 `wpt.live` 응답은 SHA-256 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`로 일치했다. 원본 크기는 5,060 bytes다. 기계 판독 source pin은 `source-pins.json`에 있다.
- Agent Browser 0.31.1 / HeadlessChrome 156.0.0.0, viewport 1280×633 CSS px, DPR 1에서 upstream testharness는 13 tests / 13 Pass였다. 이는 Chrome의 WPT 실행이며 Spinon WPT 결과가 아니다. 관측 metadata는 `chrome156-observation.json`에 있다.

## 관측값

`#implicitColumn`의 computed values는 `grid-template-columns: 60px 40px`, `grid-template-rows: 10px`, `grid-auto-columns: fit-content(40px)`이다. Root는 300×300px, item은 root-relative x=60px에서 40×10px, child는 같은 위치에서 400×10px이다. Item의 computed `min-width`는 0px, `grid-column`은 `2`다. Child는 item 바깥으로 360px, Grid root 바깥으로 160px 넘는다. 화면 캡처의 색상·outline은 harness 및 geometry observation 이후 넣은 시각화 overlay이며 sizing 입력을 바꾸지 않았다.

Chrome 화면 캡처는 `chrome156-fit-content-implicit-column.png`에 있다. 캡처의 색상·outline은 harness 및 geometry observation 이후 넣은 시각화 overlay다.

비교 산출물 4개의 SHA-256은 `SHA256SUMS.txt`에 기록했다.

## 독립 규범 기대값

CSS Grid Layout Module Level 1, 2025 CRD [§7.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#implicit-track-sizing)에서 양의 implicit column은 `grid-auto-columns`로 sizing된다. [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)은 `fit-content(limit)`을 auto minimum과 max-content 사이에서 limit으로 제한한다. [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content)에서 `min-width:0`인 item의 minimum contribution은 0px이고, 400px child는 400px max-content contribution을 준다. §11.5는 fit-content track의 max-content growth limit을 인자로 clamp하므로 implicit column은 40px이다. [§11.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-maximize-tracks)의 maximize 단계에서 두 번째 track은 40px limit에 도달한다. Fit-content의 max sizing function은 auto가 아니어서 300px root의 남은 200px을 받지 않는다. 최종 columns는 `[60,40]px`이다. 이 계산은 Chrome 관측값을 역산하지 않았다.

WPT의 auto row는 10px child로 10px에 sizing된다. Item은 40px track에 맞고 400px child는 가로로 overflow한다. 독립적으로 기대한 computed track list와 geometry는 `expected.json`의 `expected`에 있다. WPT는 내부 sizing stage를 관찰하지 않으므로 `stageExpectations`는 구현 뒤 Taffy trace에서 기록한 내부 회귀 기준이며 CSS 규범 또는 Chrome oracle로 취급하지 않는다.

내부 trace의 회귀 기준은 Columns base sizes `[60,0]`, `[60,0]`, `[60,0]`, `[60,40]`, `[60,40]`, `[60,40]px`; Rows `[0]`, `[10]`, `[10]`, `[10]`, `[10]`, `[10]px`이다. Fixed 60px track의 base는 변하지 않는다. Trace는 row의 10px content contribution을 `IntrinsicItemBatch`에서 반영한다. Item minimum 0px, 400px max-content contribution과 최종 40px implicit column은 별도 Columns contribution/base matcher로 확인한다. 이 단계 배열은 현재 Taffy trace vocabulary를 고정할 뿐 규범 stage별 상태를 판정하지 않는다.

Taffy trace의 row contribution 회귀 기준은 `Minimum=10px` (`AutomaticMinimum`), `MinContent=10px`, `MaxContent=10px`다. CSS [§6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto)와 10px child가 이 cross-axis expectation을 뒷받침한다. `StretchAutoTracks { applied }`는 축 정렬이 해당 단계를 활성화했는지를 나타내는 구현 진단이다. Columns는 `false`, Rows는 `true`를 기록한다. Rows의 `true`는 track base가 늘었다는 뜻이 아니다. `align-content:normal`이 row 축 stretch 단계를 활성화해도 `fit-content(40px)`의 max sizing function은 `auto`가 아니므로 final row는 10px 그대로이고 남은 공간을 받지 않는다. row contribution 객체와 이 플래그도 Taffy-specific regression expectation이며 WPT/CRD에서 직접 관찰한 trace는 아니다.

## 적용 범위와 한계

현재 C11.4 fixture의 `implicit-fit-content-column`도 implicit `fit-content()` column을 다루지만, 160px root·`grid-auto-flow:column`·두 Ahem text item·30px cap을 사용한다. 이 사전 비교의 WPT 입력인 definite 300px root, explicit 60px column, `grid-column:2`, `min-width:0`, 400px child를 조합하지 않는다. 이 사전 비교는 WPT assertion을 Chrome으로 실행한 결과이며 Spinon 구현·WPT runner·C11.4 전체 완료를 증명하지 않는다.
