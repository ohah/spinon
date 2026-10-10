# C10 Flexbox 계획의 독립 실패 경로 검토

검토 대상은 `plan/c10-flexbox.md`와 `spec/STATUS.md`의 C10.1 제안이다. 기능 코드는 바꾸지 않았다. 서로 다른 실패 경로를 검토하고 계획 문구에 반영한 뒤 현재 계획을 다시 확인했다.

| # | 검토 관점 | 공격 질문과 확인 결과 | 반영·판정 |
| --- | --- | --- | --- |
| 1 | 로드맵 순서 | C09.4가 C10보다 먼저 끝나야 한다고 읽히지 않는가? | C09.4의 C12/C14/C15/C26 선행 조건을 인용하고, 독립 P1 작업인 C10부터 진행한다고 명시했다. C09 상태는 그대로 미완료다. |
| 2 | 기존 C04/S02 범위 중복 | 이미 시험한 row Flex 동작을 C10에서 다시 제품 지원으로 선언하지 않는가? | C10.1을 기존 단일 line/기초 layout의 재구현이 아니라 현재 모델에 없는 line wrapping 추가로 한정했다. C10 전체는 미완료로 둔다. |
| 3 | 기준 자료와 구현 경계 | 표준·기준 브라우저·계산 라이브러리의 권한이 혼동되지 않는가? | W3C Flexbox §5.2/§5.3/§6/§9.2를 동작 기준, 고정 Chromium을 수치 oracle, pinned Taffy 0.14.0을 후보 계산기로 분리했다. |
| 4 | profile·적용 대상 | Block·Flex 이전 profile이 `wrap`을 조용히 받아들이지 않으며, Flex profile의 non-flex box에선 geometry를 바꾸지 않는가? | 지원 computed style은 `RuntimeFlexLayoutV1`·`RuntimeFlexPaintV1`로 한정했다. 그 안에서는 Block 요소의 값을 보존하되 Taffy가 Flex container에서만 적용하는지 대조하고, 그 밖의 profile에서는 실패한다. |
| 5 | CSS 초기값 | 선언을 생략했을 때 브라우저 기본 `nowrap`과 다른 결과가 나오지 않는가? | 기본 `nowrap`과 명시 `nowrap`을 모두 관찰값에 포함했다. |
| 6 | CSS 상속 | 부모 `wrap` 값이 nested flex container에 잘못 상속되지 않는가? | 고정 크기 중첩 fixture로 자식 container의 초기 `nowrap`을 확인하도록 추가했다. |
| 7 | shorthand source order | `flex-flow`와 longhand 순서가 달라도 같은 결과를 만드는가? | shorthand 및 그 뒤 longhand override case를 포함하고 computed style과 geometry를 비교한다. |
| 8 | line collection 입력 | line break 비교에 flex shrink/grow 결과가 섞이지 않는가? | explicit size, `flex-grow:0`, `flex-shrink:0`, 0 margin과 `flex-basis:auto`를 명시해 line 수집 기준을 분리했다. |
| 9 | 자동 최소 크기 | 기본 `min-width:auto`의 내재 크기가 폭을 바꾸지 않는가? | fixture item에 0 min-size와 빈 비텍스트 요소를 사용하고 intrinsic sizing을 C14 후속으로 남겼다. |
| 10 | exact-fit 경계 | 부동소수점 허용 오차가 fit/wrap 판정을 가리지 않는가? | 정수 CSS px exact-fit과 1 CSS px 초과를 별도 case로 고정한다. 허용 오차는 결과 좌표 비교에만 적용한다. |
| 11 | gap 방향 | row와 column의 주축 gap·줄 간 gap을 서로 뒤바꿔 측정하지 않는가? | row에서는 column-gap과 row-gap, column에서는 row-gap과 column-gap을 각각 실제 line 위치에서 관찰하도록 했다. |
| 12 | line 정렬 영향 | 기본 `align-content:stretch`와 `align-items:stretch`가 줄·항목 크기를 바꾸어 wrap 결과를 오염시키지 않는가? | container cross size를 line 크기 합과 gap 합에 맞춰 free space를 0으로 만들고, 기존에 지원하는 `align-items:flex-start`를 명시한다. 비기본 `align-content`는 실패 경계로 남긴다. |
| 13 | row/column available size | column wrap이 indefinite main size 때문에 다르게 계산되지 않는가? | row에는 fixed width, column에는 fixed height를 요구하고, 해당 축 밖의 auto main-size 계산을 제외했다. |
| 14 | empty·single·hidden item | 자식 0개·1개와 `display:none` 자식이 가짜 line이나 간격을 만들지 않는가? | empty/single-item cases와 숨김 subtree frame 0·line 제외 검증을 추가했다. |
| 15 | DOM 순서·`order` | 지원 안 하는 order를 시각 순서만 바꾸어 처리하지 않는가? | C10.1은 DOM order를 유지하고 비기본 `order`는 성공을 거부하도록 했다. |
| 16 | reverse 동작 | `wrap-reverse`나 reverse main axis가 ordinary wrap으로 오인되지 않는가? | 해당 CSS 값은 이번 범위 밖이며 구체적 unsupported diagnostic을 요구한다. |
| 17 | text/replaced intrinsic size | 텍스트·이미지의 측정 공백이 flex algorithm 오차로 오인되지 않는가? | 빈 요소만 사용하고 text/replaced item은 C14/C15 이후로 분리했다. |
| 18 | cascade·author CSS | V8 inline style만 통과하고 연결된 `<style>` 값이 빠지지 않는가? | fixture의 author stylesheet, inline style, shorthand override를 비교 모델에 포함했다. |
| 19 | revision과 stale output | 계산 실패·오래된 revision이 성공 scene/frame처럼 공개되지 않는가? | 기존 source/document/style/environment tuple을 유지하고 unsupported 입력은 전체 실패해야 한다고 완료 조건에 명시했다. |
| 20 | platform·DPR 주장 | simulator screenshot이나 pixel 차이를 CSS geometry 정확성으로 잘못 주장하지 않는가? | frame field를 CSS px로 DPR 1/2에서 독립 비교하고, simulator를 실기기·hardware GPU 성능 증거와 분리했다. |

