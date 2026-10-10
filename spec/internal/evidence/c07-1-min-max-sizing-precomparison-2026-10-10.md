# C07.1 · 최소·최대 크기 구현 전 비교 모델

## 범위

이 문서는 C07의 첫 하위 작업인 물리 축 `min-width`, `max-width`, `min-height`, `max-height`의 레이아웃 동작만 다룬다. 내부 Rust 레이아웃 입력과 고정 Chromium 결과를 비교하기 위한 준비 기록이며 제품 지원 선언이 아니다.

외부 stylesheet URL, 텍스트 intrinsic sizing, `min-content`·`max-content`·`fit-content`, 논리 축 크기, aspect ratio, border 렌더링, 전체 CSSOM은 이 비교에서 제외한다. 빈 박스와 이미 runtime profile에서 지원하는 block/flex 입력만 쓴다.

## 고정 환경과 원본

- 기준 브라우저: Google Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`
- 기준 실행 파일 SHA-256과 Chromium flags: [고정 reference](../../tests/fixtures/css/references/c07-min-max-sizing-v1.json)
- 입력: [HTML fixture](../../tests/fixtures/css/c07/min-max-sizing.html), [35개 관찰 노드 인벤토리](../../tests/fixtures/css/c07/min-max-sizing-inventory.json), [캡처기](../../tools/css-reference/capture-c07-min-max-sizing.mjs)
- viewport: `400 × 1000 CSS px`, DPR `1`과 `2`; locale `en-US`, time zone `UTC`, light, coarse pointer, hover 없음
- 판정: 계산값은 고정 Chromium의 CSS Typed OM 관찰값과 property별로 비교하고, 레이아웃 frame의 `x/y/width/height` 각 축은 최대 절대 오차 `0.5 CSS px` 이내여야 한다. DPR 간 CSS 값과 geometry는 동일해야 한다. 평균 오차로 개별 노드 실패를 가리지 않는다.

## Chromium 사전 관찰

| 입력 | 관찰된 frame 또는 값 | 구현상 의미 |
| --- | --- | --- |
| `width:60px; min-width:90px` | width `90px` | 최소값이 지정 크기를 올린다. |
| `width:120px; max-width:90px` | width `90px` | 최대값이 지정 크기를 낮춘다. |
| `height:20px; min-height:55px` | height `55px` | 세로 최소 크기도 같은 단계에 포함한다. |
| `height:80px; max-height:55px` | height `55px` | 세로 최대 크기를 별도로 검사한다. |
| `min-width:80px; max-width:50px` | width `80px` | 최소값이 최대값보다 크면 최소값이 우선한다. |
| content-box `max-width:100px`, 좌우 padding 각 `10px` | border-box frame width `120px` | content-box 최대 크기는 padding을 뺀 콘텐츠 크기에 적용된다. |
| border-box `max-width:100px`, 좌우 padding 각 `10px` | frame width `100px` | border-box 최대 크기는 padding을 포함한다. |
| border-box `min-width:100px`, 좌우 padding 각 `10px` | frame width `100px` | border-box 최소 크기에 padding이 포함된다. |
| border-box `min-width:10px`, 좌우 padding 각 `12px` | frame width `24px` | 콘텐츠 영역은 음수가 될 수 없어 padding 합계가 지정 크기보다 크면 frame이 padding까지 커진다. |
| 부모 width `200px`, `min-width:50%` | width `100px` | definite containing block의 백분율 최소값을 보존한다. |
| 부모 width `200px`, `max-width:75%` | width `150px` | 백분율 최대값도 별도 투영이 필요하다. |
| 부모 height `120px`, `max-height:50%` | height `60px` | 세로 백분율은 세로 containing-block 크기를 기준으로 한다. |
| `min-width:calc(20px + 15%)` | width `50px` | 이미 제한 지원하는 typed CSS math를 min 값에도 연결한다. |
| `max-width:min(120px, 40%)` | width `80px` | `min()` math의 percentage basis를 유지한다. |
| custom property에서 온 `calc(20px + 20%)` | width `60px` | cascade 승자와 typed math AST가 min 값에 보존된다. |
| min/max `0` 및 음수 literal 선언 | 0은 유효하고 음수 선언은 무시되어 앞선 값 유지 | 0과 선언 단계의 invalid 값은 구별한다. |
| flex basis `100px` 두 개, 부모 `150px`, 각 `min-width:80px` | 각 width `80px`, 합계 `160px` | min constraint는 flex shrink 후에도 지켜진다. |
| 같은 flex 구성, 각 `min-width:0` | 각 width `75px` | 명시적 0 최소값은 shrink를 허용한다. |
| 부모 `200px`, flex-grow 두 항목 각각 `max-width:60px` | 각 width `60px`, 마지막 x `140px` | max constraint와 `space-between` 배치 결과를 함께 확인한다. |
| 빈 flex 자식, `min-width:auto` | 각 width `50px` | 빈 콘텐츠에서의 자동 최소 크기 결과만 관찰했다. 텍스트 기반 intrinsic minimum을 증명하지 않는다. |

캡처 결과는 두 DPR 모두 35개 노드에서 같은 computed observation과 CSS px frame을 반환했다. 원본 JSON에는 fixture, inventory, 캡처 코드와 실행 파일의 SHA-256이 들어 있다.

## 기준 자료와 제한

- [CSS Sizing Module Level 3](https://www.w3.org/TR/css-sizing-3/)의 min/max size, percentage containing block, box-sizing 정의를 동작 기준으로 삼는다. 이 문서의 실험 범위는 해당 표준 전체 적합성을 의미하지 않는다.
- 고정된 Taffy `0.14.0` 소스의 `Style.min_size`·`Style.max_size`는 `Size<LengthPercentageAuto>`이며, 현재 `calc` feature가 켜져 있다. 최종 사용값은 반드시 Chromium fixture와 별도 대조해야 한다.
- Chromium reference는 관찰 oracle이지 규범 원본이 아니다. 표준·Taffy·Chromium 결과가 다르면 차이를 기록하고 작업 범위를 멈춰 재판정한다.
- 첫 캡처에서는 percentage `max-height`가 `800px` 부모를 물려받아 clamp되지 않았다. fixture를 `120px` definite-height 부모 아래로 옮기고 다시 캡처해 기대한 `60px` 기준을 확보했다. 현재 고정 reference는 수정된 HTML 해시와 35개 노드를 기록한다.
