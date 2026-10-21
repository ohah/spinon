# C11.4 직접 text/image mixed multi-span 사전 비교

## 대상과 실행 조건

같은 고정 Grid 안에서 Ahem direct-leaf text와 replaced `img`가 서로 겹치는 span-2 및 span-3 기여를 만드는 네 scene을 비교한다. Chrome 155의 독립 관찰과 CSS Grid 표준에서 도출한 기대값을 구현 전에 고정했다. 이 자료는 Spinon 구현 결과나 WPT 적합성 결과가 아니다.

- 브라우저: Chrome `HeadlessChrome/155.0.0.0`, `MacIntel`, DPR 1
- 준비 조건: Ahem `document.fonts.ready`와 네 image decode 완료
- 이미지 natural size: `24×16`; item의 `height:48px;width:auto` 결과는 `72×48 CSS px`
- 텍스트 intrinsic width: `XX XXXXX` min-content `50px`; 끊을 수 없는 18개의 `X` min-content `180px`
- Grid: definite `220×180px`, 네 개 `min-content` column, 세 개 fixed `60px` row, gap 0
- 표준 기준: [CSS Grid Layout Level 1 §11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items) 및 [공간 배분 절차](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#distribute-extra-space)

## 독립 기대값

span-2 group에서 두 item은 서로 다른 열 범위에 걸린다. text의 50px 기여는 첫 두 열에 25px씩 계획하고, image의 72px 기여는 두 번째와 세 번째 열에 36px씩 계획한다. 표준 알고리즘은 한 span group에서 item별 planned increase를 계산한 뒤 각 열의 가장 큰 증가를 적용하므로 열 크기는 `[25, 36, 36, 0]`이다. DOM 순서를 뒤집어도 같은 값이어야 한다.

span-3 text의 min-content 기여는 180px이다. 앞선 span-2 group 뒤 첫 세 열의 합은 97px이므로 남은 83px을 세 열에 같은 크기로 추가한다. 독립 기대값은 `[52⅔, 63⅔, 63⅔, 0]`이다. 네 번째 열은 이 scene들의 어느 item에도 포함되지 않아 0px이다.

| Scene | Source order | 독립 Columns (CSS px) | Chrome CSSOM 관찰 |
| --- | --- | --- | --- |
| `mixed-text-image-span2-forward` | text span-2, image span-2 | `25, 36, 36, 0` | `25, 36, 36, 0` |
| `mixed-text-image-span2-reversed` | image span-2, text span-2 | `25, 36, 36, 0` | `25, 36, 36, 0` |
| `mixed-text-image-span2-before-span3` | text span-2, image span-2, text span-3 | `52⅔, 63⅔, 63⅔, 0` | `52.6562, 63.6719, 63.6719, 0` |
| `mixed-text-image-span3-before-span2` | text span-3, image span-2, text span-2 | `52⅔, 63⅔, 63⅔, 0` | `52.6562, 63.6719, 63.6719, 0` |

Chrome의 fractional used-track 관찰은 CSSOM의 1/64 CSS px quantization 때문에 독립 기대값과 최대 약 `0.01042 CSS px` 차이가 난다. 구현 비교에서는 Chrome observation과 rational 기대값을 따로 보존하고 각각 `1/64 CSS px` 이내로 검사한다.

## 보존 파일과 재현 입력

- [Chrome observation JSON](./chrome155-observation.json)
- [Agent Browser 화면](./chrome155-agent-browser.png), `1280×804`
- [계획 실패 관점 검토 20개](../../../../plan/c11-4-mixed-text-image-multispan-plan-review-20.md)
- [계획 PR 변경분 검토 20개](./plan-pr-diff-review-20.md)
- [실행 fixture](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-mixed-multispan-precomparison.html)
- Fixture SHA-256: `36fb1d84e3992c38534391e61c0936d260b8d2bc76bd86996019b3b08674c9df`
- Observation SHA-256: `b52f1a360d18f0a1d65ea410f95fc0c179915c22c631d3c3a179be45b0f4e164`
- Screenshot SHA-256: `3924088e74493276660d836b207feeb5194f12664c081ec6756bbddb0836eaed`
- 관찰 당시 저장소 기준 commit: `f3a3a8ac319e25e4d4b360ac58d0d1db8a92cbda`

각 item은 별도의 60px fixed row에 놓이므로 48px image가 다른 item과 세로로 겹치지 않는다. Row sizing은 intrinsic contribution 대상이 아니다.

Chrome screenshot은 기준 브라우저에서의 DOM/CSS 결과만 보여준다. Android/iOS runtime, image texel paint, border stroke, OS compositor pixel, WPT 또는 전체 C11.4 지원 근거로 확대하지 않는다.
