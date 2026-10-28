# C11.4 Grid automatic minimum · overflow 분류 사전 비교

**상태:** 계획 단계의 고정 입력, 규범에서 독립적으로 도출한 기대값, Chrome Agent Browser 관측이다. Spinon 변경 코드, 앱 runtime 또는 Android/iOS 실행 결과가 아니다.

## 고정 입력과 환경

- [HTML fixture](../../../../tests/fixtures/css/c11/grid-auto-min-scrollable-overflow-c114-precomparison.html)는 Columns와 Rows 각각에서 `visible`, `clip`, `hidden`, `auto`, `scroll` 및 명시적 최소값 0 대조군을 비교한다. Columns에는 Ahem 10px/10px text, Rows에는 natural 10×10 image를 쓴다.
- Agent Browser `0.31.1`, macOS arm64, `HeadlessChrome/156`, viewport `1280×633` CSS px, DPR 1. `document.fonts.check("10px Ahem")`은 true였고 `document.fonts.status`는 `loaded`였다.
- [관측 JSON](./chrome156-observation.json), [브라우저 metadata](./chrome156-browser-metadata.json), [전체 페이지 화면](./chrome156-agent-browser.png), [규범 기대값](./expected.json)을 보존했다.
- 입력 글꼴 SHA-256: `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`.
- fixture와 결과 파일 SHA-256은 [manifest](./SHA256SUMS.txt)에 기록했다.

## 규범 기준과 독립 기대값

[CSS Grid Layout Module Level 1, 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto)는 Grid item의 computed overflow가 scrollable 값이 아니고, item이 automatic minimum track을 span하는 등 조건을 만족하면 content-based automatic minimum을 사용한다고 규정한다. 이 fixture는 한 개의 `minmax(auto, 1fr)` track만 span하므로, 다른 span 조건은 고정되어 있다. [CSS Overflow Module Level 3, 2025-10-07 WD](https://www.w3.org/TR/2025/WD-css-overflow-3-20251007/#overflow-properties)는 `clip`을 scroll container가 아닌 값으로, `hidden`, `auto`, `scroll`을 scrollable 값으로 구분한다.

Columns fixture의 Ahem 문자열은 한 줄에 10개 glyph, 각 10px이므로 min-content 폭은 100px이다. Grid 폭은 60px이고 track은 `minmax(auto, 1fr)`이다. 따라서 `visible`과 `clip`은 track/item 폭 100px을 유지한다. `hidden`, `auto`, `scroll`의 item automatic minimum은 0이므로 track/item은 available 60px까지 줄어든다. `min-width:0`과 `minmax(0,1fr)` controls도 각각 같은 60px 결과를 만들되 서로 다른 원인을 검증한다.

Rows fixture는 natural 10×10 image를 10px fixed-width column track에 두고 item CSS width도 10px로 고정해 종횡비가 row 축 결과를 바꾸지 않게 한다. 이미지 UA 기본 overflow `clip`을 덮도록 기본 `overflow:visible`도 명시한다. Grid 높이는 5px이고 row는 `minmax(auto,1fr)`이다. `visible`과 `clip`은 content-based minimum으로 row/item 높이 10px을 유지한다. `hidden`, `auto`, `scroll`은 0 automatic minimum으로 track과 stretched item을 5px로 줄인다. `min-height:0` control은 specified minimum 0을 확인한다. `minmax(0,1fr)` control은 고정 track minimum 때문에 item Minimum query가 없는 것을 별도로 확인한다.

| 입력 | 독립 기대값 | Chrome 156 |
| --- | --- | --- |
| Columns `visible`, `clip` | track/item 폭 100px; Grid 폭 60px | 동일 |
| Columns `hidden`, `auto`, `scroll` | track/item 폭 60px; Grid 폭 60px | 동일 |
| Columns item `min-width:0` | track/item 폭 60px | 동일 |
| Rows `visible`, `clip` | track/item `10×10px`; Grid `30×5px` | 동일 |
| Rows `hidden`, `auto`, `scroll` | track `5px`, item `10×5px`; Grid `30×5px` | 동일 |
| Rows item `min-height:0` | track `5px`, item `10×5px` | 동일 |
| Columns/Rows `minmax(0,1fr)` | track `60px`/`5px`; item `60×20px`/`10×5px` | 고정 track minimum이라 item Minimum 질의가 없음 |

Chrome의 `scrollWidth`·`scrollHeight` 값은 fixture의 콘텐츠가 넘친다는 관측일 뿐, Spinon scroll position, clipping, scrollbar 또는 paint 동작 근거가 아니다. screenshot은 HTML 비교 화면만 보여준다.

`minmax(0,1fr)` control은 `min-width/height:0` control과 다르다. zero track minimum은 item automatic minimum을 계산하지 않으므로 Rust trace에서 Minimum query가 없어야 한다. 최종 60px/5px track은 available space 확장 결과다.

## Agent Browser 종료

전용 Agent Browser sessions를 닫았다. 마지막 확인에서 session 목록은 비어 있었고 session info는 `active=false`, `pid=null`이었다. 종료 직후 `agent-browser-darwin-arm64` 프로세스는 남아 있지 않았다. 이 확인은 [종료 기록](./agent-browser-close.json)에 둔다.

## 범위 한계

구현 피드백에 따라 Rows 입력은 text에서 natural replaced image로 바꿨다. overflow automatic minimum으로 row/item이 5px이 되면 현재 `TextLayoutHeightMismatch` guard가 10px text layout을 거부한다. 첫 Rust 실행에서는 image의 aspect ratio가 5px row에 따라 auto width도 5px로 줄여 결과를 혼합했다. CSS width를 10px로 고정해 행 축 minimum만 검증하며 C15 text-layout 동작은 바꾸지 않는다.

이 자료는 단일 Chrome 156 build의 browser observation이다. Spinon layout, WPT 적합성, 일반 CSS parsing/admission, clipping/scroll UI, Android/iOS, 실기기, GPU/compositor 표면을 확인하지 않는다. 구현 후보는 기존 C11.4 `TaffyLayoutEngine` 경로에 한정된 Rust layout tests이며, C13 overflow 사용자 동작을 추가하지 않는다.

## 구현 실행

최종 fixture의 14개 입력은 Chrome computed grid track, root/item frame, 양축 overflow 및 natural image size를 독립 기대값과 대조해 모두 일치했다. 검증 결과와 놓친 실패 경로의 복구 사항은 [구현/runtime 실패 검토 20개](./implementation-review-20.md), 별도 [구현 PR diff 검토 20개](./pr-diff-review-20.md), [계획 실행 기록](../../../../plan/c11-4-overflow-scrollable-auto-minimum.md)에 있다. Rust unit test 결과만 확인했으며 Android/iOS simulator는 실행하지 않았다.
