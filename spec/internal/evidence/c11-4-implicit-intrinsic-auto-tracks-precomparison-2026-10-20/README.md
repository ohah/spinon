# C11.4 positive implicit track sizing 사전 비교

## 범위

이 자료는 C11.4 `RuntimeBlockFlexGridTrackSizingV1`에서 양의 implicit row/column에 적용될 `grid-auto-rows`·`grid-auto-columns` track-size와 반복 순서를 고정한다. Spinon 구현 결과가 아니라 Chrome 155의 독립 관찰과 CSS Grid 규범에서 계산한 기대값이다.

## 규범 입력

[CSS Grid Level 1 2025 CRD §7.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-tracks)는 두 속성을 `<track-size>+`로 정의한다. 여러 값을 주면 implicit track 수에 맞게 순서를 반복하며, explicit grid 뒤 첫 implicit track에는 목록의 첫 값부터 적용한다. §7.5와 §8.5는 명시 배치 범위 밖 또는 auto-placement로 implicit track이 생길 수 있음을 규정한다. Agent Browser가 연 공식 문서의 해당 절 텍스트는 [원문 capture](css-grid-1-2025-crd-section-7-6.txt)에 보존했다.

이 계획은 기존 C11.4 TrackSizing projection이 표현하는 track-size 목록을 양의 implicit track에 연결한다. explicit grid 뒤쪽에서 track list를 authored order로 순환한다. CSS 규범의 negative implicit track 역방향 목록 배정은 현재 placement adapter가 negative implicit track을 표현하지 않으므로 구현 범위 밖에 두고 fail-closed로 유지한다.

## Chrome 155 관찰 환경

| 항목 | 값 |
| --- | --- |
| Browser | HeadlessChrome 155.0.8059.85, macOS arm64 |
| Agent Browser | 0.31.1 |
| OS | macOS 26.5.1 |
| viewport / DPR | 1280 × 633 CSS px / 1 |
| font | Ahem, 로드 확인, SHA-256은 `browser-environment.json` |
| 입력 | `tests/fixtures/css/c11/grid-auto-intrinsic-implicit-tracks-c114.html` |

실제 Chrome 관찰 JSON은 [chrome155-observation.json](chrome155-observation.json), 화면은 [chrome155-agent-browser.png](chrome155-agent-browser.png), 독립 규범 계산은 [spec-derived-expectations.json](spec-derived-expectations.json)에 저장했다. Chrome과 규범 기대값은 모두 20개 case에서 일치했다. 입력·관찰·화면·폰트·규범 절 capture의 SHA-256은 [SHA256SUMS](SHA256SUMS)에 저장했다. 이 결과는 Spinon, Android/iOS 앱, WPT 적합성의 증거가 아니다.

## 고정 결과

| Case | 입력과 기대 track list | Chrome 155 관찰 |
| --- | --- | --- |
| `row-min-content` | 40px explicit column, 12px explicit row, `grid-auto-rows:min-content` → rows `12px 20px` | 일치 |
| `row-max-content` | 같은 text item, `grid-auto-rows:max-content` → rows `12px 20px` | 일치 |
| `column-min-content` | `grid-auto-columns:min-content` → columns `24px 70px` | 일치 |
| `column-max-content` | `grid-auto-columns:max-content` → columns `24px 100px` | 일치 |
| `row-auto-placement-min-content` | row flow로 생성한 row → `12px 20px` | 일치 |
| `column-auto-placement-max-content` | column flow로 생성한 column → `24px 100px` | 일치 |
| `row-auto-placement-auto` | 기본 `auto` row → `12px 20px` | 일치 |
| `column-auto-placement-auto` | 기본 `auto` column → `24px 100px` | 일치 |
| `two-implicit-columns-min-content` | 서로 다른 두 item → columns `24px 20px 40px` | 일치 |
| `cycled-intrinsic-columns` | `[min-content,max-content]` × 3 → `24px 20px 100px 40px` | 일치 |
| `cycled-fixed-max-rows` | `[10px,max-content]` × 3 → `12px 10px 20px 10px` | 일치 |
| `implicit-percentage-column` | 160px basis의 25% → `24px 40px` | 일치 |
| `implicit-fraction-column` | 160px basis에서 24px explicit 다음의 1fr → `24px 136px` | 일치 |
| `implicit-minmax-column` | `minmax(20px,40px)` → `24px 40px` | 일치 |
| `implicit-fit-content-column` | `fit-content(30px)`, 70px min-content floor → `24px 70px` | 일치 |
| `implicit-min-content-overflow-border` | 44px border-box 밖의 70px implicit item → tracks `20px 70px`; item은 border 오른쪽으로 48px 넘침 | 일치 |
| `implicit-calc-column` | `calc(10% + 5px)` at 160px → `24px 21px` | 일치 |
| `explicit-track-control` | implicit track 없음 → columns `40px`, rows `24px` 유지 | 일치 |
| `row-positive-implicit-locked-item` | row 2 / 3만 고정하고 column auto-placement → rows `12px 20px` | 일치 |
| `column-positive-implicit-locked-item` | column 2 / 3만 고정하고 row auto-placement → columns `24px 70px` | 일치 |

정수 track/frame은 정확 비교한다. 분수 CSS px이 생기는 확장 case는 계획에서 별도 1/64 CSS px 허용치를 고정한다. overflow case의 관찰 root는 44×44 border-box이고, implicit item의 right edge는 root right edge보다 48 CSS px 바깥이다. 표면 색·border stroke pixel은 이 Chrome screenshot에서 측정하지 않았다.

## 해석 한계

- Chrome은 구현 비교 oracle이며 CSS 규범을 대체하지 않는다.
- 이 fixture는 양의 implicit tracks, horizontal-tb/LTR, fixed Ahem text 및 고정 typed track values만 관찰한다.
- negative implicit track, out-of-flow item, line names/areas, WPT, Spinon layout, native surface, 실기기와 hardware GPU는 검사하지 않았다.
- implementation PR은 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기존 53개 scene과 20개 implicit-track scene을 모두 실행하고, border 안·밖의 native surface 표본 및 trace·draw/present를 보존해야 한다.
