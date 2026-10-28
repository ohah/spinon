# C11.4 Grid automatic minimum · overflow 분류 사전 비교

**상태:** 계획 단계의 고정 입력, 규범에서 독립적으로 도출한 기대값, Chrome Agent Browser 관측이다. Spinon 변경 코드, 앱 runtime 또는 Android/iOS 실행 결과가 아니다.

## 고정 입력과 환경

- [HTML fixture](../../../../tests/fixtures/css/c11/grid-auto-min-scrollable-overflow-c114-precomparison.html)는 Columns와 Rows 각각에서 `visible`, `clip`, `hidden`, `auto`, `scroll` 및 명시적 최소값 0 대조군을 비교한다. `Ahem` 10px/10px 글꼴을 사용한다.
- Agent Browser `0.31.1`, macOS arm64, `HeadlessChrome/156`, viewport `1280×633` CSS px, DPR 1. `document.fonts.check("10px Ahem")`은 true였고 `document.fonts.status`는 `loaded`였다.
- [관측 JSON](./chrome156-observation.json), [브라우저 metadata](./chrome156-browser-metadata.json), [전체 페이지 화면](./chrome156-agent-browser.png), [규범 기대값](./expected.json)을 보존했다.
- 입력 글꼴 SHA-256: `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`.
- fixture와 결과 파일 SHA-256은 [manifest](./SHA256SUMS.txt)에 기록했다.

## 규범 기준과 독립 기대값

[CSS Grid Layout Module Level 1, 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto)는 Grid item의 computed overflow가 scrollable 값이 아니고, item이 automatic minimum track을 span하는 등 조건을 만족하면 content-based automatic minimum을 사용한다고 규정한다. 이 fixture는 한 개의 `minmax(auto, 1fr)` track만 span하므로, 다른 span 조건은 고정되어 있다. [CSS Overflow Module Level 3, 2025-10-07 WD](https://www.w3.org/TR/2025/WD-css-overflow-3-20251007/#overflow-properties)는 `clip`을 scroll container가 아닌 값으로, `hidden`, `auto`, `scroll`을 scrollable 값으로 구분한다.

Columns fixture의 Ahem 문자열은 한 줄에 10개 glyph, 각 10px이므로 min-content 폭은 100px이다. Grid 폭은 60px이고 track은 `minmax(auto, 1fr)`이다. 따라서 `visible`과 `clip`은 track/item 폭 100px을 유지한다. `hidden`, `auto`, `scroll`의 item automatic minimum은 0이므로 track/item은 available 60px까지 줄어든다. `min-width:0`과 `minmax(0,1fr)` controls도 각각 같은 60px 결과를 만들되 서로 다른 원인을 검증한다.

Rows fixture는 Ahem 한 glyph의 line-height가 10px인 반면 Grid 높이는 5px이다. `visible`과 `clip`의 content-based minimum은 row/item을 10px로 유지하고, `hidden`, `auto`, `scroll`은 0 automatic minimum으로 5px에 맞춘다. 명시적 `min-height:0`과 `minmax(0,1fr)` controls는 각각 item minimum과 track minimum의 별도 대조군이다.

| 입력 | 독립 기대값 | Chrome 156 |
| --- | --- | --- |
| Columns `visible`, `clip` | track/item 폭 100px; Grid 폭 60px | 동일 |
| Columns `hidden`, `auto`, `scroll` | track/item 폭 60px; Grid 폭 60px | 동일 |
| Columns item `min-width:0`, track `minmax(0,1fr)` | track/item 폭 60px | 동일 |
| Rows `visible`, `clip` | track/item 높이 10px; Grid 높이 5px | 동일 |
| Rows `hidden`, `auto`, `scroll` | track/item 높이 5px; Grid 높이 5px | 동일 |
| Rows item `min-height:0`, track `minmax(0,1fr)` | track/item 높이 5px | 동일 |

Chrome의 `scrollWidth`·`scrollHeight` 값은 fixture의 콘텐츠가 넘친다는 관측일 뿐, Spinon scroll position, clipping, scrollbar 또는 paint 동작 근거가 아니다. screenshot은 HTML 비교 화면만 보여준다.

## Agent Browser 종료

전용 Agent Browser sessions를 닫았다. 마지막 확인에서 session 목록은 비어 있었고 session info는 `active=false`, `pid=null`이었다. 종료 직후 `agent-browser-darwin-arm64` 프로세스는 남아 있지 않았다. 이 확인은 [종료 기록](./agent-browser-close.json)에 둔다.

## 범위 한계

이 자료는 단일 Chrome 156 build의 browser observation이다. Spinon layout, WPT 적합성, 일반 CSS parsing/admission, clipping/scroll UI, Android/iOS, 실기기, GPU/compositor 표면을 확인하지 않는다. 구현 후보는 기존 C11.4 `TaffyLayoutEngine` 경로에 한정된 Rust layout tests이며, C13 overflow 사용자 동작을 추가하지 않는다.