계획 단계에서 발견한 모호점은 C09.4와 C10의 순서, `nowrap`의 초기·비상속 의미, shorthand override, hidden/empty item, cross-axis free space, profile 밖 진단이었다. 모두 계획에 경계나 관찰 조건으로 반영했다. 코드는 아직 변경하지 않았다.

계획 문구 최종 재확인: line cross-size를 item stretch가 바꾸는 경로를 추가로 배제했고, container/item 모두 `border-box`와 zero border/padding을 고정했다. Flex profile에서 Block 요소에 선언된 wrap 값의 no-op semantics도 따로 분리했다. 수정은 적용 대상·축·cascade·line sizing 관점을 다시 대조했다.

## 2026-10-10 · 기본 runtime profile 경로 재검토

구현 전 코드 경로를 따라가며 처음 계획의 profile 경계가 실제 앱 실행을 포함하는지 다시 공격했다. 기본 V8 GPU host의 `RuntimeCalculationProfile::FlexPaint`는 `RuntimeFlexPaintV1`이 아니라 `RuntimeFlexCustomPropertiesPaintV1` computed style을 사용한다. 따라서 처음 적은 두 profile만 확장하면 단위 시험은 통과해도 실제 앱의 C10.1 fixture는 `flex-wrap`을 소비하지 못한다. 계획을 여섯 runtime Flex profile 전체로 넓혔고, Block 전용 profile은 기존 fail-closed 경계에 남겼다.

