# C11.4 cyclic percentage track runtime slice — 사전 비교와 계획 근거

## 목적

기존 고정 Chrome 155 C11.4 matrix에는 `indefinite-percent-track` column case가 있고 `spinon-layout`에서 최종 Columns `[12px]`, Rows `[16px]`, Grid `24×16px`, natural image `24×16px`을 비교한다. 아직 같은 `%` track을 CSS parser·style projection·runtime stage trace·Android/iOS surface까지 잇는 scene은 없다. 이 계획은 cyclic percentage track의 column·row 두 축과 definite column control 세 scene만 opt-in fixture로 연결한다.

## 기준과 버전 구분

- 규범 기준은 [CSS Grid Layout Module Level 1 §7.2.1, 2025-03-26 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)이다. 컨테이너 크기가 track 크기에 의존하면 intrinsic container-size 계산 동안 percentage를 `auto`처럼 취급하고, 계산된 inner size로 최종 layout percentage를 푼다.
- 기존 `indefinite-percent-track`의 고정 사전 oracle은 Chrome 155 `155.0.8059.85` 관찰이다: max-content column `50%` → `12px`, Grid `24×16px`, natural image `24×16px`.
- 새 3-scene HTML은 Agent Browser가 실행한 HeadlessChrome `156.0.0.0` (`156.0.8078.48`, DPR 1, viewport `1280×633`) 관찰이다. 이는 Chrome 155 baseline을 교체하거나 pinned reference라고 주장하지 않는다. 원본 값은 [Chrome 156 observation](./chrome156-observation.json)에 보존한다.
- 독립 기대값과 각 axis basis 계산은 [spec-derived expectations](./spec-derived-expectations.json)에 보존한다. 새 row case의 Chrome 156 관찰은 독립 규범 계산과 일치하지만, Chrome 155 실행 근거로 확대하지 않는다.

## 고정 입력과 관찰값

| scene | 입력 | track 기대값 | Grid/item 기대값 |
| --- | --- | --- | --- |
| `cyclic-percent-column` | max-content width, one `50%` column, fixed `24×16px` item | Columns `[12]`, Rows `[16]` | Grid `24×16`, item `24×16`, track 오른쪽으로 12px 초과 |
| `cyclic-percent-row` | width `24px`, max-content height, one `50%` row, fixed `24×16px` item | Columns `[24]`, Rows `[8]` | Grid `24×16`, item `24×16`, row 아래로 8px 초과 |
| `definite-percent-column` | width `40px`, one `50%` column | Columns `[20]`, Rows `[16]` | Grid `40×16`, item `24×16`, track 오른쪽으로 4px 초과 |

HTML fixture는 [`grid-cyclic-percent-tracks-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-cyclic-percent-tracks-c114-precomparison.html)이다. Chrome CSSOM·rect 결과는 fixture의 세 scene과 [observation JSON](./chrome156-observation.json)을 대조한다. Grid border/padding/gap은 모두 0이고, writing mode를 기본 horizontal-tb로 고정한다.

## 구현 판정 기준

1. 기존 `RuntimeBlockFlexGridTrackSizingV1`의 percentage track adapter를 재사용한다. Taffy 내부를 새로 복제하는 계산기는 만들지 않는다.
2. runtime CSS computed track input, used Columns/Rows stage trace, Grid/item frame이 세 scene의 고정 기대값과 각각 일치해야 한다. 계산 오류는 partial output을 게시하지 않는다.
3. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 세 scene을 모두 실행하고 draw/present 및 scene revision을 확인한다. column/row case의 track 경계 양쪽 표면 sample은 item의 `#e5484d` 픽셀인지 확인한다. Simulator 증거를 실기기나 iOS compositor-after 증거로 표현하지 않는다.
4. opt-in fixture-test artifact에는 세 scene ID와 fixture entry point가 포함되고, 기본/product artifact에는 해당 scene ID·전용 payload가 빠지는지 positive/negative control로 확인한다.
5. fixture-only 구현 뒤에도 `spec/STATUS.md`의 C11.4 checkbox와 공식 완료 수는 바꾸지 않는다. WPT 및 public CSS 지원을 주장하지 않는다.

## 범위 밖

`calc()` track, percentage `minmax()`·`fit-content()` limit, multiple percentage tracks, 같은 Grid의 두 cyclic axis 동시 계산, nested Grid, content-box edge·min/max constraints, 다른 writing mode, WPT, physical Android 및 public CSS support는 별도 계획/증거가 필요하다. Chrome 156에서 본 추가 `calc()`/intrinsic function probe 값은 이번 고정 oracle 및 acceptance에 포함하지 않는다.
