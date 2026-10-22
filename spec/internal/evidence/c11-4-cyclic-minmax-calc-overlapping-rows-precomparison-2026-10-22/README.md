# C11.4 cyclic `minmax(calc())` 겹치는 row span 사전 비교

이 자료는 cyclic percentage minimum을 쓰는 세 explicit row track에서 두 span-2 item과 span-3 item이 겹칠 때 planned increase, source-order invariance, 세로 text overflow를 고정한다. 계획과 Chrome 관측이며 Spinon runtime 성공을 뜻하지 않는다.

## 입력과 환경

- HTML fixture: [`grid-cyclic-minmax-calc-overlapping-rows-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-cyclic-minmax-calc-overlapping-rows-c114-precomparison.html)
- Font: 저장소 Ahem `10px/10px`; observation에서 `document.fonts.status=loaded`, `document.fonts.check("10px Ahem")=true`.
- 브라우저: Agent Browser `0.31.1`, `HeadlessChrome/156.0.0.0`, viewport `1280×633` CSS px, DPR 1.
- 기준: [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§11.3–§11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing).
- 독립 기대값: [spec-derived-expectations.json](./spec-derived-expectations.json). Ahem line-height와 Grid span 분배에서 계산했으며 Chrome 관측을 기대값의 근거로 삼지 않는다.
- Chrome 관측: [chrome156-observation.json](./chrome156-observation.json). 화면: [chrome-agent-browser-pass01.png](./chrome-agent-browser-pass01.png).
- Fixture, font, expectation, observation, screenshot SHA-256: [SHA256SUMS.txt](./SHA256SUMS.txt).
- Agent Browser 전용 session `spinon-c114-overlap-rows-oracle-20261022`를 사용했다. 확인 후 `agent-browser close --all`과 session list를 실행해 활성 session이 없는 것을 확인했다. 전용 Agent Browser PID `31413`과 Chrome PID `31414`도 종료됐다. 기존 사용자 Chrome PID `20311` 및 Tailnet preview PID `1191`는 살아 있어 보존했다. terminal-browser는 사용하지 않았다.

## 독립 계산

세 track은 `minmax(calc(10% + 2px), max-content)`, row gap은 2px이다. grid의 inline size를 10px로 고정하고 10px Ahem glyph에 `word-break:break-all`을 적용해 각 글자가 10px line 하나를 만든다. 7/8/14 glyph text의 block contribution은 따라서 70/80/140px이다.

CSS Grid §7.2.1의 cyclic percentage 규칙에 따라 intrinsic height 계산 중 calc minimum의 percentage를 확정 크기로 대입하지 않는다. §11.5.1의 span 길이별 그룹 및 track별 planned increase는 두 span-2 item을 `[34,39,39]`로 정리한다. span-3 item에서는 `140 − 112 − 4 = 24px`가 남아 각 행에 8px씩 더해지므로 max-content Rows는 `[42,47,47]`, content height는 140px이다. cyclic final minimum `10% × 140 + 2 = 16px`은 이 결과를 바꾸지 않고, definite 140px control도 같은 배열이어야 한다.

100px definite control은 final minimum 12px 세 개와 gap 4px 뒤 60px이 남고, track당 20px을 더해 `[32,32,32]`가 된다. span-2 frame은 각각 66px 높이지만 text는 70/80px이고, span-3 frame은 100px 높이지만 text는 140px이다. 이는 content box 아래 40px의 overflow다. Root border-box는 높이 104px이므로 화면 표본 `(7,126)`은 border box 아래 text 내부, `(7,150)`은 text 이후를 가리킨다. 이 좌표의 native 색상은 아직 측정하지 않았고 이후 simulator 구현 검증에서만 판정한다.

## Chrome 관측

Chrome 156에서 Ahem 로드는 성공했다. cyclic/definite 140px 장면 네 개는 모두 Rows `42px 47px 47px`, root border box `14×144px`였고, definite 100px 장면 둘은 Rows `32px 32px 32px`, root `14×104px`였다. forward/reverse에서 kind별 frame과 line rect 개수가 같았다. 작은 장면의 line rect 수는 7/8/14이고, item frame height는 66/66/100px, `scrollHeight`는 70/80/140px이다.

Observation JSON은 원본 DOM 순서, computed track list, root/item frame 및 개별 line rect를 보존한다. Screenshot은 입력의 실제 배치와 100px root 바깥으로 이어지는 text를 보여 주며 pixel color oracle이나 CSS border stroke 판정은 아니다. Browser page의 흰 배경은 앱 native host surface와 다르다.
