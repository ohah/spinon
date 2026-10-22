# C11.4 cyclic `minmax(calc())` two-axis spanning text 사전 비교

## 판정 질문

한 direct Ahem text Grid item이 두 column과 두 row를 동시에 span할 때, column sizing이 정한 inline available size에 따라 text line count와 row intrinsic contribution이 달라지는지 확인한다. 기존 C11.4 fixture는 cyclic `minmax(calc())` minimum, column span group, row span group을 각각 다뤘지만 한 item의 양축 span과 열→행 text-wrap 의존성은 함께 확인하지 않았다.

## 고정 입력

- fixture: [`grid-cyclic-minmax-calc-two-axis-spanning-text-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-cyclic-minmax-calc-two-axis-spanning-text-c114-precomparison.html)
- 한 `div` Grid item이 두 explicit columns와 두 explicit rows를 모두 span한다. Root의 양축 track은 `minmax(calc(10% + 2px), max-content)` 두 개이고 두 gap은 2px이다.
- Item에는 Ahem `aaaaaa`, `font-size:10px`, `line-height:10px`, `word-break:break-all`, `min-width:auto`, `min-height:auto`를 고정했다. Ahem bytes SHA-256은 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`이다.
- Scene은 `min-content` width, `max-content` width, definite 10px width, definite 60px width다. 네 root 모두 `height:max-content`, 2px border, 0 padding을 사용한다.

## 독립 기대값

CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.2.1은 track percentage를 intrinsic grid-container size를 계산할 때 `auto`처럼 취급한 뒤 그 resulting size로 resolve한다. §11.1은 columns를 먼저 sizing하고, 그 column sizes로 block contribution이 필요한 item의 inline available space를 정한다. §11.5.1은 span item contribution에서 gap을 제외하고 spanning track들에 분배한다. [고정 W3C 문서](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)를 기준으로 계산했다.

로컬 Ahem의 각 `a`는 10×10px이다. `word-break:break-all`에서 10px available width는 여섯 줄, 60px available width는 한 줄을 만든다. 두 개의 max-content track을 span하는 contribution에서 2px gap을 먼저 빼고 나누면:

1. min-content inline basis 10px은 Columns `[4,4]`와 10px content width를 만든다. 열 minimum `10% + 2px = 3px`은 각 used track보다 작다.
2. max-content inline basis 60px은 Columns `[29,29]`와 60px content width를 만든다. 열 minimum `10% + 2px = 8px`은 각 used track보다 작다.
3. Columns `[4,4]`로 배치한 text는 10px width에서 여섯 줄·60px block contribution을 낸다. Rows는 `[29,29]`, content height 60px이고 cyclic row minimum은 `8px`이다.
4. Columns `[29,29]`로 배치한 text는 60px width에서 한 줄·10px block contribution을 낸다. Rows는 `[4,4]`, content height 10px이고 cyclic row minimum은 `3px`이다.
5. Definite 10px·60px controls는 해당 intrinsic width scene의 Columns, line count, Rows를 각각 재현한다. Border-box root size는 content size에 각 방향 4px을 더한다.

전체 수치는 [`spec-derived-expectations.json`](./spec-derived-expectations.json)이다. Chrome 관측에서 기대값을 역산하지 않았다.

## Chrome 관측

- Agent Browser `0.31.1`, `HeadlessChrome/156.0.0.0`, macOS arm64, viewport `1280×633`, DPR 1.
- `document.fonts.status=loaded`, `document.fonts.check("10px Ahem")=true`.
- 네 scene 모두 CSSOM used Columns/Rows, root/item rect, line Range rects와 scroll size가 독립 기대값과 일치했다. 원본은 [`chrome156-observation.json`](./chrome156-observation.json), 전체 화면은 [`chrome-agent-browser-pass02.png`](./chrome-agent-browser-pass02.png)이다.
- Screenshot은 CSS box/background를 보여준다. 이 자료는 Spinon glyph paint나 mobile surface를 증명하지 않는다.

첫 탐색 입력에서 `min-width:0;min-height:0`을 썼을 때 min-content scene의 root content width가 2px까지 줄고 10px Range line box가 밖으로 넘쳤다. 이를 잘못된 고정 입력으로 판정해 final fixture는 두 automatic minimum을 명시한다. 원본 값은 [거부된 탐색 관측](./min-width-zero-counterexample.json)과 [화면](./min-width-zero-counterexample-agent-browser.png)에 보존했다. 이 exploratory 결과는 최종 acceptance matrix에 포함하지 않는다.

## Hashes

| File | SHA-256 |
| --- | --- |
| HTML fixture | `26d41488ed7d273ff3bb29ada4486eefc72b40e412ddf585215fbfb52519b4ca` |
| Ahem font | `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448` |
| Chrome observation | `53524fe5b53479e32d14e6dc9aecb9ff22ab3cf476408216abfb9131ec2db533` |
| Accepted Chrome screenshot | `d33d1af4dad514103f1920c2b5978211060f38309ee8af6a81391b39df92bcfc` |
| Rejected `min-width:0` observation | `cd496143caadf5d2f9ad153fcc4e66de2a48d315f2ea715eebf4cbe98300dd93` |
| Rejected exploratory screenshot | `858773d28b98357d96590ca6aa25303f2a32e15287b24b382b173263e65a1969` |
