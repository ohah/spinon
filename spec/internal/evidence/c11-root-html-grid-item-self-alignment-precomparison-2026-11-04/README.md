# C11.5 Grid item self-alignment 사전 비교

## 목적과 관찰 환경

고정 track Root HTML Grid에서 `align-self`, `justify-self`, `place-self`가 grid item의 cell 내 위치를 어떻게 바꾸는지 고정했다. 기존 PR #1124 계획은 `align-items`와 `justify-items`를 다루고 child의 self-alignment는 계속 거부한다. 이 비교는 다음 제한 slice의 기준이며 Spinon 구현 결과가 아니다.

Agent Browser를 통해 HeadlessChrome 156.0.0.0, DPR 1, viewport 1280×1100에서 로컬 고정 HTML을 열었다. 17개 유효 정렬 scene과 문법상 invalid인 `align-self:left`·`justify-self:top` declaration control 두 개를 관찰했다. fixture item은 자식 text node가 없는 빈 `div`이고 `overflow:visible`이며 custom font를 사용하지 않는다. metadata에는 item 19개의 child node 수가 모두 0임을 기록했다. 각 장면은 40×30 CSS px grid area와 명시 크기 item, zero margin/padding/border를 쓴다. Overflow 장면은 item 50×40 CSS px다.

- [Agent Browser 관찰 및 runtime 공용 fixture](../../../../tests/fixtures/css/c11/root-html-grid-item-self-alignment.html)
- [Chrome DOM 관측 JSON](./chrome-observation.json)
- [Chrome/viewport metadata](./browser-metadata.json)
- [Agent Browser 화면](./chrome-agent-browser.png)
- [독립 기대값](./expected.json)
- [재실행 가능한 19-scene 비교기](./verify-precomparison.mjs) 및 [결과](./comparison-verification.json)
- [SHA-256 manifest](./SHA256SUMS.txt)

## 규범 근거

- [CSS Box Alignment Module Level 3, 2026-10-08 Working Draft](https://www.w3.org/TR/2026/WD-css-align-3-20261008/), §§4.1, 4.4, 6.1.5, 6.2.5, 6.3: `justify-self`는 grid item의 inline 축, `align-self`는 block 축에서 margin box를 grid area 안에 배치한다. `place-self`의 첫 값은 `align-self`, 두 번째는 `justify-self`에 대응하고 생략 시 첫 값을 복사한다. `auto`는 부모의 대응 `*-items` computed value를 사용한다. `safe`는 overflow 시 start fallback을 선택하고, `unsafe`는 지정한 정렬을 유지한다. 해당 Working Draft는 `<overflow-position>`을 at-risk로 표기하므로, 본 slice에서의 support는 고정 profile 관찰 범위로만 기록한다.
- [CSS Grid Layout Module Level 1, 2025-03-26 Candidate Recommendation Draft](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/), §§10.3–10.4: grid item의 self-alignment는 그 item의 grid area 안에서 동작한다.

## 독립 계산

Horizontal-tb, LTR, zero edges에서 16×10 item과 40×30 area의 free space는 24×20이다. 따라서 start/center/end 좌표는 inline `x={0,12,24}` 및 block `y={0,10,20}`이다. `self-start`/`self-end`는 이 입력에서 각각 같은 start/end edge다. `flex-start`/`flex-end`는 non-flex Grid context에서 start/end처럼 동작한다. `left`/`right`는 `justify-self`에서 물리 inline start/end로 확인한다.

Overflow 대조에서 50×40 item은 area보다 각 축에서 10px 크다. Safe center/end는 start `(0,0)`으로 물러난다. Unsafe center는 `(-5,-5)`, unsafe end는 `(-10,-10)`이다. `place-self:unsafe center safe end`의 computed longhand 순서는 align=`unsafe center`, justify=`safe end`라서 item 좌표는 `(0,-5)`이며, 반대 순서 `place-self:safe end unsafe center`는 `(-5,0)`이다.

17개 유효 scene의 computed self values, parent defaults, item frame과 두 invalid declaration의 `auto` cascade fallback이 Chrome DOM 관측 및 독립 기대값과 총 19/19 exact 일치했다. 상세 행렬은 [expected.json](./expected.json)과 [chrome-observation.json](./chrome-observation.json)을 대조한다.

## 한계

이 입력은 fixed size, fixed single cell, zero edges의 parsed Root HTML Grid 관찰뿐이다. 일반 CSS 지원, auto/stretch sizing, baseline groups, auto margins, intrinsic track sizing, WPT 적합성, Android/iOS runtime, surface pixels 및 실제 glyph rasterization은 검증하지 않았다. Safe/unsafe 관찰은 at-risk Working Draft의 해당 keyword를 Chrome이 구현한 결과다.
