# C11.4 cross-axis fixed-repeat + auto-repeat + areas 사전 비교

## 고정 입력과 비교 기준

- Fixture: `tests/fixtures/css/c11/grid-cross-axis-fixed-auto-repeat-template-areas-c114.html`
- Normative expectation: [`expected-values.json`](./expected-values.json), CSS Grid Layout Module Level 1 2025 CRD §7.1, §7.2.3.1–2, §7.3.2에서 반복 수·area dimension·used track과 child frame을 독립 계산했다.
- Browser oracle: Chromium `156.0.8078.48`. CSSOM resolved track vectors와 `getBoundingClientRect()`를 Agent Browser 0.31.1로 읽고 fixture screenshot 및 브라우저/OS/viewport/device scale을 [`capture-metadata.json`](./capture-metadata.json)에 기록한다.
- 판정: 규범 계산과 Chrome을 항목별 대조한다. 충돌하면 Chrome 결과를 자동 채택하지 않고 CSSOM/레이아웃 규칙을 다시 계산하고 차이를 기록한다. 정수 pixel fixture라 geometry 허용 오차는 1/64 CSS px다. 현재 projection 제약과 범위를 맞춰 fixed-repeat 축의 area dimension은 확장된 fixed track 수와 정확히 같고, area로 추가되는 explicit track은 auto-repeat 축에서만 확인한다.
- 입력 제약: definite root content size, 4px gap, single-track 20px auto-repeat group with no prefix/suffix, fixed-repeat patterns `[20px]`/`[16px 20px]`, zero padding/border, `horizontal-tb`, LTR. Area 확장 track은 12px `grid-auto-*`다.

## 실행 결과

- Agent Browser 0.31.1 / HeadlessChrome `156.0.8078.48`에서 11개 scene을 관측했다. viewport는 `1280×633`, DPR 1이며, resolved `gridTemplateColumns`/`gridTemplateRows`, `gridTemplateAreas`, root box 및 모든 child frame을 기록했다.
- 11/11 scene에서 CRD 독립 계산과 Chrome 관측이 columns, rows, named-area matrix 셀, 모든 child key/frame에서 일치한다. fixed-repeat `[16px 20px]` 2-item pattern은 `[16,20,16,20]` 순서로 확장됐다. auto-repeat 수는 44px 축에서 2회, 76px 축에서 3회이며 기대값의 auto-repeat index 범위와 area-added track 경계도 일치한다. auto-fit의 완전 빈 track과 가운데 빈 track이 0px로 접히고 인접 gutter도 사라졌다. named area 정의 자체는 반복 track을 점유시키지 않았다.
- area가 auto rows/columns list보다 커진 장면은 `grid-auto-* : 12px` explicit track을 추가했고 auto-repeat count는 그대로였다. overflow는 `max(0, child.right - root.right)` 및 `max(0, child.bottom - root.bottom)`으로 산출했다. 기대 초과량은 `fixed-cols-fill-rows-area-adds-two-rows` bottom 24px, `fixed-cols-fill-rows-area-adds-one-row` bottom 16px, `fixed-rows-fill-cols-area-adds-one-column` right 16px다. 이는 frame geometry 증거이며 실제 표면 pixel paint 증거가 아니다.
- scene 1과 `fixed-rows-fill-cols-shorthand-area-short`는 축·root 크기·area·item span을 정확히 전치해 columns/rows 벡터와 x/y frame의 상호 전치를 검사한다. 가운데 auto-fit row 장면은 양쪽 occupied row 사이의 빈 row 및 인접 gutter collapse를 별도로 검사한다.
- capture 중 발견한 비직사각형 named-area 입력을 dot cell로 수정했고, 최종 CSSOM/frame 관측에서 11개 scene 모두 유효한 rectangular area matrix를 반환했다.
- 화면: [`chrome-agent-browser.png`](./chrome-agent-browser.png) · 원본 관측: [`chrome-observation.json`](./chrome-observation.json) · browser evaluator: [`capture-observation.js`](./capture-observation.js) · 고정 입력/관측 자동 대조: [`comparison-validation.json`](./comparison-validation.json) · 재현 명령/환경/hash: [`capture-metadata.json`](./capture-metadata.json)
- 실행 후 이 작업의 Agent Browser session과 local HTTP server를 종료했다. 캡처 종료 뒤 `agent-browser session list --json`은 `sessions: []`였고, 프로세스 확인에서 Agent Browser daemon 및 `HeadlessChrome`은 없었다. 사용자 Google Chrome 프로세스는 별도 실행으로 남아 있어 그대로 두었다.

## 비교 방법

`./validate-comparison.py`가 `expected-values.json`과 `chrome-observation.json`을 scene ID로 조인해 expanded track vectors, root size/auto-repeat axis, exact named-area matrix, fixed-axis area/track dimension, 모든 child key/frame 및 overflow geometry를 비교한다. 반복 수는 `n * 20 + (n - 1) * 4 <= availableSize`로 독립 계산하고, auto-repeat index와 named-area 확장으로 늘어난 explicit track 수를 구별한다. 재실행 명령은 `python3 spec/internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-precomparison-2026-10-25/validate-comparison.py`다. Root/frame geometry 허용 오차는 1/64 CSS px다.

## 한계

Chrome CSSOM은 독립 규범 oracle이 아니다. 이 fixture는 bounded opt-in runtime slice의 기하 경계만 고정하며 WPT, 공개 CSS 지원 또는 전체 CSS Grid 적합성을 증명하지 않는다.
