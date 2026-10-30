# C11.4 anonymous Grid text `white-space:break-spaces` 사전 비교

## 판정 범위

이 자료는 새 제한 fixture 계획을 위한 CSS Text 규범 계산과 Chrome 관측이다. Spinon 구현, WPT 실행, 앱 runtime 또는 일반 CSS 지원을 검증하지 않는다. 고정 HTML은 [fixture source](../../../../tests/fixtures/css/c11/anonymous-grid-text-break-spaces-precomparison.html)이고 독립 기대값과 브라우저 관측은 각각 [expected-values.json](./expected-values.json), [chrome-observation-agent-browser.json](./chrome-observation-agent-browser.json)에 분리했다.

## 규범과 계산

[CSS Text Module Level 4 §3 white-space](https://www.w3.org/TR/css-text-4/#white-space-property)의 informative summary는 `break-spaces`의 preserved end-of-line spaces를 wrap 대상으로 구분하며, section 4 white-space processing은 이를 collapsing/hanging 단계와 연계한다. `pre-wrap`은 line-end spaces를 hang하지만 `break-spaces`는 wrap하므로 둘은 서로 대체할 수 없다. [CSS Grid Layout Level 1 §6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-items)는 공백뿐인 direct text sequence를 anonymous Grid item으로 렌더링하지 않는다.

Ahem 10px에서 ASCII `X`와 U+0020의 advance는 각각 10 CSS px다. `XX  XX`의 max-content는 20+10+10+20=60px이다. `break-spaces`는 각 보존 공백 뒤에 wrap opportunity를 두므로 가장 긴 unbreakable 조각은 `XX ` 30px이며, 같은 source의 min-content는 30px이다. `pre-wrap` control은 줄 끝 공백이 hang하므로 min-content 20px이다. 25px track에서 `XX  XX`는 30px인 `XX `와 ` XX` 두 조각으로 나뉘어 각 줄이 5px 넘친다. 강제 LF 앞의 보존 공백은 hang하지 않으므로 `XX  \nXX`의 max-content columns는 40px이고, min-content track 30px에서는 공백이 별도 줄이 되어 root 높이가 30px이다. 독립 기대값 전체는 [JSON](./expected-values.json)에 있다.

## Agent Browser 관측

Agent Browser 0.31.1 / HeadlessChrome 156.0.8078.76에서 viewport `1280×633`, DPR 1로 실행했다. Ahem face 상태는 `loaded`이고 `document.fonts.check('10px Ahem')`도 true였다. `break-spaces` 장면의 computed `white-space-collapse`는 `break-spaces`, `text-wrap-mode`는 `wrap`이다. Chrome의 모든 Grid columns/rows와 root frame은 독립 계산과 일치했다. [관측 JSON](./chrome-observation-agent-browser.json)에는 각 text Range fragment와 source index가 포함된다. [screenshot](./chrome-156-agent-browser.png)은 layout과 fragment 위치를 보여 주며 glyph raster 또는 GPU 표면 픽셀을 증명하지 않는다.

고정 HTML fixture SHA-256은 `f25ab1d2358090cc933e8b77a57f02277c5dcf0439e3feb6e70b55efc77e514f`, observation JSON은 `b2a3ac54b71c7d09f68a1a1c693a464d91c1f9a89d36562a96d96d760791db68`, screenshot은 `831c7eddd84f9330f5b7f5fb0ec886728d0ce5fde5e7a03be6e641c7264e7ff7`이다. Ahem font SHA-256은 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`이다.

고정 입력은 Ahem 10px, horizontal-tb/LTR, 직접 text와 U+0020/LF만 사용한다. CSS shorthand 계산, 일반 authored CSS admission, `tab-size`, 임의 font/script, DOM root HTML 지원, Spinon runtime, WPT, hardware GPU, 실기기 및 전체 C11.4는 이 사전 비교 범위 밖이다.
