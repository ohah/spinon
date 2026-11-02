# C11.1 `fr` implicit columns 사전 비교

이 자료는 기본 parsed Root HTML의 sparse column-flow에서 `grid-auto-columns` direct `fr` scalar/list를 추가하기 전에 만든 기준이다. CSS 규범, 독립 산술, Chrome 관측을 구분한다.

## 고정 입력과 관측

- 고정 fixture: [root-html-grid-column-flow-fr-auto-columns-precomparison.html](../../../../tests/fixtures/css/c11/root-html-grid-column-flow-fr-auto-columns-precomparison.html)
- 기준 문서: [CSS Grid Layout Module Level 1, 2025-03-26 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/) §6.6 automatic minimum, §7.6 implicit track pattern, §7.7 sparse column auto-placement, §11.7/§11.7.1 flexible track expansion.
- Browser: Agent Browser가 실행한 `HeadlessChrome/156.0.0.0`, viewport `1280×633`, DPR 1.
- 고정 direct empty child는 10×10 CSS px다. 첫 세 장면은 240px definite content width, explicit columns `[30,40]px`, rows `[20,40]px`, `column-gap:5px`, 12 items를 쓴다. composite는 300px width와 16 items를 쓰며, factor-sum 장면은 120px width와 explicit `[30]px` column, 6 items를 쓴다.
- Raw computed values와 root/item `DOMRect`: [Chrome observation JSON](./chrome156-observation.json). 독립 산술 및 정규화 frame: [expected.json](./expected.json). 화면: [Chrome overview](./chrome156-overview.png).
- SHA-256: fixture `f542209436c3f6fd6ac73dacbc5abc9945c350c745b9380360afddc4edb0134a`; observation `12391b9e31012e08f35af40269c2c41c51a192e1a0f5d05bb98fbf553fc43877`; expected `4d295c8ded384df5bd9a44a298bf83b30ec9988033de643f88a16f0c1aa3d893`; screenshot `a9ad7d414169b3bcca3348a0c9c8681686e8a43d8b75dc85fbf48df4590c3310`.

## 독립 계산과 관찰 차이

`1fr 2fr` list 장면의 definite leftover은 `240 − 30 − 40 − 5×5 = 145px`다. 네 implicit track의 factor 합은 6이므로 ideal used sizes는 `145/6`, `145/3` 반복이다. Chrome은 `24.1562px`와 `48.3438px`를 관측했고, root-relative column starts는 독립 좌표와 최대 `0.0105 CSS px` 차이다. 비교 허용치는 `0.02 CSS px`다. 나머지 scalar, px/fr 혼합, px/%/fr 혼합, sub-unit factor와 zero-factor 장면은 expected 파일의 CSS 산술과 일치한다.

`0fr 1fr` 장면의 첫 track은 0px이 되지 않는다. direct grid items의 고정 width가 10px이고 automatic minimum이 그 base를 보존하므로 첫 track은 10px, 두 번째는 남은 70px다. 이는 zero factor를 단순히 zero breadth로 가정하지 않도록 하는 control이다. `0.25fr` 두 implicit tracks의 factor 합 `0.5`는 §11.7.1의 1 floor를 적용해 각 20px이 되고, 40px은 trailing free space로 남는다.

Chrome은 규범적 기대값의 출처가 아니라 별도 관측자다. 이 파일은 Spinon 구현·모바일 runtime·surface pixel 동등성의 근거가 아니다. 관측 뒤 `agent-browser close --all`을 실행했고 Agent Browser 및 HeadlessChrome 프로세스 조회 결과는 각각 0개였다.
