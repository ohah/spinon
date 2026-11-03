# C11.1 parsed Root HTML `grid-auto-rows: calc(px + %)` 사전 비교

이 자료는 제한된 parsed Root HTML Grid calc slice의 구현 전 기준이다. Chrome 156 관측은 독립 oracle observation이며 규범 판정은 CSS Grid와 CSS Values 명세 및 고정 입력의 산술을 따른다.

## 자료

- [고정 HTML fixture](../../../../tests/fixtures/css/c11/root-html-grid-auto-row-calc-precomparison.html)
- [Chrome 156 Agent Browser 관측값](./chrome156-observation.json)
- [독립 기대값과 산술](./expected-values.json)
- [시각 캡처](./chrome156-observation.png)
- [계획](../../../../plan/c11-root-html-grid-auto-row-calc.md)

## 환경과 방법

Agent Browser의 Chrome 156 Headless, viewport `1280×633`, DPR `1`에서 fixture를 연다. `CSS.supports`, computed style, used `gridTemplateRows`, root/item `getBoundingClientRect()`를 수집한다. CSS 외부 의존성은 없다. 캡처는 layout을 사람이 확인하기 위한 시각 자료이며 표면 픽셀 동등성이나 Spinon 동작의 증거가 아니다.

관측 fixture에는 직접 definite CSS-pixel height, 하나의 non-nested row-sparse Grid, 0 padding/border, visible overflow와 빈 direct block items를 쓴다. main list의 두 explicit row 뒤에 필요한 implicit rows 6개가 만들어지도록 item 8개를 둔다. row-gap은 calc percentage basis와 별도인 고정 gutter다.

## 독립 기준과 한계

CSS Grid Layout Module Level 1, 2025-03-26 CRD §§7.2.1, 7.6은 row track의 percentage basis와 implicit track list 순환을 정한다. CSS Values and Units Level 4, 2024-03-12 WD의 math range 규칙에 따라 non-negative track range를 넘는 negative result는 0으로 clamp한다. Definite 100px block-size에서 `10% + 2px = 12px`, `25% = 25px`, `120% + 1px = 121px`; definite zero basis에서는 `10% + 5px = 5px`다. Row gap은 height basis에서 차감하지 않는다.

이 사전 비교는 고정된 단순 grammar만 다룬다. calc grammar 전체, intrinsic sizing, indefinite/cyclic percentage sizing, 일반 CSS admission, WPT 적합성, 플랫폼 runtime, draw/present, GPU surface pixel 및 C11 완료를 증명하지 않는다.