| # | 독립 검토 관점 | 실패 가능성 및 확인 | 계획에 반영한 통제 |
| --- | --- | --- | --- |
| 1 | 실제 앱 profile 선택 | C0410 기본 GPU host가 어떤 cascade profile을 만드는가? | 기본 host가 쓰는 `RuntimeFlexCustomPropertiesPaintV1`을 명시적인 완료 조건에 넣었다. |
| 2 | layout 전용 runtime | paint 없이 layout 계산하는 앱 경로에서 값이 사라지지 않는가? | `RuntimeFlexCustomPropertiesV1`도 같은 typed wrap 입력을 가져야 한다. |
| 3 | base layout profile | 직접 호출 가능한 `RuntimeFlexLayoutV1`에서 initial `nowrap`이 보존되는가? | base profile을 여섯 profile 목록과 test matrix에 유지한다. |
| 4 | base paint profile | S04 이후 base paint가 layout 결과와 다른가? | `RuntimeFlexPaintV1`도 동일한 typed projection에 연결한다. |
| 5 | custom property paint | `var()`가 있는 앱 스타일에서 wrap 선언이 preflight나 cascade에서 누락되는가? | custom property paint cascade와 author allowlist를 같은 범위로 수정하도록 계획을 구체화했다. |
| 6 | custom property layout | paint 없는 custom-property cascade가 별도 기본값을 사용하는가? | `RuntimeFlexCustomPropertiesV1`의 computed property·layout projection 일치를 요구한다. |
| 7 | registered property paint | 등록 색상 등 `@property` 사용 시 layout property가 바뀌지 않는가? | registered property paint profile에도 같은 wrap 값을 적용하고 `@property` 자체 결과는 유지한다. |
| 8 | registered property layout | registered custom property 전용 layout 경로를 놓치지 않는가? | `RuntimeFlexRegisteredPropertiesV1`도 matrix에 포함한다. |
| 9 | inline property preflight | CSS 선언은 통과하지만 computed snapshot에는 값이 없는 반쪽 지원이 되는가? | allowlist·computed property 목록·Taffy typed field를 profile별로 함께 비교한다. |
| 10 | author stylesheet gate | `<style>` 선언과 inline 선언이 다른 profile에서 허용되는가? | 여섯 runtime Flex profile의 inline 및 author stylesheet gate를 완료 기준에 명시했다. |
| 11 | Block paint 누출 | 공용 Flex property 목록 확장으로 Block 전용 계산이 flex-wrap을 노출하는가? | Block 전용 computed property 목록을 별도로 유지하고 `flex-wrap` author 입력은 거부하도록 계획에 분리 경계를 추가했다. |
| 12 | Block formatting 누출 | C09 `flow-root`/Block 경로가 wrap을 무시한 채 성공하는가? | `RuntimeBlockFormattingV1`은 C10.1 밖 fail-closed profile로 분명히 구분했다. |
| 13 | static compatibility profile | 기존 C04/C06 static profile의 성공 조건이 넓어지는가? | 여섯 runtime Flex profile만 지원하며 static profile은 그대로 제외한다. |
| 14 | incremental cache property set | profile 재사용 경로가 full cascade와 다른 computed property 집합을 만드는가? | incremental profile property selection도 여섯 profile에서 동일해야 한다. |
| 15 | cache/profile identity | 이전 C05 snapshot이 wrap 없는 입력으로 새 wrap 요청을 만족시키는가? | 기존 profile·revision tuple을 유지하되 profile별 property set과 revision 변화 후 full fallback을 검증한다. |
| 16 | `var()` 값 경로 | custom property로 `flex-flow` 토큰을 만들 때 wrap 값이 확장되는가? | C10.1에서는 shorthand의 현재 지원 문법만 허용하고 custom property 조합의 computed result를 실제 custom profile에서 비교한다. |
| 17 | `@property` 영향 | 등록 속성 이름이나 syntax가 flex-wrap parser와 충돌하는가? | CSS `flex-wrap` longhand만 projection 대상으로 두고 등록 custom property 처리 순서는 그대로 둔다. |
| 18 | 지원 밖 값 진단 | `wrap-reverse`가 profile 확장 때문에 default로 성공하는가? | 모든 runtime Flex profile에서 `wrap-reverse`와 비기본 `order`/`align-content`가 fail closed인지 확인한다. |
| 19 | shorthand cascade | custom property profile에서 `flex-flow`와 longhand 승자가 달라지는가? | 실제 runtime profile에서 shorthand 뒤 longhand override fixture를 실행하도록 완료 조건을 좁혔다. |
| 20 | platform integration과 완료 주장 | Android/iOS 데모가 예전 C05 custom-property renderer를 써서 테스트 결과가 잘못 귀속되는가? | Android API 37 및 iOS 26.2 Simulator가 기본 앱의 실제 사용자 지정 속성 paint profile로 보고한 node frame을 Chromium과 비교해야 완료한다. |

수정 계획 경계는 여섯 runtime Flex profile과 Block 전용 profile의 구분이다. 아직 기능 구현 결과를 완료로 표시하지 않는다.
