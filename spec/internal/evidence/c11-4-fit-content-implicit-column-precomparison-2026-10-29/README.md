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

WPT의 auto row는 10px child로 10px에 sizing된다. Item은 40px track에 맞고 400px child는 가로로 overflow한다. Machine-readable expected values와 base-size stage arrays는 `expected.json`에 있다.

Stage oracle도 규범 계산에서 따로 고정했다. Columns base sizes는 `Initialize`, `IntrinsicItemBatch`, `IntrinsicSizingComplete`, `MaximizeTracks`, `ExpandFlexibleTracks`, `StretchAutoTracks` 뒤에 각각 `[60,0]`, `[60,0]`, `[60,0]`, `[60,40]`, `[60,40]`, `[60,40]px`이다. Rows는 `[0]`, `[0]`, `[10]`, `[10]`, `[10]`, `[10]px`다. Fixed 60px track의 base는 변하지 않는다. Item minimum 0px이라 implicit column base가 intrinsic stage에서 커지지 않고, fit-content max-content contribution cap 40px을 `MaximizeTracks`에서 받는다. Row는 10px content-based minimum으로 intrinsic stage에서 base가 10px이 된다. Unused free space는 두 축에서 fit-content track을 stretch하지 않는다.

## 적용 범위와 한계

현재 C11.4 fixture의 `implicit-fit-content-column`도 implicit `fit-content()` column을 다루지만, 160px root·`grid-auto-flow:column`·두 Ahem text item·30px cap을 사용한다. 이 사전 비교의 WPT 입력인 definite 300px root, explicit 60px column, `grid-column:2`, `min-width:0`, 400px child를 조합하지 않는다. 이 사전 비교는 WPT assertion을 Chrome으로 실행한 결과이며 Spinon 구현·WPT runner·C11.4 전체 완료를 증명하지 않는다.
