# C11.4 anonymous Grid text computed `word-spacing` 사전 비교

## 범위

이 자료는 `RuntimeBlockFlexGridTrackSizingV1`의 기존 anonymous direct-text intrinsic fixture에 computed `word-spacing`을 전달할 후보를 위한 사전 비교다. 일반 CSS admission, glyph paint, WPT, 전체 C11.4/C11의 결과가 아니다.

## 독립 기대값

규범 원본은 [CSS Text Level 3 §7.1 `word-spacing`](https://www.w3.org/TR/css-text-3/#word-spacing-property), [CSS Box Sizing Level 3 §5.1 intrinsic sizes](https://www.w3.org/TR/css-sizing-3/#intrinsic-sizes), [CSS Grid Level 1 2025 CRD §11](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)다. CSS Text는 `normal`을 추가 spacing 0으로 정의하고, computed value를 absolute length로 둔다. 추가 간격은 whitespace processing 뒤 남은 word separator에 적용된다.

고정 Ahem `X`와 U+0020은 `unitsPerEm=1000`, 각각 advance 1000 units다. FontTools로 직접 읽은 결과이며 폰트 SHA-256은 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`이다. 10px Ahem, normal whitespace에서 공백 묶음은 separator 하나로 접히고 line edge 공백은 제거된다. 따라서 문자열의 단어 길이를 `wᵢ`, collapsed separator 수를 `s`, computed spacing을 `d`라 하면 독립 기대값은 다음과 같다.

- min-content track width: `10px × max(wᵢ)`
- max-content track width: `10px × Σ(wᵢ) + s × (10px + d)`
- min-content row height: `10px × 단어 수`; max-content row height: `10px`
- separator가 없는 `XX`는 `word-spacing` 값과 관계없이 `20×10px`이다.

`XX XX`, repeated-space source `XX   XX XX  `, `XX`를 각각 min-/max-content track에서 측정해 4개 cascade value (`normal`, authored `0px`, `-1px`, `1px`)의 24개 고정 장면을 만들었다. Chrome CSSOM은 `normal`과 `0px`를 모두 computed `0px`로 반환했다. 독립 계산표는 [`spec-derived-expectations.json`](./spec-derived-expectations.json)에 있고 [`validate-precomparison.mjs`](./validate-precomparison.mjs)가 24개 browser observation과 비교한다.

## Chrome 156 관측 환경

- Agent Browser 0.31.1, HeadlessChrome `156.0.0.0`, macOS 26.5.1
- viewport `1280×633` CSS px, DPR 1, Ahem `document.fonts.check()` true
- fixture SHA-256: `feed66c8e8ef34b20aca20d91899b3f3369e9346e6b70e95646c79908f3f2d65`
- screenshot SHA-256: `1bb2551c7fb40b1a76af3485ffce16cb5075242d443bc51db129550d5189894d`

결과는 24/24 장면에서 independent expected track columns, rows 및 grid frame과 exact match다. Raw CSSOM/DOMRect 관측은 [`chrome156-observation.json`](./chrome156-observation.json)에, 사람이 확인할 Chrome 화면은 [`chrome156-agent-browser.png`](./chrome156-agent-browser.png)에 있다. 화면은 Chrome oracle 결과이며 Spinon 구현 화면이 아니다.

## 남은 범위

이 사전 비교는 fixed Ahem, horizontal-tb/LTR, normal whitespace, `normal | -1px | 0px | 1px`, anonymous text와 단일 min-/max-content column만 다룬다. `em` 및 다른 CSS length, nested/inline run, writing mode, text justification, arbitrary font/shaping, app runtime, Android/iOS surface pixels, 실기기·hardware GPU 및 WPT는 포함하지 않는다.
