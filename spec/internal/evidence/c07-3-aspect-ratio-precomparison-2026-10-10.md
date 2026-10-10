# C07.3 종횡비 구현 전 비교 기준

## 고정 입력

- 기준: Google Chrome `154.0.8037.98`, CDP product `Chrome/154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`.
- viewport `400×1200` CSS px, DPR 1·2, locale `en-US`, timezone `UTC`, light scheme, coarse pointer, hover 없음.
- fixture: `tests/fixtures/css/c07/aspect-ratio.html`, inventory: `tests/fixtures/css/c07/aspect-ratio-inventory.json`, 캡처 도구: `tools/css-reference/capture-c07-3-aspect-ratio.mjs`.
- 35개의 고유 node를 DPR 1·2로 캡처했다. 그중 30개는 positive geometry 비교 대상이고, 네 standalone·Flex child를 포함한 5개 min/max+ratio 조합은 별도 fail-closed negative 시험 대상이다. positive layout fixture에서는 해당 다섯 위치에 Chrome used size로 고정한 spacer style을 적용해 뒤쪽 node의 y 비교를 보존한다. spacer는 지원 근거로 세지 않는다. 캡처 스크립트는 다른 Chrome revision, 입력 digest, viewport, 환경, ID 순서, 유한하지 않은 rect, DPR 간 차이가 있으면 실패한다. 고정 기준 파일이 있으면 덮어쓰지 않는다.
- 기준 JSON: `tests/fixtures/css/references/c07-3-aspect-ratio-v1.json`.

## 사전 관찰

| 사례 | computed `aspect-ratio` | Chrome rect (CSS px) | 판정할 동작 |
| --- | --- | --- | --- |
| `ratio-16-9-width` | `16 / 9` | `160×90` | width definite, height auto |
| `ratio-empty-auto-sizes` | `16 / 9` | `400×225` | 빈 block, width available, height auto |
| `ratio-definite-both` | `16 / 9` | `160×80` | 두 크기 definite면 ratio로 덮지 않음 |
| `ratio-content-box` | `2 / 1` | outer `120×70` | content `100×50`, padding 포함 |
| `ratio-border-box` | `2 / 1` | outer `120×60` | 지정 border-box 비율 |
| `ratio-content-box-border` | `2 / 1` | outer `124×74` | content-box ratio와 border/padding |
| `ratio-min-height-transfer` | `2 / 1` | `100×80` | min-height가 auto height를 제한 |
| `ratio-max-height-transfer` | `2 / 1` | `160×60` | max-height가 auto height를 제한 |
| `ratio-zero-numerator` | `0 / 1` | `100×0` | degenerate는 auto처럼 선호 비율 없음 |
| `ratio-zero-denominator` | `1 / 0` | `100×0` | degenerate는 auto처럼 선호 비율 없음 |
| `ratio-zero-both` | `0 / 0` | `100×0` | degenerate는 auto처럼 선호 비율 없음 |
| `ratio-var` | `4 / 3` | `120×90` | custom property substitution |
| `ratio-invalid-fallback` | `2 / 1` | `100×50` | 뒤의 invalid negative는 이전 winner 보존 |
| `ratio-not-inherited` | `auto` | `100×0` | 부모의 ratio 비상속 |
| `ratio-flex-row-child` | `2 / 1` | `100×100` | 기본 cross-axis stretch가 preferred ratio를 덮을 수 있음 |
| `ratio-flex-row-start-child` | `2 / 1` | `100×50` | flex-start에서 row child ratio 적용 |
| `ratio-flex-column-child` | `2 / 1` | `100×100` | 기본 stretch의 column 대조 |
| `ratio-flex-column-start-child` | `2 / 1` | `200×100` | flex-start에서 column child ratio 적용 |
| `ratio-auto-explicit`·`ratio-auto-initial` | `auto` | 각각 `100×0` | 초기값과 명시 auto 동등 |

## computed value와 geometry의 비교 경계

Chrome `getComputedStyle(width/height/...)`는 이 fixture에서 `auto`를 resolved px로 반환하는 반면 Stylo snapshot은 계산값 `auto`를 보존한다. 두 문자열은 서로 다른 CSS 단계이므로 비교 oracle로 사용하지 않는다. `computedStyleMap()`의 Typed OM 값 중 `aspect-ratio`와 `box-sizing`만 Stylo computed style과 비교하고, width/height/min/max/padding/border의 실제 사용 결과는 각 node의 `getBoundingClientRect()`와 Spinon frame으로 비교한다. raw `getComputedStyle()` 결과는 진단용으로 JSON에 남긴다.

## 판정과 제한

Rust reference test는 reference와 입력 digest, 정확한 node 목록 및 양 DPR를 확인한 뒤 30개 supported node/frame의 `x/y/width/height` 차이를 각각 계산한다. 각 field 최대 절대 오차는 `0.5 CSS px` 이하여야 한다. 다섯 min/max+ratio observation은 Chrome 결과만 보존하며 layout projection에서 오류가 나야 한다. node 누락, 추가, 중복, Chromium revision 변경 또는 DPR 불일치는 즉시 실패다.

이 precomparison은 고정 Chrome의 관찰을 남긴다. CSS Sizing Level 4 Editor’s Draft의 모든 동작, Stylo→Taffy 일치, WGPU 표시, Android/iOS runtime 실행 또는 실기기 성능을 증명하지 않는다. `auto <ratio>`, non-default min/max+ratio 및 ratio를 표현하지 못하는 입력은 별도 unsupported negative fixture로 확인한다.
