# C07.2 · 테두리 폭 레이아웃 구현 전 비교 모델

## 비교할 질문

테두리 폭이 box geometry에 어떻게 포함되는지, 면별 shorthand/cascade가 어떤 폭을 승자로 선택하는지, border style이 사용 폭을 0으로 만드는 시점, box-sizing·min/max·flex 계산과의 연결을 고정한다. 이 기준은 Chrome 관찰 oracle이며 CSS 전체 적합성 또는 Spinon 구현 결과가 아니다.

## 고정 입력과 환경

- 브라우저: Google Chrome `154.0.8037.98`, Chromium revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`
- 기준 환경: macOS `25.5.0`, `arm64`, Node `v26.7.0`, viewport `400 × 1000 CSS px`, DPR `1`과 `2`, locale `en-US`, timezone `UTC`, light, coarse pointer, hover 없음
- 원본 HTML: `tests/fixtures/css/c07/border-width-layout.html`
- 입력 목록: `tests/fixtures/css/c07/border-width-layout-inventory.json` (`50` 고유 node)
- 캡처기: `tools/css-reference/capture-c07-2-border-width-layout.mjs`; Chromium 세션 helper는 `tools/css-reference/chromium-session.mjs`
- 고정 관찰: `tests/fixtures/css/references/c07-2-border-width-layout-v1.json`
- 입력 digest: inventory `8e3c09f1e6ff000f8afc18a297ed680e02823f4d4b56219d11c3171c8ec99277`; HTML `dd80ce607ee65aa2fd09c7c5d7e8a68c8523615d9386181cc168841104693465`; capture script `95ee5f3ee953f41444b426da761f576127f55f48a3e3b47f1dcebb1cebde34a6`; Chromium helper `b010ada41ed75dc2d67ee25abd4266b17930f1de95fc683eebc5cdab6ad2cdc1`; Chrome executable `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`.
- 인벤토리와 관찰 JSON은 두 DPR에서 각각 50개 node를 보유한다. 캡처기는 viewport·환경·node 순서·필드·CSS computed value·rect를 검증하고 두 DPR의 computed values 및 CSS px rect 일치를 검사한다.

## 고정 관찰

| 입력·관찰 node | Chrome computed 값 및 border-box frame | 구현 전에 고정할 의미 |
| --- | --- | --- |
| 기본값 `default-none` | width `50`, border 면 `0/0/0/0`; frame `50 × 30` | 기본 style none은 폭을 만들지 않는다. |
| `border-width:2px` shorthand `explicit-one-value` | 면 `2/2/2/2`; 지정 content `100 × 30`; frame `104 × 34` | 네 면에 두 배의 폭이 더해진다. |
| 두 값 `2px 4px` / 세 값 `1px 2px 3px` / 네 값 `1px 2px 3px 4px` | 면별 값 각각 `2/4/2/4`, `1/2/3/2`, `1/2/3/4` | CSS shorthand side mapping을 그대로 보존한다. |
| `side-overrides` | border shorthand 뒤 네 longhand로 면 `4/5/6/7`; frame width `112` | shorthand 다음의 면별 cascade winner를 적용한다. |
| `longhand-cascade` | 위/오른/아래/왼 `1/2/5/7`; frame width `109`, height `36` | 하위 longhand가 shorthand의 일부 면만 교체한다. |
| longhand 네 면 뒤 `border:3px solid` `border-shorthand-reset` | 면 `3/3/3/3`; frame `106 × 36` | 뒤의 shorthand는 앞선 면별 폭과 style을 다시 설정한다. |
| 기본 `border-style:solid` 또는 `border-width:medium` | computed 폭 `3px`; frame `106 × 36` | medium 기본 폭의 실제 Chrome 값을 사용한다. |
| `thin` / `medium` / `thick` | 각각 `1px` / `3px` / `5px` | 임의 상수를 별도 추정하지 않고 typed computed 값을 소비한다. |
| `border-style:none` 또는 `hidden`, 선언 폭 `6px` | used/computed 폭 네 면 `0px`; frame `100 × 30` | none과 hidden은 box 기하에 폭을 더하지 않는다. |
| `dashed`, `dotted`, `double`, `groove`, 폭 `4px` | frame `108 × 38` | none/hidden 외에는 시각적 style과 별개로 폭을 반영한다. |
| `ridge`, `inset`, `outset`, 폭 `4px` | 각각 frame `108 × 38` | 나머지 표준 비-none 선 스타일도 폭 계산을 바꾸지 않는다. |
| `transparent` solid, 폭 `3px` | frame `106 × 36` | border color는 layout 폭을 제거하지 않는다. |
| content-box `100 × 40`, padding `10px`, border `2px` | frame `124 × 64` | 각 축에 padding과 양면 border가 더해진다. |
| border-box `100 × 40`, padding `10px`, border `2px` | frame `100 × 40` | 지정 outer size에 padding·border를 포함한다. |
| border-box 지정 `10 × 10`, padding `12px`, border `2px` | 실제 frame `28 × 28` | content box가 음수가 되지 않고 outer frame이 padding·border 합에 맞춰 커진다. |
| border-box `min-width:80px` 또는 `max-width:80px`, padding `5px`, border `2px` | frame width `80` | C07.1 min/max clamp를 테두리가 포함된 box 기준으로 유지한다. |
| content-box `min-width:80px` 또는 `max-width:80px`, padding `5px`, border `2px` | frame width `94` | content-box 제약은 콘텐츠 폭으로 clamp한 뒤 padding·border를 더한다. |
| `calc(1px + 1.5px)` / `var(--edge:3px)` | computed border `2px` / `3px` | 계산식 결과가 Stylo typed computed CSS px 폭으로 정규화된다. |
| 네 면 각각 `calc(1px + .5px)`부터 `calc(4px + .5px)` | computed 면 `1/2/3/4px`; frame `106 × 34` | 계산식 합과 side별 폭 대응을 고정한다. |
| fractional border values `.25`, `.5`, `.75`, `1.25`, `1.5`, `1.75`, `2.25`, `2.5`, `2.75`, `3.5px` | Chrome computed 폭은 순서대로 `1,1,1,1,1,1,2,2,2,3px`; DPR 1·2 동일 | 0이 아닌 폭의 fractional snapping은 이 pinned 환경 관찰을 따른다. 이를 다른 브라우저 보편 결과로 일반화하지 않는다. zero는 `0px`로 남는다. |
| invalid `-1px` 또는 `10%` 다음의 이전 유효 `4px` | computed 폭 `4px`; frame `108 × 38` | 무효 declaration은 앞선 유효 winner를 지우지 않는다. |
| `outline:20px solid` | 기본 frame에 outline 크기가 더해지지 않음 | outline은 C07.2의 border와 다른 paint 범위다. |
| Flex 부모 `120px`; 첫 자식 content basis `100px`와 좌우 border `5px`; 둘째 자식 content basis `100px` | 첫 자식 content width `55`, frame width `65`; 둘째 width `55`, x `65` | flex shrink factor에서 border outer contribution을 포함해 형제 공간을 배분한다. |

Chrome `getComputedStyle()`와 `computedStyleMap()`에서 `calc-width`, `calc-asymmetric`, `var-width`의 `border-top-width`는 각각 `2px`, `1px`, `3px`이며 Typed OM 값 형식은 모두 `CSSUnitValue`다. DPR 2에서 Stylo는 `calc-width` 입력을 `2.5px`로 보존해 Taffy 외곽 폭이 Chrome보다 1 CSS px 커지는 사전 비교 실패를 보였다. 구현은 `max(1, floor(widthCssPx))`를 layout 경계에 적용해 두 DPR을 맞춘다. 따라서 AST는 필요 없지만 Stylo typed scalar를 그대로 쓰지도 않는다.

## 사전 판정 기준

- Capture test는 input file hashes, 고정 Chrome product/revision, node ID uniqueness, 모든 관찰 필드와 50-node 분모, DPR 1·2 동일성을 확인한다.
- 구현 비교는 50개 node 각각의 `x`, `y`, `width`, `height`와 필요한 border resolved value를 대조한다. 필드별 최대 절대 오차 `0.5 CSS px`; 개별 실패는 평균으로 상쇄할 수 없다.
- Chrome observation의 `none`/`hidden`, fractional snapping, `thin/medium/thick`은 pinned build와 환경 한정 사실로 취급한다. Chrome·Stylo 차이를 숨기지 않고 fixture와 오류 진단으로 보존한다.
- 숫자 비교 전 fixture/input/executable digest를 확인한다. 일치하지 않으면 해당 비교 실행은 무효이며 새 기준을 승인 없이 덮어쓰지 않는다.
- 기하 oracle은 HostDocument→Stylo→layout/Taffy 내부 결과를 검증한다. Android/iOS Simulator는 runtime 경로 연결과 WGPU presentation만 별도로 확인하며, 모바일 child-frame 숫자 비교로 승격하지 않는다.

## 실행하지 않은 범위

이 계획 전 비교는 Spinon 구현과의 pass/fail을 아직 주장하지 않는다. Rust DTO/Taffy 결과, Android/iOS runtime 경로, border paint, 실기기, hardware GPU, 공개 CSS 지원은 구현 이후 별도 검증 대상이다.
